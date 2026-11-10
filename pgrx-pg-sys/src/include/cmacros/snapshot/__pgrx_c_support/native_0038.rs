// Shared generated C macro support.

#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xs_cbuf>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexFetchHeapData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_xs_blk;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexFetchHeapData, xs_blk) == 20);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xs_blk>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexFetchHeapData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexFetchHeapData, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexFetchHeapData> as c::expression::OrdinaryField<Field_xs_blk>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xs_blk) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_xs_vmbuffer;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexFetchHeapData, xs_vmbuffer) == 24
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xs_vmbuffer>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexFetchHeapData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexFetchHeapData, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexFetchHeapData> as c::expression::OrdinaryField<Field_xs_vmbuffer>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xs_vmbuffer) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::IndexFetchTableData>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::IndexFetchTableData>() == 8);
};
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexFetchTableData, flags) == 8);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexFetchTableData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexFetchTableData, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexFetchTableData> as c::expression::OrdinaryField<Field_flags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).flags) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_disabled;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOptInfo, disabled) == 172);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_disabled>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 172;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo> as c::expression::OrdinaryField<Field_disabled>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).disabled) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOptInfo, hypothetical) == 173);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hypothetical>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 173;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOptInfo, amcanorderbyop) == 174);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amcanorderbyop>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 174;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOptInfo, amoptionalkey) == 175);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amoptionalkey>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 175;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOptInfo, amsearcharray) == 176);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amsearcharray>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 176;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOptInfo, amsearchnulls) == 177);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amsearchnulls>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 177;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOptInfo, amhasgettuple) == 178);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amhasgettuple>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 178;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOptInfo, amhasgetbitmap) == 179);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amhasgetbitmap>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 179;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOptInfo, amcanparallel) == 180);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amcanparallel>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 180;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOptInfo, amcanmarkpos) == 181);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amcanmarkpos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 181;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_is_autovacuum;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexVacuumInfo, is_autovacuum) == 17
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_autovacuum>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexVacuumInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 17;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexVacuumInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexVacuumInfo> as c::expression::OrdinaryField<Field_is_autovacuum>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_autovacuum) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexVacuumInfo, report_progress) == 18
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_report_progress>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexVacuumInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 18;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexVacuumInfo, estimated_count) == 19
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_estimated_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexVacuumInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 19;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Instrumentation>() == 360);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Instrumentation>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JoinPathExtraData>() == 64);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JoinPathExtraData>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_pgs_mask;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JoinPathExtraData, pgs_mask) == 56);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pgs_mask>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JoinPathExtraData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JoinPathExtraData, <<c::expression::CRecord<crate::__pgrx_c_bindings::JoinPathExtraData> as c::expression::OrdinaryField<Field_pgs_mask>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pgs_mask) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JsonConstructorExpr>() == 64);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JsonConstructorExpr>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonConstructorExpr, absent_on_null) == 56
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_absent_on_null>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonConstructorExpr>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonConstructorExpr, unique) == 57);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_unique>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonConstructorExpr>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 57;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonConstructorExpr, location) == 60);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonConstructorExpr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 60;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_exprBaseType;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonIsPredicate, exprBaseType) == 32);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_exprBaseType>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonIsPredicate>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonIsPredicate, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonIsPredicate> as c::expression::OrdinaryField<Field_exprBaseType>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).exprBaseType) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonIsPredicate, location) == 36);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonIsPredicate>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JsonbInState>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JsonbInState>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JsonbInState {}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonbInState, unique_keys) == 32);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_unique_keys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonbInState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonbInState, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonbInState> as c::expression::OrdinaryField<Field_unique_keys>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).unique_keys) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_idxisreplident;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepRelMapEntry, idxisreplident) == 96
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_idxisreplident>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepRelMapEntry>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepRelMapEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepRelMapEntry> as c::expression::OrdinaryField<Field_idxisreplident>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).idxisreplident) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepRelMapEntry, state) == 97);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_state>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepRelMapEntry>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 97;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_localrelid;
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LogicalRepSequenceInfo>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LogicalRepSequenceInfo>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::LogicalRepSequenceInfo {}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepSequenceInfo, localrelid) == 16
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_localrelid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepSequenceInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepSequenceInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepSequenceInfo> as c::expression::OrdinaryField<Field_localrelid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).localrelid) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_page_lsn;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepSequenceInfo, page_lsn) == 24
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_page_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepSequenceInfo>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepSequenceInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepSequenceInfo> as c::expression::OrdinaryField<Field_page_lsn>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).page_lsn) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepSequenceInfo, last_value) == 32
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_last_value>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepSequenceInfo>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepSequenceInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepSequenceInfo> as c::expression::OrdinaryField<Field_last_value>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).last_value) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepSequenceInfo, is_called) == 40
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_called>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepSequenceInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepSequenceInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepSequenceInfo> as c::expression::OrdinaryField<Field_is_called>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_called) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_found_on_pub;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepSequenceInfo, found_on_pub) == 41
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_found_on_pub>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepSequenceInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 41;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepSequenceInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepSequenceInfo> as c::expression::OrdinaryField<Field_found_on_pub>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).found_on_pub) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LogicalRepWorker>() == 144);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LogicalRepWorker>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_oldest_nonremovable_xid;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, oldest_nonremovable_xid)
        == 88
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldest_nonremovable_xid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepWorker, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker> as c::expression::OrdinaryField<Field_oldest_nonremovable_xid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oldest_nonremovable_xid) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, last_lsn) == 96);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_last_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, last_send_time) == 104
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_last_send_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, last_recv_time) == 112
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_last_recv_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, reply_lsn) == 120);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_reply_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, reply_time) == 128);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_reply_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_last_seqsync_start_time;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, last_seqsync_start_time)
        == 136
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_last_seqsync_start_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepWorker, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker> as c::expression::OrdinaryField<Field_last_seqsync_start_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).last_seqsync_start_time) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Memoize>() == 176);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Memoize>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MemoizePath>() == 136);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MemoizePath>() == 8);
};
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MemoizePath, est_entries) == 108);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_est_entries>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MemoizePath>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 108;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MergeAppend>() == 176);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MergeAppend>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeAppend, child_append_relid_sets) == 112
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_child_append_relid_sets>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeAppend>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MergeAppend, <<c::expression::CRecord<crate::__pgrx_c_bindings::MergeAppend> as c::expression::OrdinaryField<Field_child_append_relid_sets>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).child_append_relid_sets) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeAppend, mergeplans) == 120);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mergeplans>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeAppend>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeAppend, numCols) == 128);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numCols>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeAppend>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeAppend, part_prune_index) == 168
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_part_prune_index>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeAppend>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 168;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MinimalTupleTableSlot>() == 120);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MinimalTupleTableSlot>() == 8);
};
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MinimalTupleTableSlot, off) == 112);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_off>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MinimalTupleTableSlot>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTable, resultRelations) == 120);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resultRelations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTable, updateColnosLists) == 128
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_updateColnosLists>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTable, withCheckOptionLists) == 136
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_withCheckOptionLists>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTable, returningLists) == 160);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_returningLists>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTable, fdwPrivLists) == 168);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fdwPrivLists>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 168;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTable, rowMarks) == 184);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rowMarks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 184;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTable, epqParam) == 192);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_epqParam>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 192;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTable, onConflictAction) == 196
);
#[cfg(feature = "pg19")]
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
    const OFFSET: usize = 196;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTable, arbiterIndexes) == 200);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_arbiterIndexes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 200;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_onConflictLockStrength;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTable, onConflictLockStrength) == 208
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_onConflictLockStrength>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__LockClauseStrength,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 208;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTable, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable> as c::expression::OrdinaryField<Field_onConflictLockStrength>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).onConflictLockStrength) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ModifyTableState>() == 472);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ModifyTableState>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_mt_fdwPrivLists;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTableState, mt_fdwPrivLists) == 464
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mt_fdwPrivLists>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTableState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 464;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTableState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTableState> as c::expression::OrdinaryField<Field_mt_fdwPrivLists>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mt_fdwPrivLists) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::NodeInstrumentation>() == 440);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::NodeInstrumentation>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::NodeInstrumentation {}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::NodeInstrumentation, async_mode) == 360
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_async_mode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::NodeInstrumentation>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 360;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::NodeInstrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::NodeInstrumentation> as c::expression::OrdinaryField<Field_async_mode>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).async_mode) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::NodeInstrumentation, running) == 361);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_running>
    for c::expression::CRecord<crate::__pgrx_c_bindings::NodeInstrumentation>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 361;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::NodeInstrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::NodeInstrumentation> as c::expression::OrdinaryField<Field_running>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).running) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::NodeInstrumentation, ntuples2) == 408
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ntuples2>
    for c::expression::CRecord<crate::__pgrx_c_bindings::NodeInstrumentation>
{
    type Member = c::expression::CDouble;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 408;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::NodeInstrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::NodeInstrumentation> as c::expression::OrdinaryField<Field_ntuples2>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ntuples2) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::NodeInstrumentation, nfiltered1) == 424
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nfiltered1>
    for c::expression::CRecord<crate::__pgrx_c_bindings::NodeInstrumentation>
{
    type Member = c::expression::CDouble;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 424;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::NodeInstrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::NodeInstrumentation> as c::expression::OrdinaryField<Field_nfiltered1>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nfiltered1) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::NodeInstrumentation, nfiltered2) == 432
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nfiltered2>
    for c::expression::CRecord<crate::__pgrx_c_bindings::NodeInstrumentation>
{
    type Member = c::expression::CDouble;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 432;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::NodeInstrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::NodeInstrumentation> as c::expression::OrdinaryField<Field_nfiltered2>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nfiltered2) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::OnConflictActionState>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::OnConflictActionState>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::OnConflictActionState {}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::OnConflictActionState, type_) == 0);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OnConflictActionState>
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
const _: Projection<crate::__pgrx_c_bindings::OnConflictActionState, <<c::expression::CRecord<crate::__pgrx_c_bindings::OnConflictActionState> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_oc_LockStrength;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::OnConflictActionState, oc_LockStrength) == 32
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oc_LockStrength>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OnConflictActionState>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__LockClauseStrength,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::OnConflictActionState, <<c::expression::CRecord<crate::__pgrx_c_bindings::OnConflictActionState> as c::expression::OrdinaryField<Field_oc_LockStrength>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oc_LockStrength) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::OnConflictClause>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::OnConflictClause>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_lockStrength;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::OnConflictClause, lockStrength) == 16
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lockStrength>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OnConflictClause>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__LockClauseStrength,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::OnConflictClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::OnConflictClause> as c::expression::OrdinaryField<Field_lockStrength>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lockStrength) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::OnConflictClause, targetList) == 24);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_targetList>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OnConflictClause>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::OnConflictClause, location) == 40);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OnConflictClause>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::OnConflictExpr, lockStrength) == 28);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lockStrength>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OnConflictExpr>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__LockClauseStrength,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::OnConflictExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::OnConflictExpr> as c::expression::OrdinaryField<Field_lockStrength>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lockStrength) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PGPROC>() == 896);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PGPROC>() == 128);
};
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, procgloballist) == 0);
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_freeProcsLink;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, freeProcsLink) == 8);
#[cfg(feature = "pg19")]
impl c::expression::OffsetField<Field_freeProcsLink>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::dlist_node>;
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, freeProcsLink);
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, pid) == 24);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_backendType;
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, backendType) == 28);
#[cfg(feature = "pg19")]
impl c::expression::OffsetField<Field_backendType>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = ();
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, backendType);
}
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_backendType>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__BackendType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PGPROC,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC> as c::expression::OrdinaryField<
        Field_backendType,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).backendType) };
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, databaseId) == 32);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_databaseId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, roleId) == 36);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_roleId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, tempNamespaceId) == 40);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tempNamespaceId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, pgxactoff) == 44);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pgxactoff>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, statusFlags) == 48);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_statusFlags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, vxid) == 52);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vxid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC__bindgen_ty_1>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, xid) == 60);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 60;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, xmin) == 64);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xmin>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, subxidStatus) == 68);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, subxids) == 72);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, procLatch) == 328);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, sem) == 344);
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, delayChkptFlags) == 352);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_delayChkptFlags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 352;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_pendingRecoveryConflicts;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, pendingRecoveryConflicts) == 356
);
#[cfg(feature = "pg19")]
impl c::expression::OffsetField<Field_pendingRecoveryConflicts>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::pg_atomic_uint32>;
    const OFFSET: usize =
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, pendingRecoveryConflicts);
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, lwWaiting) == 360);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lwWaiting>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 360;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, lwWaitMode) == 361);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lwWaitMode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 361;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, lwWaitLink) == 364);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, cvWaitLink) == 372);
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, lockGroupLeader) == 384);
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, lockGroupMembers) == 392);
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, lockGroupLink) == 408);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, waitLock) == 424);
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_waitLink;
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, waitLink) == 432);
#[cfg(feature = "pg19")]
impl c::expression::OffsetField<Field_waitLink>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::dlist_node>;
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, waitLink);
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, waitProcLock) == 448);
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, waitLockMode) == 456);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_waitLockMode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 456;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, heldLocks) == 460);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_heldLocks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 460;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, waitStart) == 464);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, waitStatus) == 472);
#[cfg(feature = "pg19")]
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
    const OFFSET: usize = 472;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, myProcLocks) == 480);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, fpInfoLock) == 736);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, fpLockBits) == 752);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, fpRelId) == 760);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, fpVXIDLock) == 768);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fpVXIDLock>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 768;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, fpLocalTransactionId) == 772);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fpLocalTransactionId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 772;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, waitLSN) == 776);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_waitLSN>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 776;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, syncRepState) == 784);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_syncRepState>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 784;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, syncRepLinks) == 792);
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, procArrayGroupMember) == 808);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_procArrayGroupMember>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 808;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, procArrayGroupNext) == 812);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, procArrayGroupMemberXid) == 816
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_procArrayGroupMemberXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 816;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, clogGroupMember) == 820);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_clogGroupMember>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 820;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, clogGroupNext) == 824);
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, clogGroupMemberXid) == 828);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_clogGroupMemberXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 828;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, clogGroupMemberXidStatus) == 832
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_clogGroupMemberXidStatus>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 832;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, clogGroupMemberPage) == 840);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_clogGroupMemberPage>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 840;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, clogGroupMemberLsn) == 848);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_clogGroupMemberLsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 848;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, wait_event_info) == 856);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wait_event_info>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 856;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PGShmemHeader>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PGShmemHeader>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_content_offset;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGShmemHeader, content_offset) == 16);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_content_offset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGShmemHeader>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PGShmemHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::PGShmemHeader> as c::expression::OrdinaryField<Field_content_offset>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).content_offset) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGShmemHeader, device) == 32);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_device>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGShmemHeader>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGShmemHeader, inode) == 40);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGShmemHeader>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_freeProcsLock;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PROC_HDR, freeProcsLock) == 36);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_freeProcsLock>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PROC_HDR>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PROC_HDR,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PROC_HDR> as c::expression::OrdinaryField<
        Field_freeProcsLock,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).freeProcsLock) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_nplanned;
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PVWorkerStats>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PVWorkerStats>() == 4);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PVWorkerStats {}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PVWorkerStats, nplanned) == 0);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nplanned>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PVWorkerStats>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PVWorkerStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PVWorkerStats> as c::expression::OrdinaryField<Field_nplanned>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nplanned) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_nlaunched;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PVWorkerStats, nlaunched) == 4);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nlaunched>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PVWorkerStats>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PVWorkerStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PVWorkerStats> as c::expression::OrdinaryField<Field_nlaunched>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nlaunched) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PageHeaderData>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PageHeaderData>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_phs_numblock;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelBlockTableScanDescData, phs_numblock)
        == 36
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_phs_numblock>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelBlockTableScanDescData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelBlockTableScanDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelBlockTableScanDescData> as c::expression::OrdinaryField<Field_phs_numblock>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).phs_numblock) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ParallelIndexScanDescData>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ParallelIndexScanDescData>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelIndexScanDescData, ps_offset_am)
        == 24
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ps_offset_am>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelIndexScanDescData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionPruneInfo, relids) == 8);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relids>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionPruneInfo>
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
const _: Projection<crate::__pgrx_c_bindings::PartitionPruneInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionPruneInfo> as c::expression::OrdinaryField<Field_relids>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relids) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionPruneState, initialized) == 34
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_initialized>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionPruneState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 34;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionPruneState, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionPruneState> as c::expression::OrdinaryField<Field_initialized>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).initialized) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_Backend>() == 2928);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_Backend>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_BackendSubEntry>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_BackendSubEntry>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_sync_seq_error_count;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_BackendSubEntry, sync_seq_error_count)
        == 8
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sync_seq_error_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_BackendSubEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_BackendSubEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_BackendSubEntry> as c::expression::OrdinaryField<Field_sync_seq_error_count>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sync_seq_error_count) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_sync_table_error_count;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::PgStat_BackendSubEntry,
        sync_table_error_count
    ) == 16
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sync_table_error_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_BackendSubEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_BackendSubEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_BackendSubEntry> as c::expression::OrdinaryField<Field_sync_table_error_count>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sync_table_error_count) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_Lock>() == 296);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_Lock>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgStat_Lock {}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_Lock, stat_reset_timestamp) == 0
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stat_reset_timestamp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_Lock>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_Lock, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_Lock> as c::expression::OrdinaryField<Field_stat_reset_timestamp>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stat_reset_timestamp) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_waits;
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_LockEntry>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_LockEntry>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgStat_LockEntry {}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_LockEntry, waits) == 0);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_waits>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_LockEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_LockEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_LockEntry> as c::expression::OrdinaryField<Field_waits>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).waits) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_wait_time;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_LockEntry, wait_time) == 8);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wait_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_LockEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_LockEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_LockEntry> as c::expression::OrdinaryField<Field_wait_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wait_time) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_fastpath_exceeded;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_LockEntry, fastpath_exceeded) == 16
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fastpath_exceeded>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_LockEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_LockEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_LockEntry> as c::expression::OrdinaryField<Field_fastpath_exceeded>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fastpath_exceeded) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_StatFuncEntry>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_StatFuncEntry>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatFuncEntry, stat_reset_timestamp)
        == 24
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stat_reset_timestamp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatFuncEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatFuncEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatFuncEntry> as c::expression::OrdinaryField<Field_stat_reset_timestamp>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stat_reset_timestamp) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_StatSubEntry>() == 96);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_StatSubEntry>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatSubEntry, sync_seq_error_count)
        == 8
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sync_seq_error_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatSubEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatSubEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatSubEntry> as c::expression::OrdinaryField<Field_sync_seq_error_count>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sync_seq_error_count) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatSubEntry, sync_table_error_count)
        == 16
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sync_table_error_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatSubEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatSubEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatSubEntry> as c::expression::OrdinaryField<Field_sync_table_error_count>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sync_table_error_count) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatSubEntry, stat_reset_timestamp)
        == 88
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stat_reset_timestamp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatSubEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_StatTabEntry>() == 224);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_StatTabEntry>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_stat_reset_time;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatTabEntry, stat_reset_time) == 216
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stat_reset_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 216;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatTabEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry> as c::expression::OrdinaryField<Field_stat_reset_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stat_reset_time) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_WalCounters>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_WalCounters>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_WalCounters, wal_fpi_bytes) == 24
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wal_fpi_bytes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_WalCounters>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_WalCounters, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_WalCounters> as c::expression::OrdinaryField<Field_wal_fpi_bytes>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wal_fpi_bytes) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_WalCounters, wal_buffers_full) == 32
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wal_buffers_full>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_WalCounters>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_WalStats>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_WalStats>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_WalStats, stat_reset_timestamp) == 40
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stat_reset_timestamp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_WalStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::NodeInstrumentation>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::NodeInstrumentation>() == 8);
};
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_instrument>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlanState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::NodeInstrumentation>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PlannedStmt>() == 192);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PlannedStmt>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_planOrigin;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, planOrigin) == 24);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_planOrigin>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PlannedStmtOrigin,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannedStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt> as c::expression::OrdinaryField<Field_planOrigin>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).planOrigin) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, hasReturning) == 28);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasReturning>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, hasModifyingCTE) == 29);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasModifyingCTE>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 29;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, canSetTag) == 30);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_canSetTag>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 30;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, transientPlan) == 31);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_transientPlan>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 31;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, dependsOnRole) == 32);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dependsOnRole>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, parallelModeNeeded) == 33
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parallelModeNeeded>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 33;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, jitFlags) == 36);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_jitFlags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_subrtinfos;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, subrtinfos) == 104);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_subrtinfos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannedStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt> as c::expression::OrdinaryField<Field_subrtinfos>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).subrtinfos) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, rowMarks) == 120);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rowMarks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, relationOids) == 136);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relationOids>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, invalItems) == 144);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_invalItems>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, paramExecTypes) == 152);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_paramExecTypes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_elidedNodes;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, elidedNodes) == 168);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_elidedNodes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 168;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannedStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt> as c::expression::OrdinaryField<Field_elidedNodes>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).elidedNodes) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_extension_state;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, extension_state) == 176);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_extension_state>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 176;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannedStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt> as c::expression::OrdinaryField<Field_extension_state>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).extension_state) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, stmt_location) == 184);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmt_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 184;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, stmt_len) == 188);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmt_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 188;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PlannerGlobal>() == 224);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PlannerGlobal>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_subplanNames;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, subplanNames) == 40);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_subplanNames>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerGlobal, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal> as c::expression::OrdinaryField<Field_subplanNames>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).subplanNames) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, finalrtable) == 56);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_finalrtable>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, finalrteperminfos) == 80
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_finalrteperminfos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, subrtinfos) == 88);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_subrtinfos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerGlobal, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal> as c::expression::OrdinaryField<Field_subrtinfos>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).subrtinfos) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, finalrowmarks) == 96);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_finalrowmarks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, resultRelations) == 104
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resultRelations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, appendRelations) == 112
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_appendRelations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, partPruneInfos) == 120
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_partPruneInfos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, relationOids) == 128);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relationOids>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, invalItems) == 136);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_invalItems>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, paramExecTypes) == 144
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_paramExecTypes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, elidedNodes) == 152);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_elidedNodes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerGlobal, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal> as c::expression::OrdinaryField<Field_elidedNodes>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).elidedNodes) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, lastPHId) == 160);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lastPHId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, lastRowMarkId) == 164);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lastRowMarkId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 164;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, lastPlanNodeId) == 168
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lastPlanNodeId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 168;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, transientPlan) == 172);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_transientPlan>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 172;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, dependsOnRole) == 173);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dependsOnRole>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 173;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, parallelModeOK) == 174
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parallelModeOK>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 174;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, parallelModeNeeded) == 175
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parallelModeNeeded>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 175;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, maxParallelHazard) == 176
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_maxParallelHazard>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 176;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_default_pgs_mask;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, default_pgs_mask) == 184
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_default_pgs_mask>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 184;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerGlobal, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal> as c::expression::OrdinaryField<Field_default_pgs_mask>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).default_pgs_mask) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, extension_state) == 208
);
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<*mut *mut ::core::ffi::c_void>() == 8);
    assert!(::core::mem::align_of::<*mut *mut ::core::ffi::c_void>() == 8);
};
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_extension_state>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CPointer<c::expression::CVoid, c::expression::ReadWrite>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 208;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerGlobal, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal> as c::expression::OrdinaryField<Field_extension_state>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).extension_state) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, extension_state_allocated)
        == 216
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_extension_state_allocated>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 216;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerGlobal, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal> as c::expression::OrdinaryField<Field_extension_state_allocated>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).extension_state_allocated) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PlannerInfo>() == 744);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PlannerInfo>() == 8);
};
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, plan_params) == 56);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_plan_params>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, simple_rel_array_size) == 80
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_simple_rel_array_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, simple_rte_array) == 88);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_simple_rte_array>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CPointer<
            c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>,
            c::expression::ReadWrite,
        >,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, join_rel_list) == 128);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_join_rel_list>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, join_cur_level) == 152);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_join_cur_level>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, init_plans) == 160);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_init_plans>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, cte_plan_ids) == 168);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cte_plan_ids>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 168;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, multiexpr_params) == 176
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_multiexpr_params>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 176;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, join_domains) == 184);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_join_domains>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 184;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, eq_classes) == 192);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_eq_classes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 192;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, ec_merging_done) == 200);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ec_merging_done>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 200;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, canon_pathkeys) == 208);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_canon_pathkeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 208;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, left_join_clauses) == 216
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_left_join_clauses>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 216;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, right_join_clauses) == 224
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_right_join_clauses>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 224;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, full_join_clauses) == 232
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_full_join_clauses>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 232;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, join_info_list) == 240);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_join_info_list>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, last_rinfo_serial) == 248
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_last_rinfo_serial>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 248;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, append_rel_list) == 272);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_append_rel_list>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 272;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, row_identity_vars) == 280
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_row_identity_vars>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 280;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, rowMarks) == 288);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rowMarks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 288;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, placeholder_list) == 296
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_placeholder_list>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 296;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_agg_clause_list;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, agg_clause_list) == 304);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_agg_clause_list>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
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
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_agg_clause_list>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).agg_clause_list) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_group_expr_list;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, group_expr_list) == 312);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_group_expr_list>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
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
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_group_expr_list>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).group_expr_list) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_tlist_vars;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, tlist_vars) == 320);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tlist_vars>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 320;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_tlist_vars>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tlist_vars) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, placeholder_array_size) == 336
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_placeholder_array_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 336;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, fkey_list) == 344);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fkey_list>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 344;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, query_pathkeys) == 352);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_query_pathkeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
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
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, group_pathkeys) == 360);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_group_pathkeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 360;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, num_groupby_pathkeys) == 368
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_groupby_pathkeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 368;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, window_pathkeys) == 376);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_window_pathkeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 376;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, distinct_pathkeys) == 384
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_distinct_pathkeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 384;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, sort_pathkeys) == 392);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sort_pathkeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 392;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, setop_pathkeys) == 400);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_setop_pathkeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 400;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, part_schemes) == 408);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_part_schemes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 408;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, initial_rels) == 416);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_initial_rels>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 416;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, processed_groupClause) == 552
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_processed_groupClause>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 552;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, processed_distinctClause) == 560
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_processed_distinctClause>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 560;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, processed_tlist) == 568);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_processed_tlist>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 568;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, update_colnos) == 576);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_update_colnos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 576;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, minmax_aggs) == 592);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_minmax_aggs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 592;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, qual_security_level) == 632
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_qual_security_level>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 632;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, hasJoinRTEs) == 636);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasJoinRTEs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 636;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, hasLateralRTEs) == 637);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasLateralRTEs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 637;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, hasHavingQual) == 638);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasHavingQual>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 638;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, hasPseudoConstantQuals) == 639
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasPseudoConstantQuals>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 639;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, hasAlternativeSubPlans) == 640
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasAlternativeSubPlans>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 640;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, placeholdersFrozen) == 641
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_placeholdersFrozen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 641;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, hasRecursion) == 642);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasRecursion>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 642;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_assumeReplanning;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, assumeReplanning) == 643
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_assumeReplanning>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 643;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_assumeReplanning>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).assumeReplanning) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, group_rtindex) == 644);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_group_rtindex>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 644;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, agginfos) == 648);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_agginfos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 648;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, aggtransinfos) == 656);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aggtransinfos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 656;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, numOrderedAggs) == 664);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numOrderedAggs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 664;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, hasNonPartialAggs) == 668
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasNonPartialAggs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 668;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, hasNonSerialAggs) == 669
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasNonSerialAggs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 669;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, wt_param_id) == 672);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wt_param_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 672;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, curOuterParams) == 696);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_curOuterParams>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 696;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, partPruneInfos) == 720);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_partPruneInfos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 720;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, extension_state) == 728);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_extension_state>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CPointer<c::expression::CVoid, c::expression::ReadWrite>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 728;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_extension_state>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).extension_state) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, extension_state_allocated)
        == 736
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_extension_state_allocated>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 736;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_extension_state_allocated>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).extension_state_allocated) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PruneFreezeParams>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PruneFreezeParams>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PruneFreezeParams {}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PruneFreezeParams, buffer) == 8);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_buffer>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PruneFreezeParams>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PruneFreezeParams, <<c::expression::CRecord<crate::__pgrx_c_bindings::PruneFreezeParams> as c::expression::OrdinaryField<Field_buffer>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).buffer) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PruneFreezeParams, vmbuffer) == 12);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vmbuffer>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PruneFreezeParams>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PruneFreezeParams, <<c::expression::CRecord<crate::__pgrx_c_bindings::PruneFreezeParams> as c::expression::OrdinaryField<Field_vmbuffer>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).vmbuffer) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PruneFreezeParams, reason) == 16);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_reason>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PruneFreezeParams>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_PruneReason,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PruneFreezeParams, <<c::expression::CRecord<crate::__pgrx_c_bindings::PruneFreezeParams> as c::expression::OrdinaryField<Field_reason>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).reason) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PruneFreezeParams, options) == 20);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PruneFreezeParams>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PruneFreezeParams, <<c::expression::CRecord<crate::__pgrx_c_bindings::PruneFreezeParams> as c::expression::OrdinaryField<Field_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PruneFreezeResult>() == 612);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PruneFreezeResult>() == 4);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_newly_all_visible;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PruneFreezeResult, newly_all_visible) == 20
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_newly_all_visible>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PruneFreezeResult>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
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
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, extension_state) == 488);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_extension_state>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CPointer<c::expression::CVoid, c::expression::ReadWrite>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 488;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_extension_state>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).extension_state) };
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
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SlruDesc, shared) == 104);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_shared>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SlruDesc>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::SlruSharedData>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SlruDesc,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SlruDesc> as c::expression::OrdinaryField<
        Field_shared,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).shared) };
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
