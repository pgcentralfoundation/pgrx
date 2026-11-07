// Shared generated C macro support.

// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TSParserCacheEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TSParserCacheEntry> as c::expression::OrdinaryField<Field_headlineOid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).headlineOid) };
#[doc(hidden)]
pub struct Field_lextypeOid;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TSParserCacheEntry, lextypeOid) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lextypeOid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TSParserCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TSParserCacheEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TSParserCacheEntry> as c::expression::OrdinaryField<Field_lextypeOid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lextypeOid) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TableFunc>() == 104);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TableFunc>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TableFunc {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TableFunc, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableFunc>
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
    crate::__pgrx_c_bindings::TableFunc,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::TableFunc> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_ns_uris;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TableFunc, ns_uris) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ns_uris>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableFunc>
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
    crate::__pgrx_c_bindings::TableFunc,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::TableFunc> as c::expression::OrdinaryField<
        Field_ns_uris,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ns_uris) };
#[doc(hidden)]
pub struct Field_ns_names;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TableFunc, ns_names) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ns_names>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableFunc>
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
    crate::__pgrx_c_bindings::TableFunc,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::TableFunc> as c::expression::OrdinaryField<
        Field_ns_names,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ns_names) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TableFunc, colnames) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_colnames>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableFunc>
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
    crate::__pgrx_c_bindings::TableFunc,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::TableFunc> as c::expression::OrdinaryField<
        Field_colnames,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).colnames) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TableFunc, coltypes) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_coltypes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableFunc>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::TableFunc,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::TableFunc> as c::expression::OrdinaryField<
        Field_coltypes,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).coltypes) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TableFunc, coltypmods) == 56);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_coltypmods>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableFunc>
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
    crate::__pgrx_c_bindings::TableFunc,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::TableFunc> as c::expression::OrdinaryField<
        Field_coltypmods,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).coltypmods) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TableFunc, colcollations) == 64);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_colcollations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableFunc>
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
    crate::__pgrx_c_bindings::TableFunc,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::TableFunc> as c::expression::OrdinaryField<
        Field_colcollations,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).colcollations) };
#[doc(hidden)]
pub struct Field_colexprs;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TableFunc, colexprs) == 72);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_colexprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableFunc>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::TableFunc,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::TableFunc> as c::expression::OrdinaryField<
        Field_colexprs,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).colexprs) };
#[doc(hidden)]
pub struct Field_coldefexprs;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TableFunc, coldefexprs) == 80);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_coldefexprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableFunc>
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
    crate::__pgrx_c_bindings::TableFunc,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::TableFunc> as c::expression::OrdinaryField<
        Field_coldefexprs,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).coldefexprs) };
#[doc(hidden)]
pub struct Field_ordinalitycol;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TableFunc, ordinalitycol) == 96);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ordinalitycol>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableFunc>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::TableFunc,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::TableFunc> as c::expression::OrdinaryField<
        Field_ordinalitycol,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ordinalitycol) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TableFunc, location) == 100);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableFunc>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 100;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::TableFunc,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::TableFunc> as c::expression::OrdinaryField<
        Field_location,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TableFuncScanState>() == 336);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TableFuncScanState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TableFuncScanState {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TableFuncScanState, colexprs) == 240);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_colexprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableFuncScanState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableFuncScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableFuncScanState> as c::expression::OrdinaryField<Field_colexprs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).colexprs) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TableFuncScanState, coldefexprs) == 248
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_coldefexprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableFuncScanState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 248;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableFuncScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableFuncScanState> as c::expression::OrdinaryField<Field_coldefexprs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).coldefexprs) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TableFuncScanState, ns_names) == 256);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ns_names>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableFuncScanState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 256;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableFuncScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableFuncScanState> as c::expression::OrdinaryField<Field_ns_names>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ns_names) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TableFuncScanState, ns_uris) == 264);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ns_uris>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableFuncScanState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 264;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableFuncScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableFuncScanState> as c::expression::OrdinaryField<Field_ns_uris>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ns_uris) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TableFuncScanState, ordinal) == 312);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ordinal>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableFuncScanState>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 312;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableFuncScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableFuncScanState> as c::expression::OrdinaryField<Field_ordinal>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ordinal) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TableLikeClause>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TableLikeClause>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TableLikeClause {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TableLikeClause, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableLikeClause>
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
const _: Projection<crate::__pgrx_c_bindings::TableLikeClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableLikeClause> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TableLikeClause, options) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableLikeClause>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableLikeClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableLikeClause> as c::expression::OrdinaryField<Field_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
#[doc(hidden)]
pub struct Field_relationOid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TableLikeClause, relationOid) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relationOid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableLikeClause>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableLikeClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableLikeClause> as c::expression::OrdinaryField<Field_relationOid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relationOid) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TableSampleClause>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TableSampleClause>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TableSampleClause {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TableSampleClause, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableSampleClause>
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
const _: Projection<crate::__pgrx_c_bindings::TableSampleClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableSampleClause> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_tsmhandler;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TableSampleClause, tsmhandler) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tsmhandler>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableSampleClause>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableSampleClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableSampleClause> as c::expression::OrdinaryField<Field_tsmhandler>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tsmhandler) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TableSampleClause, args) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_args>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableSampleClause>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableSampleClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableSampleClause> as c::expression::OrdinaryField<Field_args>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).args) };
#[doc(hidden)]
pub struct Field_rs_nkeys;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TableScanDescData>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TableScanDescData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TableScanDescData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TableScanDescData, rs_nkeys) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rs_nkeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableScanDescData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableScanDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableScanDescData> as c::expression::OrdinaryField<Field_rs_nkeys>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rs_nkeys) };
#[doc(hidden)]
pub struct Field_rs_flags;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TableScanDescData, rs_flags) == 44);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rs_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableScanDescData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableScanDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableScanDescData> as c::expression::OrdinaryField<Field_rs_flags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rs_flags) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TableSpaceOpts>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TableSpaceOpts>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TableSpaceOpts {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TableSpaceOpts, vl_len_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vl_len_>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableSpaceOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableSpaceOpts, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableSpaceOpts> as c::expression::OrdinaryField<Field_vl_len_>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).vl_len_) };
#[doc(hidden)]
pub struct Field_effective_io_concurrency;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TableSpaceOpts, effective_io_concurrency)
        == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_effective_io_concurrency>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableSpaceOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableSpaceOpts, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableSpaceOpts> as c::expression::OrdinaryField<Field_effective_io_concurrency>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).effective_io_concurrency) };
#[doc(hidden)]
pub struct Field_maintenance_io_concurrency;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TableSpaceOpts, maintenance_io_concurrency)
        == 28
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_maintenance_io_concurrency>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableSpaceOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableSpaceOpts, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableSpaceOpts> as c::expression::OrdinaryField<Field_maintenance_io_concurrency>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).maintenance_io_concurrency) };
#[doc(hidden)]
pub struct Field_resno;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TargetEntry>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TargetEntry>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TargetEntry {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TargetEntry, resno) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TargetEntry>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TargetEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TargetEntry> as c::expression::OrdinaryField<Field_resno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).resno) };
#[doc(hidden)]
pub struct Field_ressortgroupref;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TargetEntry, ressortgroupref) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ressortgroupref>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TargetEntry>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TargetEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TargetEntry> as c::expression::OrdinaryField<Field_ressortgroupref>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ressortgroupref) };
#[doc(hidden)]
pub struct Field_resorigtbl;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TargetEntry, resorigtbl) == 36);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resorigtbl>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TargetEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TargetEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TargetEntry> as c::expression::OrdinaryField<Field_resorigtbl>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).resorigtbl) };
#[doc(hidden)]
pub struct Field_resorigcol;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TargetEntry, resorigcol) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resorigcol>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TargetEntry>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TargetEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TargetEntry> as c::expression::OrdinaryField<Field_resorigcol>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).resorigcol) };
#[doc(hidden)]
pub struct Field_resjunk;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TargetEntry, resjunk) == 42);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resjunk>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TargetEntry>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 42;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TargetEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TargetEntry> as c::expression::OrdinaryField<Field_resjunk>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).resjunk) };
#[doc(hidden)]
pub struct Field_trss_tidexprs;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TidRangeScanState>() == 248);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TidRangeScanState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TidRangeScanState {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TidRangeScanState, trss_tidexprs) == 224
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_trss_tidexprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TidRangeScanState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 224;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TidRangeScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::TidRangeScanState> as c::expression::OrdinaryField<Field_trss_tidexprs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).trss_tidexprs) };
#[doc(hidden)]
pub struct Field_trss_inScan;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TidRangeScanState, trss_inScan) == 244
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_trss_inScan>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TidRangeScanState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 244;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TidRangeScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::TidRangeScanState> as c::expression::OrdinaryField<Field_trss_inScan>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).trss_inScan) };
#[doc(hidden)]
pub struct Field_tss_tidexprs;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TidScanState>() == 280);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TidScanState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TidScanState {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TidScanState, tss_tidexprs) == 224);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tss_tidexprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TidScanState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 224;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TidScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::TidScanState> as c::expression::OrdinaryField<Field_tss_tidexprs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tss_tidexprs) };
#[doc(hidden)]
pub struct Field_tss_isCurrentOf;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TidScanState, tss_isCurrentOf) == 232
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tss_isCurrentOf>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TidScanState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 232;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TidScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::TidScanState> as c::expression::OrdinaryField<Field_tss_isCurrentOf>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tss_isCurrentOf) };
#[doc(hidden)]
pub struct Field_tss_NumTids;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TidScanState, tss_NumTids) == 236);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tss_NumTids>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TidScanState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 236;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TidScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::TidScanState> as c::expression::OrdinaryField<Field_tss_NumTids>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tss_NumTids) };
#[doc(hidden)]
pub struct Field_tss_TidPtr;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TidScanState, tss_TidPtr) == 240);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tss_TidPtr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TidScanState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TidScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::TidScanState> as c::expression::OrdinaryField<Field_tss_TidPtr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tss_TidPtr) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TimeLineHistoryCmd>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TimeLineHistoryCmd>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TimeLineHistoryCmd {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TimeLineHistoryCmd, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TimeLineHistoryCmd>
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
const _: Projection<crate::__pgrx_c_bindings::TimeLineHistoryCmd, <<c::expression::CRecord<crate::__pgrx_c_bindings::TimeLineHistoryCmd> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TimeLineHistoryCmd, timeline) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_timeline>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TimeLineHistoryCmd>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TimeLineHistoryCmd, <<c::expression::CRecord<crate::__pgrx_c_bindings::TimeLineHistoryCmd> as c::expression::OrdinaryField<Field_timeline>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).timeline) };
#[doc(hidden)]
pub struct Field_tblsize;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TimeZoneAbbrevTable>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TimeZoneAbbrevTable>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TimeZoneAbbrevTable {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TimeZoneAbbrevTable, tblsize) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tblsize>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TimeZoneAbbrevTable>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TimeZoneAbbrevTable, <<c::expression::CRecord<crate::__pgrx_c_bindings::TimeZoneAbbrevTable> as c::expression::OrdinaryField<Field_tblsize>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tblsize) };
#[doc(hidden)]
pub struct Field_numabbrevs;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TimeZoneAbbrevTable, numabbrevs) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numabbrevs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TimeZoneAbbrevTable>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TimeZoneAbbrevTable, <<c::expression::CRecord<crate::__pgrx_c_bindings::TimeZoneAbbrevTable> as c::expression::OrdinaryField<Field_numabbrevs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numabbrevs) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TransactionStmt>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TransactionStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TransactionStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TransactionStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TransactionStmt>
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
const _: Projection<crate::__pgrx_c_bindings::TransactionStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::TransactionStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TransactionStmt, kind) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_kind>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TransactionStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__TransactionStmtKind,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TransactionStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::TransactionStmt> as c::expression::OrdinaryField<Field_kind>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).kind) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TransactionStmt, options) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TransactionStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TransactionStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::TransactionStmt> as c::expression::OrdinaryField<Field_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TransactionStmt, chain) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_chain>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TransactionStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TransactionStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::TransactionStmt> as c::expression::OrdinaryField<Field_chain>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).chain) };
#[doc(hidden)]
pub struct Field_tcs_delete_old_table;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TransitionCaptureState>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TransitionCaptureState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TransitionCaptureState {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TransitionCaptureState, tcs_delete_old_table)
        == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tcs_delete_old_table>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TransitionCaptureState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TransitionCaptureState, <<c::expression::CRecord<crate::__pgrx_c_bindings::TransitionCaptureState> as c::expression::OrdinaryField<Field_tcs_delete_old_table>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tcs_delete_old_table) };
#[doc(hidden)]
pub struct Field_tcs_update_old_table;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TransitionCaptureState, tcs_update_old_table)
        == 1
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tcs_update_old_table>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TransitionCaptureState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TransitionCaptureState, <<c::expression::CRecord<crate::__pgrx_c_bindings::TransitionCaptureState> as c::expression::OrdinaryField<Field_tcs_update_old_table>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tcs_update_old_table) };
#[doc(hidden)]
pub struct Field_tcs_update_new_table;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TransitionCaptureState, tcs_update_new_table)
        == 2
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tcs_update_new_table>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TransitionCaptureState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 2;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TransitionCaptureState, <<c::expression::CRecord<crate::__pgrx_c_bindings::TransitionCaptureState> as c::expression::OrdinaryField<Field_tcs_update_new_table>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tcs_update_new_table) };
#[doc(hidden)]
pub struct Field_tcs_insert_new_table;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TransitionCaptureState, tcs_insert_new_table)
        == 3
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tcs_insert_new_table>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TransitionCaptureState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 3;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TransitionCaptureState, <<c::expression::CRecord<crate::__pgrx_c_bindings::TransitionCaptureState> as c::expression::OrdinaryField<Field_tcs_insert_new_table>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tcs_insert_new_table) };
#[doc(hidden)]
pub struct Field_tgoid;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Trigger>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Trigger>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::Trigger {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Trigger, tgoid) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tgoid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Trigger>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Trigger,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Trigger> as c::expression::OrdinaryField<
        Field_tgoid,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tgoid) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Trigger, tgfoid) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tgfoid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Trigger>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Trigger,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Trigger> as c::expression::OrdinaryField<
        Field_tgfoid,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tgfoid) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Trigger, tgtype) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tgtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Trigger>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Trigger,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Trigger> as c::expression::OrdinaryField<
        Field_tgtype,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tgtype) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Trigger, tgenabled) == 22);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tgenabled>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Trigger>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 22;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Trigger,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Trigger> as c::expression::OrdinaryField<
        Field_tgenabled,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tgenabled) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Trigger, tgisinternal) == 23);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tgisinternal>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Trigger>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 23;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Trigger,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Trigger> as c::expression::OrdinaryField<
        Field_tgisinternal,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tgisinternal) };
#[doc(hidden)]
pub struct Field_tgisclone;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Trigger, tgisclone) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tgisclone>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Trigger>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Trigger,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Trigger> as c::expression::OrdinaryField<
        Field_tgisclone,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tgisclone) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Trigger, tgconstrrelid) == 28);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tgconstrrelid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Trigger>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Trigger,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Trigger> as c::expression::OrdinaryField<
        Field_tgconstrrelid,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tgconstrrelid) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Trigger, tgconstrindid) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tgconstrindid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Trigger>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Trigger,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Trigger> as c::expression::OrdinaryField<
        Field_tgconstrindid,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tgconstrindid) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Trigger, tgconstraint) == 36);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tgconstraint>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Trigger>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Trigger,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Trigger> as c::expression::OrdinaryField<
        Field_tgconstraint,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tgconstraint) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Trigger, tgdeferrable) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tgdeferrable>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Trigger>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Trigger,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Trigger> as c::expression::OrdinaryField<
        Field_tgdeferrable,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tgdeferrable) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Trigger, tginitdeferred) == 41);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tginitdeferred>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Trigger>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 41;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Trigger,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Trigger> as c::expression::OrdinaryField<
        Field_tginitdeferred,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tginitdeferred) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Trigger, tgnargs) == 42);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tgnargs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Trigger>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 42;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Trigger,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Trigger> as c::expression::OrdinaryField<
        Field_tgnargs,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tgnargs) };
#[doc(hidden)]
pub struct Field_tgnattr;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Trigger, tgnattr) == 44);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tgnattr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Trigger>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Trigger,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Trigger> as c::expression::OrdinaryField<
        Field_tgnattr,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tgnattr) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TriggerData>() == 80);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TriggerData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TriggerData {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TriggerData, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TriggerData>
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
const _: Projection<crate::__pgrx_c_bindings::TriggerData, <<c::expression::CRecord<crate::__pgrx_c_bindings::TriggerData> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_tg_event;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TriggerData, tg_event) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tg_event>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TriggerData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TriggerData, <<c::expression::CRecord<crate::__pgrx_c_bindings::TriggerData> as c::expression::OrdinaryField<Field_tg_event>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tg_event) };
#[doc(hidden)]
pub struct Field_numtriggers;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TriggerDesc>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TriggerDesc>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TriggerDesc {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TriggerDesc, numtriggers) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numtriggers>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TriggerDesc>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TriggerDesc, <<c::expression::CRecord<crate::__pgrx_c_bindings::TriggerDesc> as c::expression::OrdinaryField<Field_numtriggers>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numtriggers) };
#[doc(hidden)]
pub struct Field_trig_insert_before_row;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TriggerDesc, trig_insert_before_row) == 12
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_trig_insert_before_row>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TriggerDesc>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TriggerDesc, <<c::expression::CRecord<crate::__pgrx_c_bindings::TriggerDesc> as c::expression::OrdinaryField<Field_trig_insert_before_row>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).trig_insert_before_row) };
#[doc(hidden)]
pub struct Field_trig_insert_after_row;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TriggerDesc, trig_insert_after_row) == 13
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_trig_insert_after_row>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TriggerDesc>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 13;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TriggerDesc, <<c::expression::CRecord<crate::__pgrx_c_bindings::TriggerDesc> as c::expression::OrdinaryField<Field_trig_insert_after_row>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).trig_insert_after_row) };
#[doc(hidden)]
pub struct Field_trig_insert_instead_row;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TriggerDesc, trig_insert_instead_row) == 14
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_trig_insert_instead_row>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TriggerDesc>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 14;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TriggerDesc, <<c::expression::CRecord<crate::__pgrx_c_bindings::TriggerDesc> as c::expression::OrdinaryField<Field_trig_insert_instead_row>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).trig_insert_instead_row) };
#[doc(hidden)]
pub struct Field_trig_insert_before_statement;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TriggerDesc, trig_insert_before_statement)
        == 15
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_trig_insert_before_statement>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TriggerDesc>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 15;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TriggerDesc, <<c::expression::CRecord<crate::__pgrx_c_bindings::TriggerDesc> as c::expression::OrdinaryField<Field_trig_insert_before_statement>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).trig_insert_before_statement) };
#[doc(hidden)]
pub struct Field_trig_insert_after_statement;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TriggerDesc, trig_insert_after_statement)
        == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_trig_insert_after_statement>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TriggerDesc>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TriggerDesc, <<c::expression::CRecord<crate::__pgrx_c_bindings::TriggerDesc> as c::expression::OrdinaryField<Field_trig_insert_after_statement>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).trig_insert_after_statement) };
#[doc(hidden)]
pub struct Field_trig_update_before_row;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TriggerDesc, trig_update_before_row) == 17
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_trig_update_before_row>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TriggerDesc>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 17;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TriggerDesc, <<c::expression::CRecord<crate::__pgrx_c_bindings::TriggerDesc> as c::expression::OrdinaryField<Field_trig_update_before_row>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).trig_update_before_row) };
#[doc(hidden)]
pub struct Field_trig_update_after_row;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TriggerDesc, trig_update_after_row) == 18
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_trig_update_after_row>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TriggerDesc>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 18;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TriggerDesc, <<c::expression::CRecord<crate::__pgrx_c_bindings::TriggerDesc> as c::expression::OrdinaryField<Field_trig_update_after_row>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).trig_update_after_row) };
#[doc(hidden)]
pub struct Field_trig_update_instead_row;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TriggerDesc, trig_update_instead_row) == 19
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_trig_update_instead_row>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TriggerDesc>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 19;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TriggerDesc, <<c::expression::CRecord<crate::__pgrx_c_bindings::TriggerDesc> as c::expression::OrdinaryField<Field_trig_update_instead_row>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).trig_update_instead_row) };
#[doc(hidden)]
pub struct Field_trig_update_before_statement;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TriggerDesc, trig_update_before_statement)
        == 20
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_trig_update_before_statement>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TriggerDesc>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TriggerDesc, <<c::expression::CRecord<crate::__pgrx_c_bindings::TriggerDesc> as c::expression::OrdinaryField<Field_trig_update_before_statement>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).trig_update_before_statement) };
#[doc(hidden)]
pub struct Field_trig_update_after_statement;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TriggerDesc, trig_update_after_statement)
        == 21
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_trig_update_after_statement>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TriggerDesc>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 21;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TriggerDesc, <<c::expression::CRecord<crate::__pgrx_c_bindings::TriggerDesc> as c::expression::OrdinaryField<Field_trig_update_after_statement>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).trig_update_after_statement) };
#[doc(hidden)]
pub struct Field_trig_delete_before_row;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TriggerDesc, trig_delete_before_row) == 22
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_trig_delete_before_row>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TriggerDesc>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 22;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TriggerDesc, <<c::expression::CRecord<crate::__pgrx_c_bindings::TriggerDesc> as c::expression::OrdinaryField<Field_trig_delete_before_row>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).trig_delete_before_row) };
#[doc(hidden)]
pub struct Field_trig_delete_after_row;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TriggerDesc, trig_delete_after_row) == 23
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_trig_delete_after_row>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TriggerDesc>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 23;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TriggerDesc, <<c::expression::CRecord<crate::__pgrx_c_bindings::TriggerDesc> as c::expression::OrdinaryField<Field_trig_delete_after_row>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).trig_delete_after_row) };
#[doc(hidden)]
pub struct Field_trig_delete_instead_row;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TriggerDesc, trig_delete_instead_row) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_trig_delete_instead_row>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TriggerDesc>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TriggerDesc, <<c::expression::CRecord<crate::__pgrx_c_bindings::TriggerDesc> as c::expression::OrdinaryField<Field_trig_delete_instead_row>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).trig_delete_instead_row) };
#[doc(hidden)]
pub struct Field_trig_delete_before_statement;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TriggerDesc, trig_delete_before_statement)
        == 25
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_trig_delete_before_statement>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TriggerDesc>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 25;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TriggerDesc, <<c::expression::CRecord<crate::__pgrx_c_bindings::TriggerDesc> as c::expression::OrdinaryField<Field_trig_delete_before_statement>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).trig_delete_before_statement) };
#[doc(hidden)]
pub struct Field_trig_delete_after_statement;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TriggerDesc, trig_delete_after_statement)
        == 26
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_trig_delete_after_statement>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TriggerDesc>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 26;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TriggerDesc, <<c::expression::CRecord<crate::__pgrx_c_bindings::TriggerDesc> as c::expression::OrdinaryField<Field_trig_delete_after_statement>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).trig_delete_after_statement) };
#[doc(hidden)]
pub struct Field_trig_truncate_before_statement;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TriggerDesc, trig_truncate_before_statement)
        == 27
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_trig_truncate_before_statement>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TriggerDesc>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 27;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TriggerDesc, <<c::expression::CRecord<crate::__pgrx_c_bindings::TriggerDesc> as c::expression::OrdinaryField<Field_trig_truncate_before_statement>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).trig_truncate_before_statement) };
#[doc(hidden)]
pub struct Field_trig_truncate_after_statement;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TriggerDesc, trig_truncate_after_statement)
        == 28
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_trig_truncate_after_statement>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TriggerDesc>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TriggerDesc, <<c::expression::CRecord<crate::__pgrx_c_bindings::TriggerDesc> as c::expression::OrdinaryField<Field_trig_truncate_after_statement>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).trig_truncate_after_statement) };
#[doc(hidden)]
pub struct Field_trig_insert_new_table;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TriggerDesc, trig_insert_new_table) == 29
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_trig_insert_new_table>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TriggerDesc>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 29;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TriggerDesc, <<c::expression::CRecord<crate::__pgrx_c_bindings::TriggerDesc> as c::expression::OrdinaryField<Field_trig_insert_new_table>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).trig_insert_new_table) };
#[doc(hidden)]
pub struct Field_trig_update_old_table;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TriggerDesc, trig_update_old_table) == 30
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_trig_update_old_table>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TriggerDesc>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 30;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TriggerDesc, <<c::expression::CRecord<crate::__pgrx_c_bindings::TriggerDesc> as c::expression::OrdinaryField<Field_trig_update_old_table>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).trig_update_old_table) };
#[doc(hidden)]
pub struct Field_trig_update_new_table;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TriggerDesc, trig_update_new_table) == 31
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_trig_update_new_table>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TriggerDesc>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 31;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TriggerDesc, <<c::expression::CRecord<crate::__pgrx_c_bindings::TriggerDesc> as c::expression::OrdinaryField<Field_trig_update_new_table>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).trig_update_new_table) };
#[doc(hidden)]
pub struct Field_trig_delete_old_table;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TriggerDesc, trig_delete_old_table) == 32
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_trig_delete_old_table>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TriggerDesc>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TriggerDesc, <<c::expression::CRecord<crate::__pgrx_c_bindings::TriggerDesc> as c::expression::OrdinaryField<Field_trig_delete_old_table>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).trig_delete_old_table) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TriggerTransition>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TriggerTransition>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TriggerTransition {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TriggerTransition, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TriggerTransition>
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
const _: Projection<crate::__pgrx_c_bindings::TriggerTransition, <<c::expression::CRecord<crate::__pgrx_c_bindings::TriggerTransition> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_isNew;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TriggerTransition, isNew) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isNew>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TriggerTransition>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TriggerTransition, <<c::expression::CRecord<crate::__pgrx_c_bindings::TriggerTransition> as c::expression::OrdinaryField<Field_isNew>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isNew) };
#[doc(hidden)]
pub struct Field_isTable;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TriggerTransition, isTable) == 17);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isTable>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TriggerTransition>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 17;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TriggerTransition, <<c::expression::CRecord<crate::__pgrx_c_bindings::TriggerTransition> as c::expression::OrdinaryField<Field_isTable>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isTable) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TruncateStmt>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TruncateStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TruncateStmt {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TruncateStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TruncateStmt>
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
const _: Projection<crate::__pgrx_c_bindings::TruncateStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::TruncateStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TruncateStmt, relations) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TruncateStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TruncateStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::TruncateStmt> as c::expression::OrdinaryField<Field_relations>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relations) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TruncateStmt, restart_seqs) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_restart_seqs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TruncateStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TruncateStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::TruncateStmt> as c::expression::OrdinaryField<Field_restart_seqs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).restart_seqs) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TruncateStmt, behavior) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_behavior>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TruncateStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__DropBehavior,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TruncateStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::TruncateStmt> as c::expression::OrdinaryField<Field_behavior>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).behavior) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TsmRoutine>() == 72);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TsmRoutine>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TsmRoutine {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TsmRoutine, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TsmRoutine>
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
const _: Projection<crate::__pgrx_c_bindings::TsmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::TsmRoutine> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_parameterTypes;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TsmRoutine, parameterTypes) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parameterTypes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TsmRoutine>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TsmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::TsmRoutine> as c::expression::OrdinaryField<Field_parameterTypes>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).parameterTypes) };
#[doc(hidden)]
pub struct Field_repeatable_across_queries;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TsmRoutine, repeatable_across_queries) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_repeatable_across_queries>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TsmRoutine>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TsmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::TsmRoutine> as c::expression::OrdinaryField<Field_repeatable_across_queries>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).repeatable_across_queries) };
#[doc(hidden)]
pub struct Field_repeatable_across_scans;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TsmRoutine, repeatable_across_scans) == 17
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_repeatable_across_scans>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TsmRoutine>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 17;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TsmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::TsmRoutine> as c::expression::OrdinaryField<Field_repeatable_across_scans>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).repeatable_across_scans) };
#[doc(hidden)]
pub struct Field_num_defval;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TupleConstr>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TupleConstr>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TupleConstr {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleConstr, num_defval) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_defval>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleConstr>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TupleConstr, <<c::expression::CRecord<crate::__pgrx_c_bindings::TupleConstr> as c::expression::OrdinaryField<Field_num_defval>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_defval) };
#[doc(hidden)]
pub struct Field_num_check;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleConstr, num_check) == 26);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_check>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleConstr>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 26;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TupleConstr, <<c::expression::CRecord<crate::__pgrx_c_bindings::TupleConstr> as c::expression::OrdinaryField<Field_num_check>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_check) };
#[doc(hidden)]
pub struct Field_has_not_null;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleConstr, has_not_null) == 28);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_has_not_null>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleConstr>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TupleConstr, <<c::expression::CRecord<crate::__pgrx_c_bindings::TupleConstr> as c::expression::OrdinaryField<Field_has_not_null>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).has_not_null) };
#[doc(hidden)]
pub struct Field_has_generated_stored;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleConstr, has_generated_stored) == 29
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_has_generated_stored>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleConstr>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 29;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TupleConstr, <<c::expression::CRecord<crate::__pgrx_c_bindings::TupleConstr> as c::expression::OrdinaryField<Field_has_generated_stored>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).has_generated_stored) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TupleDescData>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TupleDescData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TupleDescData {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleDescData, natts) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_natts>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleDescData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TupleDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::TupleDescData> as c::expression::OrdinaryField<Field_natts>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).natts) };
#[doc(hidden)]
pub struct Field_tdtypeid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleDescData, tdtypeid) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tdtypeid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleDescData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TupleDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::TupleDescData> as c::expression::OrdinaryField<Field_tdtypeid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tdtypeid) };
#[doc(hidden)]
pub struct Field_tdtypmod;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleDescData, tdtypmod) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tdtypmod>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleDescData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TupleDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::TupleDescData> as c::expression::OrdinaryField<Field_tdtypmod>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tdtypmod) };
#[doc(hidden)]
pub struct Field_tdrefcount;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleDescData, tdrefcount) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tdrefcount>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleDescData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TupleDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::TupleDescData> as c::expression::OrdinaryField<Field_tdrefcount>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tdrefcount) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
#[doc(hidden)]
pub struct Field_attrs;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleDescData, attrs) == 24);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
impl c::expression::OffsetField<Field_attrs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleDescData>
{
    type Member = ();
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleDescData, attrs);
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            crate::__pgrx_c_bindings::__IncompleteArrayField<
                crate::__pgrx_c_bindings::FormData_pg_attribute,
            >,
        >() == 0
    );
    assert!(
        ::core::mem::align_of::<
            crate::__pgrx_c_bindings::__IncompleteArrayField<
                crate::__pgrx_c_bindings::FormData_pg_attribute,
            >,
        >() == 4
    );
};
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attrs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleDescData>
{
    type Member = c::expression::CFlexibleArray<
        c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>,
        crate::__pgrx_c_bindings::__IncompleteArrayField<
            crate::__pgrx_c_bindings::FormData_pg_attribute,
        >,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TupleDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::TupleDescData> as c::expression::OrdinaryField<Field_attrs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attrs) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TupleHashEntryData>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TupleHashEntryData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TupleHashEntryData {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleHashEntryData, status) == 16);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_status>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleHashEntryData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TupleHashEntryData, <<c::expression::CRecord<crate::__pgrx_c_bindings::TupleHashEntryData> as c::expression::OrdinaryField<Field_status>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).status) };
#[doc(hidden)]
pub struct Field_hash;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleHashEntryData, hash) == 20);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleHashEntryData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TupleHashEntryData, <<c::expression::CRecord<crate::__pgrx_c_bindings::TupleHashEntryData> as c::expression::OrdinaryField<Field_hash>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hash) };
#[doc(hidden)]
pub struct Field_hashtab;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TupleHashTableData>() == 120);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TupleHashTableData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TupleHashTableData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleHashTableData, hashtab) == 0);
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::tuplehash_hash>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::tuplehash_hash>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hashtab>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleHashTableData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::tuplehash_hash>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TupleHashTableData, <<c::expression::CRecord<crate::__pgrx_c_bindings::TupleHashTableData> as c::expression::OrdinaryField<Field_hashtab>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hashtab) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleHashTableData, numCols) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numCols>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleHashTableData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TupleHashTableData, <<c::expression::CRecord<crate::__pgrx_c_bindings::TupleHashTableData> as c::expression::OrdinaryField<Field_numCols>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numCols) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleHashTableData, entrysize) == 64);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_entrysize>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleHashTableData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TupleHashTableData, <<c::expression::CRecord<crate::__pgrx_c_bindings::TupleHashTableData> as c::expression::OrdinaryField<Field_entrysize>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).entrysize) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
#[doc(hidden)]
pub struct Field_hash_iv;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleHashTableData, hash_iv) == 104);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash_iv>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleHashTableData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TupleHashTableData, <<c::expression::CRecord<crate::__pgrx_c_bindings::TupleHashTableData> as c::expression::OrdinaryField<Field_hash_iv>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hash_iv) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TupleTableSlot>() == 64);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TupleTableSlot>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TupleTableSlot {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleTableSlot, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot>
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
const _: Projection<crate::__pgrx_c_bindings::TupleTableSlot, <<c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_tts_flags;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleTableSlot, tts_flags) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tts_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TupleTableSlot, <<c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot> as c::expression::OrdinaryField<Field_tts_flags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tts_flags) };
#[doc(hidden)]
pub struct Field_tts_nvalid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleTableSlot, tts_nvalid) == 6);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tts_nvalid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 6;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TupleTableSlot, <<c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot> as c::expression::OrdinaryField<Field_tts_nvalid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tts_nvalid) };
#[doc(hidden)]
pub struct Field_tts_ops;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleTableSlot, tts_ops) == 8);
const _: () = {
    assert!(::core::mem::size_of::<*const crate::__pgrx_c_bindings::TupleTableSlotOps>() == 8);
    assert!(::core::mem::align_of::<*const crate::__pgrx_c_bindings::TupleTableSlotOps>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tts_ops>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlotOps>,
        c::expression::ReadOnly,
    >;
    type Declared = FieldReadOnly;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TupleTableSlot, <<c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot> as c::expression::OrdinaryField<Field_tts_ops>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tts_ops) };
#[doc(hidden)]
pub struct Field_tts_tableOid;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleTableSlot, tts_tableOid) == 56);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tts_tableOid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TupleTableSlot, <<c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot> as c::expression::OrdinaryField<Field_tts_tableOid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tts_tableOid) };
#[doc(hidden)]
pub struct Field_sortMethod;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TuplesortInstrumentation>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TuplesortInstrumentation>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TuplesortInstrumentation {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TuplesortInstrumentation, sortMethod) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sortMethod>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TuplesortInstrumentation>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_TuplesortMethod,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TuplesortInstrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::TuplesortInstrumentation> as c::expression::OrdinaryField<Field_sortMethod>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sortMethod) };
#[doc(hidden)]
pub struct Field_spaceType;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TuplesortInstrumentation, spaceType) == 4
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_spaceType>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TuplesortInstrumentation>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_TuplesortSpaceType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TuplesortInstrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::TuplesortInstrumentation> as c::expression::OrdinaryField<Field_spaceType>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).spaceType) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TuplesortInstrumentation, spaceUsed) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_spaceUsed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TuplesortInstrumentation>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TuplesortInstrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::TuplesortInstrumentation> as c::expression::OrdinaryField<Field_spaceUsed>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).spaceUsed) };
#[doc(hidden)]
pub struct Field_filler;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TwoPhasePredicateLockRecord>() == 20);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TwoPhasePredicateLockRecord>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TwoPhasePredicateLockRecord {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TwoPhasePredicateLockRecord, filler) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_filler>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TwoPhasePredicateLockRecord>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TwoPhasePredicateLockRecord, <<c::expression::CRecord<crate::__pgrx_c_bindings::TwoPhasePredicateLockRecord> as c::expression::OrdinaryField<Field_filler>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).filler) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TwoPhasePredicateRecord>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TwoPhasePredicateRecord>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TwoPhasePredicateRecord {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TwoPhasePredicateRecord, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TwoPhasePredicateRecord>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__TwoPhasePredicateRecordType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TwoPhasePredicateRecord, <<c::expression::CRecord<crate::__pgrx_c_bindings::TwoPhasePredicateRecord> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TwoPhasePredicateRecord, data) == 4);
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::TwoPhasePredicateRecord__bindgen_ty_1>()
            == 20
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::TwoPhasePredicateRecord__bindgen_ty_1>()
            == 4
    );
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_data>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TwoPhasePredicateRecord>
{
    type Member =
        c::expression::CRecord<crate::__pgrx_c_bindings::TwoPhasePredicateRecord__bindgen_ty_1>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TwoPhasePredicateRecord, <<c::expression::CRecord<crate::__pgrx_c_bindings::TwoPhasePredicateRecord> as c::expression::OrdinaryField<Field_data>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).data) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TwoPhasePredicateXactRecord>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TwoPhasePredicateXactRecord>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TwoPhasePredicateXactRecord {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TwoPhasePredicateXactRecord, xmin) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xmin>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TwoPhasePredicateXactRecord>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TwoPhasePredicateXactRecord, <<c::expression::CRecord<crate::__pgrx_c_bindings::TwoPhasePredicateXactRecord> as c::expression::OrdinaryField<Field_xmin>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xmin) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TwoPhasePredicateXactRecord, flags) == 4
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TwoPhasePredicateXactRecord>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TwoPhasePredicateXactRecord, <<c::expression::CRecord<crate::__pgrx_c_bindings::TwoPhasePredicateXactRecord> as c::expression::OrdinaryField<Field_flags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).flags) };
#[doc(hidden)]
pub struct Field_type_id;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TypeCacheEntry>() == 488);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TypeCacheEntry>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TypeCacheEntry {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, type_id) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TypeCacheEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry> as c::expression::OrdinaryField<Field_type_id>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_id) };
#[doc(hidden)]
pub struct Field_type_id_hash;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, type_id_hash) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type_id_hash>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TypeCacheEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry> as c::expression::OrdinaryField<Field_type_id_hash>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_id_hash) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, typlen) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typlen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TypeCacheEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry> as c::expression::OrdinaryField<Field_typlen>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typlen) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, typbyval) == 10);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typbyval>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 10;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TypeCacheEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry> as c::expression::OrdinaryField<Field_typbyval>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typbyval) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, typalign) == 11);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typalign>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 11;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TypeCacheEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry> as c::expression::OrdinaryField<Field_typalign>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typalign) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, typstorage) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typstorage>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TypeCacheEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry> as c::expression::OrdinaryField<Field_typstorage>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typstorage) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, typtype) == 13);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 13;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TypeCacheEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry> as c::expression::OrdinaryField<Field_typtype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typtype) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, typrelid) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typrelid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TypeCacheEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry> as c::expression::OrdinaryField<Field_typrelid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typrelid) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, typsubscript) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typsubscript>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TypeCacheEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry> as c::expression::OrdinaryField<Field_typsubscript>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typsubscript) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, typelem) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typelem>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TypeCacheEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry> as c::expression::OrdinaryField<Field_typelem>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typelem) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, typcollation) == 28);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typcollation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TypeCacheEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry> as c::expression::OrdinaryField<Field_typcollation>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typcollation) };
#[doc(hidden)]
pub struct Field_btree_opf;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, btree_opf) == 32);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_btree_opf>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TypeCacheEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry> as c::expression::OrdinaryField<Field_btree_opf>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).btree_opf) };
#[doc(hidden)]
pub struct Field_btree_opintype;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, btree_opintype) == 36
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_btree_opintype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TypeCacheEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry> as c::expression::OrdinaryField<Field_btree_opintype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).btree_opintype) };
#[doc(hidden)]
pub struct Field_hash_opf;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, hash_opf) == 40);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash_opf>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TypeCacheEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry> as c::expression::OrdinaryField<Field_hash_opf>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hash_opf) };
#[doc(hidden)]
pub struct Field_hash_opintype;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, hash_opintype) == 44);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash_opintype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TypeCacheEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry> as c::expression::OrdinaryField<Field_hash_opintype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hash_opintype) };
#[doc(hidden)]
pub struct Field_eq_opr;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, eq_opr) == 48);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_eq_opr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TypeCacheEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry> as c::expression::OrdinaryField<Field_eq_opr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).eq_opr) };
#[doc(hidden)]
pub struct Field_lt_opr;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, lt_opr) == 52);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lt_opr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TypeCacheEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry> as c::expression::OrdinaryField<Field_lt_opr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lt_opr) };
#[doc(hidden)]
pub struct Field_gt_opr;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, gt_opr) == 56);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_gt_opr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TypeCacheEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry> as c::expression::OrdinaryField<Field_gt_opr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).gt_opr) };
#[doc(hidden)]
pub struct Field_cmp_proc;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, cmp_proc) == 60);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cmp_proc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 60;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TypeCacheEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry> as c::expression::OrdinaryField<Field_cmp_proc>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cmp_proc) };
#[doc(hidden)]
pub struct Field_hash_proc;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, hash_proc) == 64);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash_proc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TypeCacheEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry> as c::expression::OrdinaryField<Field_hash_proc>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hash_proc) };
#[doc(hidden)]
pub struct Field_hash_extended_proc;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, hash_extended_proc) == 68
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash_extended_proc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 68;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TypeCacheEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry> as c::expression::OrdinaryField<Field_hash_extended_proc>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hash_extended_proc) };
#[doc(hidden)]
pub struct Field_tupDesc_identifier;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, tupDesc_identifier) == 272
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tupDesc_identifier>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 272;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TypeCacheEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry> as c::expression::OrdinaryField<Field_tupDesc_identifier>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tupDesc_identifier) };
#[doc(hidden)]
pub struct Field_rng_collation;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, rng_collation) == 288
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rng_collation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 288;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TypeCacheEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry> as c::expression::OrdinaryField<Field_rng_collation>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rng_collation) };
#[doc(hidden)]
pub struct Field_domainBaseType;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, domainBaseType) == 448
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_domainBaseType>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 448;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TypeCacheEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry> as c::expression::OrdinaryField<Field_domainBaseType>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).domainBaseType) };
#[doc(hidden)]
pub struct Field_domainBaseTypmod;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, domainBaseTypmod) == 452
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_domainBaseTypmod>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 452;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TypeCacheEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry> as c::expression::OrdinaryField<Field_domainBaseTypmod>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).domainBaseTypmod) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, flags) == 464);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 464;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TypeCacheEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry> as c::expression::OrdinaryField<Field_flags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).flags) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TypeCast>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TypeCast>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TypeCast {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCast, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCast>
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
    crate::__pgrx_c_bindings::TypeCast,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::TypeCast> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCast, location) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCast>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::TypeCast,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::TypeCast> as c::expression::OrdinaryField<
        Field_location,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Unique>() == 136);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Unique>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::Unique {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Unique, numCols) == 104);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numCols>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Unique>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Unique,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Unique> as c::expression::OrdinaryField<
        Field_numCols,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numCols) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
pub struct Field_umethod;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::UniquePath>() == 104);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::UniquePath>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::UniquePath {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::UniquePath, umethod) == 80);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
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
    const OFFSET: usize = 80;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::UniquePath, <<c::expression::CRecord<crate::__pgrx_c_bindings::UniquePath> as c::expression::OrdinaryField<Field_umethod>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).umethod) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
pub struct Field_in_operators;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::UniquePath, in_operators) == 88);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_in_operators>
    for c::expression::CRecord<crate::__pgrx_c_bindings::UniquePath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::UniquePath, <<c::expression::CRecord<crate::__pgrx_c_bindings::UniquePath> as c::expression::OrdinaryField<Field_in_operators>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).in_operators) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
pub struct Field_uniq_exprs;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::UniquePath, uniq_exprs) == 96);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_uniq_exprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::UniquePath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::UniquePath, <<c::expression::CRecord<crate::__pgrx_c_bindings::UniquePath> as c::expression::OrdinaryField<Field_uniq_exprs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).uniq_exprs) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::UnlistenStmt>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::UnlistenStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::UnlistenStmt {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::UnlistenStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::UnlistenStmt>
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
const _: Projection<crate::__pgrx_c_bindings::UnlistenStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::UnlistenStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::UpdateStmt>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::UpdateStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::UpdateStmt {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::UpdateStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::UpdateStmt>
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
const _: Projection<crate::__pgrx_c_bindings::UpdateStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::UpdateStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::UpdateStmt, targetList) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_targetList>
    for c::expression::CRecord<crate::__pgrx_c_bindings::UpdateStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::UpdateStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::UpdateStmt> as c::expression::OrdinaryField<Field_targetList>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).targetList) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::UpdateStmt, fromClause) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fromClause>
    for c::expression::CRecord<crate::__pgrx_c_bindings::UpdateStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::UpdateStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::UpdateStmt> as c::expression::OrdinaryField<Field_fromClause>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fromClause) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::UpdateStmt, returningList) == 40);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_returningList>
    for c::expression::CRecord<crate::__pgrx_c_bindings::UpdateStmt>
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
const _: Projection<crate::__pgrx_c_bindings::UpdateStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::UpdateStmt> as c::expression::OrdinaryField<Field_returningList>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).returningList) };
#[doc(hidden)]
pub struct Field_numkeys;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::UpperUniquePath>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::UpperUniquePath>() == 8);
};
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::UpperUniquePath {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::UpperUniquePath, numkeys) == 80);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numkeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::UpperUniquePath>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::UpperUniquePath, <<c::expression::CRecord<crate::__pgrx_c_bindings::UpperUniquePath> as c::expression::OrdinaryField<Field_numkeys>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numkeys) };
#[doc(hidden)]
pub struct Field_umid;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::UserMapping>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::UserMapping>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::UserMapping {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::UserMapping, umid) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_umid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::UserMapping>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::UserMapping, <<c::expression::CRecord<crate::__pgrx_c_bindings::UserMapping> as c::expression::OrdinaryField<Field_umid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).umid) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::UserMapping, userid) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_userid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::UserMapping>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::UserMapping, <<c::expression::CRecord<crate::__pgrx_c_bindings::UserMapping> as c::expression::OrdinaryField<Field_userid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).userid) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::UserMapping, serverid) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_serverid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::UserMapping>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::UserMapping, <<c::expression::CRecord<crate::__pgrx_c_bindings::UserMapping> as c::expression::OrdinaryField<Field_serverid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).serverid) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::UserMapping, options) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::UserMapping>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::UserMapping, <<c::expression::CRecord<crate::__pgrx_c_bindings::UserMapping> as c::expression::OrdinaryField<Field_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
#[doc(hidden)]
pub struct Field_attrtypid;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::VacAttrStats>() == 344);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::VacAttrStats>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::VacAttrStats {}
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacAttrStats, attrtypid) == 8);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attrtypid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacAttrStats>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VacAttrStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::VacAttrStats> as c::expression::OrdinaryField<Field_attrtypid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attrtypid) };
#[doc(hidden)]
pub struct Field_attrtypmod;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacAttrStats, attrtypmod) == 12);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attrtypmod>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacAttrStats>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VacAttrStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::VacAttrStats> as c::expression::OrdinaryField<Field_attrtypmod>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attrtypmod) };
#[doc(hidden)]
pub struct Field_attrcollid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacAttrStats, attrcollid) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attrcollid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacAttrStats>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VacAttrStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::VacAttrStats> as c::expression::OrdinaryField<Field_attrcollid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attrcollid) };
#[doc(hidden)]
pub struct Field_minrows;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacAttrStats, minrows) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_minrows>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacAttrStats>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VacAttrStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::VacAttrStats> as c::expression::OrdinaryField<Field_minrows>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).minrows) };
#[doc(hidden)]
pub struct Field_stats_valid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacAttrStats, stats_valid) == 64);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stats_valid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacAttrStats>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VacAttrStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::VacAttrStats> as c::expression::OrdinaryField<Field_stats_valid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stats_valid) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacAttrStats, stawidth) == 72);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stawidth>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacAttrStats>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VacAttrStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::VacAttrStats> as c::expression::OrdinaryField<Field_stawidth>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stawidth) };
#[doc(hidden)]
pub struct Field_tupattnum;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacAttrStats, tupattnum) == 296);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tupattnum>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacAttrStats>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 296;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VacAttrStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::VacAttrStats> as c::expression::OrdinaryField<Field_tupattnum>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tupattnum) };
#[doc(hidden)]
pub struct Field_rowstride;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacAttrStats, rowstride) == 336);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rowstride>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacAttrStats>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 336;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VacAttrStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::VacAttrStats> as c::expression::OrdinaryField<Field_rowstride>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rowstride) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
#[doc(hidden)]
pub struct Field_max_items;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::VacDeadItems>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::VacDeadItems>() == 4);
};
#[cfg(any(feature = "pg15", feature = "pg16"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::VacDeadItems {}
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacDeadItems, max_items) == 0);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_max_items>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacDeadItems>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VacDeadItems, <<c::expression::CRecord<crate::__pgrx_c_bindings::VacDeadItems> as c::expression::OrdinaryField<Field_max_items>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).max_items) };
#[doc(hidden)]
pub struct Field_num_items;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacDeadItems, num_items) == 4);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_items>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacDeadItems>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VacDeadItems, <<c::expression::CRecord<crate::__pgrx_c_bindings::VacDeadItems> as c::expression::OrdinaryField<Field_num_items>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_items) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacDeadItems, items) == 8);
#[cfg(any(feature = "pg15", feature = "pg16"))]
impl c::expression::OffsetField<Field_items>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacDeadItems>
{
    type Member = ();
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::VacDeadItems, items);
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::VacuumParams>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::VacuumParams>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::VacuumParams {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacuumParams, options) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacuumParams>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VacuumParams, <<c::expression::CRecord<crate::__pgrx_c_bindings::VacuumParams> as c::expression::OrdinaryField<Field_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacuumParams, freeze_min_age) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_freeze_min_age>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacuumParams>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VacuumParams, <<c::expression::CRecord<crate::__pgrx_c_bindings::VacuumParams> as c::expression::OrdinaryField<Field_freeze_min_age>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).freeze_min_age) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacuumParams, freeze_table_age) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_freeze_table_age>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacuumParams>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VacuumParams, <<c::expression::CRecord<crate::__pgrx_c_bindings::VacuumParams> as c::expression::OrdinaryField<Field_freeze_table_age>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).freeze_table_age) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::VacuumParams, multixact_freeze_min_age) == 12
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_multixact_freeze_min_age>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacuumParams>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VacuumParams, <<c::expression::CRecord<crate::__pgrx_c_bindings::VacuumParams> as c::expression::OrdinaryField<Field_multixact_freeze_min_age>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).multixact_freeze_min_age) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::VacuumParams, multixact_freeze_table_age)
        == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_multixact_freeze_table_age>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacuumParams>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VacuumParams, <<c::expression::CRecord<crate::__pgrx_c_bindings::VacuumParams> as c::expression::OrdinaryField<Field_multixact_freeze_table_age>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).multixact_freeze_table_age) };
#[doc(hidden)]
pub struct Field_is_wraparound;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacuumParams, is_wraparound) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_wraparound>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacuumParams>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VacuumParams, <<c::expression::CRecord<crate::__pgrx_c_bindings::VacuumParams> as c::expression::OrdinaryField<Field_is_wraparound>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_wraparound) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::VacuumParams, log_min_duration) == 24
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_log_min_duration>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacuumParams>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VacuumParams, <<c::expression::CRecord<crate::__pgrx_c_bindings::VacuumParams> as c::expression::OrdinaryField<Field_log_min_duration>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).log_min_duration) };
#[doc(hidden)]
pub struct Field_index_cleanup;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacuumParams, index_cleanup) == 28);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_index_cleanup>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacuumParams>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__VacOptValue,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VacuumParams, <<c::expression::CRecord<crate::__pgrx_c_bindings::VacuumParams> as c::expression::OrdinaryField<Field_index_cleanup>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).index_cleanup) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacuumParams, truncate) == 32);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_truncate>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacuumParams>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__VacOptValue,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VacuumParams, <<c::expression::CRecord<crate::__pgrx_c_bindings::VacuumParams> as c::expression::OrdinaryField<Field_truncate>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).truncate) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacuumParams, nworkers) == 36);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nworkers>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacuumParams>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VacuumParams, <<c::expression::CRecord<crate::__pgrx_c_bindings::VacuumParams> as c::expression::OrdinaryField<Field_nworkers>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nworkers) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::VacuumRelation>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::VacuumRelation>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::VacuumRelation {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacuumRelation, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacuumRelation>
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
const _: Projection<crate::__pgrx_c_bindings::VacuumRelation, <<c::expression::CRecord<crate::__pgrx_c_bindings::VacuumRelation> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacuumRelation, oid) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacuumRelation>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VacuumRelation, <<c::expression::CRecord<crate::__pgrx_c_bindings::VacuumRelation> as c::expression::OrdinaryField<Field_oid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oid) };
#[doc(hidden)]
pub struct Field_va_cols;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacuumRelation, va_cols) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_va_cols>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacuumRelation>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VacuumRelation, <<c::expression::CRecord<crate::__pgrx_c_bindings::VacuumRelation> as c::expression::OrdinaryField<Field_va_cols>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).va_cols) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::VacuumStmt>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::VacuumStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::VacuumStmt {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacuumStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacuumStmt>
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
const _: Projection<crate::__pgrx_c_bindings::VacuumStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::VacuumStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacuumStmt, options) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacuumStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VacuumStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::VacuumStmt> as c::expression::OrdinaryField<Field_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
#[doc(hidden)]
pub struct Field_rels;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacuumStmt, rels) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rels>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacuumStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VacuumStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::VacuumStmt> as c::expression::OrdinaryField<Field_rels>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rels) };
#[doc(hidden)]
pub struct Field_is_vacuumcmd;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacuumStmt, is_vacuumcmd) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_vacuumcmd>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacuumStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VacuumStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::VacuumStmt> as c::expression::OrdinaryField<Field_is_vacuumcmd>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_vacuumcmd) };
#[doc(hidden)]
pub struct Field_array_len;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ValuesScanState>() == 256);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ValuesScanState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ValuesScanState {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ValuesScanState, array_len) == 248);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_array_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ValuesScanState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 248;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ValuesScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ValuesScanState> as c::expression::OrdinaryField<Field_array_len>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).array_len) };
#[doc(hidden)]
pub struct Field_curr_idx;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ValuesScanState, curr_idx) == 252);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_curr_idx>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ValuesScanState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 252;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ValuesScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ValuesScanState> as c::expression::OrdinaryField<Field_curr_idx>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).curr_idx) };
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Var>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Var>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::Var {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Var, varno) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_varno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Var>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Var,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Var> as c::expression::OrdinaryField<
        Field_varno,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).varno) };
#[doc(hidden)]
pub struct Field_varattno;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Var, varattno) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_varattno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Var>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Var,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Var> as c::expression::OrdinaryField<
        Field_varattno,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).varattno) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Var, vartype) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vartype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Var>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Var,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Var> as c::expression::OrdinaryField<
        Field_vartype,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).vartype) };
#[doc(hidden)]
pub struct Field_vartypmod;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Var, vartypmod) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vartypmod>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Var>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Var,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Var> as c::expression::OrdinaryField<
        Field_vartypmod,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).vartypmod) };
#[doc(hidden)]
pub struct Field_varcollid;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Var, varcollid) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_varcollid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Var>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Var,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Var> as c::expression::OrdinaryField<
        Field_varcollid,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).varcollid) };
#[doc(hidden)]
pub struct Field_varlevelsup;
#[cfg(feature = "pg15")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Var, varlevelsup) == 24);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_varlevelsup>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Var>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Var,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Var> as c::expression::OrdinaryField<
        Field_varlevelsup,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).varlevelsup) };
#[doc(hidden)]
pub struct Field_varnosyn;
#[cfg(feature = "pg15")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Var, varnosyn) == 28);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_varnosyn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Var>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Var,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Var> as c::expression::OrdinaryField<
        Field_varnosyn,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).varnosyn) };
#[doc(hidden)]
pub struct Field_varattnosyn;
#[cfg(feature = "pg15")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Var, varattnosyn) == 32);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_varattnosyn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Var>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Var,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Var> as c::expression::OrdinaryField<
        Field_varattnosyn,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).varattnosyn) };
#[cfg(feature = "pg15")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Var, location) == 36);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Var>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Var,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Var> as c::expression::OrdinaryField<
        Field_location,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::VariableCacheData>() == 72);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::VariableCacheData>() == 8);
};
#[cfg(any(feature = "pg15", feature = "pg16"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::VariableCacheData {}
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VariableCacheData, nextOid) == 0);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nextOid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VariableCacheData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VariableCacheData, <<c::expression::CRecord<crate::__pgrx_c_bindings::VariableCacheData> as c::expression::OrdinaryField<Field_nextOid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nextOid) };
#[doc(hidden)]
pub struct Field_oidCount;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VariableCacheData, oidCount) == 4);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oidCount>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VariableCacheData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VariableCacheData, <<c::expression::CRecord<crate::__pgrx_c_bindings::VariableCacheData> as c::expression::OrdinaryField<Field_oidCount>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oidCount) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VariableCacheData, oldestXid) == 16);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldestXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VariableCacheData>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VariableCacheData, <<c::expression::CRecord<crate::__pgrx_c_bindings::VariableCacheData> as c::expression::OrdinaryField<Field_oldestXid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oldestXid) };
#[doc(hidden)]
pub struct Field_xidVacLimit;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::VariableCacheData, xidVacLimit) == 20
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xidVacLimit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VariableCacheData>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VariableCacheData, <<c::expression::CRecord<crate::__pgrx_c_bindings::VariableCacheData> as c::expression::OrdinaryField<Field_xidVacLimit>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xidVacLimit) };
#[doc(hidden)]
pub struct Field_xidWarnLimit;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::VariableCacheData, xidWarnLimit) == 24
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xidWarnLimit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VariableCacheData>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VariableCacheData, <<c::expression::CRecord<crate::__pgrx_c_bindings::VariableCacheData> as c::expression::OrdinaryField<Field_xidWarnLimit>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xidWarnLimit) };
#[doc(hidden)]
pub struct Field_xidStopLimit;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::VariableCacheData, xidStopLimit) == 28
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xidStopLimit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VariableCacheData>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VariableCacheData, <<c::expression::CRecord<crate::__pgrx_c_bindings::VariableCacheData> as c::expression::OrdinaryField<Field_xidStopLimit>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xidStopLimit) };
#[doc(hidden)]
pub struct Field_xidWrapLimit;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::VariableCacheData, xidWrapLimit) == 32
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xidWrapLimit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VariableCacheData>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VariableCacheData, <<c::expression::CRecord<crate::__pgrx_c_bindings::VariableCacheData> as c::expression::OrdinaryField<Field_xidWrapLimit>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xidWrapLimit) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::VariableCacheData, oldestXidDB) == 36
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldestXidDB>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VariableCacheData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VariableCacheData, <<c::expression::CRecord<crate::__pgrx_c_bindings::VariableCacheData> as c::expression::OrdinaryField<Field_oldestXidDB>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oldestXidDB) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::VariableCacheData, oldestCommitTsXid) == 40
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldestCommitTsXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VariableCacheData>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VariableCacheData, <<c::expression::CRecord<crate::__pgrx_c_bindings::VariableCacheData> as c::expression::OrdinaryField<Field_oldestCommitTsXid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oldestCommitTsXid) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::VariableCacheData, newestCommitTsXid) == 44
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_newestCommitTsXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VariableCacheData>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VariableCacheData, <<c::expression::CRecord<crate::__pgrx_c_bindings::VariableCacheData> as c::expression::OrdinaryField<Field_newestCommitTsXid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).newestCommitTsXid) };
#[doc(hidden)]
pub struct Field_xactCompletionCount;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::VariableCacheData, xactCompletionCount) == 56
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xactCompletionCount>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VariableCacheData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VariableCacheData, <<c::expression::CRecord<crate::__pgrx_c_bindings::VariableCacheData> as c::expression::OrdinaryField<Field_xactCompletionCount>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xactCompletionCount) };
#[doc(hidden)]
pub struct Field_oldestClogXid;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::VariableCacheData, oldestClogXid) == 64
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldestClogXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VariableCacheData>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VariableCacheData, <<c::expression::CRecord<crate::__pgrx_c_bindings::VariableCacheData> as c::expression::OrdinaryField<Field_oldestClogXid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oldestClogXid) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::VariableSetStmt>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::VariableSetStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::VariableSetStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VariableSetStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VariableSetStmt>
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
const _: Projection<crate::__pgrx_c_bindings::VariableSetStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::VariableSetStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VariableSetStmt, kind) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_kind>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VariableSetStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__VariableSetKind,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VariableSetStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::VariableSetStmt> as c::expression::OrdinaryField<Field_kind>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).kind) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VariableSetStmt, args) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_args>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VariableSetStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VariableSetStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::VariableSetStmt> as c::expression::OrdinaryField<Field_args>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).args) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VariableSetStmt, is_local) == 24);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_local>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VariableSetStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VariableSetStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::VariableSetStmt> as c::expression::OrdinaryField<Field_is_local>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_local) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::VariableShowStmt>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::VariableShowStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::VariableShowStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VariableShowStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VariableShowStmt>
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
const _: Projection<crate::__pgrx_c_bindings::VariableShowStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::VariableShowStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_statsTuple;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::VariableStatData>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::VariableStatData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::VariableStatData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VariableStatData, statsTuple) == 16);
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::HeapTupleData>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::HeapTupleData>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_statsTuple>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VariableStatData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::HeapTupleData>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VariableStatData, <<c::expression::CRecord<crate::__pgrx_c_bindings::VariableStatData> as c::expression::OrdinaryField<Field_statsTuple>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).statsTuple) };
#[doc(hidden)]
pub struct Field_freefunc;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VariableStatData, freefunc) == 24);
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::HeapTupleData) -> (),
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::HeapTupleData) -> (),
            >,
        >() == 8
    );
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_freefunc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VariableStatData>
{
    type Member = c::expression::CFunction<
        crate::__pgrx_c_generated::Signature_void__0l__struct__HeapTupleData__0p__0r,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VariableStatData, <<c::expression::CRecord<crate::__pgrx_c_bindings::VariableStatData> as c::expression::OrdinaryField<Field_freefunc>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).freefunc) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VariableStatData, vartype) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vartype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VariableStatData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VariableStatData, <<c::expression::CRecord<crate::__pgrx_c_bindings::VariableStatData> as c::expression::OrdinaryField<Field_vartype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).vartype) };
#[doc(hidden)]
pub struct Field_atttype;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VariableStatData, atttype) == 36);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_atttype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VariableStatData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VariableStatData, <<c::expression::CRecord<crate::__pgrx_c_bindings::VariableStatData> as c::expression::OrdinaryField<Field_atttype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).atttype) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VariableStatData, atttypmod) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_atttypmod>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VariableStatData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VariableStatData, <<c::expression::CRecord<crate::__pgrx_c_bindings::VariableStatData> as c::expression::OrdinaryField<Field_atttypmod>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).atttypmod) };
#[doc(hidden)]
pub struct Field_isunique;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VariableStatData, isunique) == 44);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isunique>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VariableStatData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VariableStatData, <<c::expression::CRecord<crate::__pgrx_c_bindings::VariableStatData> as c::expression::OrdinaryField<Field_isunique>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isunique) };
#[doc(hidden)]
pub struct Field_acl_ok;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VariableStatData, acl_ok) == 45);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_acl_ok>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VariableStatData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 45;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VariableStatData, <<c::expression::CRecord<crate::__pgrx_c_bindings::VariableStatData> as c::expression::OrdinaryField<Field_acl_ok>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).acl_ok) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ViewOptions>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ViewOptions>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ViewOptions {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ViewOptions, vl_len_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vl_len_>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ViewOptions>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ViewOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::ViewOptions> as c::expression::OrdinaryField<Field_vl_len_>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).vl_len_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ViewOptions, security_barrier) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_security_barrier>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ViewOptions>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ViewOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::ViewOptions> as c::expression::OrdinaryField<Field_security_barrier>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).security_barrier) };
#[doc(hidden)]
pub struct Field_security_invoker;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ViewOptions, security_invoker) == 5);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_security_invoker>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ViewOptions>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 5;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ViewOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::ViewOptions> as c::expression::OrdinaryField<Field_security_invoker>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).security_invoker) };
#[doc(hidden)]
pub struct Field_check_option;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ViewOptions, check_option) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_check_option>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ViewOptions>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ViewOptCheckOption,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ViewOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::ViewOptions> as c::expression::OrdinaryField<Field_check_option>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).check_option) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ViewStmt>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ViewStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ViewStmt {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ViewStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ViewStmt>
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
    crate::__pgrx_c_bindings::ViewStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ViewStmt> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_aliases;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ViewStmt, aliases) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aliases>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ViewStmt>
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
    crate::__pgrx_c_bindings::ViewStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ViewStmt> as c::expression::OrdinaryField<
        Field_aliases,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).aliases) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ViewStmt, replace) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_replace>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ViewStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ViewStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ViewStmt> as c::expression::OrdinaryField<
        Field_replace,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).replace) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ViewStmt, options) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ViewStmt>
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
    crate::__pgrx_c_bindings::ViewStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ViewStmt> as c::expression::OrdinaryField<
        Field_options,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
#[doc(hidden)]
pub struct Field_withCheckOption;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ViewStmt, withCheckOption) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_withCheckOption>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ViewStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ViewCheckOption,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ViewStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ViewStmt> as c::expression::OrdinaryField<
        Field_withCheckOption,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).withCheckOption) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::VirtualTupleTableSlot>() == 72);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::VirtualTupleTableSlot>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::VirtualTupleTableSlot {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VirtualTupleTableSlot, data) == 64);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_data>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VirtualTupleTableSlot>
{
    type Member = c::expression::CPointer<
        c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VirtualTupleTableSlot, <<c::expression::CRecord<crate::__pgrx_c_bindings::VirtualTupleTableSlot> as c::expression::OrdinaryField<Field_data>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).data) };
#[doc(hidden)]
pub struct Field_ws_file;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::WALOpenSegment>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::WALOpenSegment>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::WALOpenSegment {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WALOpenSegment, ws_file) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ws_file>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WALOpenSegment>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WALOpenSegment, <<c::expression::CRecord<crate::__pgrx_c_bindings::WALOpenSegment> as c::expression::OrdinaryField<Field_ws_file>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ws_file) };
#[doc(hidden)]
pub struct Field_ws_segno;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WALOpenSegment, ws_segno) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ws_segno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WALOpenSegment>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WALOpenSegment, <<c::expression::CRecord<crate::__pgrx_c_bindings::WALOpenSegment> as c::expression::OrdinaryField<Field_ws_segno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ws_segno) };
#[doc(hidden)]
pub struct Field_ws_tli;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WALOpenSegment, ws_tli) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ws_tli>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WALOpenSegment>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WALOpenSegment, <<c::expression::CRecord<crate::__pgrx_c_bindings::WALOpenSegment> as c::expression::OrdinaryField<Field_ws_tli>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ws_tli) };
#[doc(hidden)]
pub struct Field_wre_errno;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::WALReadError>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::WALReadError>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::WALReadError {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WALReadError, wre_errno) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wre_errno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WALReadError>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WALReadError, <<c::expression::CRecord<crate::__pgrx_c_bindings::WALReadError> as c::expression::OrdinaryField<Field_wre_errno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wre_errno) };
#[doc(hidden)]
pub struct Field_wre_off;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WALReadError, wre_off) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wre_off>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WALReadError>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WALReadError, <<c::expression::CRecord<crate::__pgrx_c_bindings::WALReadError> as c::expression::OrdinaryField<Field_wre_off>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wre_off) };
#[doc(hidden)]
pub struct Field_wre_req;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WALReadError, wre_req) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wre_req>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WALReadError>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WALReadError, <<c::expression::CRecord<crate::__pgrx_c_bindings::WALReadError> as c::expression::OrdinaryField<Field_wre_req>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wre_req) };
#[doc(hidden)]
pub struct Field_wre_read;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WALReadError, wre_read) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wre_read>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WALReadError>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WALReadError, <<c::expression::CRecord<crate::__pgrx_c_bindings::WALReadError> as c::expression::OrdinaryField<Field_wre_read>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wre_read) };
#[doc(hidden)]
pub struct Field_ws_segsize;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::WALSegmentContext>() == 1028);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::WALSegmentContext>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::WALSegmentContext {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WALSegmentContext, ws_segsize) == 1024
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ws_segsize>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WALSegmentContext>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1024;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WALSegmentContext, <<c::expression::CRecord<crate::__pgrx_c_bindings::WALSegmentContext> as c::expression::OrdinaryField<Field_ws_segsize>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ws_segsize) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::WaitEvent>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::WaitEvent>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::WaitEvent {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WaitEvent, pos) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WaitEvent>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::WaitEvent,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::WaitEvent> as c::expression::OrdinaryField<
        Field_pos,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pos) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WaitEvent, events) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_events>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WaitEvent>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::WaitEvent,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::WaitEvent> as c::expression::OrdinaryField<
        Field_events,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).events) };
#[doc(hidden)]
pub struct Field_fd;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WaitEvent, fd) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fd>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WaitEvent>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::WaitEvent,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::WaitEvent> as c::expression::OrdinaryField<
        Field_fd,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fd) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::WalRcvExecResult>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::WalRcvExecResult>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::WalRcvExecResult {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WalRcvExecResult, status) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_status>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvExecResult>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_WalRcvExecStatus,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WalRcvExecResult, <<c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvExecResult> as c::expression::OrdinaryField<Field_status>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).status) };
#[doc(hidden)]
pub struct Field_sqlstate;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WalRcvExecResult, sqlstate) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sqlstate>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvExecResult>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WalRcvExecResult, <<c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvExecResult> as c::expression::OrdinaryField<Field_sqlstate>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sqlstate) };
#[doc(hidden)]
pub struct Field_startpointTLI;
