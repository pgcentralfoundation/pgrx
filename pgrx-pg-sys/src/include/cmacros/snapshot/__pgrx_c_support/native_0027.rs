// Shared generated C macro support.

#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BulkInsertStateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BulkInsertStateData> as c::expression::OrdinaryField<Field_next_free>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).next_free) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_last_free;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BulkInsertStateData, last_free) == 16
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_last_free>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BulkInsertStateData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BulkInsertStateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BulkInsertStateData> as c::expression::OrdinaryField<Field_last_free>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).last_free) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_already_extended_by;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BulkInsertStateData, already_extended_by)
        == 20
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_already_extended_by>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BulkInsertStateData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BulkInsertStateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BulkInsertStateData> as c::expression::OrdinaryField<Field_already_extended_by>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).already_extended_by) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CkptSortItem, relNumber) == 4);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relNumber>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CkptSortItem>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CkptSortItem, <<c::expression::CRecord<crate::__pgrx_c_bindings::CkptSortItem> as c::expression::OrdinaryField<Field_relNumber>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relNumber) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_min_d;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ClosestMatchState>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ClosestMatchState>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ClosestMatchState {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ClosestMatchState, min_d) == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_min_d>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ClosestMatchState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ClosestMatchState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ClosestMatchState> as c::expression::OrdinaryField<Field_min_d>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).min_d) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_max_d;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ClosestMatchState, max_d) == 12);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_max_d>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ClosestMatchState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ClosestMatchState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ClosestMatchState> as c::expression::OrdinaryField<Field_max_d>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).max_d) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ColumnDef>() == 128);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ColumnDef>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ColumnDef, identity) == 64);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_identity>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ColumnDef>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ColumnDef, generated) == 80);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_generated>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ColumnDef>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ColumnDef, collOid) == 96);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_collOid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ColumnDef>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ColumnDef, constraints) == 104);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_constraints>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ColumnDef>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ColumnDef, fdwoptions) == 112);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fdwoptions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ColumnDef>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ColumnDef, location) == 120);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ColumnDef>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
#[cfg(feature = "pg16")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CopyFormatOptions>() == 152);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CopyFormatOptions>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_default_print_len;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, default_print_len) == 48
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_default_print_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFormatOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions> as c::expression::OrdinaryField<Field_default_print_len>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).default_print_len) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, force_quote) == 80
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_force_quote>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, force_quote_all) == 88
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_force_quote_all>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, force_notnull) == 104
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_force_notnull>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, force_null) == 120
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_force_null>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, convert_selectively)
        == 136
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_convert_selectively>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, convert_select) == 144
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_convert_select>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
#[cfg(feature = "pg16")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CopyFromStateData>() == 512);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CopyFromStateData>() == 8);
};
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, cur_lineno) == 256
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cur_lineno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 256;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_relname_only;
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, relname_only) == 280
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relname_only>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 280;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFromStateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData> as c::expression::OrdinaryField<Field_relname_only>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relname_only) };
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, num_defaults) == 296
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_defaults>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 296;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, volatile_defexprs) == 344
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_volatile_defexprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 344;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, range_table) == 352
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_range_table>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 352;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_rteperminfos;
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, rteperminfos) == 360
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rteperminfos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 360;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFromStateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData> as c::expression::OrdinaryField<Field_rteperminfos>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rteperminfos) };
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, max_fields) == 408
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_max_fields>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 408;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, line_buf_valid) == 448
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_line_buf_valid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 448;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, input_buf_index) == 464
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_input_buf_index>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 464;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, input_buf_len) == 468
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_input_buf_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 468;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, input_reached_eof) == 472
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_input_reached_eof>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 472;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, input_reached_error)
        == 473
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_input_reached_error>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 473;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, raw_buf_index) == 488
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_raw_buf_index>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 488;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, raw_buf_len) == 492
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_raw_buf_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 492;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, raw_reached_eof) == 496
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_raw_reached_eof>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 496;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, bytes_processed) == 504
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bytes_processed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 504;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CustomScanState>() == 264);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CustomScanState>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::EPQState>() == 104);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::EPQState>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EPQState, resultRelations) == 16);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resultRelations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EPQState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::EPQState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::EPQState> as c::expression::OrdinaryField<
        Field_resultRelations,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).resultRelations) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_tuple_table;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EPQState, tuple_table) == 24);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tuple_table>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EPQState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::EPQState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::EPQState> as c::expression::OrdinaryField<
        Field_tuple_table,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tuple_table) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EPQState, arowMarks) == 48);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_arowMarks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EPQState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EPQState, origslot) == 56);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_origslot>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EPQState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::EState>() == 304);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::EState>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_es_rteperminfos;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_rteperminfos) == 56);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_rteperminfos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::EState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::EState> as c::expression::OrdinaryField<
        Field_es_rteperminfos,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).es_rteperminfos) };
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_output_cid) == 88);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_output_cid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_opened_result_relations) == 104
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_opened_result_relations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_tuple_routing_result_relations)
        == 120
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_tuple_routing_result_relations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_trig_target_relations) == 128
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_trig_target_relations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_tupleTable) == 168);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_tupleTable>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 168;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_processed) == 176);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_processed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 176;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_es_total_processed;
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_total_processed) == 184);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_total_processed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 184;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::EState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::EState> as c::expression::OrdinaryField<
        Field_es_total_processed,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).es_total_processed) };
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_top_eflags) == 192);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_top_eflags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 192;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_instrument) == 196);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_instrument>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 196;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_finished) == 200);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_finished>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 200;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_exprcontexts) == 208);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_exprcontexts>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 208;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_subplanstates) == 216);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_subplanstates>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 216;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_auxmodifytables) == 224);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_auxmodifytables>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 224;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_per_tuple_exprcontext) == 232
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_per_tuple_exprcontext>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::ExprContext>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 232;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_use_parallel_mode) == 248);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_use_parallel_mode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 248;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_jit_flags) == 264);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_jit_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 264;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_insert_pending_result_relations)
        == 288
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_insert_pending_result_relations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 288;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_insert_pending_modifytables)
        == 296
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_insert_pending_modifytables>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 296;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EquivalenceClass, ec_broken) == 58);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ec_broken>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceClass>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 58;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::EquivalenceMember>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::EquivalenceMember>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EquivalenceMember, em_is_const) == 24
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_em_is_const>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceMember>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EquivalenceMember, em_is_child) == 25
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_em_is_child>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceMember>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 25;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EquivalenceMember, em_datatype) == 28
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_em_datatype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceMember>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ErrorSaveContext>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ErrorSaveContext>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ErrorSaveContext {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ErrorSaveContext, type_) == 0);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ErrorSaveContext>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ErrorSaveContext, <<c::expression::CRecord<crate::__pgrx_c_bindings::ErrorSaveContext> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_error_occurred;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ErrorSaveContext, error_occurred) == 4
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_error_occurred>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ErrorSaveContext>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ErrorSaveContext, <<c::expression::CRecord<crate::__pgrx_c_bindings::ErrorSaveContext> as c::expression::OrdinaryField<Field_error_occurred>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).error_occurred) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_details_wanted;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ErrorSaveContext, details_wanted) == 5
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_details_wanted>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ErrorSaveContext>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 5;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ErrorSaveContext, <<c::expression::CRecord<crate::__pgrx_c_bindings::ErrorSaveContext> as c::expression::OrdinaryField<Field_details_wanted>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).details_wanted) };
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ExplainState>() == 96);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ExplainState>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_generic;
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExplainState, generic) == 16);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_generic>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExplainState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState> as c::expression::OrdinaryField<Field_generic>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).generic) };
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExplainState, format) == 20);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_format>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ExplainFormat,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExplainState, indent) == 24);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indent>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExplainState, grouping_stack) == 32);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_grouping_stack>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExplainState, rtable) == 48);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rtable>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExplainState, rtable_names) == 56);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rtable_names>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExplainState, deparse_cxt) == 64);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_deparse_cxt>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExplainState, hide_workers) == 80);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hide_workers>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_35>(
        ) == 4
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_35>(
        ) == 4
    );
};
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_35,
        aggno
    ) == 0
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aggno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_35>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_35, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_35> as c::expression::OrdinaryField<Field_aggno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).aggno) };
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_39>(
        ) == 16
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_39>(
        ) == 8
    );
};
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_39,
        jumpnull
    ) == 8
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_jumpnull>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_39>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_40>(
        ) == 24
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_40>(
        ) == 8
    );
};
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_40,
        args
    ) == 0
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_args>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_40>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::NullableDatum>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_40, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_40> as c::expression::OrdinaryField<Field_args>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).args) };
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_40,
        nargs
    ) == 16
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nargs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_40>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_40, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_40> as c::expression::OrdinaryField<Field_nargs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nargs) };
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_40,
        jumpnull
    ) == 20
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_jumpnull>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_40>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_41>(
        ) == 8
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_41>(
        ) == 4
    );
};
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_41,
        setoff
    ) == 0
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_setoff>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_41>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_41,
        jumpnull
    ) == 4
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_jumpnull>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_41>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_41, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_41> as c::expression::OrdinaryField<Field_jumpnull>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).jumpnull) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_jumpdistinct;
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_42>(
        ) == 24
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_42>(
        ) == 8
    );
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_42
{
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_42,
        jumpdistinct
    ) == 16
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_jumpdistinct>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_42>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_42, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_42> as c::expression::OrdinaryField<Field_jumpdistinct>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).jumpdistinct) };
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_43>(
        ) == 32
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_43>(
        ) == 8
    );
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_43
{
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_43,
        setno
    ) == 16
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_setno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_43>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_43, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_43> as c::expression::OrdinaryField<Field_setno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).setno) };
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_43,
        transno
    ) == 20
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_transno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_43>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_43, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_43> as c::expression::OrdinaryField<Field_transno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).transno) };
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_43,
        setoff
    ) == 24
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_setoff>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_43>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_43, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_43> as c::expression::OrdinaryField<Field_setoff>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).setoff) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ForeignScan>() == 184);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ForeignScan>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ForeignScan, checkAsUser) == 120);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_checkAsUser>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ForeignScan>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ForeignScan, <<c::expression::CRecord<crate::__pgrx_c_bindings::ForeignScan> as c::expression::OrdinaryField<Field_checkAsUser>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).checkAsUser) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ForeignScan, fs_server) == 124);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fs_server>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ForeignScan>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 124;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ForeignScan, fsSystemCol) == 176);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fsSystemCol>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ForeignScan>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 176;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::FormData_pg_auth_members>() == 20);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::FormData_pg_auth_members>() == 4);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_auth_members, oid) == 0);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_auth_members>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_auth_members, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_auth_members> as c::expression::OrdinaryField<Field_oid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oid) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_auth_members, roleid) == 4
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_roleid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_auth_members>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_auth_members, member) == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_member>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_auth_members>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_auth_members, grantor) == 12
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_grantor>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_auth_members>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_auth_members, admin_option) == 16
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_admin_option>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_auth_members>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_inherit_option;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_auth_members, inherit_option)
        == 17
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inherit_option>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_auth_members>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 17;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_auth_members, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_auth_members> as c::expression::OrdinaryField<Field_inherit_option>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).inherit_option) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_set_option;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_auth_members, set_option) == 18
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_set_option>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_auth_members>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 18;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_auth_members, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_auth_members> as c::expression::OrdinaryField<Field_set_option>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).set_option) };
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::FormData_pg_constraint>() == 104);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::FormData_pg_constraint>() == 4);
};
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_coninhcount>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_constraint>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 100;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_constraint, connoinherit) == 102
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_connoinherit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_constraint>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 102;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_substream>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_subscription>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 86;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_subpasswordrequired;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::FormData_pg_subscription,
        subpasswordrequired
    ) == 89
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_subpasswordrequired>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_subscription>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 89;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_subscription, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_subscription> as c::expression::OrdinaryField<Field_subpasswordrequired>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).subpasswordrequired) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_subrunasowner;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_subscription, subrunasowner)
        == 90
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_subrunasowner>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_subscription>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 90;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_subscription, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_subscription> as c::expression::OrdinaryField<Field_subrunasowner>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).subrunasowner) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::GrantRoleStmt>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::GrantRoleStmt>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_opt;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GrantRoleStmt, opt) == 32);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_opt>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GrantRoleStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::GrantRoleStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::GrantRoleStmt> as c::expression::OrdinaryField<Field_opt>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).opt) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GrantRoleStmt, behavior) == 48);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_behavior>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GrantRoleStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__DropBehavior,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::HashJoinTableData>() == 272);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::HashJoinTableData>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashJoinTableData, parallel_state) == 248
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parallel_state>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTableData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinState>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 248;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashJoinTableData, current_chunk_shared)
        == 264
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_current_chunk_shared>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTableData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 264;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_freeze_required;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::HeapPageFreeze>() == 20);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::HeapPageFreeze>() == 4);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::HeapPageFreeze {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapPageFreeze, freeze_required) == 0
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_freeze_required>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapPageFreeze>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HeapPageFreeze, <<c::expression::CRecord<crate::__pgrx_c_bindings::HeapPageFreeze> as c::expression::OrdinaryField<Field_freeze_required>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).freeze_required) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_FreezePageRelfrozenXid;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapPageFreeze, FreezePageRelfrozenXid) == 4
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_FreezePageRelfrozenXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapPageFreeze>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HeapPageFreeze, <<c::expression::CRecord<crate::__pgrx_c_bindings::HeapPageFreeze> as c::expression::OrdinaryField<Field_FreezePageRelfrozenXid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).FreezePageRelfrozenXid) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_FreezePageRelminMxid;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapPageFreeze, FreezePageRelminMxid) == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_FreezePageRelminMxid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapPageFreeze>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::MultiXactId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HeapPageFreeze, <<c::expression::CRecord<crate::__pgrx_c_bindings::HeapPageFreeze> as c::expression::OrdinaryField<Field_FreezePageRelminMxid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).FreezePageRelminMxid) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_NoFreezePageRelfrozenXid;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapPageFreeze, NoFreezePageRelfrozenXid)
        == 12
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_NoFreezePageRelfrozenXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapPageFreeze>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HeapPageFreeze, <<c::expression::CRecord<crate::__pgrx_c_bindings::HeapPageFreeze> as c::expression::OrdinaryField<Field_NoFreezePageRelfrozenXid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).NoFreezePageRelfrozenXid) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_NoFreezePageRelminMxid;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapPageFreeze, NoFreezePageRelminMxid) == 16
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_NoFreezePageRelminMxid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapPageFreeze>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::MultiXactId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HeapPageFreeze, <<c::expression::CRecord<crate::__pgrx_c_bindings::HeapPageFreeze> as c::expression::OrdinaryField<Field_NoFreezePageRelminMxid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).NoFreezePageRelminMxid) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_rs_coffset;
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapScanDescData, rs_coffset) == 70);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rs_coffset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 70;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HeapScanDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData> as c::expression::OrdinaryField<Field_rs_coffset>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rs_coffset) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::HeapTupleFreeze>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::HeapTupleFreeze>() == 4);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::HeapTupleFreeze {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapTupleFreeze, xmax) == 0);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xmax>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapTupleFreeze>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HeapTupleFreeze, <<c::expression::CRecord<crate::__pgrx_c_bindings::HeapTupleFreeze> as c::expression::OrdinaryField<Field_xmax>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xmax) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapTupleFreeze, t_infomask2) == 4);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t_infomask2>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapTupleFreeze>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HeapTupleFreeze, <<c::expression::CRecord<crate::__pgrx_c_bindings::HeapTupleFreeze> as c::expression::OrdinaryField<Field_t_infomask2>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t_infomask2) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapTupleFreeze, t_infomask) == 6);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t_infomask>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapTupleFreeze>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 6;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HeapTupleFreeze, <<c::expression::CRecord<crate::__pgrx_c_bindings::HeapTupleFreeze> as c::expression::OrdinaryField<Field_t_infomask>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t_infomask) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapTupleFreeze, frzflags) == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_frzflags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapTupleFreeze>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HeapTupleFreeze, <<c::expression::CRecord<crate::__pgrx_c_bindings::HeapTupleFreeze> as c::expression::OrdinaryField<Field_frzflags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).frzflags) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_checkflags;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapTupleFreeze, checkflags) == 9);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_checkflags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapTupleFreeze>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 9;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HeapTupleFreeze, <<c::expression::CRecord<crate::__pgrx_c_bindings::HeapTupleFreeze> as c::expression::OrdinaryField<Field_checkflags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).checkflags) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapTupleFreeze, offset) == 10);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_offset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapTupleFreeze>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 10;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HeapTupleFreeze, <<c::expression::CRecord<crate::__pgrx_c_bindings::HeapTupleFreeze> as c::expression::OrdinaryField<Field_offset>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).offset) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_amsummarizing;
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexAmRoutine, amsummarizing) == 24);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amsummarizing>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine> as c::expression::OrdinaryField<Field_amsummarizing>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).amsummarizing) };
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexAmRoutine, amparallelvacuumoptions)
        == 25
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amparallelvacuumoptions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 25;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_ii_Summarizing;
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexInfo, ii_Summarizing) == 175);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ii_Summarizing>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 175;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexInfo,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexInfo> as c::expression::OrdinaryField<
        Field_ii_Summarizing,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ii_Summarizing) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_oldNumber;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexStmt, oldNumber) == 92);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldNumber>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 92;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt> as c::expression::OrdinaryField<
        Field_oldNumber,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oldNumber) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_oldFirstRelfilelocatorSubid;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexStmt, oldFirstRelfilelocatorSubid)
        == 100
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldFirstRelfilelocatorSubid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 100;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt> as c::expression::OrdinaryField<
        Field_oldFirstRelfilelocatorSubid,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oldFirstRelfilelocatorSubid) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::IndexVacuumInfo>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::IndexVacuumInfo>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexVacuumInfo, analyze_only) == 16);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_analyze_only>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexVacuumInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexVacuumInfo, report_progress) == 17
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_report_progress>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexVacuumInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 17;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexVacuumInfo, estimated_count) == 18
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_estimated_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexVacuumInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 18;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexVacuumInfo, message_level) == 20
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_message_level>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexVacuumInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(feature = "pg16")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Instrumentation>() == 368);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Instrumentation>() == 8);
};
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Instrumentation, ntuples2) == 200);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ntuples2>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Instrumentation>
{
    type Member = c::expression::CDouble;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 200;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Instrumentation, nfiltered1) == 216);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nfiltered1>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Instrumentation>
{
    type Member = c::expression::CDouble;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 216;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Instrumentation, nfiltered2) == 224);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nfiltered2>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Instrumentation>
{
    type Member = c::expression::CDouble;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 224;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JitContext>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JitContext>() == 8);
};
#[cfg(feature = "pg16")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JitInstrumentation>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JitInstrumentation>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JoinDomain>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JoinDomain>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JoinDomain {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JoinDomain, type_) == 0);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JoinDomain>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JoinDomain, <<c::expression::CRecord<crate::__pgrx_c_bindings::JoinDomain> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JsonAggConstructor>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JsonAggConstructor>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JsonAggConstructor {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonAggConstructor, type_) == 0);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonAggConstructor>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonAggConstructor, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonAggConstructor> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonAggConstructor, agg_order) == 24);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_agg_order>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonAggConstructor>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonAggConstructor, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonAggConstructor> as c::expression::OrdinaryField<Field_agg_order>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).agg_order) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonAggConstructor, location) == 40);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonAggConstructor>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonAggConstructor, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonAggConstructor> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JsonArrayAgg>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JsonArrayAgg>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JsonArrayAgg {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonArrayAgg, type_) == 0);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonArrayAgg>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonArrayAgg, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonArrayAgg> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_absent_on_null;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonArrayAgg, absent_on_null) == 24);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_absent_on_null>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonArrayAgg>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonArrayAgg, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonArrayAgg> as c::expression::OrdinaryField<Field_absent_on_null>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).absent_on_null) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JsonArrayConstructor>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JsonArrayConstructor>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JsonArrayConstructor {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonArrayConstructor, type_) == 0);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonArrayConstructor>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonArrayConstructor, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonArrayConstructor> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonArrayConstructor, exprs) == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_exprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonArrayConstructor>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonArrayConstructor, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonArrayConstructor> as c::expression::OrdinaryField<Field_exprs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).exprs) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonArrayConstructor, absent_on_null) == 24
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_absent_on_null>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonArrayConstructor>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonArrayConstructor, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonArrayConstructor> as c::expression::OrdinaryField<Field_absent_on_null>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).absent_on_null) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonArrayConstructor, location) == 28
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonArrayConstructor>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonArrayConstructor, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonArrayConstructor> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JsonArrayQueryConstructor>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JsonArrayQueryConstructor>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JsonArrayQueryConstructor {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonArrayQueryConstructor, type_) == 0
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonArrayQueryConstructor>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonArrayQueryConstructor, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonArrayQueryConstructor> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonArrayQueryConstructor, absent_on_null)
        == 32
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_absent_on_null>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonArrayQueryConstructor>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonArrayQueryConstructor, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonArrayQueryConstructor> as c::expression::OrdinaryField<Field_absent_on_null>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).absent_on_null) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonArrayQueryConstructor, location) == 36
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonArrayQueryConstructor>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonArrayQueryConstructor, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonArrayQueryConstructor> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JsonConstructorExpr>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JsonConstructorExpr>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JsonConstructorExpr {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonConstructorExpr, type_) == 4);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonConstructorExpr>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__JsonConstructorType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonConstructorExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonConstructorExpr> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonConstructorExpr, args) == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_args>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonConstructorExpr>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonConstructorExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonConstructorExpr> as c::expression::OrdinaryField<Field_args>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).args) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonConstructorExpr, absent_on_null) == 40
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_absent_on_null>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonConstructorExpr>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonConstructorExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonConstructorExpr> as c::expression::OrdinaryField<Field_absent_on_null>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).absent_on_null) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonConstructorExpr, unique) == 41);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_unique>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonConstructorExpr>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 41;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonConstructorExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonConstructorExpr> as c::expression::OrdinaryField<Field_unique>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).unique) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonConstructorExpr, location) == 44);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonConstructorExpr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonConstructorExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonConstructorExpr> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JsonConstructorExprState>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JsonConstructorExprState>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JsonConstructorExprState {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonConstructorExprState, nargs) == 40
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nargs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonConstructorExprState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonConstructorExprState, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonConstructorExprState> as c::expression::OrdinaryField<Field_nargs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nargs) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::JsonConstructorExprState__bindgen_ty_1>()
            == 8
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::JsonConstructorExprState__bindgen_ty_1>()
            == 4
    );
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::JsonConstructorExprState__bindgen_ty_1
{
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::JsonConstructorExprState__bindgen_ty_1,
        category
    ) == 0
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_category>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonConstructorExprState__bindgen_ty_1>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonConstructorExprState__bindgen_ty_1, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonConstructorExprState__bindgen_ty_1> as c::expression::OrdinaryField<Field_category>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).category) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_outfuncid;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::JsonConstructorExprState__bindgen_ty_1,
        outfuncid
    ) == 4
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_outfuncid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonConstructorExprState__bindgen_ty_1>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonConstructorExprState__bindgen_ty_1, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonConstructorExprState__bindgen_ty_1> as c::expression::OrdinaryField<Field_outfuncid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).outfuncid) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JsonFormat>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JsonFormat>() == 4);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JsonFormat {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonFormat, type_) == 0);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonFormat>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonFormat, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonFormat> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_format_type;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonFormat, format_type) == 4);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_format_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonFormat>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__JsonFormatType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonFormat, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonFormat> as c::expression::OrdinaryField<Field_format_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).format_type) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonFormat, encoding) == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_encoding>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonFormat>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__JsonEncoding,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonFormat, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonFormat> as c::expression::OrdinaryField<Field_encoding>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).encoding) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonFormat, location) == 12);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonFormat>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonFormat, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonFormat> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JsonIsPredicate>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JsonIsPredicate>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JsonIsPredicate {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonIsPredicate, type_) == 0);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonIsPredicate>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonIsPredicate, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonIsPredicate> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_item_type;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonIsPredicate, item_type) == 24);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_item_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonIsPredicate>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__JsonValueType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonIsPredicate, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonIsPredicate> as c::expression::OrdinaryField<Field_item_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).item_type) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_unique_keys;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonIsPredicate, unique_keys) == 28);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_unique_keys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonIsPredicate>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonIsPredicate, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonIsPredicate> as c::expression::OrdinaryField<Field_unique_keys>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).unique_keys) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonIsPredicate, location) == 32);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonIsPredicate>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonIsPredicate, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonIsPredicate> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JsonKeyValue>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JsonKeyValue>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JsonKeyValue {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonKeyValue, type_) == 0);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonKeyValue>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonKeyValue, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonKeyValue> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonKeyValue, value) == 16);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::JsonValueExpr>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::JsonValueExpr>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_value>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonKeyValue>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::JsonValueExpr>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonKeyValue, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonKeyValue> as c::expression::OrdinaryField<Field_value>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).value) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JsonObjectAgg>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JsonObjectAgg>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JsonObjectAgg {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonObjectAgg, type_) == 0);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonObjectAgg>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonObjectAgg, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonObjectAgg> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonObjectAgg, absent_on_null) == 24);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_absent_on_null>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonObjectAgg>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonObjectAgg, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonObjectAgg> as c::expression::OrdinaryField<Field_absent_on_null>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).absent_on_null) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonObjectAgg, unique) == 25);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_unique>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonObjectAgg>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 25;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonObjectAgg, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonObjectAgg> as c::expression::OrdinaryField<Field_unique>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).unique) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JsonObjectConstructor>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JsonObjectConstructor>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JsonObjectConstructor {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonObjectConstructor, type_) == 0);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonObjectConstructor>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonObjectConstructor, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonObjectConstructor> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonObjectConstructor, exprs) == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_exprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonObjectConstructor>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonObjectConstructor, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonObjectConstructor> as c::expression::OrdinaryField<Field_exprs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).exprs) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonObjectConstructor, absent_on_null) == 24
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_absent_on_null>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonObjectConstructor>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonObjectConstructor, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonObjectConstructor> as c::expression::OrdinaryField<Field_absent_on_null>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).absent_on_null) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonObjectConstructor, unique) == 25);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_unique>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonObjectConstructor>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 25;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonObjectConstructor, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonObjectConstructor> as c::expression::OrdinaryField<Field_unique>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).unique) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonObjectConstructor, location) == 28
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonObjectConstructor>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonObjectConstructor, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonObjectConstructor> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JsonOutput>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JsonOutput>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JsonOutput {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonOutput, type_) == 0);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonOutput>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonOutput, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonOutput> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JsonReturning>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JsonReturning>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JsonReturning {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonReturning, type_) == 0);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonReturning>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonReturning, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonReturning> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonReturning, typid) == 16);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonReturning>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonReturning, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonReturning> as c::expression::OrdinaryField<Field_typid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typid) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonReturning, typmod) == 20);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typmod>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonReturning>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonReturning, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonReturning> as c::expression::OrdinaryField<Field_typmod>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typmod) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JsonValueExpr>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JsonValueExpr>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JsonValueExpr {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonValueExpr, type_) == 0);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonValueExpr>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonValueExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonValueExpr> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JsonbParseState>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JsonbParseState>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonbParseState, unique_keys) == 48);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_unique_keys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonbParseState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonbParseState, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonbParseState> as c::expression::OrdinaryField<Field_unique_keys>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).unique_keys) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_skip_nulls;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonbParseState, skip_nulls) == 49);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_skip_nulls>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonbParseState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 49;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonbParseState, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonbParseState> as c::expression::OrdinaryField<Field_skip_nulls>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).skip_nulls) };
#[cfg(feature = "pg16")]
#[doc(hidden)]
pub struct Field_backend_id;
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LocalPgBackendStatus>() == 456);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LocalPgBackendStatus>() == 8);
};
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LocalPgBackendStatus, backend_id) == 432
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_backend_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LocalPgBackendStatus>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 432;
}
#[cfg(feature = "pg16")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LocalPgBackendStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::LocalPgBackendStatus> as c::expression::OrdinaryField<Field_backend_id>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).backend_id) };
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LocalPgBackendStatus, backend_xid) == 436
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_backend_xid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LocalPgBackendStatus>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 436;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LocalPgBackendStatus, backend_xmin) == 440
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_backend_xmin>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LocalPgBackendStatus>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 440;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_backend_subxact_count;
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LocalPgBackendStatus, backend_subxact_count)
        == 444
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_backend_subxact_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LocalPgBackendStatus>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 444;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LocalPgBackendStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::LocalPgBackendStatus> as c::expression::OrdinaryField<Field_backend_subxact_count>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).backend_subxact_count) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_backend_subxact_overflowed;
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::LocalPgBackendStatus,
        backend_subxact_overflowed
    ) == 448
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_backend_subxact_overflowed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LocalPgBackendStatus>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 448;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LocalPgBackendStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::LocalPgBackendStatus> as c::expression::OrdinaryField<Field_backend_subxact_overflowed>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).backend_subxact_overflowed) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LogicalRepRelMapEntry>() == 112);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LogicalRepRelMapEntry>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_localindexoid;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepRelMapEntry, localindexoid) == 92
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_localindexoid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepRelMapEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 92;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepRelMapEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepRelMapEntry> as c::expression::OrdinaryField<Field_localindexoid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).localindexoid) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepRelMapEntry, state) == 96);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_state>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepRelMapEntry>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepRelMapEntry, statelsn) == 104
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_statelsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepRelMapEntry>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LogicalRepStreamAbortData>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LogicalRepStreamAbortData>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::LogicalRepStreamAbortData {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepStreamAbortData, xid) == 0);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepStreamAbortData>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepStreamAbortData, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepStreamAbortData> as c::expression::OrdinaryField<Field_xid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xid) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_subxid;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepStreamAbortData, subxid) == 4
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_subxid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepStreamAbortData>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepStreamAbortData, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepStreamAbortData> as c::expression::OrdinaryField<Field_subxid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).subxid) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_abort_lsn;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepStreamAbortData, abort_lsn) == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_abort_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepStreamAbortData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepStreamAbortData, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepStreamAbortData> as c::expression::OrdinaryField<Field_abort_lsn>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).abort_lsn) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_abort_time;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepStreamAbortData, abort_time) == 16
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_abort_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepStreamAbortData>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepStreamAbortData, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepStreamAbortData> as c::expression::OrdinaryField<Field_abort_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).abort_time) };
#[cfg(feature = "pg16")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LogicalRepWorker>() == 120);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LogicalRepWorker>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_leader_pid;
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, leader_pid) == 72);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_leader_pid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepWorker, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker> as c::expression::OrdinaryField<Field_leader_pid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).leader_pid) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_parallel_apply;
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, parallel_apply) == 76
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parallel_apply>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 76;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepWorker, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker> as c::expression::OrdinaryField<Field_parallel_apply>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).parallel_apply) };
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, last_lsn) == 80);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_last_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, last_send_time) == 88
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_last_send_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, last_recv_time) == 96
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_last_recv_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, reply_lsn) == 104);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_reply_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, reply_time) == 112);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_reply_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
#[cfg(feature = "pg16")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ModifyTableState>() == 424);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ModifyTableState>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTableState, fireBSTriggers) == 336
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fireBSTriggers>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTableState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 336;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTableState, mt_resultOidAttno) == 340
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mt_resultOidAttno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTableState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 340;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTableState, mt_lastResultOid) == 344
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mt_lastResultOid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTableState>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 344;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTableState, mt_lastResultIndex) == 348
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mt_lastResultIndex>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTableState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 348;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTableState, mt_merge_subcommands)
        == 392
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mt_merge_subcommands>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTableState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 392;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::OuterJoinClauseInfo>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::OuterJoinClauseInfo>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::OuterJoinClauseInfo {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::OuterJoinClauseInfo, type_) == 0);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OuterJoinClauseInfo>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::OuterJoinClauseInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::OuterJoinClauseInfo> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg16", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PGOutputData>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PGOutputData>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_streaming>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGOutputData>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 41;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::OffsetField<Field_links>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::dlist_node>;
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, links);
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::OffsetField<Field_syncRepLinks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::dlist_node>;
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, syncRepLinks);
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_execstate>() == 240);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_execstate>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_execstate, eval_processed) == 192
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_eval_processed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_execstate>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 192;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PROC_HDR>() == 136);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PROC_HDR>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PROC_HDR, spins_per_delay) == 128);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_spins_per_delay>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PROC_HDR>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PROC_HDR, startupBufferPinWaitBufId) == 132
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_startupBufferPinWaitBufId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PROC_HDR>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 132;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_serialize_changes;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ParallelApplyWorkerInfo>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ParallelApplyWorkerInfo>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ParallelApplyWorkerInfo {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelApplyWorkerInfo, serialize_changes)
        == 24
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_serialize_changes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelApplyWorkerInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelApplyWorkerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelApplyWorkerInfo> as c::expression::OrdinaryField<Field_serialize_changes>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).serialize_changes) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelApplyWorkerInfo, in_use) == 25
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_in_use>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelApplyWorkerInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 25;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelApplyWorkerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelApplyWorkerInfo> as c::expression::OrdinaryField<Field_in_use>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).in_use) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ParallelApplyWorkerShared>() == 80);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ParallelApplyWorkerShared>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ParallelApplyWorkerShared {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelApplyWorkerShared, mutex) == 0
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mutex>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelApplyWorkerShared>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelApplyWorkerShared, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelApplyWorkerShared> as c::expression::OrdinaryField<Field_mutex>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mutex) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelApplyWorkerShared, xid) == 4);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelApplyWorkerShared>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelApplyWorkerShared, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelApplyWorkerShared> as c::expression::OrdinaryField<Field_xid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xid) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_xact_state;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelApplyWorkerShared, xact_state) == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xact_state>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelApplyWorkerShared>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ParallelTransState,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelApplyWorkerShared, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelApplyWorkerShared> as c::expression::OrdinaryField<Field_xact_state>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xact_state) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_logicalrep_worker_generation;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ParallelApplyWorkerShared,
        logicalrep_worker_generation
    ) == 12
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_logicalrep_worker_generation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelApplyWorkerShared>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelApplyWorkerShared, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelApplyWorkerShared> as c::expression::OrdinaryField<Field_logicalrep_worker_generation>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).logicalrep_worker_generation) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_logicalrep_worker_slot_no;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ParallelApplyWorkerShared,
        logicalrep_worker_slot_no
    ) == 16
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_logicalrep_worker_slot_no>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelApplyWorkerShared>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelApplyWorkerShared, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelApplyWorkerShared> as c::expression::OrdinaryField<Field_logicalrep_worker_slot_no>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).logicalrep_worker_slot_no) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_last_commit_end;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelApplyWorkerShared, last_commit_end)
        == 24
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_last_commit_end>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelApplyWorkerShared>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelApplyWorkerShared, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelApplyWorkerShared> as c::expression::OrdinaryField<Field_last_commit_end>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).last_commit_end) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_fileset_state;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelApplyWorkerShared, fileset_state)
        == 32
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fileset_state>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelApplyWorkerShared>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PartialFileSetState,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelApplyWorkerShared, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelApplyWorkerShared> as c::expression::OrdinaryField<Field_fileset_state>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fileset_state) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_skip_unmatched;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelHashJoinBatch, skip_unmatched) == 89
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_skip_unmatched>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinBatch>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 89;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelHashJoinBatch, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinBatch> as c::expression::OrdinaryField<Field_skip_unmatched>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).skip_unmatched) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_outer_eof;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelHashJoinBatchAccessor, outer_eof)
        == 49
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_outer_eof>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinBatchAccessor>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 49;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelHashJoinBatchAccessor, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinBatchAccessor> as c::expression::OrdinaryField<Field_outer_eof>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).outer_eof) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelHashJoinBatchAccessor, done) == 50
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_done>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinBatchAccessor>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 50;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ParamPathInfo>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ParamPathInfo>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ParseNamespaceItem>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ParseNamespaceItem>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseNamespaceItem, p_rel_visible) == 40
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_rel_visible>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseNamespaceItem>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseNamespaceItem, p_cols_visible) == 41
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_cols_visible>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseNamespaceItem>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 41;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseNamespaceItem, p_lateral_only) == 42
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_lateral_only>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseNamespaceItem>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 42;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseNamespaceItem, p_lateral_ok) == 43
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_lateral_ok>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseNamespaceItem>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 43;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ParseState>() == 224);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ParseState>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_p_rteperminfos;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_rteperminfos) == 24);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_rteperminfos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParseState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParseState> as c::expression::OrdinaryField<Field_p_rteperminfos>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).p_rteperminfos) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_joinexprs) == 32);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_joinexprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_p_nullingrels;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_nullingrels) == 40);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_nullingrels>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParseState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParseState> as c::expression::OrdinaryField<Field_p_nullingrels>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).p_nullingrels) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_joinlist) == 48);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_joinlist>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_namespace) == 56);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_namespace>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_lateral_active) == 64);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_lateral_active>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_ctenamespace) == 72);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_ctenamespace>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_future_ctes) == 80);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_future_ctes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_is_insert) == 112);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_is_insert>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_windowdefs) == 120);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_windowdefs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_expr_kind) == 128);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg19"))]
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
    const OFFSET: usize = 128;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_next_resno) == 132);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_next_resno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 132;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_multiassign_exprs) == 136
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_multiassign_exprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_locking_clause) == 144);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_locking_clause>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_locked_from_parent) == 152
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_locked_from_parent>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_resolve_unknowns) == 153
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_resolve_unknowns>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 153;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_hasAggs) == 168);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_hasAggs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 168;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_hasWindowFuncs) == 169);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_hasWindowFuncs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 169;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_hasTargetSRFs) == 170);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_hasTargetSRFs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 170;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_hasSubLinks) == 171);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_hasSubLinks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 171;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_hasModifyingCTE) == 172
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_hasModifyingCTE>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 172;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_strategy>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionBoundInfoData>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PartitionStrategy,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PartitionDescData>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PartitionDescData>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_last_found_datum_index;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionDescData, last_found_datum_index)
        == 32
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_last_found_datum_index>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionDescData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionDescData> as c::expression::OrdinaryField<Field_last_found_datum_index>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).last_found_datum_index) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_last_found_part_index;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionDescData, last_found_part_index)
        == 36
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_last_found_part_index>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionDescData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionDescData> as c::expression::OrdinaryField<Field_last_found_part_index>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).last_found_part_index) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_last_found_count;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionDescData, last_found_count) == 40
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_last_found_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionDescData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionDescData> as c::expression::OrdinaryField<Field_last_found_count>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).last_found_count) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PartitionSpec>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PartitionSpec>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionSpec, strategy) == 4);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_strategy>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionSpec>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PartitionStrategy,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionSpec, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionSpec> as c::expression::OrdinaryField<Field_strategy>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).strategy) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionSpec, partParams) == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_partParams>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionSpec>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionSpec, location) == 16);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionSpec>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgBackendGSSStatus>() == 67);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgBackendGSSStatus>() == 1);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_gss_delegation;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgBackendGSSStatus, gss_delegation) == 66
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_gss_delegation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgBackendGSSStatus>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 66;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgBackendGSSStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgBackendGSSStatus> as c::expression::OrdinaryField<Field_gss_delegation>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).gss_delegation) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_numcalls;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_FunctionCounts>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_FunctionCounts>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_FunctionCounts, numcalls) == 0
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numcalls>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_FunctionCounts>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_FunctionCounts, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_FunctionCounts> as c::expression::OrdinaryField<Field_numcalls>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numcalls) };
#[cfg(feature = "pg16")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_IO>() == 14344);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_IO>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgStat_IO {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_IO, stat_reset_timestamp) == 0
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stat_reset_timestamp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_IO>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PgStat_IO,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_IO> as c::expression::OrdinaryField<
        Field_stat_reset_timestamp,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stat_reset_timestamp) };
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_PendingWalStats>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_PendingWalStats>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgStat_PendingWalStats {}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_PendingWalStats, wal_buffers_full)
        == 0
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wal_buffers_full>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_PendingWalStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_PendingWalStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_PendingWalStats> as c::expression::OrdinaryField<Field_wal_buffers_full>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wal_buffers_full) };
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_PendingWalStats, wal_write) == 8
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wal_write>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_PendingWalStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_PendingWalStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_PendingWalStats> as c::expression::OrdinaryField<Field_wal_write>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wal_write) };
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_PendingWalStats, wal_sync) == 16
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wal_sync>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_PendingWalStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_PendingWalStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_PendingWalStats> as c::expression::OrdinaryField<Field_wal_sync>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wal_sync) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_xact_commit;
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_StatDBEntry>() == 248);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_StatDBEntry>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatDBEntry, xact_commit) == 0
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xact_commit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatDBEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry> as c::expression::OrdinaryField<Field_xact_commit>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xact_commit) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_xact_rollback;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatDBEntry, xact_rollback) == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xact_rollback>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatDBEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry> as c::expression::OrdinaryField<Field_xact_rollback>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xact_rollback) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatDBEntry, blocks_fetched) == 16
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_blocks_fetched>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatDBEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry> as c::expression::OrdinaryField<Field_blocks_fetched>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).blocks_fetched) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatDBEntry, blocks_hit) == 24
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_blocks_hit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatDBEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry> as c::expression::OrdinaryField<Field_blocks_hit>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).blocks_hit) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatDBEntry, tuples_returned) == 32
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tuples_returned>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatDBEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry> as c::expression::OrdinaryField<Field_tuples_returned>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tuples_returned) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatDBEntry, tuples_fetched) == 40
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tuples_fetched>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatDBEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry> as c::expression::OrdinaryField<Field_tuples_fetched>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tuples_fetched) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatDBEntry, tuples_inserted) == 48
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tuples_inserted>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
