// Shared generated C macro support.

#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Pg_abi_values, <<c::expression::CRecord<crate::__pgrx_c_bindings::Pg_abi_values> as c::expression::OrdinaryField<Field_namedatalen>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).namedatalen) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Pg_abi_values, float8byval) == 16);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_float8byval>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Pg_abi_values>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Pg_abi_values, <<c::expression::CRecord<crate::__pgrx_c_bindings::Pg_abi_values> as c::expression::OrdinaryField<Field_float8byval>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).float8byval) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Pg_magic_struct>() == 72);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Pg_magic_struct>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SharedInvalRelSyncMsg>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SharedInvalRelSyncMsg>() == 4);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SharedInvalRelSyncMsg {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SharedInvalRelSyncMsg, id) == 0);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SharedInvalRelSyncMsg>
{
    type Member = c::CSignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SharedInvalRelSyncMsg, <<c::expression::CRecord<crate::__pgrx_c_bindings::SharedInvalRelSyncMsg> as c::expression::OrdinaryField<Field_id>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).id) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SharedInvalRelSyncMsg, dbId) == 4);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dbId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SharedInvalRelSyncMsg>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SharedInvalRelSyncMsg, <<c::expression::CRecord<crate::__pgrx_c_bindings::SharedInvalRelSyncMsg> as c::expression::OrdinaryField<Field_dbId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dbId) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SharedInvalRelSyncMsg, relid) == 8);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SharedInvalRelSyncMsg>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SharedInvalRelSyncMsg, <<c::expression::CRecord<crate::__pgrx_c_bindings::SharedInvalRelSyncMsg> as c::expression::OrdinaryField<Field_relid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relid) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_procno;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::WalRcvData>() == 2248);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::WalRcvData>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WalRcvData, procno) == 0);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_procno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WalRcvData, <<c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvData> as c::expression::OrdinaryField<Field_procno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).procno) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WalRcvData, pid) == 4);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WalRcvData, walRcvState) == 8);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_walRcvState>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvData>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_WalRcvState,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WalRcvData, mutex) == 2226);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mutex>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvData>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 2226;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WalRcvData, force_reply) == 2240);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_force_reply>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 2240;
}
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::relopt_parse_elt>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::relopt_parse_elt>() == 8);
};
#[cfg(feature = "pg18")]
#[doc(hidden)]
pub struct Field_isset_offset;
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::relopt_parse_elt, isset_offset) == 16
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isset_offset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::relopt_parse_elt>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg18")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::relopt_parse_elt, <<c::expression::CRecord<crate::__pgrx_c_bindings::relopt_parse_elt> as c::expression::OrdinaryField<Field_isset_offset>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isset_offset) };
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BufferDesc>() == 64);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BufferDesc>() == 4);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CachedFunctionHashKey>() == 440);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CachedFunctionHashKey>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CachedFunctionHashKey {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedFunctionHashKey, funcOid) == 0);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_funcOid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedFunctionHashKey>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CachedFunctionHashKey, <<c::expression::CRecord<crate::__pgrx_c_bindings::CachedFunctionHashKey> as c::expression::OrdinaryField<Field_funcOid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).funcOid) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedFunctionHashKey, isTrigger) == 4
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isTrigger>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedFunctionHashKey>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CachedFunctionHashKey, <<c::expression::CRecord<crate::__pgrx_c_bindings::CachedFunctionHashKey> as c::expression::OrdinaryField<Field_isTrigger>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isTrigger) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedFunctionHashKey, isEventTrigger) == 5
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isEventTrigger>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedFunctionHashKey>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 5;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CachedFunctionHashKey, <<c::expression::CRecord<crate::__pgrx_c_bindings::CachedFunctionHashKey> as c::expression::OrdinaryField<Field_isEventTrigger>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isEventTrigger) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_cacheEntrySize;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedFunctionHashKey, cacheEntrySize) == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cacheEntrySize>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedFunctionHashKey>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CachedFunctionHashKey, <<c::expression::CRecord<crate::__pgrx_c_bindings::CachedFunctionHashKey> as c::expression::OrdinaryField<Field_cacheEntrySize>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cacheEntrySize) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedFunctionHashKey, trigOid) == 16
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_trigOid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedFunctionHashKey>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CachedFunctionHashKey, <<c::expression::CRecord<crate::__pgrx_c_bindings::CachedFunctionHashKey> as c::expression::OrdinaryField<Field_trigOid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).trigOid) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedFunctionHashKey, inputCollation) == 20
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inputCollation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedFunctionHashKey>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CachedFunctionHashKey, <<c::expression::CRecord<crate::__pgrx_c_bindings::CachedFunctionHashKey> as c::expression::OrdinaryField<Field_inputCollation>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).inputCollation) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedFunctionHashKey, nargs) == 24);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nargs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedFunctionHashKey>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CachedFunctionHashKey, <<c::expression::CRecord<crate::__pgrx_c_bindings::CachedFunctionHashKey> as c::expression::OrdinaryField<Field_nargs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nargs) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inhcount>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ColumnDef>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ColumnDef, is_local) == 34);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_local>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ColumnDef>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 34;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ColumnDef, is_not_null) == 35);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_not_null>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ColumnDef>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 35;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ColumnDef, is_from_type) == 36);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_from_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ColumnDef>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ColumnDef, storage) == 37);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_storage>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ColumnDef>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 37;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::FormData_pg_attribute>() == 100);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::FormData_pg_attribute>() == 4);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, atttypmod) == 76
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_atttypmod>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 76;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attndims>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, attbyval) == 82
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attbyval>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 82;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, attalign) == 83
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attalign>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 83;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, attstorage) == 84
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attstorage>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 84;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, attcompression) == 85
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attcompression>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 85;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, attnotnull) == 86
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attnotnull>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 86;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, atthasdef) == 87
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_atthasdef>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 87;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, atthasmissing) == 88
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_atthasmissing>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, attidentity) == 89
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attidentity>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 89;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, attgenerated) == 90
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attgenerated>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 90;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, attisdropped) == 91
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attisdropped>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 91;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, attislocal) == 92
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attislocal>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 92;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, attinhcount) == 94
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attinhcount>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 94;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, attcollation) == 96
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attcollation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_ii_WithoutOverlaps;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexInfo, ii_WithoutOverlaps) == 168
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ii_WithoutOverlaps>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 168;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexInfo,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexInfo> as c::expression::OrdinaryField<
        Field_ii_WithoutOverlaps,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ii_WithoutOverlaps) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexInfo, ii_ParallelWorkers) == 172
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ii_ParallelWorkers>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 172;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexInfo, ii_Am) == 176);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ii_Am>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 176;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_iswithoutoverlaps;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexStmt, iswithoutoverlaps) == 108);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_iswithoutoverlaps>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 108;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt> as c::expression::OrdinaryField<
        Field_iswithoutoverlaps,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).iswithoutoverlaps) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexStmt, deferrable) == 109);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_deferrable>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 109;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexStmt, initdeferred) == 110);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_initdeferred>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 110;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexStmt, transformed) == 111);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_transformed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 111;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexStmt, concurrent) == 112);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_concurrent>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexStmt, if_not_exists) == 113);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_if_not_exists>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 113;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexStmt, reset_default_tblspc) == 114
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_reset_default_tblspc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 114;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_wait_on_fd_before_close;
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::IoMethodOps>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::IoMethodOps>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::IoMethodOps {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IoMethodOps, wait_on_fd_before_close) == 0
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wait_on_fd_before_close>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IoMethodOps>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IoMethodOps, <<c::expression::CRecord<crate::__pgrx_c_bindings::IoMethodOps> as c::expression::OrdinaryField<Field_wait_on_fd_before_close>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wait_on_fd_before_close) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TableAmRoutine>() == 360);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TableAmRoutine>() == 8);
};
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::VacuumParams>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::VacuumParams>() == 8);
};
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacuumParams, nworkers) == 48);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nworkers>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacuumParams>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::WalUsage>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::WalUsage>() == 8);
};
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WalUsage, wal_buffers_full) == 24);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wal_buffers_full>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalUsage>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::WalUsage,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::WalUsage> as c::expression::OrdinaryField<
        Field_wal_buffers_full,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wal_buffers_full) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_strxfrm_is_safe;
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::collate_methods>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::collate_methods>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::collate_methods {}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::collate_methods, strxfrm_is_safe) == 24
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_strxfrm_is_safe>
    for c::expression::CRecord<crate::__pgrx_c_bindings::collate_methods>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::collate_methods, <<c::expression::CRecord<crate::__pgrx_c_bindings::collate_methods> as c::expression::OrdinaryField<Field_strxfrm_is_safe>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).strxfrm_is_safe) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ATAlterConstraint>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ATAlterConstraint>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ATAlterConstraint {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ATAlterConstraint, type_) == 0);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ATAlterConstraint>
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
const _: Projection<crate::__pgrx_c_bindings::ATAlterConstraint, <<c::expression::CRecord<crate::__pgrx_c_bindings::ATAlterConstraint> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_alterEnforceability;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ATAlterConstraint, alterEnforceability) == 16
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_alterEnforceability>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ATAlterConstraint>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ATAlterConstraint, <<c::expression::CRecord<crate::__pgrx_c_bindings::ATAlterConstraint> as c::expression::OrdinaryField<Field_alterEnforceability>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).alterEnforceability) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_is_enforced;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ATAlterConstraint, is_enforced) == 17
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_enforced>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ATAlterConstraint>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 17;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ATAlterConstraint, <<c::expression::CRecord<crate::__pgrx_c_bindings::ATAlterConstraint> as c::expression::OrdinaryField<Field_is_enforced>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_enforced) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_alterDeferrability;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ATAlterConstraint, alterDeferrability) == 18
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_alterDeferrability>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ATAlterConstraint>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 18;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ATAlterConstraint, <<c::expression::CRecord<crate::__pgrx_c_bindings::ATAlterConstraint> as c::expression::OrdinaryField<Field_alterDeferrability>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).alterDeferrability) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ATAlterConstraint, deferrable) == 19);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_deferrable>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ATAlterConstraint>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 19;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ATAlterConstraint, <<c::expression::CRecord<crate::__pgrx_c_bindings::ATAlterConstraint> as c::expression::OrdinaryField<Field_deferrable>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).deferrable) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ATAlterConstraint, initdeferred) == 20
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_initdeferred>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ATAlterConstraint>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ATAlterConstraint, <<c::expression::CRecord<crate::__pgrx_c_bindings::ATAlterConstraint> as c::expression::OrdinaryField<Field_initdeferred>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).initdeferred) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_alterInheritability;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ATAlterConstraint, alterInheritability) == 21
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_alterInheritability>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ATAlterConstraint>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 21;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ATAlterConstraint, <<c::expression::CRecord<crate::__pgrx_c_bindings::ATAlterConstraint> as c::expression::OrdinaryField<Field_alterInheritability>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).alterInheritability) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_noinherit;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ATAlterConstraint, noinherit) == 22);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_noinherit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ATAlterConstraint>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 22;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ATAlterConstraint, <<c::expression::CRecord<crate::__pgrx_c_bindings::ATAlterConstraint> as c::expression::OrdinaryField<Field_noinherit>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).noinherit) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::A_ArrayExpr>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::A_ArrayExpr>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_list_start;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::A_ArrayExpr, list_start) == 16);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_list_start>
    for c::expression::CRecord<crate::__pgrx_c_bindings::A_ArrayExpr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::A_ArrayExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::A_ArrayExpr> as c::expression::OrdinaryField<Field_list_start>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).list_start) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_list_end;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::A_ArrayExpr, list_end) == 20);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_list_end>
    for c::expression::CRecord<crate::__pgrx_c_bindings::A_ArrayExpr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::A_ArrayExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::A_ArrayExpr> as c::expression::OrdinaryField<Field_list_end>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).list_end) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::A_ArrayExpr, location) == 24);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::A_ArrayExpr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::A_Expr>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::A_Expr>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_rexpr_list_start;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::A_Expr, rexpr_list_start) == 32);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rexpr_list_start>
    for c::expression::CRecord<crate::__pgrx_c_bindings::A_Expr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::A_Expr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::A_Expr> as c::expression::OrdinaryField<
        Field_rexpr_list_start,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rexpr_list_start) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_rexpr_list_end;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::A_Expr, rexpr_list_end) == 36);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rexpr_list_end>
    for c::expression::CRecord<crate::__pgrx_c_bindings::A_Expr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::A_Expr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::A_Expr> as c::expression::OrdinaryField<
        Field_rexpr_list_end,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rexpr_list_end) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::A_Expr, location) == 40);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::A_Expr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AggPath>() == 128);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AggPath>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggPath, aggstrategy) == 88);
#[cfg(any(feature = "pg18", feature = "pg19"))]
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
    const OFFSET: usize = 88;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggPath, aggsplit) == 92);
#[cfg(any(feature = "pg18", feature = "pg19"))]
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
    const OFFSET: usize = 92;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggPath, transitionSpace) == 104);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_transitionSpace>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggPath>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggPath, groupClause) == 112);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_groupClause>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggPath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggPath, qual) == 120);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_qual>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggPath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggState, hash_batches) == 488);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash_batches>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 488;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggState, hash_ever_spilled) == 496);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash_ever_spilled>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 496;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggState, hash_spill_mode) == 497);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash_spill_mode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 497;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggState, hash_mem_limit) == 504);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash_mem_limit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggState>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 504;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggState, hash_ngroups_limit) == 512);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash_ngroups_limit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggState>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 512;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AggState, hash_planned_partitions) == 520
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash_planned_partitions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 520;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggState, hash_mem_peak) == 536);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash_mem_peak>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggState>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 536;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AggState, hash_ngroups_current) == 544
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash_ngroups_current>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggState>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 544;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggState, hash_disk_used) == 552);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash_disk_used>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggState>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 552;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggState, hash_batches_used) == 560);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash_batches_used>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 560;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_part_prune_index;
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Append, part_prune_index) == 128);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_part_prune_index>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Append>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Append,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Append> as c::expression::OrdinaryField<
        Field_part_prune_index,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).part_prune_index) };
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AppendPath>() == 104);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AppendPath>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AppendPath, subpaths) == 80);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_subpaths>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AppendPath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AppendPath, first_partial_path) == 88
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_first_partial_path>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AppendPath>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ArrayExpr>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ArrayExpr>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ArrayExpr, list_start) == 28);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_list_start>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ArrayExpr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ArrayExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ArrayExpr> as c::expression::OrdinaryField<
        Field_list_start,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).list_start) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ArrayExpr, list_end) == 32);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_list_end>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ArrayExpr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ArrayExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ArrayExpr> as c::expression::OrdinaryField<
        Field_list_end,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).list_end) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ArrayExpr, location) == 36);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ArrayExpr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AutoVacOpts>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AutoVacOpts>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_vacuum_max_threshold;
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, vacuum_max_threshold) == 8
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vacuum_max_threshold>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AutoVacOpts, <<c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts> as c::expression::OrdinaryField<Field_vacuum_max_threshold>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).vacuum_max_threshold) };
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, vacuum_ins_threshold) == 12
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vacuum_ins_threshold>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, analyze_threshold) == 16
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_analyze_threshold>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, vacuum_cost_limit) == 20
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vacuum_cost_limit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, freeze_min_age) == 24);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_freeze_min_age>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, freeze_max_age) == 28);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_freeze_max_age>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, freeze_table_age) == 32);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_freeze_table_age>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, multixact_freeze_min_age) == 36
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_multixact_freeze_min_age>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, multixact_freeze_max_age) == 40
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_multixact_freeze_max_age>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, multixact_freeze_table_age)
        == 44
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_multixact_freeze_table_age>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, log_min_duration) == 48);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_log_min_duration>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BTArrayKeyInfo>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BTArrayKeyInfo>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTArrayKeyInfo, num_elems) == 4);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_elems>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTArrayKeyInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTArrayKeyInfo, cur_elem) == 16);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cur_elem>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTArrayKeyInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTArrayKeyInfo, attlen) == 20);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attlen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTArrayKeyInfo>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTArrayKeyInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTArrayKeyInfo> as c::expression::OrdinaryField<Field_attlen>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attlen) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTArrayKeyInfo, attbyval) == 22);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attbyval>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTArrayKeyInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 22;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTArrayKeyInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTArrayKeyInfo> as c::expression::OrdinaryField<Field_attbyval>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attbyval) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_null_elem;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTArrayKeyInfo, null_elem) == 23);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_null_elem>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTArrayKeyInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 23;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTArrayKeyInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTArrayKeyInfo> as c::expression::OrdinaryField<Field_null_elem>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).null_elem) };
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTReadPageState, minoff) == 0);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_minoff>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTReadPageState>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTReadPageState, maxoff) == 2);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_maxoff>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTReadPageState>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 2;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTReadPageState, page) == 16);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_page>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTReadPageState>
{
    type Member = c::expression::CPointer<
        c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg18")]
#[doc(hidden)]
pub struct Field_firstpage;
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTReadPageState, firstpage) == 24);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_firstpage>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTReadPageState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg18")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTReadPageState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTReadPageState> as c::expression::OrdinaryField<Field_firstpage>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).firstpage) };
#[cfg(feature = "pg18")]
#[doc(hidden)]
pub struct Field_forcenonrequired;
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BTReadPageState, forcenonrequired) == 25
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_forcenonrequired>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTReadPageState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 25;
}
#[cfg(feature = "pg18")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTReadPageState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTReadPageState> as c::expression::OrdinaryField<Field_forcenonrequired>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).forcenonrequired) };
#[cfg(feature = "pg18")]
#[doc(hidden)]
pub struct Field_startikey;
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTReadPageState, startikey) == 28);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_startikey>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTReadPageState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(feature = "pg18")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTReadPageState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTReadPageState> as c::expression::OrdinaryField<Field_startikey>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).startikey) };
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTReadPageState, rechecks) == 38);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rechecks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTReadPageState>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 38;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BTReadPageState, targetdistance) == 40
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_targetdistance>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTReadPageState>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(feature = "pg18")]
#[doc(hidden)]
pub struct Field_nskipadvances;
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BTReadPageState, nskipadvances) == 42
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nskipadvances>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTReadPageState>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 42;
}
#[cfg(feature = "pg18")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTReadPageState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTReadPageState> as c::expression::OrdinaryField<Field_nskipadvances>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nskipadvances) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_skipScan;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanOpaqueData, skipScan) == 20);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_skipScan>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTScanOpaqueData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTScanOpaqueData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTScanOpaqueData> as c::expression::OrdinaryField<Field_skipScan>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).skipScan) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanOpaqueData, needPrimScan) == 21
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_needPrimScan>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTScanOpaqueData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 21;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanOpaqueData, scanBehind) == 22);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_scanBehind>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTScanOpaqueData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 22;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_oppositeDirCheck;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanOpaqueData, oppositeDirCheck) == 23
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oppositeDirCheck>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTScanOpaqueData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 23;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTScanOpaqueData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTScanOpaqueData> as c::expression::OrdinaryField<Field_oppositeDirCheck>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oppositeDirCheck) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_dropPin;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanOpaqueData, dropPin) == 60);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dropPin>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTScanOpaqueData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 60;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTScanOpaqueData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTScanOpaqueData> as c::expression::OrdinaryField<Field_dropPin>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dropPin) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanPosData, currPage) == 4);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_currPage>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTScanPosData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanPosData, prevPage) == 8);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_prevPage>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTScanPosData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTScanPosData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTScanPosData> as c::expression::OrdinaryField<Field_prevPage>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).prevPage) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanPosData, nextPage) == 12);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nextPage>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTScanPosData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanPosData, lsn) == 16);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTScanPosData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanPosData, dir) == 24);
#[cfg(any(feature = "pg18", feature = "pg19"))]
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
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanPosData, moreLeft) == 32);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_moreLeft>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTScanPosData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanPosData, moreRight) == 33);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_moreRight>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTScanPosData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 33;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BackendStartupData>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BackendStartupData>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_socket_created;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BackendStartupData, socket_created) == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_socket_created>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BackendStartupData>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BackendStartupData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BackendStartupData> as c::expression::OrdinaryField<Field_socket_created>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).socket_created) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_fork_started;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BackendStartupData, fork_started) == 16
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fork_started>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BackendStartupData>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BackendStartupData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BackendStartupData> as c::expression::OrdinaryField<Field_fork_started>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fork_started) };
#[cfg(feature = "pg18")]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::BitmapHeapScanInstrumentation>() == 16
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::BitmapHeapScanInstrumentation>() == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BitmapHeapScanInstrumentation {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BitmapHeapScanInstrumentation, exact_pages)
        == 0
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_exact_pages>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BitmapHeapScanInstrumentation>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BitmapHeapScanInstrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::BitmapHeapScanInstrumentation> as c::expression::OrdinaryField<Field_exact_pages>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).exact_pages) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BitmapHeapScanInstrumentation, lossy_pages)
        == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lossy_pages>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BitmapHeapScanInstrumentation>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BitmapHeapScanInstrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::BitmapHeapScanInstrumentation> as c::expression::OrdinaryField<Field_lossy_pages>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lossy_pages) };
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BitmapHeapScanState>() == 288);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BitmapHeapScanState>() == 8);
};
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BitmapHeapScanState, initialized) == 256
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_initialized>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BitmapHeapScanState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 256;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BitmapHeapScanState, recheck) == 280);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_recheck>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BitmapHeapScanState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 280;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BitmapHeapScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BitmapHeapScanState> as c::expression::OrdinaryField<Field_recheck>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).recheck) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BitmapIndexScanState>() == 320);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BitmapIndexScanState>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_current_blocknum;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BlockRangeReadStreamPrivate>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BlockRangeReadStreamPrivate>() == 4);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BlockRangeReadStreamPrivate {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::BlockRangeReadStreamPrivate,
        current_blocknum
    ) == 0
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_current_blocknum>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BlockRangeReadStreamPrivate>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BlockRangeReadStreamPrivate, <<c::expression::CRecord<crate::__pgrx_c_bindings::BlockRangeReadStreamPrivate> as c::expression::OrdinaryField<Field_current_blocknum>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).current_blocknum) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_last_exclusive;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BlockRangeReadStreamPrivate, last_exclusive)
        == 4
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_last_exclusive>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BlockRangeReadStreamPrivate>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BlockRangeReadStreamPrivate, <<c::expression::CRecord<crate::__pgrx_c_bindings::BlockRangeReadStreamPrivate> as c::expression::OrdinaryField<Field_last_exclusive>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).last_exclusive) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CachedFunction>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CachedFunction>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CachedFunction {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedFunction, fn_xmin) == 8);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fn_xmin>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedFunction>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CachedFunction, <<c::expression::CRecord<crate::__pgrx_c_bindings::CachedFunction> as c::expression::OrdinaryField<Field_fn_xmin>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fn_xmin) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedFunction, use_count) == 32);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_use_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedFunction>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CachedFunction, <<c::expression::CRecord<crate::__pgrx_c_bindings::CachedFunction> as c::expression::OrdinaryField<Field_use_count>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).use_count) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CachedPlanSource>() == 224);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CachedPlanSource>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlanSource, commandTag) == 32);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_commandTag>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__CommandTag,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlanSource, num_params) == 48);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_params>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlanSource, cursor_options) == 88
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cursor_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlanSource, fixed_result) == 92
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fixed_result>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 92;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlanSource, context) == 104);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_context>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::MemoryContextData>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlanSource, query_list) == 112);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_query_list>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlanSource, relationOids) == 120
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relationOids>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlanSource, invalItems) == 128);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_invalItems>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlanSource, rewriteRoleId) == 152
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rewriteRoleId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlanSource, rewriteRowSecurity) == 156
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rewriteRowSecurity>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 156;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlanSource, dependsOnRLS) == 157
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dependsOnRLS>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 157;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlanSource, is_oneshot) == 168);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_oneshot>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 168;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlanSource, is_complete) == 169
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_complete>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 169;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlanSource, is_saved) == 170);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_saved>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 170;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlanSource, is_valid) == 171);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_valid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 171;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlanSource, generation) == 172);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_generation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 172;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlanSource, num_custom_plans) == 208
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_custom_plans>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 208;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlanSource, num_generic_plans) == 216
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_generic_plans>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 216;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CancelRequestPacket>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CancelRequestPacket>() == 4);
};
#[cfg(feature = "pg18")]
#[doc(hidden)]
pub struct Field_rt_index;
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ChangeVarNodes_context>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ChangeVarNodes_context>() == 8);
};
#[cfg(feature = "pg18")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ChangeVarNodes_context {}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ChangeVarNodes_context, rt_index) == 0
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rt_index>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ChangeVarNodes_context>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg18")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ChangeVarNodes_context, <<c::expression::CRecord<crate::__pgrx_c_bindings::ChangeVarNodes_context> as c::expression::OrdinaryField<Field_rt_index>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rt_index) };
#[cfg(feature = "pg18")]
#[doc(hidden)]
pub struct Field_new_index;
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ChangeVarNodes_context, new_index) == 4
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_new_index>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ChangeVarNodes_context>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(feature = "pg18")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ChangeVarNodes_context, <<c::expression::CRecord<crate::__pgrx_c_bindings::ChangeVarNodes_context> as c::expression::OrdinaryField<Field_new_index>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).new_index) };
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ChangeVarNodes_context, sublevels_up) == 8
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sublevels_up>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ChangeVarNodes_context>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg18")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ChangeVarNodes_context, <<c::expression::CRecord<crate::__pgrx_c_bindings::ChangeVarNodes_context> as c::expression::OrdinaryField<Field_sublevels_up>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sublevels_up) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_ckpt_slru_written;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckpointStatsData, ckpt_slru_written) == 44
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ckpt_slru_written>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckpointStatsData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData> as c::expression::OrdinaryField<Field_ckpt_slru_written>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ckpt_slru_written) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckpointStatsData, ckpt_segs_added) == 48
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ckpt_segs_added>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckpointStatsData, ckpt_segs_removed) == 52
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ckpt_segs_removed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckpointStatsData, ckpt_segs_recycled)
        == 56
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ckpt_segs_recycled>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckpointStatsData, ckpt_sync_rels) == 60
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ckpt_sync_rels>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 60;
}
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CompactAttribute>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CompactAttribute>() == 4);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CompactAttribute {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CompactAttribute, attcacheoff) == 0);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attcacheoff>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CompactAttribute>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CompactAttribute, <<c::expression::CRecord<crate::__pgrx_c_bindings::CompactAttribute> as c::expression::OrdinaryField<Field_attcacheoff>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attcacheoff) };
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CompactAttribute, attlen) == 4);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attlen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CompactAttribute>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CompactAttribute, <<c::expression::CRecord<crate::__pgrx_c_bindings::CompactAttribute> as c::expression::OrdinaryField<Field_attlen>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attlen) };
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CompactAttribute, attbyval) == 6);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attbyval>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CompactAttribute>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 6;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CompactAttribute, <<c::expression::CRecord<crate::__pgrx_c_bindings::CompactAttribute> as c::expression::OrdinaryField<Field_attbyval>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attbyval) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_attispackable;
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CompactAttribute, attispackable) == 7
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attispackable>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CompactAttribute>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 7;
}
#[cfg(feature = "pg18")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CompactAttribute, <<c::expression::CRecord<crate::__pgrx_c_bindings::CompactAttribute> as c::expression::OrdinaryField<Field_attispackable>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attispackable) };
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CompactAttribute, atthasmissing) == 8
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_atthasmissing>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CompactAttribute>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg18")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CompactAttribute, <<c::expression::CRecord<crate::__pgrx_c_bindings::CompactAttribute> as c::expression::OrdinaryField<Field_atthasmissing>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).atthasmissing) };
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CompactAttribute, attisdropped) == 9);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attisdropped>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CompactAttribute>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 9;
}
#[cfg(feature = "pg18")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CompactAttribute, <<c::expression::CRecord<crate::__pgrx_c_bindings::CompactAttribute> as c::expression::OrdinaryField<Field_attisdropped>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attisdropped) };
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CompactAttribute, attgenerated) == 10
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attgenerated>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CompactAttribute>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 10;
}
#[cfg(feature = "pg18")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CompactAttribute, <<c::expression::CRecord<crate::__pgrx_c_bindings::CompactAttribute> as c::expression::OrdinaryField<Field_attgenerated>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attgenerated) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_attnullability;
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CompactAttribute, attnullability) == 11
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attnullability>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CompactAttribute>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 11;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CompactAttribute, <<c::expression::CRecord<crate::__pgrx_c_bindings::CompactAttribute> as c::expression::OrdinaryField<Field_attnullability>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attnullability) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_attalignby;
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CompactAttribute, attalignby) == 12);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attalignby>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CompactAttribute>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CompactAttribute, <<c::expression::CRecord<crate::__pgrx_c_bindings::CompactAttribute> as c::expression::OrdinaryField<Field_attalignby>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attalignby) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ConflictTupleInfo>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ConflictTupleInfo>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ConflictTupleInfo {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ConflictTupleInfo, indexoid) == 8);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indexoid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ConflictTupleInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ConflictTupleInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ConflictTupleInfo> as c::expression::OrdinaryField<Field_indexoid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indexoid) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ConflictTupleInfo, xmin) == 12);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xmin>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ConflictTupleInfo>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ConflictTupleInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ConflictTupleInfo> as c::expression::OrdinaryField<Field_xmin>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xmin) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_origin;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ConflictTupleInfo, origin) == 16);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_origin>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ConflictTupleInfo>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ConflictTupleInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ConflictTupleInfo> as c::expression::OrdinaryField<Field_origin>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).origin) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_ts;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ConflictTupleInfo, ts) == 24);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ts>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ConflictTupleInfo>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ConflictTupleInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ConflictTupleInfo> as c::expression::OrdinaryField<Field_ts>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ts) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_socket_create;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ConnectionTiming>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ConnectionTiming>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ConnectionTiming {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ConnectionTiming, socket_create) == 0
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_socket_create>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ConnectionTiming>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ConnectionTiming, <<c::expression::CRecord<crate::__pgrx_c_bindings::ConnectionTiming> as c::expression::OrdinaryField<Field_socket_create>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).socket_create) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_ready_for_use;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ConnectionTiming, ready_for_use) == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ready_for_use>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ConnectionTiming>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ConnectionTiming, <<c::expression::CRecord<crate::__pgrx_c_bindings::ConnectionTiming> as c::expression::OrdinaryField<Field_ready_for_use>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ready_for_use) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_fork_start;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ConnectionTiming, fork_start) == 16);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fork_start>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ConnectionTiming>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ConnectionTiming, <<c::expression::CRecord<crate::__pgrx_c_bindings::ConnectionTiming> as c::expression::OrdinaryField<Field_fork_start>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fork_start) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_fork_end;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ConnectionTiming, fork_end) == 24);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fork_end>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ConnectionTiming>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ConnectionTiming, <<c::expression::CRecord<crate::__pgrx_c_bindings::ConnectionTiming> as c::expression::OrdinaryField<Field_fork_end>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fork_end) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_auth_start;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ConnectionTiming, auth_start) == 32);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_auth_start>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ConnectionTiming>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ConnectionTiming, <<c::expression::CRecord<crate::__pgrx_c_bindings::ConnectionTiming> as c::expression::OrdinaryField<Field_auth_start>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).auth_start) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_auth_end;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ConnectionTiming, auth_end) == 40);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_auth_end>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ConnectionTiming>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ConnectionTiming, <<c::expression::CRecord<crate::__pgrx_c_bindings::ConnectionTiming> as c::expression::OrdinaryField<Field_auth_end>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).auth_end) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_ccenforced;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ConstrCheck, ccenforced) == 16);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ccenforced>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ConstrCheck>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ConstrCheck, <<c::expression::CRecord<crate::__pgrx_c_bindings::ConstrCheck> as c::expression::OrdinaryField<Field_ccenforced>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ccenforced) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ConstrCheck, ccvalid) == 17);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ccvalid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ConstrCheck>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 17;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ConstrCheck, ccnoinherit) == 18);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ccnoinherit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ConstrCheck>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 18;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Constraint, is_enforced) == 18);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_enforced>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Constraint>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 18;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Constraint, <<c::expression::CRecord<crate::__pgrx_c_bindings::Constraint> as c::expression::OrdinaryField<Field_is_enforced>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_enforced) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Constraint, skip_validation) == 19);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_skip_validation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Constraint>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 19;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Constraint, initially_valid) == 20);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_initially_valid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Constraint>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Constraint, is_no_inherit) == 21);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_no_inherit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Constraint>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 21;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_generated_kind;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Constraint, generated_kind) == 41);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_generated_kind>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Constraint>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 41;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Constraint, <<c::expression::CRecord<crate::__pgrx_c_bindings::Constraint> as c::expression::OrdinaryField<Field_generated_kind>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).generated_kind) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::Constraint, nulls_not_distinct) == 42
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nulls_not_distinct>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Constraint>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 42;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Constraint, keys) == 48);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_keys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Constraint>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_without_overlaps;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Constraint, without_overlaps) == 56);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_without_overlaps>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Constraint>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Constraint, <<c::expression::CRecord<crate::__pgrx_c_bindings::Constraint> as c::expression::OrdinaryField<Field_without_overlaps>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).without_overlaps) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_fk_with_period;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Constraint, fk_with_period) == 152);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fk_with_period>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Constraint>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Constraint, <<c::expression::CRecord<crate::__pgrx_c_bindings::Constraint> as c::expression::OrdinaryField<Field_fk_with_period>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fk_with_period) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_pk_with_period;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Constraint, pk_with_period) == 153);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pk_with_period>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Constraint>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 153;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Constraint, <<c::expression::CRecord<crate::__pgrx_c_bindings::Constraint> as c::expression::OrdinaryField<Field_pk_with_period>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pk_with_period) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Constraint, fk_matchtype) == 154);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fk_matchtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Constraint>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 154;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Constraint, fk_upd_action) == 155);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fk_upd_action>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Constraint>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 155;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Constraint, fk_del_action) == 156);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fk_del_action>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Constraint>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 156;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_default_char_signedness;
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, default_char_signedness)
        == 256
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_default_char_signedness>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 256;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ControlFileData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData> as c::expression::OrdinaryField<Field_default_char_signedness>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).default_char_signedness) };
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, crc) == 292);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_crc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 292;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CookedConstraint>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CookedConstraint>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CookedConstraint, is_enforced) == 32);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_enforced>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CookedConstraint>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CookedConstraint, <<c::expression::CRecord<crate::__pgrx_c_bindings::CookedConstraint> as c::expression::OrdinaryField<Field_is_enforced>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_enforced) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CookedConstraint, skip_validation) == 33
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_skip_validation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CookedConstraint>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 33;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CookedConstraint, is_local) == 34);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_local>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CookedConstraint>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 34;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inhcount>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CookedConstraint>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CookedConstraint, is_no_inherit) == 38
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_no_inherit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CookedConstraint>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 38;
}
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CopyFormatOptions>() == 184);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CopyFormatOptions>() == 8);
};
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_log_verbosity>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__CopyLogVerbosityChoice,
        c::CInt,
        i32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_reject_limit;
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, reject_limit) == 168
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_reject_limit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 168;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFormatOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions> as c::expression::OrdinaryField<Field_reject_limit>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).reject_limit) };
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, convert_select) == 176
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_convert_select>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
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
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CopyFromStateData>() == 568);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CopyFromStateData>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, copy_src) == 8);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_copy_src>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__CopySource,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, eol_type) == 32);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_eol_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__EolType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, file_encoding) == 36
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_file_encoding>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, need_transcoding) == 40
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_need_transcoding>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, conversion_proc) == 44
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_conversion_proc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, attnumlist) == 56);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attnumlist>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
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
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, is_program) == 72);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_program>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, cur_lineno) == 296
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cur_lineno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 296;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, relname_only) == 320
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relname_only>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 320;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, num_defaults) == 336
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_defaults>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 336;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, num_errors) == 368
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_errors>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 368;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, volatile_defexprs) == 400
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_volatile_defexprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 400;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, range_table) == 408
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_range_table>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 408;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, rteperminfos) == 416
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rteperminfos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 416;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, max_fields) == 464
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_max_fields>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 464;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, line_buf_valid) == 504
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_line_buf_valid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 504;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, input_buf_index) == 520
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_input_buf_index>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 520;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, input_buf_len) == 524
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_input_buf_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 524;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, input_reached_eof) == 528
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_input_reached_eof>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 528;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, input_reached_error)
        == 529
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_input_reached_error>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 529;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, raw_buf_index) == 544
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_raw_buf_index>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 544;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, raw_buf_len) == 548
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_raw_buf_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 548;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, raw_reached_eof) == 552
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_raw_reached_eof>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 552;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, bytes_processed) == 560
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bytes_processed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 560;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CreateStmt>() == 112);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CreateStmt>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_nnconstraints;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateStmt, nnconstraints) == 64);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nnconstraints>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CreateStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateStmt> as c::expression::OrdinaryField<Field_nnconstraints>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nnconstraints) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateStmt, options) == 72);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateStmt, oncommit) == 80);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oncommit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__OnCommitAction,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateStmt, if_not_exists) == 104);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_if_not_exists>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CustomPath>() == 120);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CustomPath>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CustomPath, flags) == 80);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CustomPath>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CustomPath, custom_paths) == 88);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_custom_paths>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CustomPath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CustomPath, custom_restrictinfo) == 96
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_custom_restrictinfo>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CustomPath>
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
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CustomPath, custom_private) == 104);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_custom_private>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CustomPath>
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
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::EState>() == 344);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::EState>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_es_part_prune_infos;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_part_prune_infos) == 72);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_part_prune_infos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::EState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::EState> as c::expression::OrdinaryField<
        Field_es_part_prune_infos,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).es_part_prune_infos) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_es_part_prune_states;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_part_prune_states) == 80);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_part_prune_states>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::EState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::EState> as c::expression::OrdinaryField<
        Field_es_part_prune_states,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).es_part_prune_states) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_es_part_prune_results;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_part_prune_results) == 88);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_part_prune_results>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::EState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::EState> as c::expression::OrdinaryField<
        Field_es_part_prune_results,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).es_part_prune_results) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_output_cid) == 120);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_output_cid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_opened_result_relations) == 136
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_opened_result_relations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_tuple_routing_result_relations)
        == 152
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_tuple_routing_result_relations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_trig_target_relations) == 160
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_trig_target_relations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
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
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_tupleTable) == 200);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_tupleTable>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
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
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_processed) == 208);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_processed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 208;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_total_processed) == 216);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_total_processed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 216;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_top_eflags) == 224);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_top_eflags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 224;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_instrument) == 228);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_instrument>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 228;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_finished) == 232);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_finished>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 232;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_exprcontexts) == 240);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_exprcontexts>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
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
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_subplanstates) == 248);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_subplanstates>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 248;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_auxmodifytables) == 256);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_auxmodifytables>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 256;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_per_tuple_exprcontext) == 264
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_per_tuple_exprcontext>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::ExprContext>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 264;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_use_parallel_mode) == 280);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_use_parallel_mode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 280;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_es_parallel_workers_to_launch;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_parallel_workers_to_launch) == 284
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_parallel_workers_to_launch>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 284;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::EState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::EState> as c::expression::OrdinaryField<
        Field_es_parallel_workers_to_launch,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).es_parallel_workers_to_launch) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_es_parallel_workers_launched;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_parallel_workers_launched) == 288
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_parallel_workers_launched>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 288;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::EState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::EState> as c::expression::OrdinaryField<
        Field_es_parallel_workers_launched,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).es_parallel_workers_launched) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_jit_flags) == 304);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_jit_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 304;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_insert_pending_result_relations)
        == 328
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_insert_pending_result_relations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 328;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_insert_pending_modifytables)
        == 336
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_insert_pending_modifytables>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 336;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::EquivalenceClass>() == 96);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::EquivalenceClass>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_ec_childmembers_size;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EquivalenceClass, ec_childmembers_size) == 20
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ec_childmembers_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceClass>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::EquivalenceClass, <<c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceClass> as c::expression::OrdinaryField<Field_ec_childmembers_size>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ec_childmembers_size) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EquivalenceClass, ec_sources) == 40);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ec_sources>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceClass>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_ec_derives_list;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EquivalenceClass, ec_derives_list) == 48
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ec_derives_list>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceClass>
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
const _: Projection<crate::__pgrx_c_bindings::EquivalenceClass, <<c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceClass> as c::expression::OrdinaryField<Field_ec_derives_list>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ec_derives_list) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EquivalenceClass, ec_has_const) == 72
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ec_has_const>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceClass>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EquivalenceClass, ec_has_volatile) == 73
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ec_has_volatile>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceClass>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 73;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EquivalenceClass, ec_broken) == 74);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ec_broken>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceClass>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 74;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EquivalenceClass, ec_sortref) == 76);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ec_sortref>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceClass>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 76;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EquivalenceClass, ec_min_security) == 80
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ec_min_security>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceClass>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EquivalenceClass, ec_max_security) == 84
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ec_max_security>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceClass>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 84;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ExplainState>() == 112);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ExplainState>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_rtable_size;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExplainState, rtable_size) == 84);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rtable_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 84;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExplainState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState> as c::expression::OrdinaryField<Field_rtable_size>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rtable_size) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_extension_state_allocated;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ExplainState, extension_state_allocated)
        == 104
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_extension_state_allocated>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExplainState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState> as c::expression::OrdinaryField<Field_extension_state_allocated>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).extension_state_allocated) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ExprContext>() == 144);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ExprContext>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_2>(
        ) == 12
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_2>(
        ) == 4
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_varreturningtype;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_2,
        varreturningtype
    ) == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_varreturningtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_2>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__VarReturningType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_2, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_2> as c::expression::OrdinaryField<Field_varreturningtype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).varreturningtype) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_nullflag;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_6>(
        ) == 8
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_6>(
        ) == 4
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_6,
        nullflag
    ) == 0
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nullflag>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_6>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_6, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_6> as c::expression::OrdinaryField<Field_nullflag>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nullflag) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_6,
        jumpdone
    ) == 4
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_jumpdone>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_6>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_6, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_6> as c::expression::OrdinaryField<Field_jumpdone>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).jumpdone) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_7>(
        ) == 16
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_7>(
        ) == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_7,
        value
    ) == 0
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_value>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_7>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, crate::__pgrx_c_bindings::Datum>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_7, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_7> as c::expression::OrdinaryField<Field_value>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).value) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_7,
        isnull
    ) == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isnull>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_7>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_7, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_7> as c::expression::OrdinaryField<Field_isnull>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isnull) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_8>(
        ) == 32
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_8>(
        ) == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_8,
        nargs
    ) == 24
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nargs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_8>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_8, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_8> as c::expression::OrdinaryField<Field_nargs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nargs) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_8,
        make_ro
    ) == 28
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_make_ro>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_8>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_8, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_8> as c::expression::OrdinaryField<Field_make_ro>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).make_ro) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_9>(
        ) == 16
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_9>(
        ) == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_9,
        jumpdone
    ) == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_jumpdone>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_9>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_11>(
        ) == 4
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_11>(
        ) == 4
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_11,
        jumpdone
    ) == 0
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_jumpdone>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_11>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_11, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_11> as c::expression::OrdinaryField<Field_jumpdone>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).jumpdone) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_13>(
        ) == 8
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_13>(
        ) == 4
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_13,
        paramid
    ) == 0
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_paramid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_13>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_13,
        paramtype
    ) == 4
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_paramtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_13>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_14>(
        ) == 32
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_14>(
        ) == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_14,
        paramid
    ) == 24
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_paramid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_14>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_14, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_14> as c::expression::OrdinaryField<Field_paramid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).paramid) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_14,
        paramtype
    ) == 28
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_paramtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_14>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_14, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_14> as c::expression::OrdinaryField<Field_paramtype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).paramtype) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_16>(
        ) == 16
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_16>(
        ) == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_16,
        value
    ) == 0
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_value>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_16>
{
    type Member = c::expression::CPointer<
        c::expression::CIntegerStorage<c::CUnsignedLong, crate::__pgrx_c_bindings::Datum>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_16, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_16> as c::expression::OrdinaryField<Field_value>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).value) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_16,
        isnull
    ) == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isnull>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_16>
{
    type Member = c::expression::CPointer<c::CBool, c::expression::ReadWrite>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_16, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_16> as c::expression::OrdinaryField<Field_isnull>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isnull) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_19>(
        ) == 8
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_19>(
        ) == 4
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_19,
        seqid
    ) == 0
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_seqid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_19>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_19, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_19> as c::expression::OrdinaryField<Field_seqid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).seqid) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_19,
        seqtypid
    ) == 4
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_seqtypid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_19>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_19, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_19> as c::expression::OrdinaryField<Field_seqtypid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).seqtypid) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_20>(
        ) == 32
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_20>(
        ) == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_20,
        nelems
    ) == 16
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nelems>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_20>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_20, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_20> as c::expression::OrdinaryField<Field_nelems>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nelems) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_20,
        elemtype
    ) == 20
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_elemtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_20>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_20, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_20> as c::expression::OrdinaryField<Field_elemtype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).elemtype) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_20,
        elemlength
    ) == 24
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_elemlength>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_20>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_20, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_20> as c::expression::OrdinaryField<Field_elemlength>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).elemlength) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_20,
        elembyval
    ) == 26
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_elembyval>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_20>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 26;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_20, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_20> as c::expression::OrdinaryField<Field_elembyval>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).elembyval) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_20,
        elemalign
    ) == 27
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_elemalign>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_20>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 27;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_20, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_20> as c::expression::OrdinaryField<Field_elemalign>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).elemalign) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_20,
        multidims
    ) == 28
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_multidims>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_20>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_20, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_20> as c::expression::OrdinaryField<Field_multidims>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).multidims) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_21,
        resultelemtype
    ) == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resultelemtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_21>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_21, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_21> as c::expression::OrdinaryField<Field_resultelemtype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).resultelemtype) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_23>(
        ) == 32
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_23>(
        ) == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_23,
        jumpnull
    ) == 24
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_jumpnull>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_23>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_23, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_23> as c::expression::OrdinaryField<Field_jumpnull>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).jumpnull) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_23,
        jumpdone
    ) == 28
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_jumpdone>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_23>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_23, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_23> as c::expression::OrdinaryField<Field_jumpdone>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).jumpdone) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_cmptype;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_24>(
        ) == 4
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_24>(
        ) == 4
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_24,
        cmptype
    ) == 0
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cmptype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_24>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__CompareType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_24, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_24> as c::expression::OrdinaryField<Field_cmptype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cmptype) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_25>(
        ) == 40
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_25>(
        ) == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_25,
        nelems
    ) == 16
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nelems>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_25>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_25, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_25> as c::expression::OrdinaryField<Field_nelems>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nelems) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_25,
        op
    ) == 20
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_op>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_25>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__MinMaxOp,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_25, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_25> as c::expression::OrdinaryField<Field_op>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).op) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_26>(
        ) == 24
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_26>(
        ) == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_26,
        fieldnum
    ) == 0
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fieldnum>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_26>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_26, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_26> as c::expression::OrdinaryField<Field_fieldnum>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fieldnum) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_26,
        resulttype
    ) == 4
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resulttype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_26>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_26, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_26> as c::expression::OrdinaryField<Field_resulttype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).resulttype) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_27>(
        ) == 40
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_27>(
        ) == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_27,
        ncolumns
    ) == 32
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ncolumns>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_27>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_27, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_27> as c::expression::OrdinaryField<Field_ncolumns>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ncolumns) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_28>(
        ) == 24
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_28>(
        ) == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_28,
        jumpdone
    ) == 16
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_jumpdone>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_28>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_28, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_28> as c::expression::OrdinaryField<Field_jumpdone>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).jumpdone) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_30>(
        ) == 40
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_30>(
        ) == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_30,
        resulttype
    ) == 24
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resulttype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_30>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_30, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_30> as c::expression::OrdinaryField<Field_resulttype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).resulttype) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_init_value;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_31>(
        ) == 8
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_31>(
        ) == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_31,
        init_value
    ) == 0
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_init_value>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_31>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, crate::__pgrx_c_bindings::Datum>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_31, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_31> as c::expression::OrdinaryField<Field_init_value>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).init_value) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_32,
        jumpdone
    ) == 24
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_jumpdone>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_32>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_32, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_32> as c::expression::OrdinaryField<Field_jumpdone>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).jumpdone) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_33>(
        ) == 32
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_33>(
        ) == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_33,
        inputtype
    ) == 0
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inputtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_33>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_33, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_33> as c::expression::OrdinaryField<Field_inputtype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).inputtype) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_33,
        outputtype
    ) == 4
);
