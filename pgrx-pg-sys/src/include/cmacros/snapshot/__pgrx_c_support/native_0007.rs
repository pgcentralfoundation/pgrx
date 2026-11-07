// Shared generated C macro support.

// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::BTOptions,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::BTOptions> as c::expression::OrdinaryField<
        Field_deduplicate_items,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).deduplicate_items) };
#[doc(hidden)]
pub struct Field_btpo_prev;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BTPageOpaqueData>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BTPageOpaqueData>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BTPageOpaqueData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTPageOpaqueData, btpo_prev) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_btpo_prev>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTPageOpaqueData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTPageOpaqueData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTPageOpaqueData> as c::expression::OrdinaryField<Field_btpo_prev>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).btpo_prev) };
#[doc(hidden)]
pub struct Field_btpo_next;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTPageOpaqueData, btpo_next) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_btpo_next>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTPageOpaqueData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTPageOpaqueData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTPageOpaqueData> as c::expression::OrdinaryField<Field_btpo_next>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).btpo_next) };
#[doc(hidden)]
pub struct Field_btpo_level;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTPageOpaqueData, btpo_level) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_btpo_level>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTPageOpaqueData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTPageOpaqueData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTPageOpaqueData> as c::expression::OrdinaryField<Field_btpo_level>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).btpo_level) };
#[doc(hidden)]
pub struct Field_btpo_flags;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTPageOpaqueData, btpo_flags) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_btpo_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTPageOpaqueData>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTPageOpaqueData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTPageOpaqueData> as c::expression::OrdinaryField<Field_btpo_flags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).btpo_flags) };
#[doc(hidden)]
pub struct Field_btpo_cycleid;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BTPageOpaqueData, btpo_cycleid) == 14
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_btpo_cycleid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTPageOpaqueData>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 14;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTPageOpaqueData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTPageOpaqueData> as c::expression::OrdinaryField<Field_btpo_cycleid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).btpo_cycleid) };
#[doc(hidden)]
pub struct Field_target;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BTPendingFSM>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BTPendingFSM>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BTPendingFSM {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTPendingFSM, target) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_target>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTPendingFSM>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTPendingFSM, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTPendingFSM> as c::expression::OrdinaryField<Field_target>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).target) };
#[doc(hidden)]
pub struct Field_heapkeyspace;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BTScanInsertData>() == 2328);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BTScanInsertData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BTScanInsertData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanInsertData, heapkeyspace) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_heapkeyspace>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTScanInsertData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTScanInsertData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTScanInsertData> as c::expression::OrdinaryField<Field_heapkeyspace>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).heapkeyspace) };
#[doc(hidden)]
pub struct Field_allequalimage;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanInsertData, allequalimage) == 1
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_allequalimage>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTScanInsertData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTScanInsertData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTScanInsertData> as c::expression::OrdinaryField<Field_allequalimage>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).allequalimage) };
#[doc(hidden)]
pub struct Field_anynullkeys;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanInsertData, anynullkeys) == 2);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_anynullkeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTScanInsertData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 2;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTScanInsertData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTScanInsertData> as c::expression::OrdinaryField<Field_anynullkeys>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).anynullkeys) };
#[doc(hidden)]
pub struct Field_nextkey;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanInsertData, nextkey) == 3);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nextkey>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTScanInsertData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 3;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTScanInsertData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTScanInsertData> as c::expression::OrdinaryField<Field_nextkey>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nextkey) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
#[doc(hidden)]
pub struct Field_pivotsearch;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanInsertData, pivotsearch) == 4);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pivotsearch>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTScanInsertData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTScanInsertData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTScanInsertData> as c::expression::OrdinaryField<Field_pivotsearch>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pivotsearch) };
#[doc(hidden)]
pub struct Field_keysz;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanInsertData, keysz) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_keysz>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTScanInsertData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTScanInsertData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTScanInsertData> as c::expression::OrdinaryField<Field_keysz>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).keysz) };
#[doc(hidden)]
pub struct Field_qual_ok;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BTScanOpaqueData>() == 27336);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BTScanOpaqueData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BTScanOpaqueData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanOpaqueData, qual_ok) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_qual_ok>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTScanOpaqueData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTScanOpaqueData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTScanOpaqueData> as c::expression::OrdinaryField<Field_qual_ok>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).qual_ok) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
#[doc(hidden)]
pub struct Field_arraysStarted;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanOpaqueData, arraysStarted) == 1
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_arraysStarted>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTScanOpaqueData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTScanOpaqueData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTScanOpaqueData> as c::expression::OrdinaryField<Field_arraysStarted>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).arraysStarted) };
#[doc(hidden)]
pub struct Field_numberOfKeys;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanOpaqueData, numberOfKeys) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numberOfKeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTScanOpaqueData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTScanOpaqueData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTScanOpaqueData> as c::expression::OrdinaryField<Field_numberOfKeys>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numberOfKeys) };
#[doc(hidden)]
pub struct Field_numArrayKeys;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanOpaqueData, numArrayKeys) == 24
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numArrayKeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTScanOpaqueData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTScanOpaqueData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTScanOpaqueData> as c::expression::OrdinaryField<Field_numArrayKeys>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numArrayKeys) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
#[doc(hidden)]
pub struct Field_arrayKeyCount;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanOpaqueData, arrayKeyCount) == 28
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_arrayKeyCount>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTScanOpaqueData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTScanOpaqueData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTScanOpaqueData> as c::expression::OrdinaryField<Field_arrayKeyCount>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).arrayKeyCount) };
#[doc(hidden)]
pub struct Field_numKilled;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanOpaqueData, numKilled) == 56);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numKilled>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTScanOpaqueData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTScanOpaqueData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTScanOpaqueData> as c::expression::OrdinaryField<Field_numKilled>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numKilled) };
#[doc(hidden)]
pub struct Field_markItemIndex;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanOpaqueData, markItemIndex) == 80
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_markItemIndex>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTScanOpaqueData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTScanOpaqueData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTScanOpaqueData> as c::expression::OrdinaryField<Field_markItemIndex>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).markItemIndex) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BTScanPosData>() == 13624);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BTScanPosData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BTScanPosData {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanPosData, buf) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_buf>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTScanPosData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTScanPosData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTScanPosData> as c::expression::OrdinaryField<Field_buf>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).buf) };
#[doc(hidden)]
pub struct Field_lsn;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanPosData, lsn) == 8);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTScanPosData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTScanPosData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTScanPosData> as c::expression::OrdinaryField<Field_lsn>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lsn) };
#[doc(hidden)]
pub struct Field_currPage;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanPosData, currPage) == 16);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_currPage>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTScanPosData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTScanPosData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTScanPosData> as c::expression::OrdinaryField<Field_currPage>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).currPage) };
#[doc(hidden)]
pub struct Field_nextPage;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanPosData, nextPage) == 20);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nextPage>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTScanPosData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTScanPosData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTScanPosData> as c::expression::OrdinaryField<Field_nextPage>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nextPage) };
#[doc(hidden)]
pub struct Field_moreLeft;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanPosData, moreLeft) == 24);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_moreLeft>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTScanPosData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTScanPosData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTScanPosData> as c::expression::OrdinaryField<Field_moreLeft>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).moreLeft) };
#[doc(hidden)]
pub struct Field_moreRight;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanPosData, moreRight) == 25);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_moreRight>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTScanPosData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 25;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTScanPosData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTScanPosData> as c::expression::OrdinaryField<Field_moreRight>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).moreRight) };
#[doc(hidden)]
pub struct Field_nextTupleOffset;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanPosData, nextTupleOffset) == 28
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nextTupleOffset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTScanPosData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTScanPosData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTScanPosData> as c::expression::OrdinaryField<Field_nextTupleOffset>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nextTupleOffset) };
#[doc(hidden)]
pub struct Field_firstItem;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanPosData, firstItem) == 32);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_firstItem>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTScanPosData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTScanPosData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTScanPosData> as c::expression::OrdinaryField<Field_firstItem>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).firstItem) };
#[doc(hidden)]
pub struct Field_lastItem;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanPosData, lastItem) == 36);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lastItem>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTScanPosData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTScanPosData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTScanPosData> as c::expression::OrdinaryField<Field_lastItem>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lastItem) };
#[doc(hidden)]
pub struct Field_itemIndex;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanPosData, itemIndex) == 40);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_itemIndex>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTScanPosData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTScanPosData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTScanPosData> as c::expression::OrdinaryField<Field_itemIndex>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).itemIndex) };
#[doc(hidden)]
pub struct Field_indexOffset;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BTScanPosItem>() == 10);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BTScanPosItem>() == 2);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BTScanPosItem {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanPosItem, indexOffset) == 6);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indexOffset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTScanPosItem>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 6;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTScanPosItem, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTScanPosItem> as c::expression::OrdinaryField<Field_indexOffset>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indexOffset) };
#[doc(hidden)]
pub struct Field_tupleOffset;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanPosItem, tupleOffset) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tupleOffset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTScanPosItem>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTScanPosItem, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTScanPosItem> as c::expression::OrdinaryField<Field_tupleOffset>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tupleOffset) };
#[doc(hidden)]
pub struct Field_bts_blkno;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BTStackData>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BTStackData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BTStackData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTStackData, bts_blkno) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bts_blkno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTStackData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTStackData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTStackData> as c::expression::OrdinaryField<Field_bts_blkno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).bts_blkno) };
#[doc(hidden)]
pub struct Field_bts_offset;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTStackData, bts_offset) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bts_offset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTStackData>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTStackData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTStackData> as c::expression::OrdinaryField<Field_bts_offset>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).bts_offset) };
#[doc(hidden)]
pub struct Field_cycleid;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BTVacState>() == 72);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BTVacState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BTVacState {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTVacState, cycleid) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cycleid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTVacState>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTVacState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTVacState> as c::expression::OrdinaryField<Field_cycleid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cycleid) };
#[doc(hidden)]
pub struct Field_bufsize;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTVacState, bufsize) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bufsize>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTVacState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTVacState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTVacState> as c::expression::OrdinaryField<Field_bufsize>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).bufsize) };
#[doc(hidden)]
pub struct Field_maxbufsize;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTVacState, maxbufsize) == 52);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_maxbufsize>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTVacState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTVacState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTVacState> as c::expression::OrdinaryField<Field_maxbufsize>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).maxbufsize) };
#[doc(hidden)]
pub struct Field_npendingpages;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTVacState, npendingpages) == 64);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_npendingpages>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTVacState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTVacState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTVacState> as c::expression::OrdinaryField<Field_npendingpages>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).npendingpages) };
#[doc(hidden)]
pub struct Field_updatedoffset;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BTVacuumPostingData>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BTVacuumPostingData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BTVacuumPostingData {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BTVacuumPostingData, updatedoffset) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_updatedoffset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTVacuumPostingData>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTVacuumPostingData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTVacuumPostingData> as c::expression::OrdinaryField<Field_updatedoffset>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).updatedoffset) };
#[doc(hidden)]
pub struct Field_ndeletedtids;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BTVacuumPostingData, ndeletedtids) == 10
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ndeletedtids>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTVacuumPostingData>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 10;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTVacuumPostingData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTVacuumPostingData> as c::expression::OrdinaryField<Field_ndeletedtids>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ndeletedtids) };
#[doc(hidden)]
pub struct Field_bgw_flags;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BackgroundWorker>() == 544);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BackgroundWorker>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BackgroundWorker {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BackgroundWorker, bgw_flags) == 192);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bgw_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BackgroundWorker>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 192;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BackgroundWorker, <<c::expression::CRecord<crate::__pgrx_c_bindings::BackgroundWorker> as c::expression::OrdinaryField<Field_bgw_flags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).bgw_flags) };
#[doc(hidden)]
pub struct Field_bgw_start_time;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BackgroundWorker, bgw_start_time) == 196
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bgw_start_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BackgroundWorker>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_BgWorkerStartTime,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 196;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BackgroundWorker, <<c::expression::CRecord<crate::__pgrx_c_bindings::BackgroundWorker> as c::expression::OrdinaryField<Field_bgw_start_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).bgw_start_time) };
#[doc(hidden)]
pub struct Field_bgw_restart_time;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BackgroundWorker, bgw_restart_time) == 200
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bgw_restart_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BackgroundWorker>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 200;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BackgroundWorker, <<c::expression::CRecord<crate::__pgrx_c_bindings::BackgroundWorker> as c::expression::OrdinaryField<Field_bgw_restart_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).bgw_restart_time) };
#[doc(hidden)]
pub struct Field_bgw_main_arg;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BackgroundWorker, bgw_main_arg) == 400
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bgw_main_arg>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BackgroundWorker>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, crate::__pgrx_c_bindings::Datum>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 400;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BackgroundWorker, <<c::expression::CRecord<crate::__pgrx_c_bindings::BackgroundWorker> as c::expression::OrdinaryField<Field_bgw_main_arg>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).bgw_main_arg) };
#[doc(hidden)]
pub struct Field_bgw_notify_pid;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BackgroundWorker, bgw_notify_pid) == 536
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bgw_notify_pid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BackgroundWorker>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 536;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BackgroundWorker, <<c::expression::CRecord<crate::__pgrx_c_bindings::BackgroundWorker> as c::expression::OrdinaryField<Field_bgw_notify_pid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).bgw_notify_pid) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Barrier>() == 36);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Barrier>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::Barrier {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Barrier, mutex) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mutex>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Barrier>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Barrier,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Barrier> as c::expression::OrdinaryField<
        Field_mutex,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mutex) };
#[doc(hidden)]
pub struct Field_phase;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Barrier, phase) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_phase>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Barrier>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Barrier,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Barrier> as c::expression::OrdinaryField<
        Field_phase,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).phase) };
#[doc(hidden)]
pub struct Field_participants;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Barrier, participants) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_participants>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Barrier>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Barrier,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Barrier> as c::expression::OrdinaryField<
        Field_participants,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).participants) };
#[doc(hidden)]
pub struct Field_arrived;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Barrier, arrived) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_arrived>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Barrier>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Barrier,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Barrier> as c::expression::OrdinaryField<
        Field_arrived,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).arrived) };
#[doc(hidden)]
pub struct Field_elected;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Barrier, elected) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_elected>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Barrier>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Barrier,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Barrier> as c::expression::OrdinaryField<
        Field_elected,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).elected) };
#[doc(hidden)]
pub struct Field_static_party;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Barrier, static_party) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_static_party>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Barrier>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Barrier,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Barrier> as c::expression::OrdinaryField<
        Field_static_party,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).static_party) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BaseBackupCmd>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BaseBackupCmd>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BaseBackupCmd {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BaseBackupCmd, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BaseBackupCmd>
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
const _: Projection<crate::__pgrx_c_bindings::BaseBackupCmd, <<c::expression::CRecord<crate::__pgrx_c_bindings::BaseBackupCmd> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BaseBackupCmd, options) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BaseBackupCmd>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BaseBackupCmd, <<c::expression::CRecord<crate::__pgrx_c_bindings::BaseBackupCmd> as c::expression::OrdinaryField<Field_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BitString>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BitString>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BitString {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BitString, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BitString>
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
    crate::__pgrx_c_bindings::BitString,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::BitString> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_nplans;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BitmapAndState>() == 216);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BitmapAndState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BitmapAndState {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BitmapAndState, nplans) == 208);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nplans>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BitmapAndState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 208;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BitmapAndState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BitmapAndState> as c::expression::OrdinaryField<Field_nplans>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nplans) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
#[doc(hidden)]
pub struct Field_can_skip_fetch;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BitmapHeapScanState>() == 352);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BitmapHeapScanState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BitmapHeapScanState {}
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BitmapHeapScanState, can_skip_fetch) == 256
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_can_skip_fetch>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BitmapHeapScanState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 256;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BitmapHeapScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BitmapHeapScanState> as c::expression::OrdinaryField<Field_can_skip_fetch>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).can_skip_fetch) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
#[doc(hidden)]
pub struct Field_return_empty_tuples;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BitmapHeapScanState, return_empty_tuples)
        == 260
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_return_empty_tuples>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BitmapHeapScanState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 260;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BitmapHeapScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BitmapHeapScanState> as c::expression::OrdinaryField<Field_return_empty_tuples>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).return_empty_tuples) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_vmbuffer;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BitmapHeapScanState, vmbuffer) == 264
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vmbuffer>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BitmapHeapScanState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 264;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BitmapHeapScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BitmapHeapScanState> as c::expression::OrdinaryField<Field_vmbuffer>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).vmbuffer) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
#[doc(hidden)]
pub struct Field_pvmbuffer;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BitmapHeapScanState, pvmbuffer) == 268
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pvmbuffer>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BitmapHeapScanState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 268;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BitmapHeapScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BitmapHeapScanState> as c::expression::OrdinaryField<Field_pvmbuffer>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pvmbuffer) };
#[doc(hidden)]
pub struct Field_exact_pages;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BitmapHeapScanState, exact_pages) == 272
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_exact_pages>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BitmapHeapScanState>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 272;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BitmapHeapScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BitmapHeapScanState> as c::expression::OrdinaryField<Field_exact_pages>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).exact_pages) };
#[doc(hidden)]
pub struct Field_lossy_pages;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BitmapHeapScanState, lossy_pages) == 280
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lossy_pages>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BitmapHeapScanState>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 280;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BitmapHeapScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BitmapHeapScanState> as c::expression::OrdinaryField<Field_lossy_pages>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lossy_pages) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
#[doc(hidden)]
pub struct Field_prefetch_pages;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BitmapHeapScanState, prefetch_pages) == 296
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_prefetch_pages>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BitmapHeapScanState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 296;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BitmapHeapScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BitmapHeapScanState> as c::expression::OrdinaryField<Field_prefetch_pages>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).prefetch_pages) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
#[doc(hidden)]
pub struct Field_prefetch_target;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BitmapHeapScanState, prefetch_target) == 300
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_prefetch_target>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BitmapHeapScanState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 300;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BitmapHeapScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BitmapHeapScanState> as c::expression::OrdinaryField<Field_prefetch_target>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).prefetch_target) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
#[doc(hidden)]
pub struct Field_prefetch_maximum;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BitmapHeapScanState, prefetch_maximum) == 304
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_prefetch_maximum>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BitmapHeapScanState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 304;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BitmapHeapScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BitmapHeapScanState> as c::expression::OrdinaryField<Field_prefetch_maximum>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).prefetch_maximum) };
#[doc(hidden)]
pub struct Field_pscan_len;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BitmapHeapScanState, pscan_len) == 312
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pscan_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BitmapHeapScanState>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 312;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BitmapHeapScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BitmapHeapScanState> as c::expression::OrdinaryField<Field_pscan_len>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pscan_len) };
#[doc(hidden)]
pub struct Field_initialized;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BitmapHeapScanState, initialized) == 320
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_initialized>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BitmapHeapScanState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 320;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BitmapHeapScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BitmapHeapScanState> as c::expression::OrdinaryField<Field_initialized>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).initialized) };
#[doc(hidden)]
pub struct Field_indexid;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BitmapIndexScan>() == 136);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BitmapIndexScan>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BitmapIndexScan {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BitmapIndexScan, indexid) == 112);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indexid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BitmapIndexScan>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BitmapIndexScan, <<c::expression::CRecord<crate::__pgrx_c_bindings::BitmapIndexScan> as c::expression::OrdinaryField<Field_indexid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indexid) };
#[doc(hidden)]
pub struct Field_isshared;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BitmapIndexScan, isshared) == 116);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isshared>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BitmapIndexScan>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 116;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BitmapIndexScan, <<c::expression::CRecord<crate::__pgrx_c_bindings::BitmapIndexScan> as c::expression::OrdinaryField<Field_isshared>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isshared) };
#[doc(hidden)]
pub struct Field_indexqual;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BitmapIndexScan, indexqual) == 120);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indexqual>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BitmapIndexScan>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BitmapIndexScan, <<c::expression::CRecord<crate::__pgrx_c_bindings::BitmapIndexScan> as c::expression::OrdinaryField<Field_indexqual>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indexqual) };
#[doc(hidden)]
pub struct Field_indexqualorig;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BitmapIndexScan, indexqualorig) == 128
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indexqualorig>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BitmapIndexScan>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BitmapIndexScan, <<c::expression::CRecord<crate::__pgrx_c_bindings::BitmapIndexScan> as c::expression::OrdinaryField<Field_indexqualorig>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indexqualorig) };
#[doc(hidden)]
pub struct Field_biss_NumScanKeys;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BitmapIndexScanState>() == 304);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BitmapIndexScanState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BitmapIndexScanState {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BitmapIndexScanState, biss_NumScanKeys)
        == 240
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_biss_NumScanKeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BitmapIndexScanState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BitmapIndexScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BitmapIndexScanState> as c::expression::OrdinaryField<Field_biss_NumScanKeys>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).biss_NumScanKeys) };
#[doc(hidden)]
pub struct Field_biss_NumRuntimeKeys;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BitmapIndexScanState, biss_NumRuntimeKeys)
        == 256
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_biss_NumRuntimeKeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BitmapIndexScanState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 256;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BitmapIndexScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BitmapIndexScanState> as c::expression::OrdinaryField<Field_biss_NumRuntimeKeys>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).biss_NumRuntimeKeys) };
#[doc(hidden)]
pub struct Field_biss_NumArrayKeys;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BitmapIndexScanState, biss_NumArrayKeys)
        == 272
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_biss_NumArrayKeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BitmapIndexScanState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 272;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BitmapIndexScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BitmapIndexScanState> as c::expression::OrdinaryField<Field_biss_NumArrayKeys>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).biss_NumArrayKeys) };
#[doc(hidden)]
pub struct Field_biss_RuntimeKeysReady;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BitmapIndexScanState, biss_RuntimeKeysReady)
        == 276
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_biss_RuntimeKeysReady>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BitmapIndexScanState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 276;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BitmapIndexScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BitmapIndexScanState> as c::expression::OrdinaryField<Field_biss_RuntimeKeysReady>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).biss_RuntimeKeysReady) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BitmapOr>() == 120);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BitmapOr>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BitmapOr {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BitmapOr, isshared) == 104);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isshared>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BitmapOr>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::BitmapOr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::BitmapOr> as c::expression::OrdinaryField<
        Field_isshared,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isshared) };
#[doc(hidden)]
pub struct Field_bitmapplans;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BitmapOr, bitmapplans) == 112);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bitmapplans>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BitmapOr>
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
    crate::__pgrx_c_bindings::BitmapOr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::BitmapOr> as c::expression::OrdinaryField<
        Field_bitmapplans,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).bitmapplans) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BitmapOrState>() == 216);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BitmapOrState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BitmapOrState {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BitmapOrState, nplans) == 208);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nplans>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BitmapOrState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 208;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BitmapOrState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BitmapOrState> as c::expression::OrdinaryField<Field_nplans>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nplans) };
#[doc(hidden)]
pub struct Field_bi_hi;
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BlockIdData {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BlockIdData, bi_hi) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bi_hi>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BlockIdData>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BlockIdData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BlockIdData> as c::expression::OrdinaryField<Field_bi_hi>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).bi_hi) };
#[doc(hidden)]
pub struct Field_bi_lo;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BlockIdData, bi_lo) == 2);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bi_lo>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BlockIdData>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 2;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BlockIdData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BlockIdData> as c::expression::OrdinaryField<Field_bi_lo>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).bi_lo) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BlockedProcData>() == 20);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BlockedProcData>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BlockedProcData {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BlockedProcData, pid) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BlockedProcData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BlockedProcData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BlockedProcData> as c::expression::OrdinaryField<Field_pid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pid) };
#[doc(hidden)]
pub struct Field_first_lock;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BlockedProcData, first_lock) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_first_lock>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BlockedProcData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BlockedProcData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BlockedProcData> as c::expression::OrdinaryField<Field_first_lock>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).first_lock) };
#[doc(hidden)]
pub struct Field_num_locks;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BlockedProcData, num_locks) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_locks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BlockedProcData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BlockedProcData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BlockedProcData> as c::expression::OrdinaryField<Field_num_locks>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_locks) };
#[doc(hidden)]
pub struct Field_first_waiter;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BlockedProcData, first_waiter) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_first_waiter>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BlockedProcData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BlockedProcData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BlockedProcData> as c::expression::OrdinaryField<Field_first_waiter>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).first_waiter) };
#[doc(hidden)]
pub struct Field_num_waiters;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BlockedProcData, num_waiters) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_waiters>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BlockedProcData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BlockedProcData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BlockedProcData> as c::expression::OrdinaryField<Field_num_waiters>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_waiters) };
#[doc(hidden)]
pub struct Field_nprocs;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BlockedProcsData>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BlockedProcsData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BlockedProcsData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BlockedProcsData, nprocs) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nprocs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BlockedProcsData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BlockedProcsData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BlockedProcsData> as c::expression::OrdinaryField<Field_nprocs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nprocs) };
#[doc(hidden)]
pub struct Field_maxprocs;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BlockedProcsData, maxprocs) == 28);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_maxprocs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BlockedProcsData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BlockedProcsData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BlockedProcsData> as c::expression::OrdinaryField<Field_maxprocs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).maxprocs) };
#[doc(hidden)]
pub struct Field_nlocks;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BlockedProcsData, nlocks) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nlocks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BlockedProcsData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BlockedProcsData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BlockedProcsData> as c::expression::OrdinaryField<Field_nlocks>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nlocks) };
#[doc(hidden)]
pub struct Field_maxlocks;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BlockedProcsData, maxlocks) == 36);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_maxlocks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BlockedProcsData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BlockedProcsData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BlockedProcsData> as c::expression::OrdinaryField<Field_maxlocks>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).maxlocks) };
#[doc(hidden)]
pub struct Field_npids;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BlockedProcsData, npids) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_npids>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BlockedProcsData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BlockedProcsData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BlockedProcsData> as c::expression::OrdinaryField<Field_npids>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).npids) };
#[doc(hidden)]
pub struct Field_maxpids;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BlockedProcsData, maxpids) == 44);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_maxpids>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BlockedProcsData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BlockedProcsData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BlockedProcsData> as c::expression::OrdinaryField<Field_maxpids>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).maxpids) };
#[doc(hidden)]
pub struct Field_boolop;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BoolExpr>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BoolExpr>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BoolExpr {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BoolExpr, boolop) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_boolop>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BoolExpr>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__BoolExprType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::BoolExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::BoolExpr> as c::expression::OrdinaryField<
        Field_boolop,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).boolop) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BoolExpr, args) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_args>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BoolExpr>
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
    crate::__pgrx_c_bindings::BoolExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::BoolExpr> as c::expression::OrdinaryField<
        Field_args,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).args) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BoolExpr, location) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BoolExpr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::BoolExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::BoolExpr> as c::expression::OrdinaryField<
        Field_location,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Boolean>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Boolean>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::Boolean {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Boolean, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Boolean>
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
    crate::__pgrx_c_bindings::Boolean,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Boolean> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_boolval;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Boolean, boolval) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_boolval>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Boolean>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Boolean,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Boolean> as c::expression::OrdinaryField<
        Field_boolval,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).boolval) };
#[doc(hidden)]
pub struct Field_booltesttype;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BooleanTest>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BooleanTest>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BooleanTest {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BooleanTest, booltesttype) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_booltesttype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BooleanTest>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__BoolTestType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BooleanTest, <<c::expression::CRecord<crate::__pgrx_c_bindings::BooleanTest> as c::expression::OrdinaryField<Field_booltesttype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).booltesttype) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BooleanTest, location) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BooleanTest>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BooleanTest, <<c::expression::CRecord<crate::__pgrx_c_bindings::BooleanTest> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[doc(hidden)]
pub struct Field_bd_totalstored;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BrinDesc>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BrinDesc>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BrinDesc {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BrinDesc, bd_totalstored) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bd_totalstored>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BrinDesc>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::BrinDesc,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::BrinDesc> as c::expression::OrdinaryField<
        Field_bd_totalstored,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).bd_totalstored) };
#[doc(hidden)]
pub struct Field_bt_placeholder;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BrinMemTuple>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BrinMemTuple>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BrinMemTuple {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BrinMemTuple, bt_placeholder) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bt_placeholder>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BrinMemTuple>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BrinMemTuple, <<c::expression::CRecord<crate::__pgrx_c_bindings::BrinMemTuple> as c::expression::OrdinaryField<Field_bt_placeholder>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).bt_placeholder) };
#[doc(hidden)]
pub struct Field_bt_empty_range;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BrinMemTuple, bt_empty_range) == 1);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bt_empty_range>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BrinMemTuple>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BrinMemTuple, <<c::expression::CRecord<crate::__pgrx_c_bindings::BrinMemTuple> as c::expression::OrdinaryField<Field_bt_empty_range>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).bt_empty_range) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BrinMemTuple, bt_blkno) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bt_blkno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BrinMemTuple>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BrinMemTuple, <<c::expression::CRecord<crate::__pgrx_c_bindings::BrinMemTuple> as c::expression::OrdinaryField<Field_bt_blkno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).bt_blkno) };
#[doc(hidden)]
pub struct Field_brinMagic;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BrinMetaPageData>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BrinMetaPageData>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BrinMetaPageData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BrinMetaPageData, brinMagic) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_brinMagic>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BrinMetaPageData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BrinMetaPageData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BrinMetaPageData> as c::expression::OrdinaryField<Field_brinMagic>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).brinMagic) };
#[doc(hidden)]
pub struct Field_brinVersion;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BrinMetaPageData, brinVersion) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_brinVersion>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BrinMetaPageData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BrinMetaPageData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BrinMetaPageData> as c::expression::OrdinaryField<Field_brinVersion>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).brinVersion) };
#[doc(hidden)]
pub struct Field_pagesPerRange;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BrinMetaPageData, pagesPerRange) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pagesPerRange>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BrinMetaPageData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BrinMetaPageData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BrinMetaPageData> as c::expression::OrdinaryField<Field_pagesPerRange>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pagesPerRange) };
#[doc(hidden)]
pub struct Field_lastRevmapPage;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BrinMetaPageData, lastRevmapPage) == 12
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lastRevmapPage>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BrinMetaPageData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BrinMetaPageData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BrinMetaPageData> as c::expression::OrdinaryField<Field_lastRevmapPage>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lastRevmapPage) };
#[doc(hidden)]
pub struct Field_oi_nstored;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BrinOpcInfo>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BrinOpcInfo>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BrinOpcInfo {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BrinOpcInfo, oi_nstored) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oi_nstored>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BrinOpcInfo>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BrinOpcInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::BrinOpcInfo> as c::expression::OrdinaryField<Field_oi_nstored>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oi_nstored) };
#[doc(hidden)]
pub struct Field_oi_regular_nulls;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BrinOpcInfo, oi_regular_nulls) == 2);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oi_regular_nulls>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BrinOpcInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 2;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BrinOpcInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::BrinOpcInfo> as c::expression::OrdinaryField<Field_oi_regular_nulls>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oi_regular_nulls) };
#[doc(hidden)]
pub struct Field_oi_typcache;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BrinOpcInfo, oi_typcache) == 16);
impl c::expression::OffsetField<Field_oi_typcache>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BrinOpcInfo>
{
    type Member = ();
    const OFFSET: usize =
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::BrinOpcInfo, oi_typcache);
}
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BrinOptions>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BrinOptions>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BrinOptions {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BrinOptions, vl_len_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vl_len_>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BrinOptions>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BrinOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::BrinOptions> as c::expression::OrdinaryField<Field_vl_len_>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).vl_len_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BrinOptions, pagesPerRange) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pagesPerRange>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BrinOptions>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BrinOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::BrinOptions> as c::expression::OrdinaryField<Field_pagesPerRange>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pagesPerRange) };
#[doc(hidden)]
pub struct Field_autosummarize;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BrinOptions, autosummarize) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_autosummarize>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BrinOptions>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BrinOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::BrinOptions> as c::expression::OrdinaryField<Field_autosummarize>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).autosummarize) };
#[doc(hidden)]
pub struct Field_vector;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BrinSpecialSpace>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BrinSpecialSpace>() == 2);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BrinSpecialSpace {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BrinSpecialSpace, vector) == 0);
const _: () = {
    assert!(::core::mem::size_of::<[u16; 4]>() == 8);
    assert!(::core::mem::align_of::<[u16; 4]>() == 2);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vector>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BrinSpecialSpace>
{
    type Member = c::expression::CArray<c::CUnsignedShort, 4>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BrinSpecialSpace, <<c::expression::CRecord<crate::__pgrx_c_bindings::BrinSpecialSpace> as c::expression::OrdinaryField<Field_vector>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).vector) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BrinStatsData>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BrinStatsData>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BrinStatsData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BrinStatsData, pagesPerRange) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pagesPerRange>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BrinStatsData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BrinStatsData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BrinStatsData> as c::expression::OrdinaryField<Field_pagesPerRange>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pagesPerRange) };
#[doc(hidden)]
pub struct Field_revmapNumPages;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BrinStatsData, revmapNumPages) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_revmapNumPages>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BrinStatsData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BrinStatsData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BrinStatsData> as c::expression::OrdinaryField<Field_revmapNumPages>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).revmapNumPages) };
#[doc(hidden)]
pub struct Field_bv_attno;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BrinValues>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BrinValues>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BrinValues {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BrinValues, bv_attno) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bv_attno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BrinValues>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BrinValues, <<c::expression::CRecord<crate::__pgrx_c_bindings::BrinValues> as c::expression::OrdinaryField<Field_bv_attno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).bv_attno) };
#[doc(hidden)]
pub struct Field_bv_hasnulls;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BrinValues, bv_hasnulls) == 2);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bv_hasnulls>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BrinValues>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 2;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BrinValues, <<c::expression::CRecord<crate::__pgrx_c_bindings::BrinValues> as c::expression::OrdinaryField<Field_bv_hasnulls>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).bv_hasnulls) };
#[doc(hidden)]
pub struct Field_bv_allnulls;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BrinValues, bv_allnulls) == 3);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bv_allnulls>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BrinValues>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 3;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BrinValues, <<c::expression::CRecord<crate::__pgrx_c_bindings::BrinValues> as c::expression::OrdinaryField<Field_bv_allnulls>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).bv_allnulls) };
#[doc(hidden)]
pub struct Field_bv_mem_value;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BrinValues, bv_mem_value) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bv_mem_value>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BrinValues>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, crate::__pgrx_c_bindings::Datum>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BrinValues, <<c::expression::CRecord<crate::__pgrx_c_bindings::BrinValues> as c::expression::OrdinaryField<Field_bv_mem_value>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).bv_mem_value) };
#[doc(hidden)]
pub struct Field_buf_id;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BufferDesc>() == 52);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BufferDesc>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BufferDesc {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BufferDesc, buf_id) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_buf_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BufferDesc>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BufferDesc, <<c::expression::CRecord<crate::__pgrx_c_bindings::BufferDesc> as c::expression::OrdinaryField<Field_buf_id>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).buf_id) };
#[doc(hidden)]
pub struct Field_wait_backend_pgprocno;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BufferDesc, wait_backend_pgprocno) == 28
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wait_backend_pgprocno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BufferDesc>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BufferDesc, <<c::expression::CRecord<crate::__pgrx_c_bindings::BufferDesc> as c::expression::OrdinaryField<Field_wait_backend_pgprocno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wait_backend_pgprocno) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
pub struct Field_freeNext;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BufferDesc, freeNext) == 32);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_freeNext>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BufferDesc>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BufferDesc, <<c::expression::CRecord<crate::__pgrx_c_bindings::BufferDesc> as c::expression::OrdinaryField<Field_freeNext>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).freeNext) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
pub struct Field_content_lock;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BufferDesc, content_lock) == 36);
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LWLock>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LWLock>() == 4);
};
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_content_lock>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BufferDesc>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::LWLock>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BufferDesc, <<c::expression::CRecord<crate::__pgrx_c_bindings::BufferDesc> as c::expression::OrdinaryField<Field_content_lock>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).content_lock) };
#[doc(hidden)]
pub struct Field_buffer;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BufferHeapTupleTableSlot>() == 112);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BufferHeapTupleTableSlot>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BufferHeapTupleTableSlot {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BufferHeapTupleTableSlot, buffer) == 104
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_buffer>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BufferHeapTupleTableSlot>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BufferHeapTupleTableSlot, <<c::expression::CRecord<crate::__pgrx_c_bindings::BufferHeapTupleTableSlot> as c::expression::OrdinaryField<Field_buffer>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).buffer) };
#[doc(hidden)]
pub struct Field_current_buf;
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BulkInsertStateData>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BulkInsertStateData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BulkInsertStateData {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BulkInsertStateData, current_buf) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_current_buf>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BulkInsertStateData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BulkInsertStateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BulkInsertStateData> as c::expression::OrdinaryField<Field_current_buf>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).current_buf) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CTECycleClause>() == 72);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CTECycleClause>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CTECycleClause {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CTECycleClause, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CTECycleClause>
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
const _: Projection<crate::__pgrx_c_bindings::CTECycleClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::CTECycleClause> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_cycle_col_list;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CTECycleClause, cycle_col_list) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cycle_col_list>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CTECycleClause>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CTECycleClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::CTECycleClause> as c::expression::OrdinaryField<Field_cycle_col_list>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cycle_col_list) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CTECycleClause, location) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CTECycleClause>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CTECycleClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::CTECycleClause> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[doc(hidden)]
pub struct Field_cycle_mark_type;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CTECycleClause, cycle_mark_type) == 52
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cycle_mark_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CTECycleClause>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CTECycleClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::CTECycleClause> as c::expression::OrdinaryField<Field_cycle_mark_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cycle_mark_type) };
#[doc(hidden)]
pub struct Field_cycle_mark_typmod;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CTECycleClause, cycle_mark_typmod) == 56
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cycle_mark_typmod>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CTECycleClause>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CTECycleClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::CTECycleClause> as c::expression::OrdinaryField<Field_cycle_mark_typmod>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cycle_mark_typmod) };
#[doc(hidden)]
pub struct Field_cycle_mark_collation;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CTECycleClause, cycle_mark_collation) == 60
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cycle_mark_collation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CTECycleClause>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 60;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CTECycleClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::CTECycleClause> as c::expression::OrdinaryField<Field_cycle_mark_collation>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cycle_mark_collation) };
#[doc(hidden)]
pub struct Field_cycle_mark_neop;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CTECycleClause, cycle_mark_neop) == 64
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cycle_mark_neop>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CTECycleClause>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CTECycleClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::CTECycleClause> as c::expression::OrdinaryField<Field_cycle_mark_neop>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cycle_mark_neop) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CTESearchClause>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CTESearchClause>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CTESearchClause {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CTESearchClause, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CTESearchClause>
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
const _: Projection<crate::__pgrx_c_bindings::CTESearchClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::CTESearchClause> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_search_col_list;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CTESearchClause, search_col_list) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_search_col_list>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CTESearchClause>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CTESearchClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::CTESearchClause> as c::expression::OrdinaryField<Field_search_col_list>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).search_col_list) };
#[doc(hidden)]
pub struct Field_search_breadth_first;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CTESearchClause, search_breadth_first) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_search_breadth_first>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CTESearchClause>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CTESearchClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::CTESearchClause> as c::expression::OrdinaryField<Field_search_breadth_first>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).search_breadth_first) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CTESearchClause, location) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CTESearchClause>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CTESearchClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::CTESearchClause> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[doc(hidden)]
pub struct Field_magic;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CachedExpression>() == 64);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CachedExpression>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CachedExpression {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedExpression, magic) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_magic>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedExpression>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CachedExpression, <<c::expression::CRecord<crate::__pgrx_c_bindings::CachedExpression> as c::expression::OrdinaryField<Field_magic>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).magic) };
#[doc(hidden)]
pub struct Field_is_valid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedExpression, is_valid) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_valid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedExpression>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CachedExpression, <<c::expression::CRecord<crate::__pgrx_c_bindings::CachedExpression> as c::expression::OrdinaryField<Field_is_valid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_valid) };
#[doc(hidden)]
pub struct Field_relationOids;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedExpression, relationOids) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relationOids>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedExpression>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CachedExpression, <<c::expression::CRecord<crate::__pgrx_c_bindings::CachedExpression> as c::expression::OrdinaryField<Field_relationOids>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relationOids) };
#[doc(hidden)]
pub struct Field_invalItems;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedExpression, invalItems) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_invalItems>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedExpression>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CachedExpression, <<c::expression::CRecord<crate::__pgrx_c_bindings::CachedExpression> as c::expression::OrdinaryField<Field_invalItems>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).invalItems) };
#[doc(hidden)]
pub struct Field_context;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedExpression, context) == 40);
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::MemoryContextData>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::MemoryContextData>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_context>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedExpression>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::MemoryContextData>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CachedExpression, <<c::expression::CRecord<crate::__pgrx_c_bindings::CachedExpression> as c::expression::OrdinaryField<Field_context>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).context) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CachedPlan>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CachedPlan>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CachedPlan {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlan, magic) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_magic>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlan>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CachedPlan, <<c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlan> as c::expression::OrdinaryField<Field_magic>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).magic) };
#[doc(hidden)]
pub struct Field_stmt_list;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlan, stmt_list) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmt_list>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlan>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CachedPlan, <<c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlan> as c::expression::OrdinaryField<Field_stmt_list>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stmt_list) };
#[doc(hidden)]
pub struct Field_is_oneshot;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlan, is_oneshot) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_oneshot>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlan>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CachedPlan, <<c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlan> as c::expression::OrdinaryField<Field_is_oneshot>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_oneshot) };
#[doc(hidden)]
pub struct Field_is_saved;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlan, is_saved) == 17);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_saved>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlan>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 17;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CachedPlan, <<c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlan> as c::expression::OrdinaryField<Field_is_saved>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_saved) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlan, is_valid) == 18);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_valid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlan>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 18;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CachedPlan, <<c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlan> as c::expression::OrdinaryField<Field_is_valid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_valid) };
#[doc(hidden)]
pub struct Field_planRoleId;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlan, planRoleId) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_planRoleId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlan>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CachedPlan, <<c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlan> as c::expression::OrdinaryField<Field_planRoleId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).planRoleId) };
#[doc(hidden)]
pub struct Field_dependsOnRole;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlan, dependsOnRole) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dependsOnRole>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlan>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CachedPlan, <<c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlan> as c::expression::OrdinaryField<Field_dependsOnRole>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dependsOnRole) };
#[doc(hidden)]
pub struct Field_saved_xmin;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlan, saved_xmin) == 28);
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TransactionId>() == 4);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TransactionId>() == 4);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_saved_xmin>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlan>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CachedPlan, <<c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlan> as c::expression::OrdinaryField<Field_saved_xmin>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).saved_xmin) };
#[doc(hidden)]
pub struct Field_generation;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlan, generation) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_generation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlan>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CachedPlan, <<c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlan> as c::expression::OrdinaryField<Field_generation>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).generation) };
#[doc(hidden)]
pub struct Field_refcount;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlan, refcount) == 36);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_refcount>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlan>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CachedPlan, <<c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlan> as c::expression::OrdinaryField<Field_refcount>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).refcount) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlan, context) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_context>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlan>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::MemoryContextData>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CachedPlan, <<c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlan> as c::expression::OrdinaryField<Field_context>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).context) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CachedPlanSource>() == 200);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CachedPlanSource>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CachedPlanSource {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlanSource, magic) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_magic>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CachedPlanSource, <<c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource> as c::expression::OrdinaryField<Field_magic>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).magic) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlanSource, commandTag) == 24);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
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
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CachedPlanSource, <<c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource> as c::expression::OrdinaryField<Field_commandTag>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).commandTag) };
#[doc(hidden)]
pub struct Field_num_params;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlanSource, num_params) == 40);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_params>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CachedPlanSource, <<c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource> as c::expression::OrdinaryField<Field_num_params>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_params) };
#[doc(hidden)]
pub struct Field_cursor_options;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlanSource, cursor_options) == 64
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cursor_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CachedPlanSource, <<c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource> as c::expression::OrdinaryField<Field_cursor_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cursor_options) };
#[doc(hidden)]
pub struct Field_fixed_result;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlanSource, fixed_result) == 68
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fixed_result>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 68;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CachedPlanSource, <<c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource> as c::expression::OrdinaryField<Field_fixed_result>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fixed_result) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlanSource, context) == 80);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_context>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::MemoryContextData>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CachedPlanSource, <<c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource> as c::expression::OrdinaryField<Field_context>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).context) };
#[doc(hidden)]
pub struct Field_query_list;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlanSource, query_list) == 88);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_query_list>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CachedPlanSource, <<c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource> as c::expression::OrdinaryField<Field_query_list>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).query_list) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlanSource, relationOids) == 96
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relationOids>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CachedPlanSource, <<c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource> as c::expression::OrdinaryField<Field_relationOids>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relationOids) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlanSource, invalItems) == 104);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_invalItems>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CachedPlanSource, <<c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource> as c::expression::OrdinaryField<Field_invalItems>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).invalItems) };
#[doc(hidden)]
pub struct Field_rewriteRoleId;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlanSource, rewriteRoleId) == 128
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rewriteRoleId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CachedPlanSource, <<c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource> as c::expression::OrdinaryField<Field_rewriteRoleId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rewriteRoleId) };
#[doc(hidden)]
pub struct Field_rewriteRowSecurity;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlanSource, rewriteRowSecurity) == 132
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rewriteRowSecurity>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 132;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CachedPlanSource, <<c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource> as c::expression::OrdinaryField<Field_rewriteRowSecurity>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rewriteRowSecurity) };
#[doc(hidden)]
pub struct Field_dependsOnRLS;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlanSource, dependsOnRLS) == 133
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dependsOnRLS>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 133;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CachedPlanSource, <<c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource> as c::expression::OrdinaryField<Field_dependsOnRLS>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dependsOnRLS) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlanSource, is_oneshot) == 144);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_oneshot>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CachedPlanSource, <<c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource> as c::expression::OrdinaryField<Field_is_oneshot>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_oneshot) };
#[doc(hidden)]
pub struct Field_is_complete;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlanSource, is_complete) == 145
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_complete>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 145;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CachedPlanSource, <<c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource> as c::expression::OrdinaryField<Field_is_complete>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_complete) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlanSource, is_saved) == 146);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_saved>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 146;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CachedPlanSource, <<c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource> as c::expression::OrdinaryField<Field_is_saved>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_saved) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlanSource, is_valid) == 147);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_valid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 147;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CachedPlanSource, <<c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource> as c::expression::OrdinaryField<Field_is_valid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_valid) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlanSource, generation) == 148);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_generation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 148;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CachedPlanSource, <<c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource> as c::expression::OrdinaryField<Field_generation>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).generation) };
#[doc(hidden)]
pub struct Field_num_custom_plans;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlanSource, num_custom_plans) == 184
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_custom_plans>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 184;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CachedPlanSource, <<c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource> as c::expression::OrdinaryField<Field_num_custom_plans>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_custom_plans) };
#[doc(hidden)]
pub struct Field_num_generic_plans;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CachedPlanSource, num_generic_plans) == 192
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_generic_plans>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 192;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CachedPlanSource, <<c::expression::CRecord<crate::__pgrx_c_bindings::CachedPlanSource> as c::expression::OrdinaryField<Field_num_generic_plans>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_generic_plans) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CallContext>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CallContext>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CallContext {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CallContext, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CallContext>
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
const _: Projection<crate::__pgrx_c_bindings::CallContext, <<c::expression::CRecord<crate::__pgrx_c_bindings::CallContext> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CallContext, atomic) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_atomic>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CallContext>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CallContext, <<c::expression::CRecord<crate::__pgrx_c_bindings::CallContext> as c::expression::OrdinaryField<Field_atomic>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).atomic) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CallStmt>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CallStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CallStmt {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CallStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CallStmt>
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
    crate::__pgrx_c_bindings::CallStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::CallStmt> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_outargs;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CallStmt, outargs) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_outargs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CallStmt>
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
    crate::__pgrx_c_bindings::CallStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::CallStmt> as c::expression::OrdinaryField<
        Field_outargs,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).outargs) };
#[doc(hidden)]
pub struct Field_cancelRequestCode;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CancelRequestPacket>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CancelRequestPacket>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CancelRequestPacket {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CancelRequestPacket, cancelRequestCode) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cancelRequestCode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CancelRequestPacket>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CancelRequestPacket, <<c::expression::CRecord<crate::__pgrx_c_bindings::CancelRequestPacket> as c::expression::OrdinaryField<Field_cancelRequestCode>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cancelRequestCode) };
#[doc(hidden)]
pub struct Field_backendPID;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CancelRequestPacket, backendPID) == 4
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_backendPID>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CancelRequestPacket>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CancelRequestPacket, <<c::expression::CRecord<crate::__pgrx_c_bindings::CancelRequestPacket> as c::expression::OrdinaryField<Field_backendPID>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).backendPID) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
#[doc(hidden)]
pub struct Field_cancelAuthCode;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CancelRequestPacket, cancelAuthCode) == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cancelAuthCode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CancelRequestPacket>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CancelRequestPacket, <<c::expression::CRecord<crate::__pgrx_c_bindings::CancelRequestPacket> as c::expression::OrdinaryField<Field_cancelAuthCode>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cancelAuthCode) };
#[doc(hidden)]
pub struct Field_casetype;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CaseExpr>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CaseExpr>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CaseExpr {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CaseExpr, casetype) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_casetype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CaseExpr>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::CaseExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::CaseExpr> as c::expression::OrdinaryField<
        Field_casetype,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).casetype) };
#[doc(hidden)]
pub struct Field_casecollid;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CaseExpr, casecollid) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_casecollid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CaseExpr>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::CaseExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::CaseExpr> as c::expression::OrdinaryField<
        Field_casecollid,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).casecollid) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CaseExpr, args) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_args>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CaseExpr>
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
    crate::__pgrx_c_bindings::CaseExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::CaseExpr> as c::expression::OrdinaryField<
        Field_args,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).args) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CaseExpr, location) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CaseExpr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::CaseExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::CaseExpr> as c::expression::OrdinaryField<
        Field_location,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[doc(hidden)]
pub struct Field_typeId;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CaseTestExpr>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CaseTestExpr>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CaseTestExpr {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CaseTestExpr, typeId) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typeId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CaseTestExpr>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CaseTestExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::CaseTestExpr> as c::expression::OrdinaryField<Field_typeId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typeId) };
#[doc(hidden)]
pub struct Field_typeMod;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CaseTestExpr, typeMod) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typeMod>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CaseTestExpr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CaseTestExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::CaseTestExpr> as c::expression::OrdinaryField<Field_typeMod>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typeMod) };
#[doc(hidden)]
pub struct Field_collation;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CaseTestExpr, collation) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_collation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CaseTestExpr>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CaseTestExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::CaseTestExpr> as c::expression::OrdinaryField<Field_collation>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).collation) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CaseWhen>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CaseWhen>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CaseWhen {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CaseWhen, location) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CaseWhen>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::CaseWhen,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::CaseWhen> as c::expression::OrdinaryField<
        Field_location,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[doc(hidden)]
pub struct Field_redo;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CheckPoint>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CheckPoint>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CheckPoint {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, redo) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_redo>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckPoint, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint> as c::expression::OrdinaryField<Field_redo>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).redo) };
#[doc(hidden)]
pub struct Field_ThisTimeLineID;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, ThisTimeLineID) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ThisTimeLineID>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckPoint, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint> as c::expression::OrdinaryField<Field_ThisTimeLineID>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ThisTimeLineID) };
#[doc(hidden)]
pub struct Field_PrevTimeLineID;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, PrevTimeLineID) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_PrevTimeLineID>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckPoint, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint> as c::expression::OrdinaryField<Field_PrevTimeLineID>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).PrevTimeLineID) };
#[doc(hidden)]
pub struct Field_fullPageWrites;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, fullPageWrites) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fullPageWrites>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckPoint, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint> as c::expression::OrdinaryField<Field_fullPageWrites>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fullPageWrites) };
#[doc(hidden)]
pub struct Field_nextOid;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, nextOid) == 32);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nextOid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckPoint, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint> as c::expression::OrdinaryField<Field_nextOid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nextOid) };
#[doc(hidden)]
pub struct Field_nextMulti;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, nextMulti) == 36);
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MultiXactId>() == 4);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MultiXactId>() == 4);
};
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nextMulti>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::MultiXactId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckPoint, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint> as c::expression::OrdinaryField<Field_nextMulti>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nextMulti) };
#[doc(hidden)]
pub struct Field_nextMultiOffset;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, nextMultiOffset) == 40);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nextMultiOffset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckPoint, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint> as c::expression::OrdinaryField<Field_nextMultiOffset>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nextMultiOffset) };
#[doc(hidden)]
pub struct Field_oldestXid;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, oldestXid) == 44);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldestXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckPoint, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint> as c::expression::OrdinaryField<Field_oldestXid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oldestXid) };
#[doc(hidden)]
pub struct Field_oldestXidDB;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, oldestXidDB) == 48);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldestXidDB>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckPoint, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint> as c::expression::OrdinaryField<Field_oldestXidDB>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oldestXidDB) };
#[doc(hidden)]
pub struct Field_oldestMulti;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, oldestMulti) == 52);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldestMulti>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::MultiXactId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckPoint, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint> as c::expression::OrdinaryField<Field_oldestMulti>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oldestMulti) };
#[doc(hidden)]
pub struct Field_oldestMultiDB;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, oldestMultiDB) == 56);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldestMultiDB>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckPoint, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint> as c::expression::OrdinaryField<Field_oldestMultiDB>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oldestMultiDB) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, time) == 64);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckPoint, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint> as c::expression::OrdinaryField<Field_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).time) };
#[doc(hidden)]
pub struct Field_oldestCommitTsXid;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, oldestCommitTsXid) == 72);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldestCommitTsXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckPoint, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint> as c::expression::OrdinaryField<Field_oldestCommitTsXid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oldestCommitTsXid) };
#[doc(hidden)]
pub struct Field_newestCommitTsXid;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, newestCommitTsXid) == 76);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_newestCommitTsXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 76;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckPoint, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint> as c::expression::OrdinaryField<Field_newestCommitTsXid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).newestCommitTsXid) };
#[doc(hidden)]
pub struct Field_oldestActiveXid;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, oldestActiveXid) == 80);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldestActiveXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckPoint, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint> as c::expression::OrdinaryField<Field_oldestActiveXid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oldestActiveXid) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CheckPointStmt>() == 4);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CheckPointStmt>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CheckPointStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPointStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPointStmt>
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
const _: Projection<crate::__pgrx_c_bindings::CheckPointStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckPointStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_ckpt_start_t;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CheckpointStatsData>() == 80);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CheckpointStatsData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CheckpointStatsData {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckpointStatsData, ckpt_start_t) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ckpt_start_t>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckpointStatsData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData> as c::expression::OrdinaryField<Field_ckpt_start_t>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ckpt_start_t) };
#[doc(hidden)]
pub struct Field_ckpt_write_t;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckpointStatsData, ckpt_write_t) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ckpt_write_t>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckpointStatsData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData> as c::expression::OrdinaryField<Field_ckpt_write_t>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ckpt_write_t) };
#[doc(hidden)]
pub struct Field_ckpt_sync_t;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckpointStatsData, ckpt_sync_t) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ckpt_sync_t>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckpointStatsData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData> as c::expression::OrdinaryField<Field_ckpt_sync_t>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ckpt_sync_t) };
#[doc(hidden)]
pub struct Field_ckpt_sync_end_t;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckpointStatsData, ckpt_sync_end_t) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ckpt_sync_end_t>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckpointStatsData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData> as c::expression::OrdinaryField<Field_ckpt_sync_end_t>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ckpt_sync_end_t) };
#[doc(hidden)]
pub struct Field_ckpt_end_t;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckpointStatsData, ckpt_end_t) == 32
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ckpt_end_t>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckpointStatsData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData> as c::expression::OrdinaryField<Field_ckpt_end_t>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ckpt_end_t) };
#[doc(hidden)]
pub struct Field_ckpt_bufs_written;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckpointStatsData, ckpt_bufs_written) == 40
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ckpt_bufs_written>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckpointStatsData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData> as c::expression::OrdinaryField<Field_ckpt_bufs_written>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ckpt_bufs_written) };
#[doc(hidden)]
pub struct Field_ckpt_segs_added;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckpointStatsData, ckpt_segs_added) == 44
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ckpt_segs_added>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckpointStatsData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData> as c::expression::OrdinaryField<Field_ckpt_segs_added>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ckpt_segs_added) };
#[doc(hidden)]
pub struct Field_ckpt_segs_removed;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckpointStatsData, ckpt_segs_removed) == 48
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ckpt_segs_removed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckpointStatsData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData> as c::expression::OrdinaryField<Field_ckpt_segs_removed>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ckpt_segs_removed) };
#[doc(hidden)]
pub struct Field_ckpt_segs_recycled;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckpointStatsData, ckpt_segs_recycled)
        == 52
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ckpt_segs_recycled>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckpointStatsData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData> as c::expression::OrdinaryField<Field_ckpt_segs_recycled>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ckpt_segs_recycled) };
#[doc(hidden)]
pub struct Field_ckpt_sync_rels;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckpointStatsData, ckpt_sync_rels) == 56
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ckpt_sync_rels>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckpointStatsData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData> as c::expression::OrdinaryField<Field_ckpt_sync_rels>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ckpt_sync_rels) };
#[doc(hidden)]
pub struct Field_ckpt_longest_sync;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckpointStatsData, ckpt_longest_sync) == 64
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ckpt_longest_sync>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckpointStatsData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData> as c::expression::OrdinaryField<Field_ckpt_longest_sync>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ckpt_longest_sync) };
#[doc(hidden)]
pub struct Field_ckpt_agg_sync_time;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckpointStatsData, ckpt_agg_sync_time)
        == 72
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ckpt_agg_sync_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckpointStatsData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData> as c::expression::OrdinaryField<Field_ckpt_agg_sync_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ckpt_agg_sync_time) };
#[doc(hidden)]
pub struct Field_tour2_position;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::City>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::City>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::City {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::City, tour2_position) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tour2_position>
    for c::expression::CRecord<crate::__pgrx_c_bindings::City>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::City,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::City> as c::expression::OrdinaryField<
        Field_tour2_position,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tour2_position) };
#[doc(hidden)]
pub struct Field_tour1_position;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::City, tour1_position) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tour1_position>
    for c::expression::CRecord<crate::__pgrx_c_bindings::City>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::City,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::City> as c::expression::OrdinaryField<
        Field_tour1_position,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tour1_position) };
#[doc(hidden)]
pub struct Field_used;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::City, used) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_used>
    for c::expression::CRecord<crate::__pgrx_c_bindings::City>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::City,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::City> as c::expression::OrdinaryField<
        Field_used,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).used) };
#[doc(hidden)]
pub struct Field_select_list;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::City, select_list) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_select_list>
    for c::expression::CRecord<crate::__pgrx_c_bindings::City>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::City,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::City> as c::expression::OrdinaryField<
        Field_select_list,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).select_list) };
#[doc(hidden)]
pub struct Field_tsId;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CkptSortItem>() == 20);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CkptSortItem>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CkptSortItem {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CkptSortItem, tsId) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tsId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CkptSortItem>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CkptSortItem, <<c::expression::CRecord<crate::__pgrx_c_bindings::CkptSortItem> as c::expression::OrdinaryField<Field_tsId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tsId) };
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CkptSortItem, relNode) == 4);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relNode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CkptSortItem>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CkptSortItem, <<c::expression::CRecord<crate::__pgrx_c_bindings::CkptSortItem> as c::expression::OrdinaryField<Field_relNode>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relNode) };
#[doc(hidden)]
pub struct Field_forkNum;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CkptSortItem, forkNum) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_forkNum>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CkptSortItem>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ForkNumber,
        c::CInt,
        i32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CkptSortItem, <<c::expression::CRecord<crate::__pgrx_c_bindings::CkptSortItem> as c::expression::OrdinaryField<Field_forkNum>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).forkNum) };
#[doc(hidden)]
pub struct Field_blockNum;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CkptSortItem, blockNum) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_blockNum>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CkptSortItem>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CkptSortItem, <<c::expression::CRecord<crate::__pgrx_c_bindings::CkptSortItem> as c::expression::OrdinaryField<Field_blockNum>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).blockNum) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CkptSortItem, buf_id) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_buf_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CkptSortItem>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CkptSortItem, <<c::expression::CRecord<crate::__pgrx_c_bindings::CkptSortItem> as c::expression::OrdinaryField<Field_buf_id>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).buf_id) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ClosePortalStmt>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ClosePortalStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ClosePortalStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ClosePortalStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ClosePortalStmt>
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
const _: Projection<crate::__pgrx_c_bindings::ClosePortalStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::ClosePortalStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ClusterParams>() == 4);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ClusterParams>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ClusterParams {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ClusterParams, options) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ClusterParams>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ClusterParams, <<c::expression::CRecord<crate::__pgrx_c_bindings::ClusterParams> as c::expression::OrdinaryField<Field_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ClusterStmt>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ClusterStmt>() == 8);
};
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ClusterStmt {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ClusterStmt, type_) == 0);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ClusterStmt>
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
const _: Projection<crate::__pgrx_c_bindings::ClusterStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::ClusterStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_params;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ClusterStmt, params) == 24);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_params>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ClusterStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ClusterStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::ClusterStmt> as c::expression::OrdinaryField<Field_params>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).params) };
#[doc(hidden)]
pub struct Field_coalescetype;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CoalesceExpr>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CoalesceExpr>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CoalesceExpr {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CoalesceExpr, coalescetype) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_coalescetype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CoalesceExpr>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CoalesceExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::CoalesceExpr> as c::expression::OrdinaryField<Field_coalescetype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).coalescetype) };
#[doc(hidden)]
pub struct Field_coalescecollid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CoalesceExpr, coalescecollid) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_coalescecollid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CoalesceExpr>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CoalesceExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::CoalesceExpr> as c::expression::OrdinaryField<Field_coalescecollid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).coalescecollid) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CoalesceExpr, args) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_args>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CoalesceExpr>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CoalesceExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::CoalesceExpr> as c::expression::OrdinaryField<Field_args>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).args) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CoalesceExpr, location) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CoalesceExpr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CoalesceExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::CoalesceExpr> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CoerceToDomain>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CoerceToDomain>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CoerceToDomain {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CoerceToDomain, resulttype) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resulttype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CoerceToDomain>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CoerceToDomain, <<c::expression::CRecord<crate::__pgrx_c_bindings::CoerceToDomain> as c::expression::OrdinaryField<Field_resulttype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).resulttype) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CoerceToDomain, resulttypmod) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resulttypmod>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CoerceToDomain>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CoerceToDomain, <<c::expression::CRecord<crate::__pgrx_c_bindings::CoerceToDomain> as c::expression::OrdinaryField<Field_resulttypmod>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).resulttypmod) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CoerceToDomain, resultcollid) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resultcollid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CoerceToDomain>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CoerceToDomain, <<c::expression::CRecord<crate::__pgrx_c_bindings::CoerceToDomain> as c::expression::OrdinaryField<Field_resultcollid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).resultcollid) };
#[doc(hidden)]
pub struct Field_coercionformat;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CoerceToDomain, coercionformat) == 28
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_coercionformat>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CoerceToDomain>
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
const _: Projection<crate::__pgrx_c_bindings::CoerceToDomain, <<c::expression::CRecord<crate::__pgrx_c_bindings::CoerceToDomain> as c::expression::OrdinaryField<Field_coercionformat>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).coercionformat) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CoerceToDomain, location) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CoerceToDomain>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CoerceToDomain, <<c::expression::CRecord<crate::__pgrx_c_bindings::CoerceToDomain> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CoerceToDomainValue>() == 20);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CoerceToDomainValue>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CoerceToDomainValue {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CoerceToDomainValue, typeId) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typeId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CoerceToDomainValue>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CoerceToDomainValue, <<c::expression::CRecord<crate::__pgrx_c_bindings::CoerceToDomainValue> as c::expression::OrdinaryField<Field_typeId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typeId) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CoerceToDomainValue, typeMod) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typeMod>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CoerceToDomainValue>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CoerceToDomainValue, <<c::expression::CRecord<crate::__pgrx_c_bindings::CoerceToDomainValue> as c::expression::OrdinaryField<Field_typeMod>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typeMod) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CoerceToDomainValue, collation) == 12
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_collation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CoerceToDomainValue>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CoerceToDomainValue, <<c::expression::CRecord<crate::__pgrx_c_bindings::CoerceToDomainValue> as c::expression::OrdinaryField<Field_collation>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).collation) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CoerceToDomainValue, location) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CoerceToDomainValue>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CoerceToDomainValue, <<c::expression::CRecord<crate::__pgrx_c_bindings::CoerceToDomainValue> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CoerceViaIO>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CoerceViaIO>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CoerceViaIO {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CoerceViaIO, resulttype) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resulttype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CoerceViaIO>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CoerceViaIO, <<c::expression::CRecord<crate::__pgrx_c_bindings::CoerceViaIO> as c::expression::OrdinaryField<Field_resulttype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).resulttype) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CoerceViaIO, resultcollid) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resultcollid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CoerceViaIO>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CoerceViaIO, <<c::expression::CRecord<crate::__pgrx_c_bindings::CoerceViaIO> as c::expression::OrdinaryField<Field_resultcollid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).resultcollid) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CoerceViaIO, coerceformat) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_coerceformat>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CoerceViaIO>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__CoercionForm,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CoerceViaIO, <<c::expression::CRecord<crate::__pgrx_c_bindings::CoerceViaIO> as c::expression::OrdinaryField<Field_coerceformat>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).coerceformat) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CoerceViaIO, location) == 28);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CoerceViaIO>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CoerceViaIO, <<c::expression::CRecord<crate::__pgrx_c_bindings::CoerceViaIO> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CollateClause>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CollateClause>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CollateClause {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CollateClause, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CollateClause>
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
const _: Projection<crate::__pgrx_c_bindings::CollateClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::CollateClause> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CollateClause, collname) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_collname>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CollateClause>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CollateClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::CollateClause> as c::expression::OrdinaryField<Field_collname>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).collname) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CollateClause, location) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CollateClause>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CollateClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::CollateClause> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[doc(hidden)]
pub struct Field_collOid;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CollateExpr>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CollateExpr>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CollateExpr {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CollateExpr, collOid) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_collOid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CollateExpr>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CollateExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::CollateExpr> as c::expression::OrdinaryField<Field_collOid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).collOid) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CollateExpr, location) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CollateExpr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CollateExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::CollateExpr> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CollectedCommand>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CollectedCommand>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CollectedCommand {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CollectedCommand, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CollectedCommand>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__CollectedCommandType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CollectedCommand, <<c::expression::CRecord<crate::__pgrx_c_bindings::CollectedCommand> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_in_extension;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CollectedCommand, in_extension) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_in_extension>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CollectedCommand>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CollectedCommand, <<c::expression::CRecord<crate::__pgrx_c_bindings::CollectedCommand> as c::expression::OrdinaryField<Field_in_extension>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).in_extension) };
const _: () = {
    assert!(
        ::core::mem::size_of::<
            crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_2,
        >() == 16
    );
    assert!(
        ::core::mem::align_of::<
            crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_2,
        >() == 8
    );
};
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_2
{
}
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_2,
        objectId
    ) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_objectId>
    for c::expression::CRecord<
        crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_2,
    >
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_2, <<c::expression::CRecord<crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_2> as c::expression::OrdinaryField<Field_objectId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).objectId) };
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_2,
        classId
    ) == 4
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_classId>
    for c::expression::CRecord<
        crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_2,
    >
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _:
    Projection<
        crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_2,
        <<c::expression::CRecord<
            crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_2,
        > as c::expression::OrdinaryField<Field_classId>>::Member as c::expression::CType>::Storage,
    > = |base| unsafe { ::core::ptr::addr_of_mut!((*base).classId) };
#[doc(hidden)]
pub struct Field_subcmds;
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_2,
        subcmds
    ) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_subcmds>
    for c::expression::CRecord<
        crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_2,
    >
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _:
    Projection<
        crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_2,
        <<c::expression::CRecord<
            crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_2,
        > as c::expression::OrdinaryField<Field_subcmds>>::Member as c::expression::CType>::Storage,
    > = |base| unsafe { ::core::ptr::addr_of_mut!((*base).subcmds) };
#[doc(hidden)]
pub struct Field_ndicts;
const _: () = {
    assert!(
        ::core::mem::size_of::<
            crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_6,
        >() == 32
    );
    assert!(
        ::core::mem::align_of::<
            crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_6,
        >() == 8
    );
};
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_6
{
}
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_6,
        ndicts
    ) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ndicts>
    for c::expression::CRecord<
        crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_6,
    >
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _:
    Projection<
        crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_6,
        <<c::expression::CRecord<
            crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_6,
        > as c::expression::OrdinaryField<Field_ndicts>>::Member as c::expression::CType>::Storage,
    > = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ndicts) };
const _: () = {
    assert!(
        ::core::mem::size_of::<
            crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_7,
        >() == 4
    );
    assert!(
        ::core::mem::align_of::<
            crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_7,
        >() == 4
    );
};
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_7
{
}
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_7,
        objtype
    ) == 0
);
