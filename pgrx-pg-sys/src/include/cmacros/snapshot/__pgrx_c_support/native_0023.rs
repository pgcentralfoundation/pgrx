// Shared generated C macro support.

#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_topwindow>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggPath>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowAggPath, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggPath> as c::expression::OrdinaryField<Field_topwindow>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).topwindow) };
#[doc(hidden)]
pub struct Field_funcs;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::WindowAggState>() == 632);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::WindowAggState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::WindowAggState {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, funcs) == 224);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_funcs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 224;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowAggState, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState> as c::expression::OrdinaryField<Field_funcs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).funcs) };
#[doc(hidden)]
pub struct Field_numfuncs;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, numfuncs) == 232);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numfuncs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 232;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowAggState, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState> as c::expression::OrdinaryField<Field_numfuncs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numfuncs) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, numaggs) == 236);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numaggs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 236;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowAggState, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState> as c::expression::OrdinaryField<Field_numaggs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numaggs) };
#[doc(hidden)]
pub struct Field_current_ptr;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, current_ptr) == 280);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_current_ptr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 280;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowAggState, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState> as c::expression::OrdinaryField<Field_current_ptr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).current_ptr) };
#[doc(hidden)]
pub struct Field_framehead_ptr;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, framehead_ptr) == 284
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_framehead_ptr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 284;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowAggState, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState> as c::expression::OrdinaryField<Field_framehead_ptr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).framehead_ptr) };
#[doc(hidden)]
pub struct Field_frametail_ptr;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, frametail_ptr) == 288
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_frametail_ptr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 288;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowAggState, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState> as c::expression::OrdinaryField<Field_frametail_ptr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).frametail_ptr) };
#[doc(hidden)]
pub struct Field_grouptail_ptr;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, grouptail_ptr) == 292
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_grouptail_ptr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 292;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowAggState, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState> as c::expression::OrdinaryField<Field_grouptail_ptr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).grouptail_ptr) };
#[doc(hidden)]
pub struct Field_spooled_rows;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, spooled_rows) == 296);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_spooled_rows>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 296;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowAggState, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState> as c::expression::OrdinaryField<Field_spooled_rows>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).spooled_rows) };
#[doc(hidden)]
pub struct Field_currentpos;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, currentpos) == 304);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_currentpos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 304;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowAggState, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState> as c::expression::OrdinaryField<Field_currentpos>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).currentpos) };
#[doc(hidden)]
pub struct Field_frameheadpos;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, frameheadpos) == 312);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_frameheadpos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 312;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowAggState, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState> as c::expression::OrdinaryField<Field_frameheadpos>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).frameheadpos) };
#[doc(hidden)]
pub struct Field_frametailpos;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, frametailpos) == 320);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_frametailpos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 320;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowAggState, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState> as c::expression::OrdinaryField<Field_frametailpos>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).frametailpos) };
#[doc(hidden)]
pub struct Field_aggregatedbase;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, aggregatedbase) == 336
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aggregatedbase>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 336;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowAggState, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState> as c::expression::OrdinaryField<Field_aggregatedbase>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).aggregatedbase) };
#[doc(hidden)]
pub struct Field_aggregatedupto;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, aggregatedupto) == 344
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aggregatedupto>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 344;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowAggState, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState> as c::expression::OrdinaryField<Field_aggregatedupto>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).aggregatedupto) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, status) == 352);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_status>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__WindowAggStatus,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 352;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowAggState, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState> as c::expression::OrdinaryField<Field_status>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).status) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, frameOptions) == 356);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_frameOptions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 356;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowAggState, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState> as c::expression::OrdinaryField<Field_frameOptions>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).frameOptions) };
#[doc(hidden)]
pub struct Field_startOffsetValue;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, startOffsetValue) == 376
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_startOffsetValue>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, crate::__pgrx_c_bindings::Datum>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 376;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowAggState, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState> as c::expression::OrdinaryField<Field_startOffsetValue>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).startOffsetValue) };
#[doc(hidden)]
pub struct Field_endOffsetValue;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, endOffsetValue) == 384
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_endOffsetValue>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, crate::__pgrx_c_bindings::Datum>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 384;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowAggState, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState> as c::expression::OrdinaryField<Field_endOffsetValue>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).endOffsetValue) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, inRangeColl) == 488);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inRangeColl>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 488;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowAggState, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState> as c::expression::OrdinaryField<Field_inRangeColl>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).inRangeColl) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, inRangeAsc) == 492);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inRangeAsc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 492;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowAggState, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState> as c::expression::OrdinaryField<Field_inRangeAsc>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).inRangeAsc) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, inRangeNullsFirst) == 493
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inRangeNullsFirst>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 493;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowAggState, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState> as c::expression::OrdinaryField<Field_inRangeNullsFirst>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).inRangeNullsFirst) };
#[doc(hidden)]
pub struct Field_currentgroup;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, currentgroup) == 496);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_currentgroup>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 496;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowAggState, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState> as c::expression::OrdinaryField<Field_currentgroup>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).currentgroup) };
#[doc(hidden)]
pub struct Field_frameheadgroup;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, frameheadgroup) == 504
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_frameheadgroup>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 504;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowAggState, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState> as c::expression::OrdinaryField<Field_frameheadgroup>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).frameheadgroup) };
#[doc(hidden)]
pub struct Field_frametailgroup;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, frametailgroup) == 512
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_frametailgroup>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 512;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowAggState, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState> as c::expression::OrdinaryField<Field_frametailgroup>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).frametailgroup) };
#[doc(hidden)]
pub struct Field_groupheadpos;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, groupheadpos) == 520);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_groupheadpos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 520;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowAggState, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState> as c::expression::OrdinaryField<Field_groupheadpos>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).groupheadpos) };
#[doc(hidden)]
pub struct Field_grouptailpos;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, grouptailpos) == 528);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_grouptailpos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 528;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowAggState, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState> as c::expression::OrdinaryField<Field_grouptailpos>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).grouptailpos) };
#[doc(hidden)]
pub struct Field_use_pass_through;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, use_pass_through) == 576
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_use_pass_through>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 576;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowAggState, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState> as c::expression::OrdinaryField<Field_use_pass_through>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).use_pass_through) };
#[doc(hidden)]
pub struct Field_top_window;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, top_window) == 577);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_top_window>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 577;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowAggState, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState> as c::expression::OrdinaryField<Field_top_window>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).top_window) };
#[doc(hidden)]
pub struct Field_all_first;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, all_first) == 578);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_all_first>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 578;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowAggState, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState> as c::expression::OrdinaryField<Field_all_first>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).all_first) };
#[doc(hidden)]
pub struct Field_partition_spooled;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, partition_spooled) == 579
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_partition_spooled>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 579;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowAggState, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState> as c::expression::OrdinaryField<Field_partition_spooled>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).partition_spooled) };
#[doc(hidden)]
pub struct Field_more_partitions;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, more_partitions) == 580
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_more_partitions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 580;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowAggState, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState> as c::expression::OrdinaryField<Field_more_partitions>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).more_partitions) };
#[doc(hidden)]
pub struct Field_framehead_valid;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, framehead_valid) == 581
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_framehead_valid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 581;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowAggState, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState> as c::expression::OrdinaryField<Field_framehead_valid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).framehead_valid) };
#[doc(hidden)]
pub struct Field_frametail_valid;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, frametail_valid) == 582
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_frametail_valid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 582;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowAggState, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState> as c::expression::OrdinaryField<Field_frametail_valid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).frametail_valid) };
#[doc(hidden)]
pub struct Field_grouptail_valid;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, grouptail_valid) == 583
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_grouptail_valid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 583;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowAggState, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState> as c::expression::OrdinaryField<Field_grouptail_valid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).grouptail_valid) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::WindowClause>() == 96);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::WindowClause>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::WindowClause {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowClause, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowClause>
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
const _: Projection<crate::__pgrx_c_bindings::WindowClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowClause> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_partitionClause;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowClause, partitionClause) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_partitionClause>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowClause>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowClause> as c::expression::OrdinaryField<Field_partitionClause>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).partitionClause) };
#[doc(hidden)]
pub struct Field_orderClause;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowClause, orderClause) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_orderClause>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowClause>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowClause> as c::expression::OrdinaryField<Field_orderClause>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).orderClause) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowClause, frameOptions) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_frameOptions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowClause>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowClause> as c::expression::OrdinaryField<Field_frameOptions>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).frameOptions) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowClause, runCondition) == 64);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_runCondition>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowClause>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowClause> as c::expression::OrdinaryField<Field_runCondition>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).runCondition) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowClause, startInRangeFunc) == 72
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_startInRangeFunc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowClause>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowClause> as c::expression::OrdinaryField<Field_startInRangeFunc>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).startInRangeFunc) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowClause, endInRangeFunc) == 76);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_endInRangeFunc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowClause>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 76;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowClause> as c::expression::OrdinaryField<Field_endInRangeFunc>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).endInRangeFunc) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowClause, inRangeColl) == 80);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inRangeColl>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowClause>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowClause> as c::expression::OrdinaryField<Field_inRangeColl>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).inRangeColl) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowClause, inRangeAsc) == 84);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inRangeAsc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowClause>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 84;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowClause> as c::expression::OrdinaryField<Field_inRangeAsc>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).inRangeAsc) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowClause, inRangeNullsFirst) == 85
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inRangeNullsFirst>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowClause>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 85;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowClause> as c::expression::OrdinaryField<Field_inRangeNullsFirst>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).inRangeNullsFirst) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowClause, winref) == 88);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_winref>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowClause>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowClause> as c::expression::OrdinaryField<Field_winref>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).winref) };
#[doc(hidden)]
pub struct Field_copiedOrder;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowClause, copiedOrder) == 92);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_copiedOrder>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowClause>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 92;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowClause> as c::expression::OrdinaryField<Field_copiedOrder>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).copiedOrder) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::WindowDef>() == 72);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::WindowDef>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::WindowDef {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowDef, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowDef>
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
    crate::__pgrx_c_bindings::WindowDef,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowDef> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowDef, partitionClause) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_partitionClause>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowDef>
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
    crate::__pgrx_c_bindings::WindowDef,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowDef> as c::expression::OrdinaryField<
        Field_partitionClause,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).partitionClause) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowDef, orderClause) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_orderClause>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowDef>
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
    crate::__pgrx_c_bindings::WindowDef,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowDef> as c::expression::OrdinaryField<
        Field_orderClause,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).orderClause) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowDef, frameOptions) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_frameOptions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowDef>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::WindowDef,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowDef> as c::expression::OrdinaryField<
        Field_frameOptions,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).frameOptions) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowDef, location) == 64);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowDef>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::WindowDef,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowDef> as c::expression::OrdinaryField<
        Field_location,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[doc(hidden)]
pub struct Field_winfnoid;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::WindowFunc>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::WindowFunc>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::WindowFunc {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowFunc, winfnoid) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_winfnoid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowFunc>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowFunc, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowFunc> as c::expression::OrdinaryField<Field_winfnoid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).winfnoid) };
#[doc(hidden)]
pub struct Field_wintype;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowFunc, wintype) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wintype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowFunc>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowFunc, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowFunc> as c::expression::OrdinaryField<Field_wintype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wintype) };
#[doc(hidden)]
pub struct Field_wincollid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowFunc, wincollid) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wincollid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowFunc>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowFunc, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowFunc> as c::expression::OrdinaryField<Field_wincollid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wincollid) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowFunc, inputcollid) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inputcollid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowFunc>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowFunc, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowFunc> as c::expression::OrdinaryField<Field_inputcollid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).inputcollid) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowFunc, args) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_args>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowFunc>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowFunc, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowFunc> as c::expression::OrdinaryField<Field_args>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).args) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowFunc, winref) == 40);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_winref>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowFunc>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowFunc, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowFunc> as c::expression::OrdinaryField<Field_winref>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).winref) };
#[doc(hidden)]
pub struct Field_winstar;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowFunc, winstar) == 44);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_winstar>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowFunc>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowFunc, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowFunc> as c::expression::OrdinaryField<Field_winstar>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).winstar) };
#[doc(hidden)]
pub struct Field_winagg;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowFunc, winagg) == 45);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_winagg>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowFunc>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 45;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowFunc, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowFunc> as c::expression::OrdinaryField<Field_winagg>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).winagg) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowFunc, location) == 48);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowFunc>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowFunc, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowFunc> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::WindowFuncExprState>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::WindowFuncExprState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::WindowFuncExprState {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowFuncExprState, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowFuncExprState>
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
const _: Projection<crate::__pgrx_c_bindings::WindowFuncExprState, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowFuncExprState> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowFuncExprState, args) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_args>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowFuncExprState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowFuncExprState, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowFuncExprState> as c::expression::OrdinaryField<Field_args>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).args) };
#[doc(hidden)]
pub struct Field_wfuncno;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowFuncExprState, wfuncno) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wfuncno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowFuncExprState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowFuncExprState, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowFuncExprState> as c::expression::OrdinaryField<Field_wfuncno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wfuncno) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::WithCheckOption>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::WithCheckOption>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::WithCheckOption {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WithCheckOption, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WithCheckOption>
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
const _: Projection<crate::__pgrx_c_bindings::WithCheckOption, <<c::expression::CRecord<crate::__pgrx_c_bindings::WithCheckOption> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WithCheckOption, kind) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_kind>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WithCheckOption>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__WCOKind,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WithCheckOption, <<c::expression::CRecord<crate::__pgrx_c_bindings::WithCheckOption> as c::expression::OrdinaryField<Field_kind>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).kind) };
#[doc(hidden)]
pub struct Field_cascaded;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WithCheckOption, cascaded) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cascaded>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WithCheckOption>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WithCheckOption, <<c::expression::CRecord<crate::__pgrx_c_bindings::WithCheckOption> as c::expression::OrdinaryField<Field_cascaded>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cascaded) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::WithClause>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::WithClause>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::WithClause {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WithClause, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WithClause>
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
const _: Projection<crate::__pgrx_c_bindings::WithClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::WithClause> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_ctes;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WithClause, ctes) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ctes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WithClause>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WithClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::WithClause> as c::expression::OrdinaryField<Field_ctes>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ctes) };
#[doc(hidden)]
pub struct Field_recursive;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WithClause, recursive) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_recursive>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WithClause>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WithClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::WithClause> as c::expression::OrdinaryField<Field_recursive>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).recursive) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WithClause, location) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WithClause>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WithClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::WithClause> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::WorkTableScan>() == 120);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::WorkTableScan>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::WorkTableScan {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WorkTableScan, wtParam) == 112);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wtParam>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WorkTableScan>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WorkTableScan, <<c::expression::CRecord<crate::__pgrx_c_bindings::WorkTableScan> as c::expression::OrdinaryField<Field_wtParam>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wtParam) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::WorkerInstrumentation>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::WorkerInstrumentation>() == 8);
};
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::WorkerInstrumentation {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WorkerInstrumentation, num_workers) == 0
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_workers>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WorkerInstrumentation>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WorkerInstrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::WorkerInstrumentation> as c::expression::OrdinaryField<Field_num_workers>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_workers) };
#[doc(hidden)]
pub struct Field_nr_pending;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::WritebackContext>() == 5136);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::WritebackContext>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::WritebackContext {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WritebackContext, nr_pending) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nr_pending>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WritebackContext>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WritebackContext, <<c::expression::CRecord<crate::__pgrx_c_bindings::WritebackContext> as c::expression::OrdinaryField<Field_nr_pending>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nr_pending) };
#[doc(hidden)]
pub struct Field_xlp_sysid;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::XLogLongPageHeaderData>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::XLogLongPageHeaderData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::XLogLongPageHeaderData {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogLongPageHeaderData, xlp_sysid) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xlp_sysid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogLongPageHeaderData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogLongPageHeaderData, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogLongPageHeaderData> as c::expression::OrdinaryField<Field_xlp_sysid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xlp_sysid) };
#[doc(hidden)]
pub struct Field_xlp_seg_size;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogLongPageHeaderData, xlp_seg_size) == 32
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xlp_seg_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogLongPageHeaderData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogLongPageHeaderData, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogLongPageHeaderData> as c::expression::OrdinaryField<Field_xlp_seg_size>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xlp_seg_size) };
#[doc(hidden)]
pub struct Field_xlp_xlog_blcksz;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogLongPageHeaderData, xlp_xlog_blcksz)
        == 36
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xlp_xlog_blcksz>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogLongPageHeaderData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogLongPageHeaderData, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogLongPageHeaderData> as c::expression::OrdinaryField<Field_xlp_xlog_blcksz>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xlp_xlog_blcksz) };
#[doc(hidden)]
pub struct Field_xlp_magic;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::XLogPageHeaderData>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::XLogPageHeaderData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::XLogPageHeaderData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogPageHeaderData, xlp_magic) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xlp_magic>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogPageHeaderData>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogPageHeaderData, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogPageHeaderData> as c::expression::OrdinaryField<Field_xlp_magic>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xlp_magic) };
#[doc(hidden)]
pub struct Field_xlp_info;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogPageHeaderData, xlp_info) == 2);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xlp_info>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogPageHeaderData>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 2;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogPageHeaderData, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogPageHeaderData> as c::expression::OrdinaryField<Field_xlp_info>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xlp_info) };
#[doc(hidden)]
pub struct Field_xlp_tli;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogPageHeaderData, xlp_tli) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xlp_tli>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogPageHeaderData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogPageHeaderData, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogPageHeaderData> as c::expression::OrdinaryField<Field_xlp_tli>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xlp_tli) };
#[doc(hidden)]
pub struct Field_xlp_pageaddr;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogPageHeaderData, xlp_pageaddr) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xlp_pageaddr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogPageHeaderData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogPageHeaderData, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogPageHeaderData> as c::expression::OrdinaryField<Field_xlp_pageaddr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xlp_pageaddr) };
#[doc(hidden)]
pub struct Field_xlp_rem_len;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogPageHeaderData, xlp_rem_len) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xlp_rem_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogPageHeaderData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogPageHeaderData, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogPageHeaderData> as c::expression::OrdinaryField<Field_xlp_rem_len>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xlp_rem_len) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::XLogReaderState>() == 1320);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::XLogReaderState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::XLogReaderState {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogReaderState, system_identifier) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_system_identifier>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogReaderState, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState> as c::expression::OrdinaryField<Field_system_identifier>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).system_identifier) };
#[doc(hidden)]
pub struct Field_ReadRecPtr;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogReaderState, ReadRecPtr) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ReadRecPtr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogReaderState, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState> as c::expression::OrdinaryField<Field_ReadRecPtr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ReadRecPtr) };
#[doc(hidden)]
pub struct Field_EndRecPtr;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogReaderState, EndRecPtr) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_EndRecPtr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogReaderState, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState> as c::expression::OrdinaryField<Field_EndRecPtr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).EndRecPtr) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogReaderState, abortedRecPtr) == 56
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_abortedRecPtr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogReaderState, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState> as c::expression::OrdinaryField<Field_abortedRecPtr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).abortedRecPtr) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogReaderState, missingContrecPtr) == 64
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_missingContrecPtr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogReaderState, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState> as c::expression::OrdinaryField<Field_missingContrecPtr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).missingContrecPtr) };
#[doc(hidden)]
pub struct Field_overwrittenRecPtr;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogReaderState, overwrittenRecPtr) == 72
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_overwrittenRecPtr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogReaderState, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState> as c::expression::OrdinaryField<Field_overwrittenRecPtr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).overwrittenRecPtr) };
#[doc(hidden)]
pub struct Field_DecodeRecPtr;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogReaderState, DecodeRecPtr) == 80);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_DecodeRecPtr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogReaderState, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState> as c::expression::OrdinaryField<Field_DecodeRecPtr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).DecodeRecPtr) };
#[doc(hidden)]
pub struct Field_NextRecPtr;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogReaderState, NextRecPtr) == 88);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_NextRecPtr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogReaderState, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState> as c::expression::OrdinaryField<Field_NextRecPtr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).NextRecPtr) };
#[doc(hidden)]
pub struct Field_PrevRecPtr;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogReaderState, PrevRecPtr) == 96);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_PrevRecPtr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogReaderState, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState> as c::expression::OrdinaryField<Field_PrevRecPtr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).PrevRecPtr) };
#[doc(hidden)]
pub struct Field_record;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogReaderState, record) == 104);
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::DecodedXLogRecord>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::DecodedXLogRecord>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_record>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::DecodedXLogRecord>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogReaderState, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState> as c::expression::OrdinaryField<Field_record>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).record) };
#[doc(hidden)]
pub struct Field_decode_buffer_size;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogReaderState, decode_buffer_size) == 120
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_decode_buffer_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogReaderState, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState> as c::expression::OrdinaryField<Field_decode_buffer_size>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).decode_buffer_size) };
#[doc(hidden)]
pub struct Field_free_decode_buffer;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogReaderState, free_decode_buffer) == 128
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_free_decode_buffer>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogReaderState, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState> as c::expression::OrdinaryField<Field_free_decode_buffer>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).free_decode_buffer) };
#[doc(hidden)]
pub struct Field_readLen;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogReaderState, readLen) == 176);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_readLen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 176;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogReaderState, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState> as c::expression::OrdinaryField<Field_readLen>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).readLen) };
#[doc(hidden)]
pub struct Field_segoff;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogReaderState, segoff) == 1232);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_segoff>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1232;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogReaderState, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState> as c::expression::OrdinaryField<Field_segoff>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).segoff) };
#[doc(hidden)]
pub struct Field_latestPagePtr;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogReaderState, latestPagePtr) == 1240
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_latestPagePtr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1240;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogReaderState, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState> as c::expression::OrdinaryField<Field_latestPagePtr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).latestPagePtr) };
#[doc(hidden)]
pub struct Field_latestPageTLI;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogReaderState, latestPageTLI) == 1248
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_latestPageTLI>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1248;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogReaderState, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState> as c::expression::OrdinaryField<Field_latestPageTLI>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).latestPageTLI) };
#[doc(hidden)]
pub struct Field_currRecPtr;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogReaderState, currRecPtr) == 1256);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_currRecPtr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1256;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogReaderState, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState> as c::expression::OrdinaryField<Field_currRecPtr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).currRecPtr) };
#[doc(hidden)]
pub struct Field_currTLI;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogReaderState, currTLI) == 1264);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_currTLI>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1264;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogReaderState, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState> as c::expression::OrdinaryField<Field_currTLI>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).currTLI) };
#[doc(hidden)]
pub struct Field_currTLIValidUntil;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogReaderState, currTLIValidUntil) == 1272
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_currTLIValidUntil>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1272;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogReaderState, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState> as c::expression::OrdinaryField<Field_currTLIValidUntil>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).currTLIValidUntil) };
#[doc(hidden)]
pub struct Field_nextTLI;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogReaderState, nextTLI) == 1280);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nextTLI>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1280;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogReaderState, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState> as c::expression::OrdinaryField<Field_nextTLI>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nextTLI) };
#[doc(hidden)]
pub struct Field_readRecordBufSize;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogReaderState, readRecordBufSize) == 1296
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_readRecordBufSize>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1296;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogReaderState, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState> as c::expression::OrdinaryField<Field_readRecordBufSize>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).readRecordBufSize) };
#[doc(hidden)]
pub struct Field_errormsg_deferred;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogReaderState, errormsg_deferred) == 1312
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_errormsg_deferred>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1312;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogReaderState, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState> as c::expression::OrdinaryField<Field_errormsg_deferred>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).errormsg_deferred) };
#[doc(hidden)]
pub struct Field_nonblocking;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogReaderState, nonblocking) == 1313
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nonblocking>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1313;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogReaderState, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogReaderState> as c::expression::OrdinaryField<Field_nonblocking>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nonblocking) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::XLogRecData>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::XLogRecData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::XLogRecData {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogRecData, data) == 8);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_data>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecData>
{
    type Member = c::expression::CPointer<
        c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogRecData, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecData> as c::expression::OrdinaryField<Field_data>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).data) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogRecData, len) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogRecData, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecData> as c::expression::OrdinaryField<Field_len>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).len) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::XLogRecStats>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::XLogRecStats>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::XLogRecStats {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogRecStats, count) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecStats>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogRecStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecStats> as c::expression::OrdinaryField<Field_count>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).count) };
#[doc(hidden)]
pub struct Field_rec_len;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogRecStats, rec_len) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rec_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecStats>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogRecStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecStats> as c::expression::OrdinaryField<Field_rec_len>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rec_len) };
#[doc(hidden)]
pub struct Field_fpi_len;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogRecStats, fpi_len) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fpi_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecStats>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogRecStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecStats> as c::expression::OrdinaryField<Field_fpi_len>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fpi_len) };
#[doc(hidden)]
pub struct Field_xl_tot_len;
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::XLogRecord {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogRecord, xl_tot_len) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xl_tot_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecord>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogRecord, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecord> as c::expression::OrdinaryField<Field_xl_tot_len>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xl_tot_len) };
#[doc(hidden)]
pub struct Field_xl_xid;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogRecord, xl_xid) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xl_xid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecord>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogRecord, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecord> as c::expression::OrdinaryField<Field_xl_xid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xl_xid) };
#[doc(hidden)]
pub struct Field_xl_prev;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogRecord, xl_prev) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xl_prev>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecord>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogRecord, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecord> as c::expression::OrdinaryField<Field_xl_prev>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xl_prev) };
#[doc(hidden)]
pub struct Field_xl_info;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogRecord, xl_info) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xl_info>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecord>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogRecord, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecord> as c::expression::OrdinaryField<Field_xl_info>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xl_info) };
#[doc(hidden)]
pub struct Field_xl_rmid;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogRecord, xl_rmid) == 17);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xl_rmid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecord>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 17;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogRecord, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecord> as c::expression::OrdinaryField<Field_xl_rmid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xl_rmid) };
#[doc(hidden)]
pub struct Field_xl_crc;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogRecord, xl_crc) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xl_crc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecord>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogRecord, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecord> as c::expression::OrdinaryField<Field_xl_crc>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xl_crc) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::XLogRecordBlockCompressHeader>() == 2);
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::XLogRecordBlockCompressHeader>() == 2
    );
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::XLogRecordBlockCompressHeader {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogRecordBlockCompressHeader, hole_length)
        == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hole_length>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecordBlockCompressHeader>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogRecordBlockCompressHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecordBlockCompressHeader> as c::expression::OrdinaryField<Field_hole_length>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hole_length) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::XLogRecordBlockHeader>() == 4);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::XLogRecordBlockHeader>() == 2);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::XLogRecordBlockHeader {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogRecordBlockHeader, id) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecordBlockHeader>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogRecordBlockHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecordBlockHeader> as c::expression::OrdinaryField<Field_id>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).id) };
#[doc(hidden)]
pub struct Field_fork_flags;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogRecordBlockHeader, fork_flags) == 1
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fork_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecordBlockHeader>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogRecordBlockHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecordBlockHeader> as c::expression::OrdinaryField<Field_fork_flags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fork_flags) };
#[doc(hidden)]
pub struct Field_data_length;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogRecordBlockHeader, data_length) == 2
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_data_length>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecordBlockHeader>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 2;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogRecordBlockHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecordBlockHeader> as c::expression::OrdinaryField<Field_data_length>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).data_length) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::XLogRecordBlockImageHeader>() == 6);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::XLogRecordBlockImageHeader>() == 2);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::XLogRecordBlockImageHeader {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogRecordBlockImageHeader, length) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_length>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecordBlockImageHeader>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogRecordBlockImageHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecordBlockImageHeader> as c::expression::OrdinaryField<Field_length>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).length) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogRecordBlockImageHeader, hole_offset) == 2
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hole_offset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecordBlockImageHeader>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 2;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogRecordBlockImageHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecordBlockImageHeader> as c::expression::OrdinaryField<Field_hole_offset>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hole_offset) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogRecordBlockImageHeader, bimg_info) == 4
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bimg_info>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecordBlockImageHeader>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogRecordBlockImageHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecordBlockImageHeader> as c::expression::OrdinaryField<Field_bimg_info>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).bimg_info) };
#[doc(hidden)]
pub struct Field_origptr;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::XLogRecordBuffer>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::XLogRecordBuffer>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::XLogRecordBuffer {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogRecordBuffer, origptr) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_origptr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecordBuffer>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogRecordBuffer, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecordBuffer> as c::expression::OrdinaryField<Field_origptr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).origptr) };
#[doc(hidden)]
pub struct Field_endptr;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogRecordBuffer, endptr) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_endptr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecordBuffer>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogRecordBuffer, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecordBuffer> as c::expression::OrdinaryField<Field_endptr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).endptr) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::XLogRecordDataHeaderLong>() == 1);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::XLogRecordDataHeaderLong>() == 1);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::XLogRecordDataHeaderLong {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogRecordDataHeaderLong, id) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecordDataHeaderLong>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogRecordDataHeaderLong, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecordDataHeaderLong> as c::expression::OrdinaryField<Field_id>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).id) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::XLogRecordDataHeaderShort>() == 2);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::XLogRecordDataHeaderShort>() == 1);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::XLogRecordDataHeaderShort {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogRecordDataHeaderShort, id) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecordDataHeaderShort>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogRecordDataHeaderShort, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecordDataHeaderShort> as c::expression::OrdinaryField<Field_id>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).id) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogRecordDataHeaderShort, data_length) == 1
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_data_length>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecordDataHeaderShort>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogRecordDataHeaderShort, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecordDataHeaderShort> as c::expression::OrdinaryField<Field_data_length>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).data_length) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::XLogStats>() == 104456);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::XLogStats>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::XLogStats {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogStats, count) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogStats>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::XLogStats,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogStats> as c::expression::OrdinaryField<
        Field_count,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).count) };
#[doc(hidden)]
pub struct Field_record_stats;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogStats, record_stats) == 6152);
const _: () = {
    assert!(::core::mem::size_of::<[[crate::__pgrx_c_bindings::XLogRecStats; 16]; 256]>() == 98304);
    assert!(::core::mem::align_of::<[[crate::__pgrx_c_bindings::XLogRecStats; 16]; 256]>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_record_stats>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogStats>
{
    type Member = c::expression::CArray<
        c::expression::CArray<c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecStats>, 16>,
        256,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 6152;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::XLogStats,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogStats> as c::expression::OrdinaryField<
        Field_record_stats,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).record_stats) };
#[doc(hidden)]
pub struct Field_xids;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::XidCache>() == 256);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::XidCache>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::XidCache {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XidCache, xids) == 0);
impl c::expression::OffsetField<Field_xids>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XidCache>
{
    type Member = ();
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::XidCache, xids);
}
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::XidCacheStatus>() == 2);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::XidCacheStatus>() == 1);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::XidCacheStatus {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XidCacheStatus, count) == 0);
impl c::expression::OffsetField<Field_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XidCacheStatus>
{
    type Member = ();
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::XidCacheStatus, count);
}
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XidCacheStatus>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XidCacheStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::XidCacheStatus> as c::expression::OrdinaryField<Field_count>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).count) };
#[doc(hidden)]
pub struct Field_overflowed;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XidCacheStatus, overflowed) == 1);
impl c::expression::OffsetField<Field_overflowed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XidCacheStatus>
{
    type Member = ();
    const OFFSET: usize =
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::XidCacheStatus, overflowed);
}
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_overflowed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XidCacheStatus>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XidCacheStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::XidCacheStatus> as c::expression::OrdinaryField<Field_overflowed>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).overflowed) };
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::XmlExpr>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::XmlExpr>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::XmlExpr {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XmlExpr, op) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_op>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XmlExpr>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__XmlExprOp,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::XmlExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::XmlExpr> as c::expression::OrdinaryField<
        Field_op,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).op) };
#[doc(hidden)]
pub struct Field_named_args;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XmlExpr, named_args) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_named_args>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XmlExpr>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::XmlExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::XmlExpr> as c::expression::OrdinaryField<
        Field_named_args,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).named_args) };
#[doc(hidden)]
pub struct Field_arg_names;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XmlExpr, arg_names) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_arg_names>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XmlExpr>
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
    crate::__pgrx_c_bindings::XmlExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::XmlExpr> as c::expression::OrdinaryField<
        Field_arg_names,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).arg_names) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XmlExpr, args) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_args>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XmlExpr>
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
    crate::__pgrx_c_bindings::XmlExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::XmlExpr> as c::expression::OrdinaryField<
        Field_args,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).args) };
#[doc(hidden)]
pub struct Field_xmloption;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XmlExpr, xmloption) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xmloption>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XmlExpr>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__XmlOptionType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::XmlExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::XmlExpr> as c::expression::OrdinaryField<
        Field_xmloption,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xmloption) };
#[cfg(feature = "pg15")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XmlExpr, type_) == 44);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XmlExpr>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::XmlExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::XmlExpr> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(feature = "pg15")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XmlExpr, typmod) == 48);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typmod>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XmlExpr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::XmlExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::XmlExpr> as c::expression::OrdinaryField<
        Field_typmod,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typmod) };
#[cfg(feature = "pg15")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XmlExpr, location) == 52);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XmlExpr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::XmlExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::XmlExpr> as c::expression::OrdinaryField<
        Field_location,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::XmlSerialize>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::XmlSerialize>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::XmlSerialize {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XmlSerialize, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XmlSerialize>
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
const _: Projection<crate::__pgrx_c_bindings::XmlSerialize, <<c::expression::CRecord<crate::__pgrx_c_bindings::XmlSerialize> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XmlSerialize, xmloption) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xmloption>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XmlSerialize>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__XmlOptionType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XmlSerialize, <<c::expression::CRecord<crate::__pgrx_c_bindings::XmlSerialize> as c::expression::OrdinaryField<Field_xmloption>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xmloption) };
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XmlSerialize, location) == 24);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XmlSerialize>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XmlSerialize, <<c::expression::CRecord<crate::__pgrx_c_bindings::XmlSerialize> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[doc(hidden)]
pub struct Field_mydest;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::_DestReceiver>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::_DestReceiver>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::_DestReceiver {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::_DestReceiver, mydest) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mydest>
    for c::expression::CRecord<crate::__pgrx_c_bindings::_DestReceiver>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_CommandDest,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::_DestReceiver, <<c::expression::CRecord<crate::__pgrx_c_bindings::_DestReceiver> as c::expression::OrdinaryField<Field_mydest>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mydest) };
#[doc(hidden)]
pub struct Field_pathpos;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::_FuncCandidateList>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::_FuncCandidateList>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::_FuncCandidateList {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::_FuncCandidateList, pathpos) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pathpos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::_FuncCandidateList>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::_FuncCandidateList, <<c::expression::CRecord<crate::__pgrx_c_bindings::_FuncCandidateList> as c::expression::OrdinaryField<Field_pathpos>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pathpos) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::_FuncCandidateList, oid) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::_FuncCandidateList>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::_FuncCandidateList, <<c::expression::CRecord<crate::__pgrx_c_bindings::_FuncCandidateList> as c::expression::OrdinaryField<Field_oid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oid) };
#[doc(hidden)]
pub struct Field_nominalnargs;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::_FuncCandidateList, nominalnargs) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nominalnargs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::_FuncCandidateList>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::_FuncCandidateList, <<c::expression::CRecord<crate::__pgrx_c_bindings::_FuncCandidateList> as c::expression::OrdinaryField<Field_nominalnargs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nominalnargs) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::_FuncCandidateList, nargs) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nargs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::_FuncCandidateList>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::_FuncCandidateList, <<c::expression::CRecord<crate::__pgrx_c_bindings::_FuncCandidateList> as c::expression::OrdinaryField<Field_nargs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nargs) };
#[doc(hidden)]
pub struct Field_nvargs;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::_FuncCandidateList, nvargs) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nvargs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::_FuncCandidateList>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::_FuncCandidateList, <<c::expression::CRecord<crate::__pgrx_c_bindings::_FuncCandidateList> as c::expression::OrdinaryField<Field_nvargs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nvargs) };
#[doc(hidden)]
pub struct Field_ndargs;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::_FuncCandidateList, ndargs) == 28);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ndargs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::_FuncCandidateList>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::_FuncCandidateList, <<c::expression::CRecord<crate::__pgrx_c_bindings::_FuncCandidateList> as c::expression::OrdinaryField<Field_ndargs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ndargs) };
#[doc(hidden)]
pub struct Field__flags;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::_IO_FILE>() == 216);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::_IO_FILE>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::_IO_FILE {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::_IO_FILE, _flags) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field__flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::_IO_FILE>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::_IO_FILE,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::_IO_FILE> as c::expression::OrdinaryField<
        Field__flags,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base)._flags) };
#[doc(hidden)]
pub struct Field__fileno;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::_IO_FILE, _fileno) == 112);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field__fileno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::_IO_FILE>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::_IO_FILE,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::_IO_FILE> as c::expression::OrdinaryField<
        Field__fileno,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base)._fileno) };
#[doc(hidden)]
pub struct Field__flags2;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::_IO_FILE, _flags2) == 116);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field__flags2>
    for c::expression::CRecord<crate::__pgrx_c_bindings::_IO_FILE>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 116;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::_IO_FILE,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::_IO_FILE> as c::expression::OrdinaryField<
        Field__flags2,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base)._flags2) };
#[doc(hidden)]
pub struct Field__old_offset;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::_IO_FILE, _old_offset) == 120);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field__old_offset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::_IO_FILE>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::_IO_FILE,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::_IO_FILE> as c::expression::OrdinaryField<
        Field__old_offset,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base)._old_offset) };
#[doc(hidden)]
pub struct Field__cur_column;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::_IO_FILE, _cur_column) == 128);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field__cur_column>
    for c::expression::CRecord<crate::__pgrx_c_bindings::_IO_FILE>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::_IO_FILE,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::_IO_FILE> as c::expression::OrdinaryField<
        Field__cur_column,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base)._cur_column) };
#[doc(hidden)]
pub struct Field__vtable_offset;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::_IO_FILE, _vtable_offset) == 130);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field__vtable_offset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::_IO_FILE>
{
    type Member = c::CSignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 130;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::_IO_FILE,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::_IO_FILE> as c::expression::OrdinaryField<
        Field__vtable_offset,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base)._vtable_offset) };
#[doc(hidden)]
pub struct Field__offset;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::_IO_FILE, _offset) == 144);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field__offset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::_IO_FILE>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::_IO_FILE,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::_IO_FILE> as c::expression::OrdinaryField<
        Field__offset,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base)._offset) };
#[doc(hidden)]
pub struct Field___pad5;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::_IO_FILE, __pad5) == 184);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field___pad5>
    for c::expression::CRecord<crate::__pgrx_c_bindings::_IO_FILE>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 184;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::_IO_FILE,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::_IO_FILE> as c::expression::OrdinaryField<
        Field___pad5,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).__pad5) };
#[doc(hidden)]
pub struct Field__mode;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::_IO_FILE, _mode) == 192);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field__mode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::_IO_FILE>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 192;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::_IO_FILE,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::_IO_FILE> as c::expression::OrdinaryField<
        Field__mode,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base)._mode) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::_SPI_plan>() == 64);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::_SPI_plan>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::_SPI_plan {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::_SPI_plan, magic) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_magic>
    for c::expression::CRecord<crate::__pgrx_c_bindings::_SPI_plan>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::_SPI_plan,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::_SPI_plan> as c::expression::OrdinaryField<
        Field_magic,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).magic) };
#[doc(hidden)]
pub struct Field_saved;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::_SPI_plan, saved) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_saved>
    for c::expression::CRecord<crate::__pgrx_c_bindings::_SPI_plan>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::_SPI_plan,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::_SPI_plan> as c::expression::OrdinaryField<
        Field_saved,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).saved) };
#[doc(hidden)]
pub struct Field_oneshot;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::_SPI_plan, oneshot) == 5);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oneshot>
    for c::expression::CRecord<crate::__pgrx_c_bindings::_SPI_plan>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 5;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::_SPI_plan,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::_SPI_plan> as c::expression::OrdinaryField<
        Field_oneshot,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oneshot) };
#[doc(hidden)]
pub struct Field_plancache_list;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::_SPI_plan, plancache_list) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_plancache_list>
    for c::expression::CRecord<crate::__pgrx_c_bindings::_SPI_plan>
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
    crate::__pgrx_c_bindings::_SPI_plan,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::_SPI_plan> as c::expression::OrdinaryField<
        Field_plancache_list,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).plancache_list) };
#[doc(hidden)]
pub struct Field_parse_mode;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::_SPI_plan, parse_mode) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parse_mode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::_SPI_plan>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_RawParseMode,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::_SPI_plan,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::_SPI_plan> as c::expression::OrdinaryField<
        Field_parse_mode,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).parse_mode) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::_SPI_plan, cursor_options) == 28);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cursor_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::_SPI_plan>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::_SPI_plan,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::_SPI_plan> as c::expression::OrdinaryField<
        Field_cursor_options,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cursor_options) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::_SPI_plan, nargs) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nargs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::_SPI_plan>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::_SPI_plan,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::_SPI_plan> as c::expression::OrdinaryField<
        Field_nargs,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nargs) };
#[doc(hidden)]
pub struct Field___mask_was_saved;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::__jmp_buf_tag>() == 200);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::__jmp_buf_tag>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::__jmp_buf_tag {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::__jmp_buf_tag, __mask_was_saved) == 64
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field___mask_was_saved>
    for c::expression::CRecord<crate::__pgrx_c_bindings::__jmp_buf_tag>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::__jmp_buf_tag, <<c::expression::CRecord<crate::__pgrx_c_bindings::__jmp_buf_tag> as c::expression::OrdinaryField<Field___mask_was_saved>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).__mask_was_saved) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_have_lookahead;
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::base_yy_extra_type>() == 136);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::base_yy_extra_type>() == 8);
};
#[cfg(feature = "pg15")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::base_yy_extra_type {}
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::base_yy_extra_type, have_lookahead) == 88
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_have_lookahead>
    for c::expression::CRecord<crate::__pgrx_c_bindings::base_yy_extra_type>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::base_yy_extra_type, <<c::expression::CRecord<crate::__pgrx_c_bindings::base_yy_extra_type> as c::expression::OrdinaryField<Field_have_lookahead>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).have_lookahead) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_lookahead_token;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::base_yy_extra_type, lookahead_token) == 92
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lookahead_token>
    for c::expression::CRecord<crate::__pgrx_c_bindings::base_yy_extra_type>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 92;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::base_yy_extra_type, <<c::expression::CRecord<crate::__pgrx_c_bindings::base_yy_extra_type> as c::expression::OrdinaryField<Field_lookahead_token>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lookahead_token) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_lookahead_yylloc;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::base_yy_extra_type, lookahead_yylloc) == 104
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lookahead_yylloc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::base_yy_extra_type>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::base_yy_extra_type, <<c::expression::CRecord<crate::__pgrx_c_bindings::base_yy_extra_type> as c::expression::OrdinaryField<Field_lookahead_yylloc>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lookahead_yylloc) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_lookahead_hold_char;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::base_yy_extra_type, lookahead_hold_char)
        == 120
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lookahead_hold_char>
    for c::expression::CRecord<crate::__pgrx_c_bindings::base_yy_extra_type>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::base_yy_extra_type, <<c::expression::CRecord<crate::__pgrx_c_bindings::base_yy_extra_type> as c::expression::OrdinaryField<Field_lookahead_hold_char>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lookahead_hold_char) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_parsetree;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::base_yy_extra_type, parsetree) == 128
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parsetree>
    for c::expression::CRecord<crate::__pgrx_c_bindings::base_yy_extra_type>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::base_yy_extra_type, <<c::expression::CRecord<crate::__pgrx_c_bindings::base_yy_extra_type> as c::expression::OrdinaryField<Field_parsetree>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).parsetree) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_rnode;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::buftag>() == 20);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::buftag>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::buftag {}
#[cfg(feature = "pg15")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::buftag, rnode) == 0);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rnode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::buftag>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::RelFileNode>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::buftag,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::buftag> as c::expression::OrdinaryField<
        Field_rnode,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rnode) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::buftag, forkNum) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_forkNum>
    for c::expression::CRecord<crate::__pgrx_c_bindings::buftag>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ForkNumber,
        c::CInt,
        i32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::buftag,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::buftag> as c::expression::OrdinaryField<
        Field_forkNum,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).forkNum) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::buftag, blockNum) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_blockNum>
    for c::expression::CRecord<crate::__pgrx_c_bindings::buftag>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::buftag,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::buftag> as c::expression::OrdinaryField<
        Field_blockNum,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).blockNum) };
#[cfg(any(feature = "pg15", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::catcache>() == 448);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::catcache>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::catcache {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::catcache, id) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::catcache>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::catcache,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::catcache> as c::expression::OrdinaryField<
        Field_id,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).id) };
#[doc(hidden)]
pub struct Field_cc_nbuckets;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::catcache, cc_nbuckets) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cc_nbuckets>
    for c::expression::CRecord<crate::__pgrx_c_bindings::catcache>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::catcache,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::catcache> as c::expression::OrdinaryField<
        Field_cc_nbuckets,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cc_nbuckets) };
#[doc(hidden)]
pub struct Field_cc_ntup;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::catcache, cc_ntup) == 120);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cc_ntup>
    for c::expression::CRecord<crate::__pgrx_c_bindings::catcache>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::catcache,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::catcache> as c::expression::OrdinaryField<
        Field_cc_ntup,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cc_ntup) };
#[doc(hidden)]
pub struct Field_cc_nkeys;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::catcache, cc_nkeys) == 124);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cc_nkeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::catcache>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 124;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::catcache,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::catcache> as c::expression::OrdinaryField<
        Field_cc_nkeys,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cc_nkeys) };
#[doc(hidden)]
pub struct Field_cc_reloid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::catcache, cc_reloid) == 136);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cc_reloid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::catcache>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::catcache,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::catcache> as c::expression::OrdinaryField<
        Field_cc_reloid,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cc_reloid) };
#[doc(hidden)]
pub struct Field_cc_indexoid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::catcache, cc_indexoid) == 140);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cc_indexoid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::catcache>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 140;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::catcache,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::catcache> as c::expression::OrdinaryField<
        Field_cc_indexoid,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cc_indexoid) };
#[doc(hidden)]
pub struct Field_cc_relisshared;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::catcache, cc_relisshared) == 144);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cc_relisshared>
    for c::expression::CRecord<crate::__pgrx_c_bindings::catcache>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::catcache,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::catcache> as c::expression::OrdinaryField<
        Field_cc_relisshared,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cc_relisshared) };
#[doc(hidden)]
pub struct Field_ch_ntup;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::catcacheheader>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::catcacheheader>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::catcacheheader {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::catcacheheader, ch_ntup) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ch_ntup>
    for c::expression::CRecord<crate::__pgrx_c_bindings::catcacheheader>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::catcacheheader, <<c::expression::CRecord<crate::__pgrx_c_bindings::catcacheheader> as c::expression::OrdinaryField<Field_ch_ntup>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ch_ntup) };
#[doc(hidden)]
pub struct Field_cl_magic;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::catclist>() == 80);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::catclist>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::catclist {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::catclist, cl_magic) == 0);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cl_magic>
    for c::expression::CRecord<crate::__pgrx_c_bindings::catclist>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::catclist,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::catclist> as c::expression::OrdinaryField<
        Field_cl_magic,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cl_magic) };
#[doc(hidden)]
pub struct Field_hash_value;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::catclist, hash_value) == 4);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash_value>
    for c::expression::CRecord<crate::__pgrx_c_bindings::catclist>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::catclist,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::catclist> as c::expression::OrdinaryField<
        Field_hash_value,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hash_value) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::catclist, refcount) == 56);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_refcount>
    for c::expression::CRecord<crate::__pgrx_c_bindings::catclist>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::catclist,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::catclist> as c::expression::OrdinaryField<
        Field_refcount,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).refcount) };
#[doc(hidden)]
pub struct Field_dead;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::catclist, dead) == 60);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dead>
    for c::expression::CRecord<crate::__pgrx_c_bindings::catclist>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 60;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::catclist,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::catclist> as c::expression::OrdinaryField<
        Field_dead,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dead) };
#[doc(hidden)]
pub struct Field_ordered;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::catclist, ordered) == 61);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ordered>
    for c::expression::CRecord<crate::__pgrx_c_bindings::catclist>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 61;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::catclist,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::catclist> as c::expression::OrdinaryField<
        Field_ordered,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ordered) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::catclist, nkeys) == 62);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nkeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::catclist>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 62;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::catclist,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::catclist> as c::expression::OrdinaryField<
        Field_nkeys,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nkeys) };
#[doc(hidden)]
pub struct Field_n_members;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::catclist, n_members) == 64);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_n_members>
    for c::expression::CRecord<crate::__pgrx_c_bindings::catclist>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::catclist,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::catclist> as c::expression::OrdinaryField<
        Field_n_members,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).n_members) };
#[doc(hidden)]
pub struct Field_ct_magic;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::catctup>() == 104);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::catctup>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::catctup {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::catctup, ct_magic) == 0);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ct_magic>
    for c::expression::CRecord<crate::__pgrx_c_bindings::catctup>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::catctup,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::catctup> as c::expression::OrdinaryField<
        Field_ct_magic,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ct_magic) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::catctup, hash_value) == 4);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash_value>
    for c::expression::CRecord<crate::__pgrx_c_bindings::catctup>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::catctup,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::catctup> as c::expression::OrdinaryField<
        Field_hash_value,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hash_value) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::catctup, refcount) == 56);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_refcount>
    for c::expression::CRecord<crate::__pgrx_c_bindings::catctup>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::catctup,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::catctup> as c::expression::OrdinaryField<
        Field_refcount,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).refcount) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::catctup, dead) == 60);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dead>
    for c::expression::CRecord<crate::__pgrx_c_bindings::catctup>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 60;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::catctup,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::catctup> as c::expression::OrdinaryField<
        Field_dead,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dead) };
#[doc(hidden)]
pub struct Field_negative;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::catctup, negative) == 61);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_negative>
    for c::expression::CRecord<crate::__pgrx_c_bindings::catctup>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 61;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::catctup,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::catctup> as c::expression::OrdinaryField<
        Field_negative,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).negative) };
#[doc(hidden)]
pub struct Field_boot_val;
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::config_bool>() == 168);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::config_bool>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::config_bool {}
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_bool, boot_val) == 120);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_boot_val>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_bool>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::config_bool, <<c::expression::CRecord<crate::__pgrx_c_bindings::config_bool> as c::expression::OrdinaryField<Field_boot_val>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).boot_val) };
#[doc(hidden)]
pub struct Field_reset_val;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_bool, reset_val) == 152);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_reset_val>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_bool>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::config_bool, <<c::expression::CRecord<crate::__pgrx_c_bindings::config_bool> as c::expression::OrdinaryField<Field_reset_val>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).reset_val) };
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::config_enum>() == 176);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::config_enum>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::config_enum {}
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_enum, boot_val) == 120);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_boot_val>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_enum>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::config_enum, <<c::expression::CRecord<crate::__pgrx_c_bindings::config_enum> as c::expression::OrdinaryField<Field_boot_val>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).boot_val) };
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_enum, reset_val) == 160);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_reset_val>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_enum>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::config_enum, <<c::expression::CRecord<crate::__pgrx_c_bindings::config_enum> as c::expression::OrdinaryField<Field_reset_val>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).reset_val) };
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::config_generic>() == 112);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::config_generic>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::config_generic {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_generic, context) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_context>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_generic>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_GucContext,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::config_generic, <<c::expression::CRecord<crate::__pgrx_c_bindings::config_generic> as c::expression::OrdinaryField<Field_context>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).context) };
#[doc(hidden)]
pub struct Field_group;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_generic, group) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_group>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_generic>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__config_group,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::config_generic, <<c::expression::CRecord<crate::__pgrx_c_bindings::config_generic> as c::expression::OrdinaryField<Field_group>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).group) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_generic, flags) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_generic>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::config_generic, <<c::expression::CRecord<crate::__pgrx_c_bindings::config_generic> as c::expression::OrdinaryField<Field_flags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).flags) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_generic, vartype) == 36);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vartype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_generic>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__config_type,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::config_generic, <<c::expression::CRecord<crate::__pgrx_c_bindings::config_generic> as c::expression::OrdinaryField<Field_vartype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).vartype) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_generic, status) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_status>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_generic>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::config_generic, <<c::expression::CRecord<crate::__pgrx_c_bindings::config_generic> as c::expression::OrdinaryField<Field_status>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).status) };
#[doc(hidden)]
pub struct Field_source;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_generic, source) == 44);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_source>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_generic>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_GucSource,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::config_generic, <<c::expression::CRecord<crate::__pgrx_c_bindings::config_generic> as c::expression::OrdinaryField<Field_source>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).source) };
#[doc(hidden)]
pub struct Field_reset_source;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_generic, reset_source) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_reset_source>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_generic>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_GucSource,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::config_generic, <<c::expression::CRecord<crate::__pgrx_c_bindings::config_generic> as c::expression::OrdinaryField<Field_reset_source>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).reset_source) };
#[doc(hidden)]
pub struct Field_scontext;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_generic, scontext) == 52);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_scontext>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_generic>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_GucContext,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::config_generic, <<c::expression::CRecord<crate::__pgrx_c_bindings::config_generic> as c::expression::OrdinaryField<Field_scontext>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).scontext) };
#[doc(hidden)]
pub struct Field_reset_scontext;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::config_generic, reset_scontext) == 56
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_reset_scontext>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_generic>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_GucContext,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::config_generic, <<c::expression::CRecord<crate::__pgrx_c_bindings::config_generic> as c::expression::OrdinaryField<Field_reset_scontext>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).reset_scontext) };
#[doc(hidden)]
pub struct Field_srole;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_generic, srole) == 60);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_srole>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_generic>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 60;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::config_generic, <<c::expression::CRecord<crate::__pgrx_c_bindings::config_generic> as c::expression::OrdinaryField<Field_srole>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).srole) };
#[doc(hidden)]
pub struct Field_reset_srole;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_generic, reset_srole) == 64);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_reset_srole>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_generic>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::config_generic, <<c::expression::CRecord<crate::__pgrx_c_bindings::config_generic> as c::expression::OrdinaryField<Field_reset_srole>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).reset_srole) };
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_generic, sourceline) == 104);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sourceline>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_generic>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::config_generic, <<c::expression::CRecord<crate::__pgrx_c_bindings::config_generic> as c::expression::OrdinaryField<Field_sourceline>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sourceline) };
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::config_int>() == 176);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::config_int>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::config_int {}
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_int, boot_val) == 120);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_boot_val>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_int>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::config_int, <<c::expression::CRecord<crate::__pgrx_c_bindings::config_int> as c::expression::OrdinaryField<Field_boot_val>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).boot_val) };
#[doc(hidden)]
pub struct Field_min;
#[cfg(feature = "pg15")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_int, min) == 124);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_min>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_int>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 124;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::config_int, <<c::expression::CRecord<crate::__pgrx_c_bindings::config_int> as c::expression::OrdinaryField<Field_min>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).min) };
#[doc(hidden)]
pub struct Field_max;
#[cfg(feature = "pg15")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_int, max) == 128);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_max>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_int>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::config_int, <<c::expression::CRecord<crate::__pgrx_c_bindings::config_int> as c::expression::OrdinaryField<Field_max>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).max) };
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_int, reset_val) == 160);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_reset_val>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_int>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::config_int, <<c::expression::CRecord<crate::__pgrx_c_bindings::config_int> as c::expression::OrdinaryField<Field_reset_val>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).reset_val) };
#[doc(hidden)]
pub struct Field_scanbuflen;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::core_yy_extra_type>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::core_yy_extra_type>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::core_yy_extra_type {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::core_yy_extra_type, scanbuflen) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_scanbuflen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::core_yy_extra_type>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::core_yy_extra_type, <<c::expression::CRecord<crate::__pgrx_c_bindings::core_yy_extra_type> as c::expression::OrdinaryField<Field_scanbuflen>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).scanbuflen) };
#[doc(hidden)]
pub struct Field_backslash_quote;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::core_yy_extra_type, backslash_quote) == 32
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_backslash_quote>
    for c::expression::CRecord<crate::__pgrx_c_bindings::core_yy_extra_type>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::core_yy_extra_type, <<c::expression::CRecord<crate::__pgrx_c_bindings::core_yy_extra_type> as c::expression::OrdinaryField<Field_backslash_quote>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).backslash_quote) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
pub struct Field_escape_string_warning;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::core_yy_extra_type, escape_string_warning)
        == 36
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_escape_string_warning>
    for c::expression::CRecord<crate::__pgrx_c_bindings::core_yy_extra_type>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::core_yy_extra_type, <<c::expression::CRecord<crate::__pgrx_c_bindings::core_yy_extra_type> as c::expression::OrdinaryField<Field_escape_string_warning>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).escape_string_warning) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
pub struct Field_standard_conforming_strings;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::core_yy_extra_type,
        standard_conforming_strings
    ) == 37
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_standard_conforming_strings>
    for c::expression::CRecord<crate::__pgrx_c_bindings::core_yy_extra_type>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 37;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::core_yy_extra_type, <<c::expression::CRecord<crate::__pgrx_c_bindings::core_yy_extra_type> as c::expression::OrdinaryField<Field_standard_conforming_strings>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).standard_conforming_strings) };
#[doc(hidden)]
pub struct Field_literallen;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::core_yy_extra_type, literallen) == 48
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_literallen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::core_yy_extra_type>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::core_yy_extra_type, <<c::expression::CRecord<crate::__pgrx_c_bindings::core_yy_extra_type> as c::expression::OrdinaryField<Field_literallen>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).literallen) };
#[doc(hidden)]
pub struct Field_literalalloc;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::core_yy_extra_type, literalalloc) == 52
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_literalalloc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::core_yy_extra_type>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::core_yy_extra_type, <<c::expression::CRecord<crate::__pgrx_c_bindings::core_yy_extra_type> as c::expression::OrdinaryField<Field_literalalloc>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).literalalloc) };
#[doc(hidden)]
pub struct Field_state_before_str_stop;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::core_yy_extra_type, state_before_str_stop)
        == 56
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_state_before_str_stop>
    for c::expression::CRecord<crate::__pgrx_c_bindings::core_yy_extra_type>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::core_yy_extra_type, <<c::expression::CRecord<crate::__pgrx_c_bindings::core_yy_extra_type> as c::expression::OrdinaryField<Field_state_before_str_stop>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).state_before_str_stop) };
#[doc(hidden)]
pub struct Field_xcdepth;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::core_yy_extra_type, xcdepth) == 60);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xcdepth>
    for c::expression::CRecord<crate::__pgrx_c_bindings::core_yy_extra_type>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 60;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::core_yy_extra_type, <<c::expression::CRecord<crate::__pgrx_c_bindings::core_yy_extra_type> as c::expression::OrdinaryField<Field_xcdepth>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xcdepth) };
#[doc(hidden)]
pub struct Field_save_yylloc;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::core_yy_extra_type, save_yylloc) == 72
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_save_yylloc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::core_yy_extra_type>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::core_yy_extra_type, <<c::expression::CRecord<crate::__pgrx_c_bindings::core_yy_extra_type> as c::expression::OrdinaryField<Field_save_yylloc>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).save_yylloc) };
#[doc(hidden)]
pub struct Field_utf16_first_part;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::core_yy_extra_type, utf16_first_part) == 76
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_utf16_first_part>
    for c::expression::CRecord<crate::__pgrx_c_bindings::core_yy_extra_type>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 76;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::core_yy_extra_type, <<c::expression::CRecord<crate::__pgrx_c_bindings::core_yy_extra_type> as c::expression::OrdinaryField<Field_utf16_first_part>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).utf16_first_part) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
pub struct Field_warn_on_first_escape;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::core_yy_extra_type, warn_on_first_escape)
        == 80
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_warn_on_first_escape>
    for c::expression::CRecord<crate::__pgrx_c_bindings::core_yy_extra_type>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::core_yy_extra_type, <<c::expression::CRecord<crate::__pgrx_c_bindings::core_yy_extra_type> as c::expression::OrdinaryField<Field_warn_on_first_escape>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).warn_on_first_escape) };
#[doc(hidden)]
pub struct Field_saw_non_ascii;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::core_yy_extra_type, saw_non_ascii) == 81
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_saw_non_ascii>
    for c::expression::CRecord<crate::__pgrx_c_bindings::core_yy_extra_type>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 81;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::core_yy_extra_type, <<c::expression::CRecord<crate::__pgrx_c_bindings::core_yy_extra_type> as c::expression::OrdinaryField<Field_saw_non_ascii>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).saw_non_ascii) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::dlist_head>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::dlist_head>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::dlist_head {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::dlist_head, head) == 0);
impl c::expression::OffsetField<Field_head>
    for c::expression::CRecord<crate::__pgrx_c_bindings::dlist_head>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::dlist_node>;
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::dlist_head, head);
}
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::dlist_node>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::dlist_node>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_head>
    for c::expression::CRecord<crate::__pgrx_c_bindings::dlist_head>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::dlist_node>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::dlist_head, <<c::expression::CRecord<crate::__pgrx_c_bindings::dlist_head> as c::expression::OrdinaryField<Field_head>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).head) };
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::dlist_node {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::dlist_node, prev) == 0);
impl c::expression::OffsetField<Field_prev>
    for c::expression::CRecord<crate::__pgrx_c_bindings::dlist_node>
{
    type Member = ();
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::dlist_node, prev);
}
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::dlist_node>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::dlist_node>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_prev>
    for c::expression::CRecord<crate::__pgrx_c_bindings::dlist_node>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::dlist_node>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::dlist_node, <<c::expression::CRecord<crate::__pgrx_c_bindings::dlist_node> as c::expression::OrdinaryField<Field_prev>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).prev) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::dlist_node, next) == 8);
impl c::expression::OffsetField<Field_next>
    for c::expression::CRecord<crate::__pgrx_c_bindings::dlist_node>
{
    type Member = ();
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::dlist_node, next);
}
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_next>
    for c::expression::CRecord<crate::__pgrx_c_bindings::dlist_node>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::dlist_node>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::dlist_node, <<c::expression::CRecord<crate::__pgrx_c_bindings::dlist_node> as c::expression::OrdinaryField<Field_next>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).next) };
#[doc(hidden)]
pub struct Field_curbucket;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::dshash_seq_status>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::dshash_seq_status>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::dshash_seq_status {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::dshash_seq_status, curbucket) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_curbucket>
    for c::expression::CRecord<crate::__pgrx_c_bindings::dshash_seq_status>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::dshash_seq_status, <<c::expression::CRecord<crate::__pgrx_c_bindings::dshash_seq_status> as c::expression::OrdinaryField<Field_curbucket>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).curbucket) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::dshash_seq_status, nbuckets) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nbuckets>
    for c::expression::CRecord<crate::__pgrx_c_bindings::dshash_seq_status>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::dshash_seq_status, <<c::expression::CRecord<crate::__pgrx_c_bindings::dshash_seq_status> as c::expression::OrdinaryField<Field_nbuckets>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nbuckets) };
#[doc(hidden)]
pub struct Field_pnextitem;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::dshash_seq_status, pnextitem) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pnextitem>
    for c::expression::CRecord<crate::__pgrx_c_bindings::dshash_seq_status>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::dshash_seq_status, <<c::expression::CRecord<crate::__pgrx_c_bindings::dshash_seq_status> as c::expression::OrdinaryField<Field_pnextitem>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pnextitem) };
#[doc(hidden)]
pub struct Field_curpartition;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::dshash_seq_status, curpartition) == 32
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_curpartition>
    for c::expression::CRecord<crate::__pgrx_c_bindings::dshash_seq_status>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::dshash_seq_status, <<c::expression::CRecord<crate::__pgrx_c_bindings::dshash_seq_status> as c::expression::OrdinaryField<Field_curpartition>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).curpartition) };
#[doc(hidden)]
pub struct Field_exclusive;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::dshash_seq_status, exclusive) == 36);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_exclusive>
    for c::expression::CRecord<crate::__pgrx_c_bindings::dshash_seq_status>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::dshash_seq_status, <<c::expression::CRecord<crate::__pgrx_c_bindings::dshash_seq_status> as c::expression::OrdinaryField<Field_exclusive>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).exclusive) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ginxlogCreatePostingTree>() == 4);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ginxlogCreatePostingTree>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ginxlogCreatePostingTree {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ginxlogCreatePostingTree, size) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ginxlogCreatePostingTree>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ginxlogCreatePostingTree, <<c::expression::CRecord<crate::__pgrx_c_bindings::ginxlogCreatePostingTree> as c::expression::OrdinaryField<Field_size>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).size) };
#[doc(hidden)]
pub struct Field_ndeleted;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ginxlogDeleteListPages>() == 64);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ginxlogDeleteListPages>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ginxlogDeleteListPages {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ginxlogDeleteListPages, ndeleted) == 56
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ndeleted>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ginxlogDeleteListPages>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ginxlogDeleteListPages, <<c::expression::CRecord<crate::__pgrx_c_bindings::ginxlogDeleteListPages> as c::expression::OrdinaryField<Field_ndeleted>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ndeleted) };
#[doc(hidden)]
pub struct Field_parentOffset;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ginxlogDeletePage>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ginxlogDeletePage>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ginxlogDeletePage {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ginxlogDeletePage, parentOffset) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parentOffset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ginxlogDeletePage>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ginxlogDeletePage, <<c::expression::CRecord<crate::__pgrx_c_bindings::ginxlogDeletePage> as c::expression::OrdinaryField<Field_parentOffset>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).parentOffset) };
#[doc(hidden)]
pub struct Field_rightLink;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ginxlogDeletePage, rightLink) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rightLink>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ginxlogDeletePage>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ginxlogDeletePage, <<c::expression::CRecord<crate::__pgrx_c_bindings::ginxlogDeletePage> as c::expression::OrdinaryField<Field_rightLink>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rightLink) };
#[doc(hidden)]
pub struct Field_deleteXid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ginxlogDeletePage, deleteXid) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_deleteXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ginxlogDeletePage>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ginxlogDeletePage, <<c::expression::CRecord<crate::__pgrx_c_bindings::ginxlogDeletePage> as c::expression::OrdinaryField<Field_deleteXid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).deleteXid) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ginxlogInsertListPage>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ginxlogInsertListPage>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ginxlogInsertListPage {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ginxlogInsertListPage, rightlink) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rightlink>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ginxlogInsertListPage>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ginxlogInsertListPage, <<c::expression::CRecord<crate::__pgrx_c_bindings::ginxlogInsertListPage> as c::expression::OrdinaryField<Field_rightlink>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rightlink) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ginxlogInsertListPage, ntuples) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ntuples>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ginxlogInsertListPage>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ginxlogInsertListPage, <<c::expression::CRecord<crate::__pgrx_c_bindings::ginxlogInsertListPage> as c::expression::OrdinaryField<Field_ntuples>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ntuples) };
#[doc(hidden)]
pub struct Field_rrlink;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ginxlogSplit>() == 28);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ginxlogSplit>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ginxlogSplit {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ginxlogSplit, rrlink) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rrlink>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ginxlogSplit>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ginxlogSplit, <<c::expression::CRecord<crate::__pgrx_c_bindings::ginxlogSplit> as c::expression::OrdinaryField<Field_rrlink>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rrlink) };
#[doc(hidden)]
pub struct Field_leftChildBlkno;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ginxlogSplit, leftChildBlkno) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_leftChildBlkno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ginxlogSplit>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ginxlogSplit, <<c::expression::CRecord<crate::__pgrx_c_bindings::ginxlogSplit> as c::expression::OrdinaryField<Field_leftChildBlkno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).leftChildBlkno) };
#[doc(hidden)]
pub struct Field_rightChildBlkno;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ginxlogSplit, rightChildBlkno) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rightChildBlkno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ginxlogSplit>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ginxlogSplit, <<c::expression::CRecord<crate::__pgrx_c_bindings::ginxlogSplit> as c::expression::OrdinaryField<Field_rightChildBlkno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rightChildBlkno) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ginxlogSplit, flags) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ginxlogSplit>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ginxlogSplit, <<c::expression::CRecord<crate::__pgrx_c_bindings::ginxlogSplit> as c::expression::OrdinaryField<Field_flags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).flags) };
#[doc(hidden)]
pub struct Field_prevTail;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ginxlogUpdateMeta>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ginxlogUpdateMeta>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ginxlogUpdateMeta {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ginxlogUpdateMeta, prevTail) == 72);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_prevTail>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ginxlogUpdateMeta>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ginxlogUpdateMeta, <<c::expression::CRecord<crate::__pgrx_c_bindings::ginxlogUpdateMeta> as c::expression::OrdinaryField<Field_prevTail>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).prevTail) };
#[doc(hidden)]
pub struct Field_newRightlink;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ginxlogUpdateMeta, newRightlink) == 76
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_newRightlink>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ginxlogUpdateMeta>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 76;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ginxlogUpdateMeta, <<c::expression::CRecord<crate::__pgrx_c_bindings::ginxlogUpdateMeta> as c::expression::OrdinaryField<Field_newRightlink>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).newRightlink) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ginxlogUpdateMeta, ntuples) == 80);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ntuples>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ginxlogUpdateMeta>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ginxlogUpdateMeta, <<c::expression::CRecord<crate::__pgrx_c_bindings::ginxlogUpdateMeta> as c::expression::OrdinaryField<Field_ntuples>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ntuples) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ginxlogVacuumDataLeafPage>() == 2);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ginxlogVacuumDataLeafPage>() == 2);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ginxlogVacuumDataLeafPage {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ginxlogVacuumDataLeafPage, data) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_data>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ginxlogVacuumDataLeafPage>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::ginxlogRecompressDataLeaf>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ginxlogVacuumDataLeafPage, <<c::expression::CRecord<crate::__pgrx_c_bindings::ginxlogVacuumDataLeafPage> as c::expression::OrdinaryField<Field_data>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).data) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_latestRemovedXid;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::gistxlogDelete>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::gistxlogDelete>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::gistxlogDelete {}
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::gistxlogDelete, latestRemovedXid) == 0
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_latestRemovedXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::gistxlogDelete>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::gistxlogDelete, <<c::expression::CRecord<crate::__pgrx_c_bindings::gistxlogDelete> as c::expression::OrdinaryField<Field_latestRemovedXid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).latestRemovedXid) };
#[doc(hidden)]
pub struct Field_ntodelete;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::gistxlogDelete, ntodelete) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ntodelete>
    for c::expression::CRecord<crate::__pgrx_c_bindings::gistxlogDelete>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::gistxlogDelete, <<c::expression::CRecord<crate::__pgrx_c_bindings::gistxlogDelete> as c::expression::OrdinaryField<Field_ntodelete>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ntodelete) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::gistxlogPage>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::gistxlogPage>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::gistxlogPage {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::gistxlogPage, blkno) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_blkno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::gistxlogPage>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::gistxlogPage, <<c::expression::CRecord<crate::__pgrx_c_bindings::gistxlogPage> as c::expression::OrdinaryField<Field_blkno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).blkno) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::gistxlogPage, num) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num>
    for c::expression::CRecord<crate::__pgrx_c_bindings::gistxlogPage>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::gistxlogPage, <<c::expression::CRecord<crate::__pgrx_c_bindings::gistxlogPage> as c::expression::OrdinaryField<Field_num>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num) };
#[doc(hidden)]
pub struct Field_downlinkOffset;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::gistxlogPageDelete>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::gistxlogPageDelete>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::gistxlogPageDelete {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::gistxlogPageDelete, downlinkOffset) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_downlinkOffset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::gistxlogPageDelete>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::gistxlogPageDelete, <<c::expression::CRecord<crate::__pgrx_c_bindings::gistxlogPageDelete> as c::expression::OrdinaryField<Field_downlinkOffset>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).downlinkOffset) };
#[doc(hidden)]
pub struct Field_block;
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::gistxlogPageReuse>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::gistxlogPageReuse>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::gistxlogPageReuse {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::gistxlogPageReuse, block) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_block>
    for c::expression::CRecord<crate::__pgrx_c_bindings::gistxlogPageReuse>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::gistxlogPageReuse, <<c::expression::CRecord<crate::__pgrx_c_bindings::gistxlogPageReuse> as c::expression::OrdinaryField<Field_block>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).block) };
#[doc(hidden)]
pub struct Field_origrlink;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::gistxlogPageSplit>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::gistxlogPageSplit>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::gistxlogPageSplit {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::gistxlogPageSplit, origrlink) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_origrlink>
    for c::expression::CRecord<crate::__pgrx_c_bindings::gistxlogPageSplit>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::gistxlogPageSplit, <<c::expression::CRecord<crate::__pgrx_c_bindings::gistxlogPageSplit> as c::expression::OrdinaryField<Field_origrlink>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).origrlink) };
#[doc(hidden)]
pub struct Field_orignsn;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::gistxlogPageSplit, orignsn) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_orignsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::gistxlogPageSplit>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::gistxlogPageSplit, <<c::expression::CRecord<crate::__pgrx_c_bindings::gistxlogPageSplit> as c::expression::OrdinaryField<Field_orignsn>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).orignsn) };
#[doc(hidden)]
pub struct Field_origleaf;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::gistxlogPageSplit, origleaf) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_origleaf>
    for c::expression::CRecord<crate::__pgrx_c_bindings::gistxlogPageSplit>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::gistxlogPageSplit, <<c::expression::CRecord<crate::__pgrx_c_bindings::gistxlogPageSplit> as c::expression::OrdinaryField<Field_origleaf>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).origleaf) };
#[doc(hidden)]
pub struct Field_npage;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::gistxlogPageSplit, npage) == 18);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_npage>
    for c::expression::CRecord<crate::__pgrx_c_bindings::gistxlogPageSplit>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 18;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::gistxlogPageSplit, <<c::expression::CRecord<crate::__pgrx_c_bindings::gistxlogPageSplit> as c::expression::OrdinaryField<Field_npage>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).npage) };
#[doc(hidden)]
pub struct Field_markfollowright;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::gistxlogPageSplit, markfollowright) == 20
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_markfollowright>
    for c::expression::CRecord<crate::__pgrx_c_bindings::gistxlogPageSplit>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::gistxlogPageSplit, <<c::expression::CRecord<crate::__pgrx_c_bindings::gistxlogPageSplit> as c::expression::OrdinaryField<Field_markfollowright>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).markfollowright) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::gistxlogPageUpdate>() == 4);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::gistxlogPageUpdate>() == 2);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::gistxlogPageUpdate {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::gistxlogPageUpdate, ntodelete) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ntodelete>
    for c::expression::CRecord<crate::__pgrx_c_bindings::gistxlogPageUpdate>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::gistxlogPageUpdate, <<c::expression::CRecord<crate::__pgrx_c_bindings::gistxlogPageUpdate> as c::expression::OrdinaryField<Field_ntodelete>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ntodelete) };
#[doc(hidden)]
pub struct Field_ntoinsert;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::gistxlogPageUpdate, ntoinsert) == 2);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ntoinsert>
    for c::expression::CRecord<crate::__pgrx_c_bindings::gistxlogPageUpdate>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 2;
}
