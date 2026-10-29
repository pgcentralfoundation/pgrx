// Shared generated C macro support.

#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_slots>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SlruSharedData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SlruSharedData, lsn_groups_per_page) == 80
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lsn_groups_per_page>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SlruSharedData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SlruSharedData, slru_stats_idx) == 96
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_slru_stats_idx>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SlruSharedData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_redirectXid;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistState, redirectXid) == 88);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_redirectXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistState>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpGistState, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpGistState> as c::expression::OrdinaryField<Field_redirectXid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).redirectXid) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SplitPageLayout>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SplitPageLayout>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SplitPageLayout {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SplitPageLayout, lenlist) == 16);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lenlist>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SplitPageLayout>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SplitPageLayout, <<c::expression::CRecord<crate::__pgrx_c_bindings::SplitPageLayout> as c::expression::OrdinaryField<Field_lenlist>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lenlist) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SplitPageLayout, page) == 32);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_page>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SplitPageLayout>
{
    type Member = c::expression::CPointer<
        c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SplitPageLayout, <<c::expression::CRecord<crate::__pgrx_c_bindings::SplitPageLayout> as c::expression::OrdinaryField<Field_page>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).page) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SplitPageLayout, buffer) == 40);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_buffer>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SplitPageLayout>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SplitPageLayout, <<c::expression::CRecord<crate::__pgrx_c_bindings::SplitPageLayout> as c::expression::OrdinaryField<Field_buffer>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).buffer) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_ownersuperuser;
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Subscription, ownersuperuser) == 28);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ownersuperuser>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Subscription>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Subscription, <<c::expression::CRecord<crate::__pgrx_c_bindings::Subscription> as c::expression::OrdinaryField<Field_ownersuperuser>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ownersuperuser) };
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Subscription, enabled) == 29);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_enabled>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Subscription>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 29;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Subscription, binary) == 30);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_binary>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Subscription>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 30;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Subscription, stream) == 31);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stream>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Subscription>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 31;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Subscription, twophasestate) == 32);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_twophasestate>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Subscription>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Subscription, disableonerr) == 33);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_disableonerr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Subscription>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 33;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::Subscription, passwordrequired) == 34
);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_passwordrequired>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Subscription>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 34;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Subscription, runasowner) == 35);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_runasowner>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Subscription>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 35;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Subscription, failover) == 36);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_failover>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Subscription>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Subscription, <<c::expression::CRecord<crate::__pgrx_c_bindings::Subscription> as c::expression::OrdinaryField<Field_failover>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).failover) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TableFunc>() == 128);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TableFunc>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_functype;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TableFunc, functype) == 4);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_functype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableFunc>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__TableFuncType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::TableFunc,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::TableFunc> as c::expression::OrdinaryField<
        Field_functype,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).functype) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_colvalexprs;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TableFunc, colvalexprs) == 88);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_colvalexprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableFunc>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::TableFunc,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::TableFunc> as c::expression::OrdinaryField<
        Field_colvalexprs,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).colvalexprs) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_passingvalexprs;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TableFunc, passingvalexprs) == 96);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_passingvalexprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableFunc>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::TableFunc,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::TableFunc> as c::expression::OrdinaryField<
        Field_passingvalexprs,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).passingvalexprs) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TableFunc, ordinalitycol) == 120);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ordinalitycol>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableFunc>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TableFunc, location) == 124);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableFunc>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 124;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TableFuncScanState>() == 352);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TableFuncScanState>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TableFuncScanState, colvalexprs) == 256
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_colvalexprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableFuncScanState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 256;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableFuncScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableFuncScanState> as c::expression::OrdinaryField<Field_colvalexprs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).colvalexprs) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TableFuncScanState, passingvalexprs) == 264
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_passingvalexprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableFuncScanState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 264;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableFuncScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableFuncScanState> as c::expression::OrdinaryField<Field_passingvalexprs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).passingvalexprs) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TableFuncScanState, ns_names) == 272);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ns_names>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableFuncScanState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 272;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TableFuncScanState, ns_uris) == 280);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ns_uris>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableFuncScanState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 280;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TableFuncScanState, ordinal) == 328);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ordinal>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableFuncScanState>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 328;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TidScanState>() == 256);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TidScanState>() == 8);
};
#[cfg(feature = "pg17")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TidStoreIterResult>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TidStoreIterResult>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TidStoreIterResult {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TidStoreIterResult, blkno) == 0);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_blkno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TidStoreIterResult>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TidStoreIterResult, <<c::expression::CRecord<crate::__pgrx_c_bindings::TidStoreIterResult> as c::expression::OrdinaryField<Field_blkno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).blkno) };
#[cfg(feature = "pg17")]
#[doc(hidden)]
pub struct Field_max_offset;
#[cfg(feature = "pg17")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TidStoreIterResult, max_offset) == 4);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_max_offset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TidStoreIterResult>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(feature = "pg17")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TidStoreIterResult, <<c::expression::CRecord<crate::__pgrx_c_bindings::TidStoreIterResult> as c::expression::OrdinaryField<Field_max_offset>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).max_offset) };
#[cfg(feature = "pg17")]
#[doc(hidden)]
pub struct Field_num_offsets;
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TidStoreIterResult, num_offsets) == 8
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_offsets>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TidStoreIterResult>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg17")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TidStoreIterResult, <<c::expression::CRecord<crate::__pgrx_c_bindings::TidStoreIterResult> as c::expression::OrdinaryField<Field_num_offsets>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_offsets) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TransactionStmt, location) == 36);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TransactionStmt>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TransactionStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::TransactionStmt> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TransamVariablesData>() == 72);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TransamVariablesData>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TransamVariablesData {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TransamVariablesData, nextOid) == 0);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nextOid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TransamVariablesData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TransamVariablesData, <<c::expression::CRecord<crate::__pgrx_c_bindings::TransamVariablesData> as c::expression::OrdinaryField<Field_nextOid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nextOid) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TransamVariablesData, oidCount) == 4);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oidCount>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TransamVariablesData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TransamVariablesData, <<c::expression::CRecord<crate::__pgrx_c_bindings::TransamVariablesData> as c::expression::OrdinaryField<Field_oidCount>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oidCount) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TransamVariablesData, oldestXid) == 16
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldestXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TransamVariablesData>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TransamVariablesData, <<c::expression::CRecord<crate::__pgrx_c_bindings::TransamVariablesData> as c::expression::OrdinaryField<Field_oldestXid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oldestXid) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TransamVariablesData, xidVacLimit) == 20
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xidVacLimit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TransamVariablesData>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TransamVariablesData, <<c::expression::CRecord<crate::__pgrx_c_bindings::TransamVariablesData> as c::expression::OrdinaryField<Field_xidVacLimit>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xidVacLimit) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TransamVariablesData, xidWarnLimit) == 24
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xidWarnLimit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TransamVariablesData>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TransamVariablesData, <<c::expression::CRecord<crate::__pgrx_c_bindings::TransamVariablesData> as c::expression::OrdinaryField<Field_xidWarnLimit>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xidWarnLimit) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TransamVariablesData, xidStopLimit) == 28
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xidStopLimit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TransamVariablesData>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TransamVariablesData, <<c::expression::CRecord<crate::__pgrx_c_bindings::TransamVariablesData> as c::expression::OrdinaryField<Field_xidStopLimit>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xidStopLimit) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TransamVariablesData, xidWrapLimit) == 32
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xidWrapLimit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TransamVariablesData>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TransamVariablesData, <<c::expression::CRecord<crate::__pgrx_c_bindings::TransamVariablesData> as c::expression::OrdinaryField<Field_xidWrapLimit>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xidWrapLimit) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TransamVariablesData, oldestXidDB) == 36
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldestXidDB>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TransamVariablesData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TransamVariablesData, <<c::expression::CRecord<crate::__pgrx_c_bindings::TransamVariablesData> as c::expression::OrdinaryField<Field_oldestXidDB>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oldestXidDB) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TransamVariablesData, oldestCommitTsXid)
        == 40
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldestCommitTsXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TransamVariablesData>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TransamVariablesData, <<c::expression::CRecord<crate::__pgrx_c_bindings::TransamVariablesData> as c::expression::OrdinaryField<Field_oldestCommitTsXid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oldestCommitTsXid) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TransamVariablesData, newestCommitTsXid)
        == 44
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_newestCommitTsXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TransamVariablesData>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TransamVariablesData, <<c::expression::CRecord<crate::__pgrx_c_bindings::TransamVariablesData> as c::expression::OrdinaryField<Field_newestCommitTsXid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).newestCommitTsXid) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TransamVariablesData, xactCompletionCount)
        == 56
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xactCompletionCount>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TransamVariablesData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TransamVariablesData, <<c::expression::CRecord<crate::__pgrx_c_bindings::TransamVariablesData> as c::expression::OrdinaryField<Field_xactCompletionCount>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xactCompletionCount) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TransamVariablesData, oldestClogXid) == 64
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldestClogXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TransamVariablesData>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TransamVariablesData, <<c::expression::CRecord<crate::__pgrx_c_bindings::TransamVariablesData> as c::expression::OrdinaryField<Field_oldestClogXid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oldestClogXid) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_rng_opfamily;
#[cfg(feature = "pg17")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, rng_opfamily) == 288);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rng_opfamily>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 288;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TypeCacheEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry> as c::expression::OrdinaryField<Field_rng_opfamily>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rng_opfamily) };
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, rng_collation) == 292
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rng_collation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 292;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::UploadManifestCmd>() == 4);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::UploadManifestCmd>() == 4);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::UploadManifestCmd {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::UploadManifestCmd, type_) == 0);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::UploadManifestCmd>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::UploadManifestCmd, <<c::expression::CRecord<crate::__pgrx_c_bindings::UploadManifestCmd> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacAttrStats, attstattarget) == 0);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attstattarget>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacAttrStats>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VacAttrStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::VacAttrStats> as c::expression::OrdinaryField<Field_attstattarget>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attstattarget) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacAttrStats, attrtypid) == 4);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attrtypid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacAttrStats>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacAttrStats, attrtypmod) == 8);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attrtypmod>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacAttrStats>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_max_bytes;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::VacDeadItemsInfo>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::VacDeadItemsInfo>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::VacDeadItemsInfo {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacDeadItemsInfo, max_bytes) == 0);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_max_bytes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacDeadItemsInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VacDeadItemsInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::VacDeadItemsInfo> as c::expression::OrdinaryField<Field_max_bytes>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).max_bytes) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacDeadItemsInfo, num_items) == 8);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_items>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacDeadItemsInfo>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VacDeadItemsInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::VacDeadItemsInfo> as c::expression::OrdinaryField<Field_num_items>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_items) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::WalReceiverFunctionsType>() == 136);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::WalReceiverFunctionsType>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *const ::core::ffi::c_char,
                    bool,
                    bool,
                    bool,
                    *const ::core::ffi::c_char,
                    *mut *mut ::core::ffi::c_char,
                )
                    -> *mut crate::__pgrx_c_bindings::WalReceiverConn,
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *const ::core::ffi::c_char,
                    bool,
                    bool,
                    bool,
                    *const ::core::ffi::c_char,
                    *mut *mut ::core::ffi::c_char,
                )
                    -> *mut crate::__pgrx_c_bindings::WalReceiverConn,
            >,
        >() == 8
    );
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_walrcv_connect>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalReceiverFunctionsType>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_struct__WalReceiverConn__0p__0l__const__char__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__const__char__0p__0c__char__0p__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_walrcv_get_dbname_from_conninfo;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::WalReceiverFunctionsType,
        walrcv_get_dbname_from_conninfo
    ) == 40
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(*const ::core::ffi::c_char) -> *mut ::core::ffi::c_char,
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(*const ::core::ffi::c_char) -> *mut ::core::ffi::c_char,
            >,
        >() == 8
    );
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_walrcv_get_dbname_from_conninfo>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalReceiverFunctionsType>
{
    type Member = c::expression::CFunction<
        crate::__pgrx_c_generated::Signature_char__0p__0l__const__char__0p__0r,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WalReceiverFunctionsType, <<c::expression::CRecord<crate::__pgrx_c_bindings::WalReceiverFunctionsType> as c::expression::OrdinaryField<Field_walrcv_get_dbname_from_conninfo>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).walrcv_get_dbname_from_conninfo) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::WalReceiverFunctionsType,
        walrcv_server_version
    ) == 48
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_walrcv_server_version>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalReceiverFunctionsType>
{
    type Member = c::expression::CFunction<
        crate::__pgrx_c_generated::Signature_int__0l__struct__WalReceiverConn__0p__0r,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::WalReceiverFunctionsType,
        walrcv_readtimelinehistoryfile
    ) == 56
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_walrcv_readtimelinehistoryfile>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalReceiverFunctionsType>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_void__0l__struct__WalReceiverConn__0p__0c__unsigned__int__0c__char__0p__0p__0c__char__0p__0p__0c__int__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::WalReceiverFunctionsType,
        walrcv_startstreaming
    ) == 64
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_walrcv_startstreaming>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalReceiverFunctionsType>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__WalReceiverConn__0p__0c__const__WalRcvStreamOptions__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::WalReceiverFunctionsType,
        walrcv_endstreaming
    ) == 72
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_walrcv_endstreaming>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalReceiverFunctionsType>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_void__0l__struct__WalReceiverConn__0p__0c__unsigned__int__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WalReceiverFunctionsType, walrcv_receive)
        == 80
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_walrcv_receive>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalReceiverFunctionsType>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_int__0l__struct__WalReceiverConn__0p__0c__char__0p__0p__0c__int__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WalReceiverFunctionsType, walrcv_send) == 88
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_walrcv_send>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalReceiverFunctionsType>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_void__0l__struct__WalReceiverConn__0p__0c__const__char__0p__0c__int__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WalReceiverFunctionsType, walrcv_create_slot)
        == 96
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::WalReceiverConn,
                    *const ::core::ffi::c_char,
                    bool,
                    bool,
                    bool,
                    u32,
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
                    *const ::core::ffi::c_char,
                    bool,
                    bool,
                    bool,
                    u32,
                    *mut u64,
                ) -> *mut ::core::ffi::c_char,
            >,
        >() == 8
    );
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_walrcv_create_slot>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalReceiverFunctionsType>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_char__0p__0l__struct__WalReceiverConn__0p__0c__const__char__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__CRSSnapshotAction__0c__unsigned__long__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_walrcv_alter_slot;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WalReceiverFunctionsType, walrcv_alter_slot)
        == 104
);
#[cfg(feature = "pg17")]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::WalReceiverConn,
                    *const ::core::ffi::c_char,
                    bool,
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
                    bool,
                ) -> (),
            >,
        >() == 8
    );
};
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_walrcv_alter_slot>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalReceiverFunctionsType>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_void__0l__struct__WalReceiverConn__0p__0c__const__char__0p__0c__0x5f426f6f6c__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WalReceiverFunctionsType, <<c::expression::CRecord<crate::__pgrx_c_bindings::WalReceiverFunctionsType> as c::expression::OrdinaryField<Field_walrcv_alter_slot>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).walrcv_alter_slot) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::WalReceiverFunctionsType,
        walrcv_get_backend_pid
    ) == 112
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_walrcv_get_backend_pid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalReceiverFunctionsType>
{
    type Member = c::expression::CFunction<
        crate::__pgrx_c_generated::Signature_int__0l__struct__WalReceiverConn__0p__0r,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WalReceiverFunctionsType, walrcv_exec) == 120
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_walrcv_exec>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalReceiverFunctionsType>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_struct__WalRcvExecResult__0p__0l__struct__WalReceiverConn__0p__0c__const__char__0p__0c__int__0c__const__unsigned__int__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WalReceiverFunctionsType, walrcv_disconnect)
        == 128
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_walrcv_disconnect>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalReceiverFunctionsType>
{
    type Member = c::expression::CFunction<
        crate::__pgrx_c_generated::Signature_void__0l__struct__WalReceiverConn__0p__0r,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
#[cfg(feature = "pg17")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::WindowAggPath>() == 112);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::WindowAggPath>() == 8);
};
#[cfg(feature = "pg17")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggPath, runCondition) == 96);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_runCondition>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggPath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowAggPath, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggPath> as c::expression::OrdinaryField<Field_runCondition>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).runCondition) };
#[cfg(feature = "pg17")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggPath, topwindow) == 104);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_topwindow>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggPath>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::WindowClause>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::WindowClause>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowClause, startInRangeFunc) == 64
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_startInRangeFunc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowClause>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowClause, endInRangeFunc) == 68);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_endInRangeFunc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowClause>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 68;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowClause, inRangeColl) == 72);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inRangeColl>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowClause>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowClause, inRangeAsc) == 76);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inRangeAsc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowClause>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 76;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowClause, inRangeNullsFirst) == 77
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inRangeNullsFirst>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowClause>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 77;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowClause, winref) == 80);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_winref>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowClause>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowClause, copiedOrder) == 84);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_copiedOrder>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowClause>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 84;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::WindowFunc>() == 64);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::WindowFunc>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowFunc, runCondition) == 40);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_runCondition>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowFunc>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowFunc, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowFunc> as c::expression::OrdinaryField<Field_runCondition>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).runCondition) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowFunc, winref) == 48);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_winref>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowFunc>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowFunc, winstar) == 52);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_winstar>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowFunc>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowFunc, winagg) == 53);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_winagg>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowFunc>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 53;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowFunc, location) == 56);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowFunc>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::WindowFuncRunCondition>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::WindowFuncRunCondition>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::WindowFuncRunCondition {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowFuncRunCondition, opno) == 4);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_opno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowFuncRunCondition>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowFuncRunCondition, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowFuncRunCondition> as c::expression::OrdinaryField<Field_opno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).opno) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowFuncRunCondition, inputcollid) == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inputcollid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowFuncRunCondition>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowFuncRunCondition, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowFuncRunCondition> as c::expression::OrdinaryField<Field_inputcollid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).inputcollid) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_wfunc_left;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowFuncRunCondition, wfunc_left) == 12
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wfunc_left>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowFuncRunCondition>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowFuncRunCondition, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowFuncRunCondition> as c::expression::OrdinaryField<Field_wfunc_left>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wfunc_left) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::catcache, cc_nkeys) == 104);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cc_nkeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::catcache>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::catcache, cc_ntup) == 108);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cc_ntup>
    for c::expression::CRecord<crate::__pgrx_c_bindings::catcache>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 108;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::catcache, cc_nlist) == 112);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cc_nlist>
    for c::expression::CRecord<crate::__pgrx_c_bindings::catcache>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::catcache, cc_nlbuckets) == 116);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cc_nlbuckets>
    for c::expression::CRecord<crate::__pgrx_c_bindings::catcache>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 116;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::spgxlogState, redirectXid) == 0);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_redirectXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::spgxlogState>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::spgxlogState, <<c::expression::CRecord<crate::__pgrx_c_bindings::spgxlogState> as c::expression::OrdinaryField<Field_redirectXid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).redirectXid) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::xl_clog_truncate>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::xl_clog_truncate>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pageno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_clog_truncate>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_clog_truncate, oldestXact) == 8);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldestXact>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_clog_truncate>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_clog_truncate, oldestXactDb) == 12
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldestXactDb>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_clog_truncate>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::xl_commit_ts_truncate>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::xl_commit_ts_truncate>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pageno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_commit_ts_truncate>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_commit_ts_truncate, oldestXid) == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldestXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_commit_ts_truncate>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::xl_end_of_recovery>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::xl_end_of_recovery>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_end_of_recovery, wal_level) == 16);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wal_level>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_end_of_recovery>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_end_of_recovery, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_end_of_recovery> as c::expression::OrdinaryField<Field_wal_level>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wal_level) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_reason;
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::xl_heap_prune>() == 2);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::xl_heap_prune>() == 1);
};
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_heap_prune, reason) == 0);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_reason>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_prune>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_heap_prune, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_prune> as c::expression::OrdinaryField<Field_reason>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).reason) };
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_heap_prune, flags) == 1);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_prune>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_heap_prune, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_prune> as c::expression::OrdinaryField<Field_flags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).flags) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::xlhp_freeze_plan>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::xlhp_freeze_plan>() == 4);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::xlhp_freeze_plan {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::xlhp_freeze_plan, xmax) == 0);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xmax>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xlhp_freeze_plan>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xlhp_freeze_plan, <<c::expression::CRecord<crate::__pgrx_c_bindings::xlhp_freeze_plan> as c::expression::OrdinaryField<Field_xmax>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xmax) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::xlhp_freeze_plan, t_infomask2) == 4);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t_infomask2>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xlhp_freeze_plan>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xlhp_freeze_plan, <<c::expression::CRecord<crate::__pgrx_c_bindings::xlhp_freeze_plan> as c::expression::OrdinaryField<Field_t_infomask2>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t_infomask2) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::xlhp_freeze_plan, t_infomask) == 6);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t_infomask>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xlhp_freeze_plan>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 6;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xlhp_freeze_plan, <<c::expression::CRecord<crate::__pgrx_c_bindings::xlhp_freeze_plan> as c::expression::OrdinaryField<Field_t_infomask>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t_infomask) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::xlhp_freeze_plan, frzflags) == 8);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_frzflags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xlhp_freeze_plan>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xlhp_freeze_plan, <<c::expression::CRecord<crate::__pgrx_c_bindings::xlhp_freeze_plan> as c::expression::OrdinaryField<Field_frzflags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).frzflags) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::xlhp_freeze_plan, ntuples) == 10);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ntuples>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xlhp_freeze_plan>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 10;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xlhp_freeze_plan, <<c::expression::CRecord<crate::__pgrx_c_bindings::xlhp_freeze_plan> as c::expression::OrdinaryField<Field_ntuples>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ntuples) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::xlhp_freeze_plans>() == 4);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::xlhp_freeze_plans>() == 4);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::xlhp_freeze_plans {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::xlhp_freeze_plans, nplans) == 0);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nplans>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xlhp_freeze_plans>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xlhp_freeze_plans, <<c::expression::CRecord<crate::__pgrx_c_bindings::xlhp_freeze_plans> as c::expression::OrdinaryField<Field_nplans>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nplans) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_ntargets;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::xlhp_prune_items>() == 2);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::xlhp_prune_items>() == 2);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::xlhp_prune_items {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::xlhp_prune_items, ntargets) == 0);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ntargets>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xlhp_prune_items>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xlhp_prune_items, <<c::expression::CRecord<crate::__pgrx_c_bindings::xlhp_prune_items> as c::expression::OrdinaryField<Field_ntargets>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ntargets) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::xlhp_prune_items, data) == 2);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_data>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xlhp_prune_items>
{
    type Member = c::expression::CFlexibleArray<
        c::CUnsignedShort,
        crate::__pgrx_c_bindings::__IncompleteArrayField<u16>,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 2;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xlhp_prune_items, <<c::expression::CRecord<crate::__pgrx_c_bindings::xlhp_prune_items> as c::expression::OrdinaryField<Field_data>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).data) };
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            crate::__pgrx_c_bindings::pg_locale_struct__bindgen_ty_1__bindgen_ty_2,
        >() == 16
    );
    assert!(
        ::core::mem::align_of::<
            crate::__pgrx_c_bindings::pg_locale_struct__bindgen_ty_1__bindgen_ty_2,
        >() == 8
    );
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::pg_locale_struct__bindgen_ty_1__bindgen_ty_2
{
}
#[cfg(feature = "pg17")]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            crate::__pgrx_c_bindings::pg_locale_struct__bindgen_ty_1__bindgen_ty_1,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            crate::__pgrx_c_bindings::pg_locale_struct__bindgen_ty_1__bindgen_ty_1,
        >() == 8
    );
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for crate::__pgrx_c_bindings::TidStoreIter {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeType for crate::__pgrx_c_bindings::TidStoreIter {
    type Marker = c::expression::COpaque<Self>;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for crate::__pgrx_c_bindings::TidStore {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeType for crate::__pgrx_c_bindings::TidStore {
    type Marker = c::expression::COpaque<Self>;
}
#[cfg(feature = "pg17")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ReplicationSlotCtlData>() == 280);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ReplicationSlotCtlData>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for crate::__pgrx_c_bindings::ReadStream {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeType for crate::__pgrx_c_bindings::ReadStream {
    type Marker = c::expression::COpaque<Self>;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ParallelWorkerInfo>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ParallelWorkerInfo>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JsonTableSiblingJoin>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JsonTableSiblingJoin>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JsonTableSiblingJoin {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::FormExtraData_pg_attribute>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::FormExtraData_pg_attribute>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::FormExtraData_pg_attribute {}
#[cfg(feature = "pg17")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ForeignPath>() == 96);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ForeignPath>() == 8);
};
#[cfg(feature = "pg17")]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_45>(
        ) == 8
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_45>(
        ) == 8
    );
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_45
{
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for crate::__pgrx_c_bindings::ClientSocket {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeType for crate::__pgrx_c_bindings::ClientSocket {
    type Marker = c::expression::COpaque<Self>;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for crate::__pgrx_c_bindings::BulkWriteState {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeType for crate::__pgrx_c_bindings::BulkWriteState {
    type Marker = c::expression::COpaque<Self>;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__FullTransactionIdFromAllowableAt"]
    fn raw_inline_FullTransactionIdFromAllowableAt(
        arg0: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>,
        arg1: crate::__pgrx_c_bindings::TransactionId,
    ) -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc = "Call the original C function `FullTransactionIdFromAllowableAt`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_FullTransactionIdFromAllowableAt(
    arg0: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>,
    arg1: crate::__pgrx_c_bindings::TransactionId,
) -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId> {
    const _: () = assert!(!::core::mem::needs_drop::<
        ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>,
    >());
    const _: () = assert!(!::core::mem::needs_drop::<crate::__pgrx_c_bindings::TransactionId>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe {
        crate::ffi::pg_guard_ffi_boundary(move || {
            raw_inline_FullTransactionIdFromAllowableAt(arg0, arg1)
        })
    }
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TransactionId>() == 4);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TransactionId>() == 4);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>>()
        == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>>(
    ) == 8
);
#[cfg(feature = "pg17")]
const _: () = assert!(::core::mem::size_of::<*mut ::core::ffi::c_char>() == 8);
#[cfg(feature = "pg17")]
const _: () = assert!(::core::mem::align_of::<*mut ::core::ffi::c_char>() == 8);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>>()
        == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>>(
    ) == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::size_of::<*const ::core::ffi::c_char>() == 8);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::align_of::<*const ::core::ffi::c_char>() == 8);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::size_of::<usize>() == 8);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::align_of::<usize>() == 8);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::Node>() == 8);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::Node>() == 8);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__newNode"]
    fn raw_inline_newNode(arg0: usize, arg1: u32) -> *mut crate::__pgrx_c_bindings::Node;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc = "Call the original C function `newNode`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_newNode(arg0: usize, arg1: u32) -> *mut crate::__pgrx_c_bindings::Node {
    const _: () = assert!(!::core::mem::needs_drop::<usize>());
    const _: () = assert!(!::core::mem::needs_drop::<u32>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_newNode(arg0, arg1)) }
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::SMgrRelationData>() == 8);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::SMgrRelationData>() == 8);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::size_of::<*mut ::core::ffi::c_void>() == 8);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::align_of::<*mut ::core::ffi::c_void>() == 8);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__smgrread"]
    fn raw_inline_smgrread(
        arg0: *mut crate::__pgrx_c_bindings::SMgrRelationData,
        arg1: i32,
        arg2: u32,
        arg3: *mut ::core::ffi::c_void,
    ) -> ();
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc = "Call the original C function `smgrread`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_smgrread(
    arg0: *mut crate::__pgrx_c_bindings::SMgrRelationData,
    arg1: i32,
    arg2: u32,
    arg3: *mut ::core::ffi::c_void,
) -> () {
    const _: () =
        assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::SMgrRelationData>());
    const _: () = assert!(!::core::mem::needs_drop::<i32>());
    const _: () = assert!(!::core::mem::needs_drop::<u32>());
    const _: () = assert!(!::core::mem::needs_drop::<*mut ::core::ffi::c_void>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe {
        crate::ffi::pg_guard_ffi_boundary(move || raw_inline_smgrread(arg0, arg1, arg2, arg3))
    }
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::SMgrRelationData>() == 8);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::SMgrRelationData>() == 8);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::size_of::<*const ::core::ffi::c_void>() == 8);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::align_of::<*const ::core::ffi::c_void>() == 8);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__smgrwrite"]
    fn raw_inline_smgrwrite(
        arg0: *mut crate::__pgrx_c_bindings::SMgrRelationData,
        arg1: i32,
        arg2: u32,
        arg3: *const ::core::ffi::c_void,
        arg4: bool,
    ) -> ();
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc = "Call the original C function `smgrwrite`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_smgrwrite(
    arg0: *mut crate::__pgrx_c_bindings::SMgrRelationData,
    arg1: i32,
    arg2: u32,
    arg3: *const ::core::ffi::c_void,
    arg4: bool,
) -> () {
    const _: () =
        assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::SMgrRelationData>());
    const _: () = assert!(!::core::mem::needs_drop::<i32>());
    const _: () = assert!(!::core::mem::needs_drop::<u32>());
    const _: () = assert!(!::core::mem::needs_drop::<*const ::core::ffi::c_void>());
    const _: () = assert!(!::core::mem::needs_drop::<bool>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe {
        crate::ffi::pg_guard_ffi_boundary(move || {
            raw_inline_smgrwrite(arg0, arg1, arg2, arg3, arg4)
        })
    }
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::size_of::<i32>() == 4);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::align_of::<i32>() == 4);
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::size_of::<u64>() == 8);
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::align_of::<u64>() == 8);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::size_of::<i32>() == 4);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::align_of::<i32>() == 4);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::size_of::<i32>() == 4);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::align_of::<i32>() == 4);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::size_of::<bool>() == 1);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::align_of::<bool>() == 1);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::size_of::<bool>() == 1);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::align_of::<bool>() == 1);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::size_of::<bool>() == 1);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::align_of::<bool>() == 1);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::size_of::<u32>() == 4);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::align_of::<u32>() == 4);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::size_of::<u32>() == 4);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::align_of::<u32>() == 4);
#[cfg(any(feature = "pg17", feature = "pg19"))]
const _: () = assert!(::core::mem::size_of::<bool>() == 1);
#[cfg(any(feature = "pg17", feature = "pg19"))]
const _: () = assert!(::core::mem::align_of::<bool>() == 1);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::size_of::<u32>() == 4);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::align_of::<u32>() == 4);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_ResourceReleasePhase;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for EnumIdentity_ResourceReleasePhase {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::EnumIdentity for EnumIdentity_ResourceReleasePhase {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::enumeration::NumericEnum for EnumIdentity_ResourceReleasePhase {
    type Compatible = c::CUnsignedInt;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__CAC_state;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for EnumIdentity_enum__CAC_state {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::EnumIdentity for EnumIdentity_enum__CAC_state {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__CAC_state {
    type Compatible = c::CUnsignedInt;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__CopyLogVerbosityChoice;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for EnumIdentity_enum__CopyLogVerbosityChoice {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::EnumIdentity for EnumIdentity_enum__CopyLogVerbosityChoice {}
#[cfg(feature = "pg17")]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__CopyLogVerbosityChoice {
    type Compatible = c::CUnsignedInt;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__CopyOnErrorChoice;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for EnumIdentity_enum__CopyOnErrorChoice {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::EnumIdentity for EnumIdentity_enum__CopyOnErrorChoice {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__CopyOnErrorChoice {
    type Compatible = c::CUnsignedInt;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__ExplainSerializeOption;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for EnumIdentity_enum__ExplainSerializeOption {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::EnumIdentity for EnumIdentity_enum__ExplainSerializeOption {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__ExplainSerializeOption {
    type Compatible = c::CUnsignedInt;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__JsonBehaviorType;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for EnumIdentity_enum__JsonBehaviorType {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::EnumIdentity for EnumIdentity_enum__JsonBehaviorType {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__JsonBehaviorType {
    type Compatible = c::CUnsignedInt;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__JsonExprOp;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for EnumIdentity_enum__JsonExprOp {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::EnumIdentity for EnumIdentity_enum__JsonExprOp {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__JsonExprOp {
    type Compatible = c::CUnsignedInt;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__JsonQuotes;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for EnumIdentity_enum__JsonQuotes {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::EnumIdentity for EnumIdentity_enum__JsonQuotes {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__JsonQuotes {
    type Compatible = c::CUnsignedInt;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__JsonTableColumnType;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for EnumIdentity_enum__JsonTableColumnType {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::EnumIdentity for EnumIdentity_enum__JsonTableColumnType {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__JsonTableColumnType {
    type Compatible = c::CUnsignedInt;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__JsonWrapper;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for EnumIdentity_enum__JsonWrapper {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::EnumIdentity for EnumIdentity_enum__JsonWrapper {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__JsonWrapper {
    type Compatible = c::CUnsignedInt;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__LogicalRepWorkerType;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for EnumIdentity_enum__LogicalRepWorkerType {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::EnumIdentity for EnumIdentity_enum__LogicalRepWorkerType {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__LogicalRepWorkerType {
    type Compatible = c::CUnsignedInt;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__MergeMatchKind;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for EnumIdentity_enum__MergeMatchKind {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::EnumIdentity for EnumIdentity_enum__MergeMatchKind {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__MergeMatchKind {
    type Compatible = c::CUnsignedInt;
}
#[cfg(feature = "pg17")]
// SAFETY: The explicit Rust integer repr matches Clang's compatible enum type and the assertions prove layout. Decoding receives a valid unit Rust variant; encoding checks every discriminant.
unsafe impl c::expression::EnumStorage<EnumIdentity_enum__NodeTag, c::CUnsignedInt>
    for crate::__pgrx_c_bindings::NodeTag
{
    fn decode(value: Self) -> u32 {
        value as u32
    }
    fn encode(value: u32) -> Self {
        match value {
            79 => crate::__pgrx_c_bindings::NodeTag::T_A_ArrayExpr,
            71 => crate::__pgrx_c_bindings::NodeTag::T_A_Const,
            70 => crate::__pgrx_c_bindings::NodeTag::T_A_Expr,
            77 => crate::__pgrx_c_bindings::NodeTag::T_A_Indices,
            78 => crate::__pgrx_c_bindings::NodeTag::T_A_Indirection,
            76 => crate::__pgrx_c_bindings::NodeTag::T_A_Star,
            151 => crate::__pgrx_c_bindings::NodeTag::T_AccessPriv,
            361 => crate::__pgrx_c_bindings::NodeTag::T_Agg,
            324 => crate::__pgrx_c_bindings::NodeTag::T_AggInfo,
            304 => crate::__pgrx_c_bindings::NodeTag::T_AggPath,
            425 => crate::__pgrx_c_bindings::NodeTag::T_AggState,
            325 => crate::__pgrx_c_bindings::NodeTag::T_AggTransInfo,
            9 => crate::__pgrx_c_bindings::NodeTag::T_Aggref,
            2 => crate::__pgrx_c_bindings::NodeTag::T_Alias,
            469 => crate::__pgrx_c_bindings::NodeTag::T_AllocSetContext,
            147 => crate::__pgrx_c_bindings::NodeTag::T_AlterCollationStmt,
            231 => crate::__pgrx_c_bindings::NodeTag::T_AlterDatabaseRefreshCollStmt,
            232 => crate::__pgrx_c_bindings::NodeTag::T_AlterDatabaseSetStmt,
            230 => crate::__pgrx_c_bindings::NodeTag::T_AlterDatabaseStmt,
            153 => crate::__pgrx_c_bindings::NodeTag::T_AlterDefaultPrivilegesStmt,
            148 => crate::__pgrx_c_bindings::NodeTag::T_AlterDomainStmt,
            226 => crate::__pgrx_c_bindings::NodeTag::T_AlterEnumStmt,
            180 => crate::__pgrx_c_bindings::NodeTag::T_AlterEventTrigStmt,
            165 => crate::__pgrx_c_bindings::NodeTag::T_AlterExtensionContentsStmt,
            164 => crate::__pgrx_c_bindings::NodeTag::T_AlterExtensionStmt,
            167 => crate::__pgrx_c_bindings::NodeTag::T_AlterFdwStmt,
            169 => crate::__pgrx_c_bindings::NodeTag::T_AlterForeignServerStmt,
            207 => crate::__pgrx_c_bindings::NodeTag::T_AlterFunctionStmt,
            213 => crate::__pgrx_c_bindings::NodeTag::T_AlterObjectDependsStmt,
            214 => crate::__pgrx_c_bindings::NodeTag::T_AlterObjectSchemaStmt,
            193 => crate::__pgrx_c_bindings::NodeTag::T_AlterOpFamilyStmt,
            216 => crate::__pgrx_c_bindings::NodeTag::T_AlterOperatorStmt,
            215 => crate::__pgrx_c_bindings::NodeTag::T_AlterOwnerStmt,
            176 => crate::__pgrx_c_bindings::NodeTag::T_AlterPolicyStmt,
            259 => crate::__pgrx_c_bindings::NodeTag::T_AlterPublicationStmt,
            448 => crate::__pgrx_c_bindings::NodeTag::T_AlterReplicationSlotCmd,
            184 => crate::__pgrx_c_bindings::NodeTag::T_AlterRoleSetStmt,
            183 => crate::__pgrx_c_bindings::NodeTag::T_AlterRoleStmt,
            187 => crate::__pgrx_c_bindings::NodeTag::T_AlterSeqStmt,
            204 => crate::__pgrx_c_bindings::NodeTag::T_AlterStatsStmt,
            261 => crate::__pgrx_c_bindings::NodeTag::T_AlterSubscriptionStmt,
            234 => crate::__pgrx_c_bindings::NodeTag::T_AlterSystemStmt,
            255 => crate::__pgrx_c_bindings::NodeTag::T_AlterTSConfigurationStmt,
            254 => crate::__pgrx_c_bindings::NodeTag::T_AlterTSDictionaryStmt,
            146 => crate::__pgrx_c_bindings::NodeTag::T_AlterTableCmd,
            162 => crate::__pgrx_c_bindings::NodeTag::T_AlterTableMoveAllStmt,
            161 => crate::__pgrx_c_bindings::NodeTag::T_AlterTableSpaceOptionsStmt,
            144 => crate::__pgrx_c_bindings::NodeTag::T_AlterTableStmt,
            217 => crate::__pgrx_c_bindings::NodeTag::T_AlterTypeStmt,
            172 => crate::__pgrx_c_bindings::NodeTag::T_AlterUserMappingStmt,
            24 => crate::__pgrx_c_bindings::NodeTag::T_AlternativeSubPlan,
            330 => crate::__pgrx_c_bindings::NodeTag::T_Append,
            287 => crate::__pgrx_c_bindings::NodeTag::T_AppendPath,
            319 => crate::__pgrx_c_bindings::NodeTag::T_AppendRelInfo,
            393 => crate::__pgrx_c_bindings::NodeTag::T_AppendState,
            29 => crate::__pgrx_c_bindings::NodeTag::T_ArrayCoerceExpr,
            35 => crate::__pgrx_c_bindings::NodeTag::T_ArrayExpr,
            445 => crate::__pgrx_c_bindings::NodeTag::T_BaseBackupCmd,
            464 => crate::__pgrx_c_bindings::NodeTag::T_BitString,
            333 => crate::__pgrx_c_bindings::NodeTag::T_BitmapAnd,
            280 => crate::__pgrx_c_bindings::NodeTag::T_BitmapAndPath,
            396 => crate::__pgrx_c_bindings::NodeTag::T_BitmapAndState,
            279 => crate::__pgrx_c_bindings::NodeTag::T_BitmapHeapPath,
            340 => crate::__pgrx_c_bindings::NodeTag::T_BitmapHeapScan,
            404 => crate::__pgrx_c_bindings::NodeTag::T_BitmapHeapScanState,
            339 => crate::__pgrx_c_bindings::NodeTag::T_BitmapIndexScan,
            403 => crate::__pgrx_c_bindings::NodeTag::T_BitmapIndexScanState,
            334 => crate::__pgrx_c_bindings::NodeTag::T_BitmapOr,
            281 => crate::__pgrx_c_bindings::NodeTag::T_BitmapOrPath,
            397 => crate::__pgrx_c_bindings::NodeTag::T_BitmapOrState,
            441 => crate::__pgrx_c_bindings::NodeTag::T_Bitmapset,
            21 => crate::__pgrx_c_bindings::NodeTag::T_BoolExpr,
            462 => crate::__pgrx_c_bindings::NodeTag::T_Boolean,
            53 => crate::__pgrx_c_bindings::NodeTag::T_BooleanTest,
            472 => crate::__pgrx_c_bindings::NodeTag::T_BumpContext,
            114 => crate::__pgrx_c_bindings::NodeTag::T_CTECycleClause,
            113 => crate::__pgrx_c_bindings::NodeTag::T_CTESearchClause,
            211 => crate::__pgrx_c_bindings::NodeTag::T_CallContext,
            210 => crate::__pgrx_c_bindings::NodeTag::T_CallStmt,
            32 => crate::__pgrx_c_bindings::NodeTag::T_CaseExpr,
            34 => crate::__pgrx_c_bindings::NodeTag::T_CaseTestExpr,
            33 => crate::__pgrx_c_bindings::NodeTag::T_CaseWhen,
            241 => crate::__pgrx_c_bindings::NodeTag::T_CheckPointStmt,
            199 => crate::__pgrx_c_bindings::NodeTag::T_ClosePortalStmt,
            235 => crate::__pgrx_c_bindings::NodeTag::T_ClusterStmt,
            38 => crate::__pgrx_c_bindings::NodeTag::T_CoalesceExpr,
            55 => crate::__pgrx_c_bindings::NodeTag::T_CoerceToDomain,
            56 => crate::__pgrx_c_bindings::NodeTag::T_CoerceToDomainValue,
            28 => crate::__pgrx_c_bindings::NodeTag::T_CoerceViaIO,
            73 => crate::__pgrx_c_bindings::NodeTag::T_CollateClause,
            31 => crate::__pgrx_c_bindings::NodeTag::T_CollateExpr,
            89 => crate::__pgrx_c_bindings::NodeTag::T_ColumnDef,
            68 => crate::__pgrx_c_bindings::NodeTag::T_ColumnRef,
            196 => crate::__pgrx_c_bindings::NodeTag::T_CommentStmt,
            115 => crate::__pgrx_c_bindings::NodeTag::T_CommonTableExpr,
            223 => crate::__pgrx_c_bindings::NodeTag::T_CompositeTypeStmt,
            7 => crate::__pgrx_c_bindings::NodeTag::T_Const,
            158 => crate::__pgrx_c_bindings::NodeTag::T_Constraint,
            244 => crate::__pgrx_c_bindings::NodeTag::T_ConstraintsSetStmt,
            30 => crate::__pgrx_c_bindings::NodeTag::T_ConvertRowtypeExpr,
            154 => crate::__pgrx_c_bindings::NodeTag::T_CopyStmt,
            177 => crate::__pgrx_c_bindings::NodeTag::T_CreateAmStmt,
            247 => crate::__pgrx_c_bindings::NodeTag::T_CreateCastStmt,
            246 => crate::__pgrx_c_bindings::NodeTag::T_CreateConversionStmt,
            189 => crate::__pgrx_c_bindings::NodeTag::T_CreateDomainStmt,
            224 => crate::__pgrx_c_bindings::NodeTag::T_CreateEnumStmt,
            179 => crate::__pgrx_c_bindings::NodeTag::T_CreateEventTrigStmt,
            163 => crate::__pgrx_c_bindings::NodeTag::T_CreateExtensionStmt,
            166 => crate::__pgrx_c_bindings::NodeTag::T_CreateFdwStmt,
            168 => crate::__pgrx_c_bindings::NodeTag::T_CreateForeignServerStmt,
            170 => crate::__pgrx_c_bindings::NodeTag::T_CreateForeignTableStmt,
            205 => crate::__pgrx_c_bindings::NodeTag::T_CreateFunctionStmt,
            191 => crate::__pgrx_c_bindings::NodeTag::T_CreateOpClassItem,
            190 => crate::__pgrx_c_bindings::NodeTag::T_CreateOpClassStmt,
            192 => crate::__pgrx_c_bindings::NodeTag::T_CreateOpFamilyStmt,
            181 => crate::__pgrx_c_bindings::NodeTag::T_CreatePLangStmt,
            175 => crate::__pgrx_c_bindings::NodeTag::T_CreatePolicyStmt,
            258 => crate::__pgrx_c_bindings::NodeTag::T_CreatePublicationStmt,
            225 => crate::__pgrx_c_bindings::NodeTag::T_CreateRangeStmt,
            446 => crate::__pgrx_c_bindings::NodeTag::T_CreateReplicationSlotCmd,
            182 => crate::__pgrx_c_bindings::NodeTag::T_CreateRoleStmt,
            143 => crate::__pgrx_c_bindings::NodeTag::T_CreateSchemaStmt,
            186 => crate::__pgrx_c_bindings::NodeTag::T_CreateSeqStmt,
            202 => crate::__pgrx_c_bindings::NodeTag::T_CreateStatsStmt,
            157 => crate::__pgrx_c_bindings::NodeTag::T_CreateStmt,
            260 => crate::__pgrx_c_bindings::NodeTag::T_CreateSubscriptionStmt,
            239 => crate::__pgrx_c_bindings::NodeTag::T_CreateTableAsStmt,
            159 => crate::__pgrx_c_bindings::NodeTag::T_CreateTableSpaceStmt,
            248 => crate::__pgrx_c_bindings::NodeTag::T_CreateTransformStmt,
            178 => crate::__pgrx_c_bindings::NodeTag::T_CreateTrigStmt,
            171 => crate::__pgrx_c_bindings::NodeTag::T_CreateUserMappingStmt,
            229 => crate::__pgrx_c_bindings::NodeTag::T_CreatedbStmt,
            347 => crate::__pgrx_c_bindings::NodeTag::T_CteScan,
            411 => crate::__pgrx_c_bindings::NodeTag::T_CteScanState,
            58 => crate::__pgrx_c_bindings::NodeTag::T_CurrentOfExpr,
            286 => crate::__pgrx_c_bindings::NodeTag::T_CustomPath,
            351 => crate::__pgrx_c_bindings::NodeTag::T_CustomScan,
            415 => crate::__pgrx_c_bindings::NodeTag::T_CustomScanState,
            251 => crate::__pgrx_c_bindings::NodeTag::T_DeallocateStmt,
            198 => crate::__pgrx_c_bindings::NodeTag::T_DeclareCursorStmt,
            92 => crate::__pgrx_c_bindings::NodeTag::T_DefElem,
            188 => crate::__pgrx_c_bindings::NodeTag::T_DefineStmt,
            136 => crate::__pgrx_c_bindings::NodeTag::T_DeleteStmt,
            242 => crate::__pgrx_c_bindings::NodeTag::T_DiscardStmt,
            18 => crate::__pgrx_c_bindings::NodeTag::T_DistinctExpr,
            208 => crate::__pgrx_c_bindings::NodeTag::T_DoStmt,
            389 => crate::__pgrx_c_bindings::NodeTag::T_DomainConstraintState,
            252 => crate::__pgrx_c_bindings::NodeTag::T_DropOwnedStmt,
            447 => crate::__pgrx_c_bindings::NodeTag::T_DropReplicationSlotCmd,
            185 => crate::__pgrx_c_bindings::NodeTag::T_DropRoleStmt,
            194 => crate::__pgrx_c_bindings::NodeTag::T_DropStmt,
            262 => crate::__pgrx_c_bindings::NodeTag::T_DropSubscriptionStmt,
            160 => crate::__pgrx_c_bindings::NodeTag::T_DropTableSpaceStmt,
            173 => crate::__pgrx_c_bindings::NodeTag::T_DropUserMappingStmt,
            233 => crate::__pgrx_c_bindings::NodeTag::T_DropdbStmt,
            385 => crate::__pgrx_c_bindings::NodeTag::T_EState,
            270 => crate::__pgrx_c_bindings::NodeTag::T_EquivalenceClass,
            271 => crate::__pgrx_c_bindings::NodeTag::T_EquivalenceMember,
            443 => crate::__pgrx_c_bindings::NodeTag::T_ErrorSaveContext,
            437 => crate::__pgrx_c_bindings::NodeTag::T_EventTriggerData,
            250 => crate::__pgrx_c_bindings::NodeTag::T_ExecuteStmt,
            238 => crate::__pgrx_c_bindings::NodeTag::T_ExplainStmt,
            378 => crate::__pgrx_c_bindings::NodeTag::T_ExprContext,
            376 => crate::__pgrx_c_bindings::NodeTag::T_ExprState,
            442 => crate::__pgrx_c_bindings::NodeTag::T_ExtensibleNode,
            440 => crate::__pgrx_c_bindings::NodeTag::T_FdwRoutine,
            200 => crate::__pgrx_c_bindings::NodeTag::T_FetchStmt,
            25 => crate::__pgrx_c_bindings::NodeTag::T_FieldSelect,
            26 => crate::__pgrx_c_bindings::NodeTag::T_FieldStore,
            461 => crate::__pgrx_c_bindings::NodeTag::T_Float,
            465 => crate::__pgrx_c_bindings::NodeTag::T_ForeignKeyCacheInfo,
            267 => crate::__pgrx_c_bindings::NodeTag::T_ForeignKeyOptInfo,
            285 => crate::__pgrx_c_bindings::NodeTag::T_ForeignPath,
            350 => crate::__pgrx_c_bindings::NodeTag::T_ForeignScan,
            414 => crate::__pgrx_c_bindings::NodeTag::T_ForeignScanState,
            64 => crate::__pgrx_c_bindings::NodeTag::T_FromExpr,
            75 => crate::__pgrx_c_bindings::NodeTag::T_FuncCall,
            15 => crate::__pgrx_c_bindings::NodeTag::T_FuncExpr,
            206 => crate::__pgrx_c_bindings::NodeTag::T_FunctionParameter,
            344 => crate::__pgrx_c_bindings::NodeTag::T_FunctionScan,
            408 => crate::__pgrx_c_bindings::NodeTag::T_FunctionScanState,
            364 => crate::__pgrx_c_bindings::NodeTag::T_Gather,
            365 => crate::__pgrx_c_bindings::NodeTag::T_GatherMerge,
            294 => crate::__pgrx_c_bindings::NodeTag::T_GatherMergePath,
            429 => crate::__pgrx_c_bindings::NodeTag::T_GatherMergeState,
            293 => crate::__pgrx_c_bindings::NodeTag::T_GatherPath,
            428 => crate::__pgrx_c_bindings::NodeTag::T_GatherState,
            470 => crate::__pgrx_c_bindings::NodeTag::T_GenerationContext,
            152 => crate::__pgrx_c_bindings::NodeTag::T_GrantRoleStmt,
            149 => crate::__pgrx_c_bindings::NodeTag::T_GrantStmt,
            360 => crate::__pgrx_c_bindings::NodeTag::T_Group,
            273 => crate::__pgrx_c_bindings::NodeTag::T_GroupByOrdering,
            302 => crate::__pgrx_c_bindings::NodeTag::T_GroupPath,
            289 => crate::__pgrx_c_bindings::NodeTag::T_GroupResultPath,
            424 => crate::__pgrx_c_bindings::NodeTag::T_GroupState,
            10 => crate::__pgrx_c_bindings::NodeTag::T_GroupingFunc,
            107 => crate::__pgrx_c_bindings::NodeTag::T_GroupingSet,
            305 => crate::__pgrx_c_bindings::NodeTag::T_GroupingSetData,
            307 => crate::__pgrx_c_bindings::NodeTag::T_GroupingSetsPath,
            366 => crate::__pgrx_c_bindings::NodeTag::T_Hash,
            355 => crate::__pgrx_c_bindings::NodeTag::T_HashJoin,
            419 => crate::__pgrx_c_bindings::NodeTag::T_HashJoinState,
            297 => crate::__pgrx_c_bindings::NodeTag::T_HashPath,
            430 => crate::__pgrx_c_bindings::NodeTag::T_HashState,
            444 => crate::__pgrx_c_bindings::NodeTag::T_IdentifySystemCmd,
            174 => crate::__pgrx_c_bindings::NodeTag::T_ImportForeignSchemaStmt,
            359 => crate::__pgrx_c_bindings::NodeTag::T_IncrementalSort,
            301 => crate::__pgrx_c_bindings::NodeTag::T_IncrementalSortPath,
            423 => crate::__pgrx_c_bindings::NodeTag::T_IncrementalSortState,
            434 => crate::__pgrx_c_bindings::NodeTag::T_IndexAmRoutine,
            278 => crate::__pgrx_c_bindings::NodeTag::T_IndexClause,
            91 => crate::__pgrx_c_bindings::NodeTag::T_IndexElem,
            377 => crate::__pgrx_c_bindings::NodeTag::T_IndexInfo,
            338 => crate::__pgrx_c_bindings::NodeTag::T_IndexOnlyScan,
            402 => crate::__pgrx_c_bindings::NodeTag::T_IndexOnlyScanState,
            266 => crate::__pgrx_c_bindings::NodeTag::T_IndexOptInfo,
            277 => crate::__pgrx_c_bindings::NodeTag::T_IndexPath,
            337 => crate::__pgrx_c_bindings::NodeTag::T_IndexScan,
            401 => crate::__pgrx_c_bindings::NodeTag::T_IndexScanState,
            201 => crate::__pgrx_c_bindings::NodeTag::T_IndexStmt,
            111 => crate::__pgrx_c_bindings::NodeTag::T_InferClause,
            60 => crate::__pgrx_c_bindings::NodeTag::T_InferenceElem,
            209 => crate::__pgrx_c_bindings::NodeTag::T_InlineCodeBlock,
            135 => crate::__pgrx_c_bindings::NodeTag::T_InsertStmt,
            466 => crate::__pgrx_c_bindings::NodeTag::T_IntList,
            460 => crate::__pgrx_c_bindings::NodeTag::T_Integer,
            5 => crate::__pgrx_c_bindings::NodeTag::T_IntoClause,
            0 => crate::__pgrx_c_bindings::NodeTag::T_Invalid,
            269 => crate::__pgrx_c_bindings::NodeTag::T_JoinDomain,
            63 => crate::__pgrx_c_bindings::NodeTag::T_JoinExpr,
            416 => crate::__pgrx_c_bindings::NodeTag::T_JoinState,
            131 => crate::__pgrx_c_bindings::NodeTag::T_JsonAggConstructor,
            119 => crate::__pgrx_c_bindings::NodeTag::T_JsonArgument,
            133 => crate::__pgrx_c_bindings::NodeTag::T_JsonArrayAgg,
            129 => crate::__pgrx_c_bindings::NodeTag::T_JsonArrayConstructor,
            130 => crate::__pgrx_c_bindings::NodeTag::T_JsonArrayQueryConstructor,
            47 => crate::__pgrx_c_bindings::NodeTag::T_JsonBehavior,
            45 => crate::__pgrx_c_bindings::NodeTag::T_JsonConstructorExpr,
            48 => crate::__pgrx_c_bindings::NodeTag::T_JsonExpr,
            42 => crate::__pgrx_c_bindings::NodeTag::T_JsonFormat,
            120 => crate::__pgrx_c_bindings::NodeTag::T_JsonFuncExpr,
            46 => crate::__pgrx_c_bindings::NodeTag::T_JsonIsPredicate,
            124 => crate::__pgrx_c_bindings::NodeTag::T_JsonKeyValue,
            132 => crate::__pgrx_c_bindings::NodeTag::T_JsonObjectAgg,
            128 => crate::__pgrx_c_bindings::NodeTag::T_JsonObjectConstructor,
            118 => crate::__pgrx_c_bindings::NodeTag::T_JsonOutput,
            125 => crate::__pgrx_c_bindings::NodeTag::T_JsonParseExpr,
            43 => crate::__pgrx_c_bindings::NodeTag::T_JsonReturning,
            126 => crate::__pgrx_c_bindings::NodeTag::T_JsonScalarExpr,
            127 => crate::__pgrx_c_bindings::NodeTag::T_JsonSerializeExpr,
            122 => crate::__pgrx_c_bindings::NodeTag::T_JsonTable,
            123 => crate::__pgrx_c_bindings::NodeTag::T_JsonTableColumn,
            49 => crate::__pgrx_c_bindings::NodeTag::T_JsonTablePath,
            50 => crate::__pgrx_c_bindings::NodeTag::T_JsonTablePathScan,
            121 => crate::__pgrx_c_bindings::NodeTag::T_JsonTablePathSpec,
            51 => crate::__pgrx_c_bindings::NodeTag::T_JsonTableSiblingJoin,
            44 => crate::__pgrx_c_bindings::NodeTag::T_JsonValueExpr,
            381 => crate::__pgrx_c_bindings::NodeTag::T_JunkFilter,
            369 => crate::__pgrx_c_bindings::NodeTag::T_Limit,
            314 => crate::__pgrx_c_bindings::NodeTag::T_LimitPath,
            433 => crate::__pgrx_c_bindings::NodeTag::T_LimitState,
            1 => crate::__pgrx_c_bindings::NodeTag::T_List,
            220 => crate::__pgrx_c_bindings::NodeTag::T_ListenStmt,
            228 => crate::__pgrx_c_bindings::NodeTag::T_LoadStmt,
            368 => crate::__pgrx_c_bindings::NodeTag::T_LockRows,
            312 => crate::__pgrx_c_bindings::NodeTag::T_LockRowsPath,
            432 => crate::__pgrx_c_bindings::NodeTag::T_LockRowsState,
            243 => crate::__pgrx_c_bindings::NodeTag::T_LockStmt,
            93 => crate::__pgrx_c_bindings::NodeTag::T_LockingClause,
            356 => crate::__pgrx_c_bindings::NodeTag::T_Material,
            290 => crate::__pgrx_c_bindings::NodeTag::T_MaterialPath,
            420 => crate::__pgrx_c_bindings::NodeTag::T_MaterialState,
            357 => crate::__pgrx_c_bindings::NodeTag::T_Memoize,
            291 => crate::__pgrx_c_bindings::NodeTag::T_MemoizePath,
            421 => crate::__pgrx_c_bindings::NodeTag::T_MemoizeState,
            54 => crate::__pgrx_c_bindings::NodeTag::T_MergeAction,
            383 => crate::__pgrx_c_bindings::NodeTag::T_MergeActionState,
            331 => crate::__pgrx_c_bindings::NodeTag::T_MergeAppend,
            288 => crate::__pgrx_c_bindings::NodeTag::T_MergeAppendPath,
            394 => crate::__pgrx_c_bindings::NodeTag::T_MergeAppendState,
            354 => crate::__pgrx_c_bindings::NodeTag::T_MergeJoin,
            418 => crate::__pgrx_c_bindings::NodeTag::T_MergeJoinState,
            296 => crate::__pgrx_c_bindings::NodeTag::T_MergePath,
            138 => crate::__pgrx_c_bindings::NodeTag::T_MergeStmt,
            13 => crate::__pgrx_c_bindings::NodeTag::T_MergeSupportFunc,
            116 => crate::__pgrx_c_bindings::NodeTag::T_MergeWhenClause,
            322 => crate::__pgrx_c_bindings::NodeTag::T_MinMaxAggInfo,
            308 => crate::__pgrx_c_bindings::NodeTag::T_MinMaxAggPath,
            39 => crate::__pgrx_c_bindings::NodeTag::T_MinMaxExpr,
            329 => crate::__pgrx_c_bindings::NodeTag::T_ModifyTable,
            313 => crate::__pgrx_c_bindings::NodeTag::T_ModifyTablePath,
            392 => crate::__pgrx_c_bindings::NodeTag::T_ModifyTableState,
            81 => crate::__pgrx_c_bindings::NodeTag::T_MultiAssignRef,
            16 => crate::__pgrx_c_bindings::NodeTag::T_NamedArgExpr,
            348 => crate::__pgrx_c_bindings::NodeTag::T_NamedTuplestoreScan,
            412 => crate::__pgrx_c_bindings::NodeTag::T_NamedTuplestoreScanState,
            352 => crate::__pgrx_c_bindings::NodeTag::T_NestLoop,
            353 => crate::__pgrx_c_bindings::NodeTag::T_NestLoopParam,
            417 => crate::__pgrx_c_bindings::NodeTag::T_NestLoopState,
            295 => crate::__pgrx_c_bindings::NodeTag::T_NestPath,
            59 => crate::__pgrx_c_bindings::NodeTag::T_NextValueExpr,
            219 => crate::__pgrx_c_bindings::NodeTag::T_NotifyStmt,
            19 => crate::__pgrx_c_bindings::NodeTag::T_NullIfExpr,
            52 => crate::__pgrx_c_bindings::NodeTag::T_NullTest,
            150 => crate::__pgrx_c_bindings::NodeTag::T_ObjectWithArgs,
            467 => crate::__pgrx_c_bindings::NodeTag::T_OidList,
            112 => crate::__pgrx_c_bindings::NodeTag::T_OnConflictClause,
            65 => crate::__pgrx_c_bindings::NodeTag::T_OnConflictExpr,
            382 => crate::__pgrx_c_bindings::NodeTag::T_OnConflictSetState,
            17 => crate::__pgrx_c_bindings::NodeTag::T_OpExpr,
            318 => crate::__pgrx_c_bindings::NodeTag::T_OuterJoinClauseInfo,
            142 => crate::__pgrx_c_bindings::NodeTag::T_PLAssignStmt,
            8 => crate::__pgrx_c_bindings::NodeTag::T_Param,
            275 => crate::__pgrx_c_bindings::NodeTag::T_ParamPathInfo,
            69 => crate::__pgrx_c_bindings::NodeTag::T_ParamRef,
            97 => crate::__pgrx_c_bindings::NodeTag::T_PartitionBoundSpec,
            100 => crate::__pgrx_c_bindings::NodeTag::T_PartitionCmd,
            95 => crate::__pgrx_c_bindings::NodeTag::T_PartitionElem,
            371 => crate::__pgrx_c_bindings::NodeTag::T_PartitionPruneInfo,
            374 => crate::__pgrx_c_bindings::NodeTag::T_PartitionPruneStepCombine,
            373 => crate::__pgrx_c_bindings::NodeTag::T_PartitionPruneStepOp,
            98 => crate::__pgrx_c_bindings::NodeTag::T_PartitionRangeDatum,
            96 => crate::__pgrx_c_bindings::NodeTag::T_PartitionSpec,
            372 => crate::__pgrx_c_bindings::NodeTag::T_PartitionedRelPruneInfo,
            276 => crate::__pgrx_c_bindings::NodeTag::T_Path,
            272 => crate::__pgrx_c_bindings::NodeTag::T_PathKey,
            274 => crate::__pgrx_c_bindings::NodeTag::T_PathTarget,
            321 => crate::__pgrx_c_bindings::NodeTag::T_PlaceHolderInfo,
            316 => crate::__pgrx_c_bindings::NodeTag::T_PlaceHolderVar,
            375 => crate::__pgrx_c_bindings::NodeTag::T_PlanInvalItem,
            370 => crate::__pgrx_c_bindings::NodeTag::T_PlanRowMark,
            326 => crate::__pgrx_c_bindings::NodeTag::T_PlannedStmt,
            263 => crate::__pgrx_c_bindings::NodeTag::T_PlannerGlobal,
            264 => crate::__pgrx_c_bindings::NodeTag::T_PlannerInfo,
            323 => crate::__pgrx_c_bindings::NodeTag::T_PlannerParamItem,
            249 => crate::__pgrx_c_bindings::NodeTag::T_PrepareStmt,
            328 => crate::__pgrx_c_bindings::NodeTag::T_ProjectSet,
            299 => crate::__pgrx_c_bindings::NodeTag::T_ProjectSetPath,
            391 => crate::__pgrx_c_bindings::NodeTag::T_ProjectSetState,
            380 => crate::__pgrx_c_bindings::NodeTag::T_ProjectionInfo,
            298 => crate::__pgrx_c_bindings::NodeTag::T_ProjectionPath,
            257 => crate::__pgrx_c_bindings::NodeTag::T_PublicationObjSpec,
            256 => crate::__pgrx_c_bindings::NodeTag::T_PublicationTable,
            66 => crate::__pgrx_c_bindings::NodeTag::T_Query,
            102 => crate::__pgrx_c_bindings::NodeTag::T_RTEPermissionInfo,
            85 => crate::__pgrx_c_bindings::NodeTag::T_RangeFunction,
            84 => crate::__pgrx_c_bindings::NodeTag::T_RangeSubselect,
            86 => crate::__pgrx_c_bindings::NodeTag::T_RangeTableFunc,
            87 => crate::__pgrx_c_bindings::NodeTag::T_RangeTableFuncCol,
            88 => crate::__pgrx_c_bindings::NodeTag::T_RangeTableSample,
            101 => crate::__pgrx_c_bindings::NodeTag::T_RangeTblEntry,
            103 => crate::__pgrx_c_bindings::NodeTag::T_RangeTblFunction,
            62 => crate::__pgrx_c_bindings::NodeTag::T_RangeTblRef,
            3 => crate::__pgrx_c_bindings::NodeTag::T_RangeVar,
            134 => crate::__pgrx_c_bindings::NodeTag::T_RawStmt,
            450 => crate::__pgrx_c_bindings::NodeTag::T_ReadReplicationSlotCmd,
            253 => crate::__pgrx_c_bindings::NodeTag::T_ReassignOwnedStmt,
            332 => crate::__pgrx_c_bindings::NodeTag::T_RecursiveUnion,
            311 => crate::__pgrx_c_bindings::NodeTag::T_RecursiveUnionPath,
            395 => crate::__pgrx_c_bindings::NodeTag::T_RecursiveUnionState,
            240 => crate::__pgrx_c_bindings::NodeTag::T_RefreshMatViewStmt,
            245 => crate::__pgrx_c_bindings::NodeTag::T_ReindexStmt,
            265 => crate::__pgrx_c_bindings::NodeTag::T_RelOptInfo,
            27 => crate::__pgrx_c_bindings::NodeTag::T_RelabelType,
            212 => crate::__pgrx_c_bindings::NodeTag::T_RenameStmt,
            145 => crate::__pgrx_c_bindings::NodeTag::T_ReplicaIdentityStmt,
            80 => crate::__pgrx_c_bindings::NodeTag::T_ResTarget,
            315 => crate::__pgrx_c_bindings::NodeTag::T_RestrictInfo,
            327 => crate::__pgrx_c_bindings::NodeTag::T_Result,
            384 => crate::__pgrx_c_bindings::NodeTag::T_ResultRelInfo,
            390 => crate::__pgrx_c_bindings::NodeTag::T_ResultState,
            379 => crate::__pgrx_c_bindings::NodeTag::T_ReturnSetInfo,
            141 => crate::__pgrx_c_bindings::NodeTag::T_ReturnStmt,
            74 => crate::__pgrx_c_bindings::NodeTag::T_RoleSpec,
            306 => crate::__pgrx_c_bindings::NodeTag::T_RollupData,
            37 => crate::__pgrx_c_bindings::NodeTag::T_RowCompareExpr,
            36 => crate::__pgrx_c_bindings::NodeTag::T_RowExpr,
            320 => crate::__pgrx_c_bindings::NodeTag::T_RowIdentityVarInfo,
            109 => crate::__pgrx_c_bindings::NodeTag::T_RowMarkClause,
            218 => crate::__pgrx_c_bindings::NodeTag::T_RuleStmt,
            40 => crate::__pgrx_c_bindings::NodeTag::T_SQLValueFunction,
            336 => crate::__pgrx_c_bindings::NodeTag::T_SampleScan,
            400 => crate::__pgrx_c_bindings::NodeTag::T_SampleScanState,
            20 => crate::__pgrx_c_bindings::NodeTag::T_ScalarArrayOpExpr,
            398 => crate::__pgrx_c_bindings::NodeTag::T_ScanState,
            197 => crate::__pgrx_c_bindings::NodeTag::T_SecLabelStmt,
            139 => crate::__pgrx_c_bindings::NodeTag::T_SelectStmt,
            335 => crate::__pgrx_c_bindings::NodeTag::T_SeqScan,
            399 => crate::__pgrx_c_bindings::NodeTag::T_SeqScanState,
            387 => crate::__pgrx_c_bindings::NodeTag::T_SetExprState,
            367 => crate::__pgrx_c_bindings::NodeTag::T_SetOp,
            310 => crate::__pgrx_c_bindings::NodeTag::T_SetOpPath,
            431 => crate::__pgrx_c_bindings::NodeTag::T_SetOpState,
            140 => crate::__pgrx_c_bindings::NodeTag::T_SetOperationStmt,
            57 => crate::__pgrx_c_bindings::NodeTag::T_SetToDefault,
            99 => crate::__pgrx_c_bindings::NodeTag::T_SinglePartitionSpec,
            471 => crate::__pgrx_c_bindings::NodeTag::T_SlabContext,
            358 => crate::__pgrx_c_bindings::NodeTag::T_Sort,
            82 => crate::__pgrx_c_bindings::NodeTag::T_SortBy,
            106 => crate::__pgrx_c_bindings::NodeTag::T_SortGroupClause,
            300 => crate::__pgrx_c_bindings::NodeTag::T_SortPath,
            422 => crate::__pgrx_c_bindings::NodeTag::T_SortState,
            317 => crate::__pgrx_c_bindings::NodeTag::T_SpecialJoinInfo,
            449 => crate::__pgrx_c_bindings::NodeTag::T_StartReplicationCmd,
            268 => crate::__pgrx_c_bindings::NodeTag::T_StatisticExtInfo,
            203 => crate::__pgrx_c_bindings::NodeTag::T_StatsElem,
            463 => crate::__pgrx_c_bindings::NodeTag::T_String,
            22 => crate::__pgrx_c_bindings::NodeTag::T_SubLink,
            23 => crate::__pgrx_c_bindings::NodeTag::T_SubPlan,
            388 => crate::__pgrx_c_bindings::NodeTag::T_SubPlanState,
            343 => crate::__pgrx_c_bindings::NodeTag::T_SubqueryScan,
            284 => crate::__pgrx_c_bindings::NodeTag::T_SubqueryScanPath,
            407 => crate::__pgrx_c_bindings::NodeTag::T_SubqueryScanState,
            14 => crate::__pgrx_c_bindings::NodeTag::T_SubscriptingRef,
            455 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestCost,
            457 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestIndexCondition,
            459 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestOptimizeWindowClause,
            456 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestRows,
            454 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestSelectivity,
            453 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestSimplify,
            458 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestWFuncMonotonic,
            473 => crate::__pgrx_c_bindings::NodeTag::T_TIDBitmap,
            435 => crate::__pgrx_c_bindings::NodeTag::T_TableAmRoutine,
            4 => crate::__pgrx_c_bindings::NodeTag::T_TableFunc,
            346 => crate::__pgrx_c_bindings::NodeTag::T_TableFuncScan,
            410 => crate::__pgrx_c_bindings::NodeTag::T_TableFuncScanState,
            90 => crate::__pgrx_c_bindings::NodeTag::T_TableLikeClause,
            104 => crate::__pgrx_c_bindings::NodeTag::T_TableSampleClause,
            61 => crate::__pgrx_c_bindings::NodeTag::T_TargetEntry,
            282 => crate::__pgrx_c_bindings::NodeTag::T_TidPath,
            283 => crate::__pgrx_c_bindings::NodeTag::T_TidRangePath,
            342 => crate::__pgrx_c_bindings::NodeTag::T_TidRangeScan,
            406 => crate::__pgrx_c_bindings::NodeTag::T_TidRangeScanState,
            341 => crate::__pgrx_c_bindings::NodeTag::T_TidScan,
            405 => crate::__pgrx_c_bindings::NodeTag::T_TidScanState,
            451 => crate::__pgrx_c_bindings::NodeTag::T_TimeLineHistoryCmd,
            222 => crate::__pgrx_c_bindings::NodeTag::T_TransactionStmt,
            438 => crate::__pgrx_c_bindings::NodeTag::T_TriggerData,
            117 => crate::__pgrx_c_bindings::NodeTag::T_TriggerTransition,
            195 => crate::__pgrx_c_bindings::NodeTag::T_TruncateStmt,
            436 => crate::__pgrx_c_bindings::NodeTag::T_TsmRoutine,
            439 => crate::__pgrx_c_bindings::NodeTag::T_TupleTableSlot,
            72 => crate::__pgrx_c_bindings::NodeTag::T_TypeCast,
            67 => crate::__pgrx_c_bindings::NodeTag::T_TypeName,
            363 => crate::__pgrx_c_bindings::NodeTag::T_Unique,
            292 => crate::__pgrx_c_bindings::NodeTag::T_UniquePath,
            427 => crate::__pgrx_c_bindings::NodeTag::T_UniqueState,
            221 => crate::__pgrx_c_bindings::NodeTag::T_UnlistenStmt,
            137 => crate::__pgrx_c_bindings::NodeTag::T_UpdateStmt,
            452 => crate::__pgrx_c_bindings::NodeTag::T_UploadManifestCmd,
            303 => crate::__pgrx_c_bindings::NodeTag::T_UpperUniquePath,
            237 => crate::__pgrx_c_bindings::NodeTag::T_VacuumRelation,
            236 => crate::__pgrx_c_bindings::NodeTag::T_VacuumStmt,
            345 => crate::__pgrx_c_bindings::NodeTag::T_ValuesScan,
            409 => crate::__pgrx_c_bindings::NodeTag::T_ValuesScanState,
            6 => crate::__pgrx_c_bindings::NodeTag::T_Var,
            155 => crate::__pgrx_c_bindings::NodeTag::T_VariableSetStmt,
            156 => crate::__pgrx_c_bindings::NodeTag::T_VariableShowStmt,
            227 => crate::__pgrx_c_bindings::NodeTag::T_ViewStmt,
            362 => crate::__pgrx_c_bindings::NodeTag::T_WindowAgg,
            309 => crate::__pgrx_c_bindings::NodeTag::T_WindowAggPath,
            426 => crate::__pgrx_c_bindings::NodeTag::T_WindowAggState,
            108 => crate::__pgrx_c_bindings::NodeTag::T_WindowClause,
            83 => crate::__pgrx_c_bindings::NodeTag::T_WindowDef,
            11 => crate::__pgrx_c_bindings::NodeTag::T_WindowFunc,
            386 => crate::__pgrx_c_bindings::NodeTag::T_WindowFuncExprState,
            12 => crate::__pgrx_c_bindings::NodeTag::T_WindowFuncRunCondition,
            474 => crate::__pgrx_c_bindings::NodeTag::T_WindowObjectData,
            105 => crate::__pgrx_c_bindings::NodeTag::T_WithCheckOption,
            110 => crate::__pgrx_c_bindings::NodeTag::T_WithClause,
            349 => crate::__pgrx_c_bindings::NodeTag::T_WorkTableScan,
            413 => crate::__pgrx_c_bindings::NodeTag::T_WorkTableScanState,
            468 => crate::__pgrx_c_bindings::NodeTag::T_XidList,
            41 => crate::__pgrx_c_bindings::NodeTag::T_XmlExpr,
            94 => crate::__pgrx_c_bindings::NodeTag::T_XmlSerialize,
            _ => panic!("C enum value has no corresponding Rust variant"),
        }
    }
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__TableFuncType;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for EnumIdentity_enum__TableFuncType {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::EnumIdentity for EnumIdentity_enum__TableFuncType {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__TableFuncType {
    type Compatible = c::CUnsignedInt;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::CopyFromStateData,
                *mut crate::__pgrx_c_bindings::ExprContext,
                *mut crate::__pgrx_c_bindings::Datum,
                *mut bool,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::CopyFromStateData,
                *mut crate::__pgrx_c_bindings::ExprContext,
                *mut crate::__pgrx_c_bindings::Datum,
                *mut bool,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__CopyFromStateData__0p__0c__struct__ExprContext__0p__0c__unsigned__long__0p__0c__0x5f426f6f6c__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__CopyFromStateData__0p__0c__struct__ExprContext__0p__0c__unsigned__long__0p__0c__0x5f426f6f6c__0p__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__struct__CopyFromStateData__0p__0c__struct__ExprContext__0p__0c__unsigned__long__0p__0c__0x5f426f6f6c__0p__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::CopyFromStateData, *mut crate::__pgrx_c_bindings::ExprContext, *mut crate::__pgrx_c_bindings::Datum, *mut bool) -> bool>>;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::Node,
                *mut crate::__pgrx_c_bindings::ChangeVarNodes_context,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::Node,
                *mut crate::__pgrx_c_bindings::ChangeVarNodes_context,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(feature = "pg18")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__Node__0p__0c__struct__ChangeVarNodes_context__0p__0r;
#[cfg(feature = "pg18")]
impl c::sealed::Sealed
    for Signature_0x5f426f6f6c__0l__struct__Node__0p__0c__struct__ChangeVarNodes_context__0p__0r
{
}
#[cfg(feature = "pg18")]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__struct__Node__0p__0c__struct__ChangeVarNodes_context__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::Node,
                *mut crate::__pgrx_c_bindings::ChangeVarNodes_context,
            ) -> bool,
        >,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PgAioHandle) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PgAioHandle) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__PgAioHandle__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__PgAioHandle__0p__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__struct__PgAioHandle__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PgAioHandle) -> bool,
        >,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableScanDescData,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                *mut bool,
                *mut u64,
                *mut u64,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableScanDescData,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                *mut bool,
                *mut u64,
                *mut u64,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__struct__TupleTableSlot__0p__0c__0x5f426f6f6c__0p__0c__unsigned__long__0p__0c__unsigned__long__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__struct__TupleTableSlot__0p__0c__0x5f426f6f6c__0p__0c__unsigned__long__0p__0c__unsigned__long__0p__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__struct__TupleTableSlot__0p__0c__0x5f426f6f6c__0p__0c__unsigned__long__0p__0c__unsigned__long__0p__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TableScanDescData, *mut crate::__pgrx_c_bindings::TupleTableSlot, *mut bool, *mut u64, *mut u64) -> bool>>;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::HeapTupleData) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::HeapTupleData) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__const__struct__HeapTupleData__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__const__struct__HeapTupleData__0p__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__const__struct__HeapTupleData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::HeapTupleData) -> bool,
        >,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::HeapTupleHeaderData,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::HeapTupleHeaderData,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__const__struct__HeapTupleHeaderData__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__const__struct__HeapTupleHeaderData__0p__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__const__struct__HeapTupleHeaderData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::HeapTupleHeaderData,
            ) -> bool,
        >,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::IndexTupleData) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::IndexTupleData) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__const__struct__IndexTupleData__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__const__struct__IndexTupleData__0p__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__const__struct__IndexTupleData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::IndexTupleData) -> bool,
        >,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::MinimalTupleData) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::MinimalTupleData) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__const__struct__MinimalTupleData__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__const__struct__MinimalTupleData__0p__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__const__struct__MinimalTupleData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::MinimalTupleData) -> bool,
        >,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const ::core::ffi::c_void, usize) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const ::core::ffi::c_void, usize) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__const__void__0p__0c__unsigned__long__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__const__void__0p__0c__unsigned__long__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__const__void__0p__0c__unsigned__long__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const ::core::ffi::c_void, usize) -> bool,
        >,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(i32, *mut i32) -> bool>,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(i32, *mut i32) -> bool>,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__int__0c__int__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__int__0c__int__0p__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__int__0c__int__0p__0r {
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<unsafe extern "C-unwind" fn(i32, *mut i32) -> bool>,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(i64, *mut i64) -> bool>,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(i64, *mut i64) -> bool>,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__long__0c__long__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__long__0c__long__0p__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__long__0c__long__0p__0r {
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<unsafe extern "C-unwind" fn(i64, *mut i64) -> bool>,
    >;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn(i16) -> bool>>() == 8
);
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn(i16) -> bool>>()
        == 8
);
#[cfg(feature = "pg18")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__short__0r;
#[cfg(feature = "pg18")]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__short__0r {}
#[cfg(feature = "pg18")]
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__short__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(i16) -> bool>,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(i16, *mut i16) -> bool>,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(i16, *mut i16) -> bool>,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__short__0c__short__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__short__0c__short__0p__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__short__0c__short__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<unsafe extern "C-unwind" fn(i16, *mut i16) -> bool>,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RestrictInfo,
                *mut crate::__pgrx_c_bindings::Bitmapset,
                *mut crate::__pgrx_c_bindings::Bitmapset,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RestrictInfo,
                *mut crate::__pgrx_c_bindings::Bitmapset,
                *mut crate::__pgrx_c_bindings::Bitmapset,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__RestrictInfo__0p__0c__struct__Bitmapset__0p__0c__struct__Bitmapset__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__RestrictInfo__0p__0c__struct__Bitmapset__0p__0c__struct__Bitmapset__0p__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__struct__RestrictInfo__0p__0c__struct__Bitmapset__0p__0c__struct__Bitmapset__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RestrictInfo, *mut crate::__pgrx_c_bindings::Bitmapset, *mut crate::__pgrx_c_bindings::Bitmapset) -> bool>>;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TBMIterator) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TBMIterator) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__TBMIterator__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__TBMIterator__0p__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__struct__TBMIterator__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TBMIterator) -> bool,
        >,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(u32, *mut i32) -> bool>,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(u32, *mut i32) -> bool>,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__unsigned__int__0c__int__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__unsigned__int__0c__int__0p__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__unsigned__int__0c__int__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<unsafe extern "C-unwind" fn(u32, *mut i32) -> bool>,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(u64, *mut i64) -> bool>,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(u64, *mut i64) -> bool>,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__unsigned__long__0c__long__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__unsigned__long__0c__long__0p__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__unsigned__long__0c__long__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<unsafe extern "C-unwind" fn(u64, *mut i64) -> bool>,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn(u16) -> bool>>() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn(u16) -> bool>>()
        == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__unsigned__short__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__unsigned__short__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__unsigned__short__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(u16) -> bool>,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(u16, *mut i16) -> bool>,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(u16, *mut i16) -> bool>,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__unsigned__short__0c__short__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__unsigned__short__0c__short__0p__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__unsigned__short__0c__short__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<unsafe extern "C-unwind" fn(u16, *mut i16) -> bool>,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
                usize,
                crate::__pgrx_c_bindings::Oid,
            ) -> *mut ::core::ffi::c_char,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
                usize,
                crate::__pgrx_c_bindings::Oid,
            ) -> *mut ::core::ffi::c_char,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_char__0p__0l__const__char__0p__0c__unsigned__long__0c__unsigned__int__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed
    for Signature_char__0p__0l__const__char__0p__0c__unsigned__long__0c__unsigned__int__0r
{
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_char__0p__0l__const__char__0p__0c__unsigned__long__0c__unsigned__int__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
                usize,
                crate::__pgrx_c_bindings::Oid,
            ) -> *mut ::core::ffi::c_char,
        >,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::PgAioTargetData,
            ) -> *mut ::core::ffi::c_char,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::PgAioTargetData,
            ) -> *mut ::core::ffi::c_char,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_char__0p__0l__const__union__PgAioTargetData__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_char__0p__0l__const__union__PgAioTargetData__0p__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_char__0p__0l__const__union__PgAioTargetData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::PgAioTargetData,
            ) -> *mut ::core::ffi::c_char,
        >,
    >;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
                *const crate::__pgrx_c_bindings::ItemIdData,
            ) -> *mut ::core::ffi::c_char,
        >,
    >() == 8
);
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
                *const crate::__pgrx_c_bindings::ItemIdData,
            ) -> *mut ::core::ffi::c_char,
        >,
    >() == 8
);
#[cfg(feature = "pg18")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_char__0p__0l__const__char__0p__0c__const__struct__ItemIdData__0p__0r;
#[cfg(feature = "pg18")]
impl c::sealed::Sealed
    for Signature_char__0p__0l__const__char__0p__0c__const__struct__ItemIdData__0p__0r
{
}
#[cfg(feature = "pg18")]
impl c::expression::NativeFunctionSignature
    for Signature_char__0p__0l__const__char__0p__0c__const__struct__ItemIdData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
                *const crate::__pgrx_c_bindings::ItemIdData,
            ) -> *mut ::core::ffi::c_char,
        >,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(u16, crate::__pgrx_c_bindings::Oid) -> u32,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(u16, crate::__pgrx_c_bindings::Oid) -> u32,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_enum__CompareType__0l__unsigned__short__0c__unsigned__int__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_enum__CompareType__0l__unsigned__short__0c__unsigned__int__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_enum__CompareType__0l__unsigned__short__0c__unsigned__int__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(u16, crate::__pgrx_c_bindings::Oid) -> u32,
        >,
    >;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
                isize,
                *const ::core::ffi::c_char,
                isize,
                *mut crate::__pgrx_c_bindings::pg_locale_struct,
            ) -> i32,
        >,
    >() == 8
);
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
                isize,
                *const ::core::ffi::c_char,
                isize,
                *mut crate::__pgrx_c_bindings::pg_locale_struct,
            ) -> i32,
        >,
    >() == 8
);
#[cfg(feature = "pg18")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__const__char__0p__0c__long__0c__const__char__0p__0c__long__0c__struct__pg_locale_struct__0p__0r;
#[cfg(feature = "pg18")]
impl c::sealed::Sealed for Signature_int__0l__const__char__0p__0c__long__0c__const__char__0p__0c__long__0c__struct__pg_locale_struct__0p__0r {}
#[cfg(feature = "pg18")]
impl c::expression::NativeFunctionSignature for Signature_int__0l__const__char__0p__0c__long__0c__const__char__0p__0c__long__0c__struct__pg_locale_struct__0p__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_char, isize, *const ::core::ffi::c_char, isize, *mut crate::__pgrx_c_bindings::pg_locale_struct) -> i32>>;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u16,
                *mut *mut crate::__pgrx_c_bindings::PgAioHandle,
            ) -> i32,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u16,
                *mut *mut crate::__pgrx_c_bindings::PgAioHandle,
            ) -> i32,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__unsigned__short__0c__struct__PgAioHandle__0p__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_int__0l__unsigned__short__0c__struct__PgAioHandle__0p__0p__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_int__0l__unsigned__short__0c__struct__PgAioHandle__0p__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u16,
                *mut *mut crate::__pgrx_c_bindings::PgAioHandle,
            ) -> i32,
        >,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::HeapTupleHeaderData,
            ) -> i32,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::HeapTupleHeaderData,
            ) -> i32,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__const__struct__HeapTupleHeaderData__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_int__0l__const__struct__HeapTupleHeaderData__0p__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_int__0l__const__struct__HeapTupleHeaderData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::HeapTupleHeaderData,
            ) -> i32,
        >,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn(i32) -> i32>>() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn(i32) -> i32>>() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__int__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_int__0l__int__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_int__0l__int__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(i32) -> i32>,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TupleDescData,
                i32,
            )
                -> *mut crate::__pgrx_c_bindings::CompactAttribute,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TupleDescData,
                i32,
            )
                -> *mut crate::__pgrx_c_bindings::CompactAttribute,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__CompactAttribute__0p__0l__struct__TupleDescData__0p__0c__int__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed
    for Signature_struct__CompactAttribute__0p__0l__struct__TupleDescData__0p__0c__int__0r
{
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_struct__CompactAttribute__0p__0l__struct__TupleDescData__0p__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TupleDescData,
                i32,
            )
                -> *mut crate::__pgrx_c_bindings::CompactAttribute,
        >,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TupleDescData,
                i32,
            )
                -> *mut crate::__pgrx_c_bindings::FormData_pg_attribute,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TupleDescData,
                i32,
            )
                -> *mut crate::__pgrx_c_bindings::FormData_pg_attribute,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__FormData_pg_attribute__0p__0l__struct__TupleDescData__0p__0c__int__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed
    for Signature_struct__FormData_pg_attribute__0p__0l__struct__TupleDescData__0p__0c__int__0r
{
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_struct__FormData_pg_attribute__0p__0l__struct__TupleDescData__0p__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TupleDescData,
                i32,
            )
                -> *mut crate::__pgrx_c_bindings::FormData_pg_attribute,
        >,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::GinTuple,
            )
                -> *mut crate::__pgrx_c_bindings::ItemPointerData,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::GinTuple,
            )
                -> *mut crate::__pgrx_c_bindings::ItemPointerData,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__ItemPointerData__0p__0l__struct__GinTuple__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_struct__ItemPointerData__0p__0l__struct__GinTuple__0p__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_struct__ItemPointerData__0p__0l__struct__GinTuple__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::GinTuple,
            )
                -> *mut crate::__pgrx_c_bindings::ItemPointerData,
        >,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                usize,
            )
                -> *mut crate::__pgrx_c_bindings::MinimalTupleData,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                usize,
            )
                -> *mut crate::__pgrx_c_bindings::MinimalTupleData,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__MinimalTupleData__0p__0l__struct__TupleTableSlot__0p__0c__unsigned__long__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_struct__MinimalTupleData__0p__0l__struct__TupleTableSlot__0p__0c__unsigned__long__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_struct__MinimalTupleData__0p__0l__struct__TupleTableSlot__0p__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_2<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TupleTableSlot, usize) -> *mut crate::__pgrx_c_bindings::MinimalTupleData>>;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TupleHashEntryData,
            )
                -> *mut crate::__pgrx_c_bindings::MinimalTupleData,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TupleHashEntryData,
            )
                -> *mut crate::__pgrx_c_bindings::MinimalTupleData,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__MinimalTupleData__0p__0l__struct__TupleHashEntryData__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed
    for Signature_struct__MinimalTupleData__0p__0l__struct__TupleHashEntryData__0p__0r
{
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_struct__MinimalTupleData__0p__0l__struct__TupleHashEntryData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TupleHashEntryData,
            )
                -> *mut crate::__pgrx_c_bindings::MinimalTupleData,
        >,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PgAioHandle,
                crate::__pgrx_c_bindings::PgAioResult,
                u8,
            ) -> crate::__pgrx_c_bindings::PgAioResult,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PgAioHandle,
                crate::__pgrx_c_bindings::PgAioResult,
                u8,
            ) -> crate::__pgrx_c_bindings::PgAioResult,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__PgAioResult__0l__struct__PgAioHandle__0p__0c__struct__PgAioResult__0c__unsigned__char__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_struct__PgAioResult__0l__struct__PgAioHandle__0p__0c__struct__PgAioResult__0c__unsigned__char__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_struct__PgAioResult__0l__struct__PgAioHandle__0p__0c__struct__PgAioResult__0c__unsigned__char__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PgAioHandle, crate::__pgrx_c_bindings::PgAioResult, u8) -> crate::__pgrx_c_bindings::PgAioResult>>;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Oid,
                i32,
                i32,
            ) -> ::core::mem::MaybeUninit<
                crate::__pgrx_c_bindings::RelPathStr,
            >,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Oid,
                i32,
                i32,
            ) -> ::core::mem::MaybeUninit<
                crate::__pgrx_c_bindings::RelPathStr,
            >,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__RelPathStr__0l__unsigned__int__0c__unsigned__int__0c__unsigned__int__0c__int__0c__enum__ForkNumber__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_struct__RelPathStr__0l__unsigned__int__0c__unsigned__int__0c__unsigned__int__0c__int__0c__enum__ForkNumber__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_struct__RelPathStr__0l__unsigned__int__0c__unsigned__int__0c__unsigned__int__0c__int__0c__enum__ForkNumber__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Oid, crate::__pgrx_c_bindings::Oid, crate::__pgrx_c_bindings::Oid, i32, i32) -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::RelPathStr>>>;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ScanState,
                *mut crate::__pgrx_c_bindings::EPQState,
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::ScanState,
                    )
                        -> *mut crate::__pgrx_c_bindings::TupleTableSlot,
                >,
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::ScanState,
                        *mut crate::__pgrx_c_bindings::TupleTableSlot,
                    ) -> bool,
                >,
            )
                -> *mut crate::__pgrx_c_bindings::TupleTableSlot,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ScanState,
                *mut crate::__pgrx_c_bindings::EPQState,
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::ScanState,
                    )
                        -> *mut crate::__pgrx_c_bindings::TupleTableSlot,
                >,
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::ScanState,
                        *mut crate::__pgrx_c_bindings::TupleTableSlot,
                    ) -> bool,
                >,
            )
                -> *mut crate::__pgrx_c_bindings::TupleTableSlot,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__TupleTableSlot__0p__0l__struct__ScanState__0p__0c__struct__EPQState__0p__0c__struct__TupleTableSlot__0p__0l__0p__0r__0l__struct__ScanState__0p__0r__0c__0x5f426f6f6c__0l__0p__0r__0l__struct__ScanState__0p__0c__struct__TupleTableSlot__0p__0r__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_struct__TupleTableSlot__0p__0l__struct__ScanState__0p__0c__struct__EPQState__0p__0c__struct__TupleTableSlot__0p__0l__0p__0r__0l__struct__ScanState__0p__0r__0c__0x5f426f6f6c__0l__0p__0r__0l__struct__ScanState__0p__0c__struct__TupleTableSlot__0p__0r__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_struct__TupleTableSlot__0p__0l__struct__ScanState__0p__0c__struct__EPQState__0p__0c__struct__TupleTableSlot__0p__0l__0p__0r__0l__struct__ScanState__0p__0r__0c__0x5f426f6f6c__0l__0p__0r__0l__struct__ScanState__0p__0c__struct__TupleTableSlot__0p__0r__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ScanState, *mut crate::__pgrx_c_bindings::EPQState, ::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ScanState) -> *mut crate::__pgrx_c_bindings::TupleTableSlot>, ::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ScanState, *mut crate::__pgrx_c_bindings::TupleTableSlot) -> bool>) -> *mut crate::__pgrx_c_bindings::TupleTableSlot>>;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ScanState,
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::ScanState,
                    )
                        -> *mut crate::__pgrx_c_bindings::TupleTableSlot,
                >,
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::ScanState,
                        *mut crate::__pgrx_c_bindings::TupleTableSlot,
                    ) -> bool,
                >,
                *mut crate::__pgrx_c_bindings::EPQState,
                *mut crate::__pgrx_c_bindings::ExprState,
                *mut crate::__pgrx_c_bindings::ProjectionInfo,
            )
                -> *mut crate::__pgrx_c_bindings::TupleTableSlot,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ScanState,
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::ScanState,
                    )
                        -> *mut crate::__pgrx_c_bindings::TupleTableSlot,
                >,
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::ScanState,
                        *mut crate::__pgrx_c_bindings::TupleTableSlot,
                    ) -> bool,
                >,
                *mut crate::__pgrx_c_bindings::EPQState,
                *mut crate::__pgrx_c_bindings::ExprState,
                *mut crate::__pgrx_c_bindings::ProjectionInfo,
            )
                -> *mut crate::__pgrx_c_bindings::TupleTableSlot,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__TupleTableSlot__0p__0l__struct__ScanState__0p__0c__struct__TupleTableSlot__0p__0l__0p__0r__0l__struct__ScanState__0p__0r__0c__0x5f426f6f6c__0l__0p__0r__0l__struct__ScanState__0p__0c__struct__TupleTableSlot__0p__0r__0c__struct__EPQState__0p__0c__struct__ExprState__0p__0c__struct__ProjectionInfo__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_struct__TupleTableSlot__0p__0l__struct__ScanState__0p__0c__struct__TupleTableSlot__0p__0l__0p__0r__0l__struct__ScanState__0p__0r__0c__0x5f426f6f6c__0l__0p__0r__0l__struct__ScanState__0p__0c__struct__TupleTableSlot__0p__0r__0c__struct__EPQState__0p__0c__struct__ExprState__0p__0c__struct__ProjectionInfo__0p__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_struct__TupleTableSlot__0p__0l__struct__ScanState__0p__0c__struct__TupleTableSlot__0p__0l__0p__0r__0l__struct__ScanState__0p__0r__0c__0x5f426f6f6c__0l__0p__0r__0l__struct__ScanState__0p__0c__struct__TupleTableSlot__0p__0r__0c__struct__EPQState__0p__0c__struct__ExprState__0p__0c__struct__ProjectionInfo__0p__0r {
 type Physical = PhysicalFunction_C_unwind_6<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ScanState, ::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ScanState) -> *mut crate::__pgrx_c_bindings::TupleTableSlot>, ::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ScanState, *mut crate::__pgrx_c_bindings::TupleTableSlot) -> bool>, *mut crate::__pgrx_c_bindings::EPQState, *mut crate::__pgrx_c_bindings::ExprState, *mut crate::__pgrx_c_bindings::ProjectionInfo) -> *mut crate::__pgrx_c_bindings::TupleTableSlot>>;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_char) -> u8>,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_char) -> u8>,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__char__0l__const__char__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_unsigned__char__0l__const__char__0p__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_unsigned__char__0l__const__char__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_char) -> u8>,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::Var,
                *mut ::core::ffi::c_void,
            ) -> crate::__pgrx_c_bindings::Oid,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::Var,
                *mut ::core::ffi::c_void,
            ) -> crate::__pgrx_c_bindings::Oid,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__int__0l__struct__Var__0p__0c__void__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_unsigned__int__0l__struct__Var__0p__0c__void__0p__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__int__0l__struct__Var__0p__0c__void__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::Var,
                *mut ::core::ffi::c_void,
            ) -> crate::__pgrx_c_bindings::Oid,
        >,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn(i32) -> u32>>() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn(i32) -> u32>>() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__int__0l__int__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_unsigned__int__0l__int__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_unsigned__int__0l__int__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(i32) -> u32>,
    >;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_char,
                usize,
                *const ::core::ffi::c_char,
                isize,
                *mut crate::__pgrx_c_bindings::pg_locale_struct,
            ) -> usize,
        >,
    >() == 8
);
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_char,
                usize,
                *const ::core::ffi::c_char,
                isize,
                *mut crate::__pgrx_c_bindings::pg_locale_struct,
            ) -> usize,
        >,
    >() == 8
);
#[cfg(feature = "pg18")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__char__0p__0c__unsigned__long__0c__const__char__0p__0c__long__0c__struct__pg_locale_struct__0p__0r;
#[cfg(feature = "pg18")]
impl c::sealed::Sealed for Signature_unsigned__long__0l__char__0p__0c__unsigned__long__0c__const__char__0p__0c__long__0c__struct__pg_locale_struct__0p__0r {}
#[cfg(feature = "pg18")]
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__char__0p__0c__unsigned__long__0c__const__char__0p__0c__long__0c__struct__pg_locale_struct__0p__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_char, usize, *const ::core::ffi::c_char, isize, *mut crate::__pgrx_c_bindings::pg_locale_struct) -> usize>>;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                i32,
                i32,
            ) -> usize,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                i32,
                i32,
            ) -> usize,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__struct__RelationData__0p__0c__int__0c__int__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed
    for Signature_unsigned__long__0l__struct__RelationData__0p__0c__int__0c__int__0r
{
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__long__0l__struct__RelationData__0p__0c__int__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                i32,
                i32,
            ) -> usize,
        >,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                crate::__pgrx_c_bindings::Datum,
                *mut bool,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                crate::__pgrx_c_bindings::Datum,
                *mut bool,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__struct__RelationData__0p__0c__unsigned__long__0c__0x5f426f6f6c__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_unsigned__long__0l__struct__RelationData__0p__0c__unsigned__long__0c__0x5f426f6f6c__0p__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__struct__RelationData__0p__0c__unsigned__long__0c__0x5f426f6f6c__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, crate::__pgrx_c_bindings::Datum, *mut bool) -> crate::__pgrx_c_bindings::Datum>>;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::IndexTupleData) -> usize,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::IndexTupleData) -> usize,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__const__struct__IndexTupleData__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_unsigned__long__0l__const__struct__IndexTupleData__0p__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__long__0l__const__struct__IndexTupleData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::IndexTupleData) -> usize,
        >,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn(i64, i64) -> u64>>()
        == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn(i64, i64) -> u64>>()
        == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__long__0c__long__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_unsigned__long__0l__long__0c__long__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__long__0c__long__0r {
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<unsafe extern "C-unwind" fn(i64, i64) -> u64>,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(u32, crate::__pgrx_c_bindings::Oid) -> u16,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(u32, crate::__pgrx_c_bindings::Oid) -> u16,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__short__0l__enum__CompareType__0c__unsigned__int__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_unsigned__short__0l__enum__CompareType__0c__unsigned__int__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__short__0l__enum__CompareType__0c__unsigned__int__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(u32, crate::__pgrx_c_bindings::Oid) -> u16,
        >,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_char) -> u16>,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_char) -> u16>,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__short__0l__const__char__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_unsigned__short__0l__const__char__0p__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_unsigned__short__0l__const__char__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_char) -> u16>,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn(i16) -> u16>>() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn(i16) -> u16>>() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__short__0l__short__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_unsigned__short__0l__short__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_unsigned__short__0l__short__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(i16) -> u16>,
    >;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn(bool) -> ()>>() == 8
);
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn(bool) -> ()>>() == 8
);
#[cfg(feature = "pg18")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__0x5f426f6f6c__0r;
#[cfg(feature = "pg18")]
impl c::sealed::Sealed for Signature_void__0l__0x5f426f6f6c__0r {}
#[cfg(feature = "pg18")]
impl c::expression::NativeFunctionSignature for Signature_void__0l__0x5f426f6f6c__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(bool) -> ()>,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::CachedFunction) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::CachedFunction) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__CachedFunction__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_void__0l__struct__CachedFunction__0p__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__CachedFunction__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::CachedFunction) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::CopyFromStateData) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::CopyFromStateData) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__CopyFromStateData__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_void__0l__struct__CopyFromStateData__0p__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__CopyFromStateData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::CopyFromStateData) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::CopyFromStateData,
                *mut crate::__pgrx_c_bindings::TupleDescData,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::CopyFromStateData,
                *mut crate::__pgrx_c_bindings::TupleDescData,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__CopyFromStateData__0p__0c__struct__TupleDescData__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed
    for Signature_void__0l__struct__CopyFromStateData__0p__0c__struct__TupleDescData__0p__0r
{
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__CopyFromStateData__0p__0c__struct__TupleDescData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::CopyFromStateData,
                *mut crate::__pgrx_c_bindings::TupleDescData,
            ) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::CopyFromStateData,
                crate::__pgrx_c_bindings::Oid,
                *mut crate::__pgrx_c_bindings::FmgrInfo,
                *mut crate::__pgrx_c_bindings::Oid,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::CopyFromStateData,
                crate::__pgrx_c_bindings::Oid,
                *mut crate::__pgrx_c_bindings::FmgrInfo,
                *mut crate::__pgrx_c_bindings::Oid,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__CopyFromStateData__0p__0c__unsigned__int__0c__struct__FmgrInfo__0p__0c__unsigned__int__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_void__0l__struct__CopyFromStateData__0p__0c__unsigned__int__0c__struct__FmgrInfo__0p__0c__unsigned__int__0p__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__CopyFromStateData__0p__0c__unsigned__int__0c__struct__FmgrInfo__0p__0c__unsigned__int__0p__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::CopyFromStateData, crate::__pgrx_c_bindings::Oid, *mut crate::__pgrx_c_bindings::FmgrInfo, *mut crate::__pgrx_c_bindings::Oid) -> ()>>;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::CopyToStateData) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::CopyToStateData) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__CopyToStateData__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_void__0l__struct__CopyToStateData__0p__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__CopyToStateData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::CopyToStateData) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::CopyToStateData,
                *mut crate::__pgrx_c_bindings::TupleDescData,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::CopyToStateData,
                *mut crate::__pgrx_c_bindings::TupleDescData,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__CopyToStateData__0p__0c__struct__TupleDescData__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed
    for Signature_void__0l__struct__CopyToStateData__0p__0c__struct__TupleDescData__0p__0r
{
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__CopyToStateData__0p__0c__struct__TupleDescData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::CopyToStateData,
                *mut crate::__pgrx_c_bindings::TupleDescData,
            ) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::CopyToStateData,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::CopyToStateData,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__CopyToStateData__0p__0c__struct__TupleTableSlot__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed
    for Signature_void__0l__struct__CopyToStateData__0p__0c__struct__TupleTableSlot__0p__0r
{
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__CopyToStateData__0p__0c__struct__TupleTableSlot__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::CopyToStateData,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
            ) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::CopyToStateData,
                crate::__pgrx_c_bindings::Oid,
                *mut crate::__pgrx_c_bindings::FmgrInfo,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::CopyToStateData,
                crate::__pgrx_c_bindings::Oid,
                *mut crate::__pgrx_c_bindings::FmgrInfo,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__CopyToStateData__0p__0c__unsigned__int__0c__struct__FmgrInfo__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_void__0l__struct__CopyToStateData__0p__0c__unsigned__int__0c__struct__FmgrInfo__0p__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__CopyToStateData__0p__0c__unsigned__int__0c__struct__FmgrInfo__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::CopyToStateData, crate::__pgrx_c_bindings::Oid, *mut crate::__pgrx_c_bindings::FmgrInfo) -> ()>>;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ExplainState,
                *mut crate::__pgrx_c_bindings::DefElem,
                *mut crate::__pgrx_c_bindings::ParseState,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ExplainState,
                *mut crate::__pgrx_c_bindings::DefElem,
                *mut crate::__pgrx_c_bindings::ParseState,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__ExplainState__0p__0c__struct__DefElem__0p__0c__struct__ParseState__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_void__0l__struct__ExplainState__0p__0c__struct__DefElem__0p__0c__struct__ParseState__0p__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__ExplainState__0p__0c__struct__DefElem__0p__0c__struct__ParseState__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ExplainState, *mut crate::__pgrx_c_bindings::DefElem, *mut crate::__pgrx_c_bindings::ParseState) -> ()>>;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ExplainState,
                *mut crate::__pgrx_c_bindings::List,
                *mut crate::__pgrx_c_bindings::ParseState,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ExplainState,
                *mut crate::__pgrx_c_bindings::List,
                *mut crate::__pgrx_c_bindings::ParseState,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__ExplainState__0p__0c__struct__List__0p__0c__struct__ParseState__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_void__0l__struct__ExplainState__0p__0c__struct__List__0p__0c__struct__ParseState__0p__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__ExplainState__0p__0c__struct__List__0p__0c__struct__ParseState__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ExplainState, *mut crate::__pgrx_c_bindings::List, *mut crate::__pgrx_c_bindings::ParseState) -> ()>>;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::FunctionCallInfoBaseData,
                *mut crate::__pgrx_c_bindings::HeapTupleData,
                *const crate::__pgrx_c_bindings::CachedFunctionHashKey,
                *mut crate::__pgrx_c_bindings::CachedFunction,
                bool,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::FunctionCallInfoBaseData,
                *mut crate::__pgrx_c_bindings::HeapTupleData,
                *const crate::__pgrx_c_bindings::CachedFunctionHashKey,
                *mut crate::__pgrx_c_bindings::CachedFunction,
                bool,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__FunctionCallInfoBaseData__0p__0c__struct__HeapTupleData__0p__0c__const__struct__CachedFunctionHashKey__0p__0c__struct__CachedFunction__0p__0c__0x5f426f6f6c__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_void__0l__struct__FunctionCallInfoBaseData__0p__0c__struct__HeapTupleData__0p__0c__const__struct__CachedFunctionHashKey__0p__0c__struct__CachedFunction__0p__0c__0x5f426f6f6c__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__FunctionCallInfoBaseData__0p__0c__struct__HeapTupleData__0p__0c__const__struct__CachedFunctionHashKey__0p__0c__struct__CachedFunction__0p__0c__0x5f426f6f6c__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::FunctionCallInfoBaseData, *mut crate::__pgrx_c_bindings::HeapTupleData, *const crate::__pgrx_c_bindings::CachedFunctionHashKey, *mut crate::__pgrx_c_bindings::CachedFunction, bool) -> ()>>;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::List,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::List,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__List__0p__0c__void__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_void__0l__struct__List__0p__0c__void__0p__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__List__0p__0c__void__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::List,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PgAioHandle) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PgAioHandle) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__PgAioHandle__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_void__0l__struct__PgAioHandle__0p__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__PgAioHandle__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PgAioHandle) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PgAioHandle, u8) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PgAioHandle, u8) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__PgAioHandle__0p__0c__unsigned__char__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_void__0l__struct__PgAioHandle__0p__0c__unsigned__char__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__PgAioHandle__0p__0c__unsigned__char__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PgAioHandle, u8) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PgAioHandle, u64) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PgAioHandle, u64) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__PgAioHandle__0p__0c__unsigned__long__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_void__0l__struct__PgAioHandle__0p__0c__unsigned__long__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__PgAioHandle__0p__0c__unsigned__long__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PgAioHandle, u64) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::PgAioResult,
                *const crate::__pgrx_c_bindings::PgAioTargetData,
                i32,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::PgAioResult,
                *const crate::__pgrx_c_bindings::PgAioTargetData,
                i32,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__PgAioResult__0c__const__union__PgAioTargetData__0p__0c__int__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed
    for Signature_void__0l__struct__PgAioResult__0c__const__union__PgAioTargetData__0p__0c__int__0r
{
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__PgAioResult__0c__const__union__PgAioTargetData__0p__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::PgAioResult,
                *const crate::__pgrx_c_bindings::PgAioTargetData,
                i32,
            ) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlanState,
                *mut crate::__pgrx_c_bindings::List,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *mut crate::__pgrx_c_bindings::ExplainState,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlanState,
                *mut crate::__pgrx_c_bindings::List,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *mut crate::__pgrx_c_bindings::ExplainState,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__PlanState__0p__0c__struct__List__0p__0c__const__char__0p__0c__const__char__0p__0c__struct__ExplainState__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_void__0l__struct__PlanState__0p__0c__struct__List__0p__0c__const__char__0p__0c__const__char__0p__0c__struct__ExplainState__0p__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__PlanState__0p__0c__struct__List__0p__0c__const__char__0p__0c__const__char__0p__0c__struct__ExplainState__0p__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PlanState, *mut crate::__pgrx_c_bindings::List, *const ::core::ffi::c_char, *const ::core::ffi::c_char, *mut crate::__pgrx_c_bindings::ExplainState) -> ()>>;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannedStmt,
                *mut crate::__pgrx_c_bindings::IntoClause,
                *mut crate::__pgrx_c_bindings::ExplainState,
                *const ::core::ffi::c_char,
                *mut crate::__pgrx_c_bindings::ParamListInfoData,
                *mut crate::__pgrx_c_bindings::QueryEnvironment,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannedStmt,
                *mut crate::__pgrx_c_bindings::IntoClause,
                *mut crate::__pgrx_c_bindings::ExplainState,
                *const ::core::ffi::c_char,
                *mut crate::__pgrx_c_bindings::ParamListInfoData,
                *mut crate::__pgrx_c_bindings::QueryEnvironment,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__PlannedStmt__0p__0c__struct__IntoClause__0p__0c__struct__ExplainState__0p__0c__const__char__0p__0c__struct__ParamListInfoData__0p__0c__struct__QueryEnvironment__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_void__0l__struct__PlannedStmt__0p__0c__struct__IntoClause__0p__0c__struct__ExplainState__0p__0c__const__char__0p__0c__struct__ParamListInfoData__0p__0c__struct__QueryEnvironment__0p__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__PlannedStmt__0p__0c__struct__IntoClause__0p__0c__struct__ExplainState__0p__0c__const__char__0p__0c__struct__ParamListInfoData__0p__0c__struct__QueryEnvironment__0p__0r {
 type Physical = PhysicalFunction_C_unwind_6<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PlannedStmt, *mut crate::__pgrx_c_bindings::IntoClause, *mut crate::__pgrx_c_bindings::ExplainState, *const ::core::ffi::c_char, *mut crate::__pgrx_c_bindings::ParamListInfoData, *mut crate::__pgrx_c_bindings::QueryEnvironment) -> ()>>;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::QueryDesc, i32, u64) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::QueryDesc, i32, u64) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__QueryDesc__0p__0c__enum__ScanDirection__0c__unsigned__long__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed
    for Signature_void__0l__struct__QueryDesc__0p__0c__enum__ScanDirection__0c__unsigned__long__0r
{
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__QueryDesc__0p__0c__enum__ScanDirection__0c__unsigned__long__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::QueryDesc, i32, u64) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
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
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
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
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__WalReceiverConn__0p__0c__const__char__0p__0c__const__0x5f426f6f6c__0p__0c__const__0x5f426f6f6c__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_void__0l__struct__WalReceiverConn__0p__0c__const__char__0p__0c__const__0x5f426f6f6c__0p__0c__const__0x5f426f6f6c__0p__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__WalReceiverConn__0p__0c__const__char__0p__0c__const__0x5f426f6f6c__0p__0c__const__0x5f426f6f6c__0p__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::WalReceiverConn, *const ::core::ffi::c_char, *const bool, *const bool) -> ()>>;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::size_of::<*const bool>() == 8);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::align_of::<*const bool>() == 8);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::size_of::<*const bool>() == 8);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::align_of::<*const bool>() == 8);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<A0: c::expression::ImplicitTo<c::expression::CPointer<c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>, c::expression::ReadWrite>>, A1: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadOnly>>, A2: c::expression::ImplicitTo<c::expression::CPointer<c::CBool, c::expression::ReadOnly>>, A3: c::expression::ImplicitTo<c::expression::CPointer<c::CBool, c::expression::ReadOnly>>> c::expression::Call<(A0,A1,A2,A3,)> for Signature_void__0l__struct__WalReceiverConn__0p__0c__const__char__0p__0c__const__0x5f426f6f6c__0p__0c__const__0x5f426f6f6c__0p__0r {
 type Output = ();
 unsafe fn call(pointer: Self::Pointer, args: (A0,A1,A2,A3,)) -> Self::Output {
let function = pointer.expect("C indirect call requires a non-null function pointer");
const { assert!(!::core::mem::needs_drop::<A0>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::WalReceiverConn>());
let native0 = <c::expression::CPointer<c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>, c::expression::ReadWrite>, _>(args.0));
const { assert!(!::core::mem::needs_drop::<A1>()); }
const _: () = assert!(!::core::mem::needs_drop::<*const ::core::ffi::c_char>());
let native1 = <c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadOnly> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadOnly>, _>(args.1));
const { assert!(!::core::mem::needs_drop::<A2>()); }
const _: () = assert!(!::core::mem::needs_drop::<*const bool>());
let native2 = <c::expression::CPointer<c::CBool, c::expression::ReadOnly> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::CBool, c::expression::ReadOnly>, _>(args.2));
const { assert!(!::core::mem::needs_drop::<A3>()); }
const _: () = assert!(!::core::mem::needs_drop::<*const bool>());
let native3 = <c::expression::CPointer<c::CBool, c::expression::ReadOnly> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::CBool, c::expression::ReadOnly>, _>(args.3));
// SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
let result = unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0, native1, native2, native3)) };
result
}
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::HeapTupleData) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::HeapTupleData) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__const__struct__HeapTupleData__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_void__0l__const__struct__HeapTupleData__0p__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__const__struct__HeapTupleData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::HeapTupleData) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ExprState,
                *mut crate::__pgrx_c_bindings::ExprContext,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ExprState,
                *mut crate::__pgrx_c_bindings::ExprContext,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__ExprState__0p__0c__struct__ExprContext__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed
    for Signature_void__0l__struct__ExprState__0p__0c__struct__ExprContext__0p__0r
{
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__ExprState__0p__0c__struct__ExprContext__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ExprState,
                *mut crate::__pgrx_c_bindings::ExprContext,
            ) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::HeapTupleHeaderData) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::HeapTupleHeaderData) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__HeapTupleHeaderData__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_void__0l__struct__HeapTupleHeaderData__0p__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__HeapTupleHeaderData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::HeapTupleHeaderData) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::HeapTupleHeaderData,
                i32,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::HeapTupleHeaderData,
                i32,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__HeapTupleHeaderData__0p__0c__int__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_void__0l__struct__HeapTupleHeaderData__0p__0c__int__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__HeapTupleHeaderData__0p__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::HeapTupleHeaderData,
                i32,
            ) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::HeapTupleHeaderData,
                *mut crate::__pgrx_c_bindings::HeapTupleFreeze,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::HeapTupleHeaderData,
                *mut crate::__pgrx_c_bindings::HeapTupleFreeze,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__HeapTupleHeaderData__0p__0c__struct__HeapTupleFreeze__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed
    for Signature_void__0l__struct__HeapTupleHeaderData__0p__0c__struct__HeapTupleFreeze__0p__0r
{
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__HeapTupleHeaderData__0p__0c__struct__HeapTupleFreeze__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::HeapTupleHeaderData,
                *mut crate::__pgrx_c_bindings::HeapTupleFreeze,
            ) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::HeapTupleHeaderData,
                u32,
                bool,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::HeapTupleHeaderData,
                u32,
                bool,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__HeapTupleHeaderData__0p__0c__unsigned__int__0c__0x5f426f6f6c__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed
    for Signature_void__0l__struct__HeapTupleHeaderData__0p__0c__unsigned__int__0c__0x5f426f6f6c__0r
{
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__HeapTupleHeaderData__0p__0c__unsigned__int__0c__0x5f426f6f6c__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::HeapTupleHeaderData,
                u32,
                bool,
            ) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::MinimalTupleData) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::MinimalTupleData) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__MinimalTupleData__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_void__0l__struct__MinimalTupleData__0p__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__MinimalTupleData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::MinimalTupleData) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ReplicationSlot,
                i64,
                bool,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ReplicationSlot,
                i64,
                bool,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__ReplicationSlot__0p__0c__long__0c__0x5f426f6f6c__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed
    for Signature_void__0l__struct__ReplicationSlot__0p__0c__long__0c__0x5f426f6f6c__0r
{
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__ReplicationSlot__0p__0c__long__0c__0x5f426f6f6c__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ReplicationSlot,
                i64,
                bool,
            ) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::HeapTupleData,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::HeapTupleData,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0p__0l__const__struct__HeapTupleData__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_void__0p__0l__const__struct__HeapTupleData__0p__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0p__0l__const__struct__HeapTupleData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::HeapTupleData,
            ) -> *mut ::core::ffi::c_void,
        >,
    >;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TupleHashTableData,
                *mut crate::__pgrx_c_bindings::TupleHashEntryData,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TupleHashTableData,
                *mut crate::__pgrx_c_bindings::TupleHashEntryData,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0p__0l__struct__TupleHashTableData__0p__0c__struct__TupleHashEntryData__0p__0r;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_void__0p__0l__struct__TupleHashTableData__0p__0c__struct__TupleHashEntryData__0p__0r {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_void__0p__0l__struct__TupleHashTableData__0p__0c__struct__TupleHashEntryData__0p__0r {
 type Physical = PhysicalFunction_C_unwind_2<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TupleHashTableData, *mut crate::__pgrx_c_bindings::TupleHashEntryData) -> *mut ::core::ffi::c_void>>;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_current_relid;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::EquivalenceMemberIterator>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::EquivalenceMemberIterator>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::EquivalenceMemberIterator {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EquivalenceMemberIterator, current_relid)
        == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_current_relid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceMemberIterator>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::EquivalenceMemberIterator, <<c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceMemberIterator> as c::expression::OrdinaryField<Field_current_relid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).current_relid) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_current_list;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EquivalenceMemberIterator, current_list)
        == 32
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_current_list>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceMemberIterator>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::EquivalenceMemberIterator, <<c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceMemberIterator> as c::expression::OrdinaryField<Field_current_list>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).current_list) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::HASH_SEQ_STATUS>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::HASH_SEQ_STATUS>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_hasHashvalue;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HASH_SEQ_STATUS, hasHashvalue) == 24);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasHashvalue>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HASH_SEQ_STATUS>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HASH_SEQ_STATUS, <<c::expression::CRecord<crate::__pgrx_c_bindings::HASH_SEQ_STATUS> as c::expression::OrdinaryField<Field_hasHashvalue>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hasHashvalue) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HASH_SEQ_STATUS, hashvalue) == 28);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hashvalue>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HASH_SEQ_STATUS>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HASH_SEQ_STATUS, <<c::expression::CRecord<crate::__pgrx_c_bindings::HASH_SEQ_STATUS> as c::expression::OrdinaryField<Field_hashvalue>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hashvalue) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PMChild>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PMChild>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PMChild {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PMChild, pid) == 0);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PMChild>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PMChild,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PMChild> as c::expression::OrdinaryField<
        Field_pid,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pid) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_child_slot;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PMChild, child_slot) == 4);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_child_slot>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PMChild>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PMChild,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PMChild> as c::expression::OrdinaryField<
        Field_child_slot,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).child_slot) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_bkend_type;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PMChild, bkend_type) == 8);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bkend_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PMChild>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__BackendType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PMChild,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PMChild> as c::expression::OrdinaryField<
        Field_bkend_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).bkend_type) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_bgworker_notify;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PMChild, bgworker_notify) == 24);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bgworker_notify>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PMChild>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PMChild,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PMChild> as c::expression::OrdinaryField<
        Field_bgworker_notify,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).bgworker_notify) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Pg_abi_values>() == 52);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Pg_abi_values>() == 4);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::Pg_abi_values {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Pg_abi_values, version) == 0);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_version>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Pg_abi_values>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Pg_abi_values, <<c::expression::CRecord<crate::__pgrx_c_bindings::Pg_abi_values> as c::expression::OrdinaryField<Field_version>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).version) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Pg_abi_values, funcmaxargs) == 4);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_funcmaxargs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Pg_abi_values>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Pg_abi_values, <<c::expression::CRecord<crate::__pgrx_c_bindings::Pg_abi_values> as c::expression::OrdinaryField<Field_funcmaxargs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).funcmaxargs) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Pg_abi_values, indexmaxkeys) == 8);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indexmaxkeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Pg_abi_values>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Pg_abi_values, <<c::expression::CRecord<crate::__pgrx_c_bindings::Pg_abi_values> as c::expression::OrdinaryField<Field_indexmaxkeys>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indexmaxkeys) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Pg_abi_values, namedatalen) == 12);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_namedatalen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Pg_abi_values>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
