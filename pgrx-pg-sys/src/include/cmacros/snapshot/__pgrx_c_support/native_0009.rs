// Shared generated C macro support.

// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CreateTrigStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateTrigStmt> as c::expression::OrdinaryField<Field_args>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).args) };
#[doc(hidden)]
pub struct Field_row;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateTrigStmt, row) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_row>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateTrigStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CreateTrigStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateTrigStmt> as c::expression::OrdinaryField<Field_row>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).row) };
#[doc(hidden)]
pub struct Field_timing;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateTrigStmt, timing) == 42);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_timing>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateTrigStmt>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 42;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CreateTrigStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateTrigStmt> as c::expression::OrdinaryField<Field_timing>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).timing) };
#[doc(hidden)]
pub struct Field_events;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateTrigStmt, events) == 44);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_events>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateTrigStmt>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CreateTrigStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateTrigStmt> as c::expression::OrdinaryField<Field_events>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).events) };
#[doc(hidden)]
pub struct Field_columns;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateTrigStmt, columns) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_columns>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateTrigStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CreateTrigStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateTrigStmt> as c::expression::OrdinaryField<Field_columns>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).columns) };
#[doc(hidden)]
pub struct Field_transitionRels;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateTrigStmt, transitionRels) == 64
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_transitionRels>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateTrigStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CreateTrigStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateTrigStmt> as c::expression::OrdinaryField<Field_transitionRels>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).transitionRels) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateTrigStmt, deferrable) == 72);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_deferrable>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateTrigStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CreateTrigStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateTrigStmt> as c::expression::OrdinaryField<Field_deferrable>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).deferrable) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateTrigStmt, initdeferred) == 73);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_initdeferred>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateTrigStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 73;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CreateTrigStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateTrigStmt> as c::expression::OrdinaryField<Field_initdeferred>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).initdeferred) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CreateUserMappingStmt>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CreateUserMappingStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CreateUserMappingStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateUserMappingStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateUserMappingStmt>
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
const _: Projection<crate::__pgrx_c_bindings::CreateUserMappingStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateUserMappingStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateUserMappingStmt, if_not_exists) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_if_not_exists>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateUserMappingStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CreateUserMappingStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateUserMappingStmt> as c::expression::OrdinaryField<Field_if_not_exists>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).if_not_exists) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateUserMappingStmt, options) == 32
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateUserMappingStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CreateUserMappingStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateUserMappingStmt> as c::expression::OrdinaryField<Field_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
#[doc(hidden)]
pub struct Field_ctePlanId;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CteScan>() == 120);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CteScan>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CteScan {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CteScan, ctePlanId) == 112);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ctePlanId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CteScan>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::CteScan,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::CteScan> as c::expression::OrdinaryField<
        Field_ctePlanId,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ctePlanId) };
#[doc(hidden)]
pub struct Field_cteParam;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CteScan, cteParam) == 116);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cteParam>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CteScan>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 116;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::CteScan,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::CteScan> as c::expression::OrdinaryField<
        Field_cteParam,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cteParam) };
#[doc(hidden)]
pub struct Field_eflags;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CteScanState>() == 264);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CteScanState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CteScanState {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CteScanState, eflags) == 224);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_eflags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CteScanState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 224;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CteScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::CteScanState> as c::expression::OrdinaryField<Field_eflags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).eflags) };
#[doc(hidden)]
pub struct Field_readptr;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CteScanState, readptr) == 228);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_readptr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CteScanState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 228;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CteScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::CteScanState> as c::expression::OrdinaryField<Field_readptr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).readptr) };
#[doc(hidden)]
pub struct Field_eof_cte;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CteScanState, eof_cte) == 256);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_eof_cte>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CteScanState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 256;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CteScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::CteScanState> as c::expression::OrdinaryField<Field_eof_cte>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).eof_cte) };
#[doc(hidden)]
pub struct Field_cvarno;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CurrentOfExpr>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CurrentOfExpr>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CurrentOfExpr {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CurrentOfExpr, cvarno) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cvarno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CurrentOfExpr>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CurrentOfExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::CurrentOfExpr> as c::expression::OrdinaryField<Field_cvarno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cvarno) };
#[doc(hidden)]
pub struct Field_cursor_param;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CurrentOfExpr, cursor_param) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cursor_param>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CurrentOfExpr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CurrentOfExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::CurrentOfExpr> as c::expression::OrdinaryField<Field_cursor_param>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cursor_param) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CustomPath>() == 104);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CustomPath>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CustomPath {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CustomPath, flags) == 72);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CustomPath>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CustomPath, <<c::expression::CRecord<crate::__pgrx_c_bindings::CustomPath> as c::expression::OrdinaryField<Field_flags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).flags) };
#[doc(hidden)]
pub struct Field_custom_paths;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CustomPath, custom_paths) == 80);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_custom_paths>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CustomPath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CustomPath, <<c::expression::CRecord<crate::__pgrx_c_bindings::CustomPath> as c::expression::OrdinaryField<Field_custom_paths>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).custom_paths) };
#[doc(hidden)]
pub struct Field_custom_private;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CustomPath, custom_private) == 88);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_custom_private>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CustomPath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CustomPath, <<c::expression::CRecord<crate::__pgrx_c_bindings::CustomPath> as c::expression::OrdinaryField<Field_custom_private>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).custom_private) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CustomScan>() == 168);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CustomScan>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CustomScan {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CustomScan, flags) == 112);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CustomScan>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CustomScan, <<c::expression::CRecord<crate::__pgrx_c_bindings::CustomScan> as c::expression::OrdinaryField<Field_flags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).flags) };
#[doc(hidden)]
pub struct Field_custom_plans;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CustomScan, custom_plans) == 120);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_custom_plans>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CustomScan>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CustomScan, <<c::expression::CRecord<crate::__pgrx_c_bindings::CustomScan> as c::expression::OrdinaryField<Field_custom_plans>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).custom_plans) };
#[doc(hidden)]
pub struct Field_custom_exprs;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CustomScan, custom_exprs) == 128);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_custom_exprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CustomScan>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CustomScan, <<c::expression::CRecord<crate::__pgrx_c_bindings::CustomScan> as c::expression::OrdinaryField<Field_custom_exprs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).custom_exprs) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CustomScan, custom_private) == 136);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_custom_private>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CustomScan>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CustomScan, <<c::expression::CRecord<crate::__pgrx_c_bindings::CustomScan> as c::expression::OrdinaryField<Field_custom_private>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).custom_private) };
#[doc(hidden)]
pub struct Field_custom_scan_tlist;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CustomScan, custom_scan_tlist) == 144
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_custom_scan_tlist>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CustomScan>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CustomScan, <<c::expression::CRecord<crate::__pgrx_c_bindings::CustomScan> as c::expression::OrdinaryField<Field_custom_scan_tlist>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).custom_scan_tlist) };
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CustomScanState>() == 256);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CustomScanState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CustomScanState {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CustomScanState, flags) == 224);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CustomScanState>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 224;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CustomScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::CustomScanState> as c::expression::OrdinaryField<Field_flags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).flags) };
#[doc(hidden)]
pub struct Field_custom_ps;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CustomScanState, custom_ps) == 232);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_custom_ps>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CustomScanState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 232;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CustomScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::CustomScanState> as c::expression::OrdinaryField<Field_custom_ps>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).custom_ps) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CustomScanState, pscan_len) == 240);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pscan_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CustomScanState>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CustomScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::CustomScanState> as c::expression::OrdinaryField<Field_pscan_len>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pscan_len) };
#[doc(hidden)]
pub struct Field_datum_len_;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::DatumTupleFields>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::DatumTupleFields>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::DatumTupleFields {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DatumTupleFields, datum_len_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_datum_len_>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DatumTupleFields>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DatumTupleFields, <<c::expression::CRecord<crate::__pgrx_c_bindings::DatumTupleFields> as c::expression::OrdinaryField<Field_datum_len_>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).datum_len_) };
#[doc(hidden)]
pub struct Field_datum_typmod;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DatumTupleFields, datum_typmod) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_datum_typmod>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DatumTupleFields>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DatumTupleFields, <<c::expression::CRecord<crate::__pgrx_c_bindings::DatumTupleFields> as c::expression::OrdinaryField<Field_datum_typmod>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).datum_typmod) };
#[doc(hidden)]
pub struct Field_datum_typeid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DatumTupleFields, datum_typeid) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_datum_typeid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DatumTupleFields>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DatumTupleFields, <<c::expression::CRecord<crate::__pgrx_c_bindings::DatumTupleFields> as c::expression::OrdinaryField<Field_datum_typeid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).datum_typeid) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::DeallocateStmt>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::DeallocateStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::DeallocateStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DeallocateStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DeallocateStmt>
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
const _: Projection<crate::__pgrx_c_bindings::DeallocateStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::DeallocateStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::DeclareCursorStmt>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::DeclareCursorStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::DeclareCursorStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DeclareCursorStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DeclareCursorStmt>
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
const _: Projection<crate::__pgrx_c_bindings::DeclareCursorStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::DeclareCursorStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DeclareCursorStmt, options) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DeclareCursorStmt>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DeclareCursorStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::DeclareCursorStmt> as c::expression::OrdinaryField<Field_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::DecodedXLogRecord>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::DecodedXLogRecord>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::DecodedXLogRecord {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodedXLogRecord, size) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodedXLogRecord>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodedXLogRecord, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodedXLogRecord> as c::expression::OrdinaryField<Field_size>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).size) };
#[doc(hidden)]
pub struct Field_oversized;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodedXLogRecord, oversized) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oversized>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodedXLogRecord>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodedXLogRecord, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodedXLogRecord> as c::expression::OrdinaryField<Field_oversized>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oversized) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodedXLogRecord, lsn) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodedXLogRecord>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodedXLogRecord, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodedXLogRecord> as c::expression::OrdinaryField<Field_lsn>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lsn) };
#[doc(hidden)]
pub struct Field_next_lsn;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodedXLogRecord, next_lsn) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_next_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodedXLogRecord>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodedXLogRecord, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodedXLogRecord> as c::expression::OrdinaryField<Field_next_lsn>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).next_lsn) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodedXLogRecord, header) == 40);
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::XLogRecord>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::XLogRecord>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_header>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodedXLogRecord>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecord>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodedXLogRecord, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodedXLogRecord> as c::expression::OrdinaryField<Field_header>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).header) };
#[doc(hidden)]
pub struct Field_record_origin;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodedXLogRecord, record_origin) == 64
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_record_origin>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodedXLogRecord>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodedXLogRecord, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodedXLogRecord> as c::expression::OrdinaryField<Field_record_origin>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).record_origin) };
#[doc(hidden)]
pub struct Field_toplevel_xid;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodedXLogRecord, toplevel_xid) == 68
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_toplevel_xid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodedXLogRecord>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 68;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodedXLogRecord, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodedXLogRecord> as c::expression::OrdinaryField<Field_toplevel_xid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).toplevel_xid) };
#[doc(hidden)]
pub struct Field_main_data;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodedXLogRecord, main_data) == 72);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_main_data>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodedXLogRecord>
{
    type Member = c::expression::CPointer<
        c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodedXLogRecord, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodedXLogRecord> as c::expression::OrdinaryField<Field_main_data>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).main_data) };
#[doc(hidden)]
pub struct Field_main_data_len;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodedXLogRecord, main_data_len) == 80
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_main_data_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodedXLogRecord>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodedXLogRecord, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodedXLogRecord> as c::expression::OrdinaryField<Field_main_data_len>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).main_data_len) };
#[doc(hidden)]
pub struct Field_max_block_id;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodedXLogRecord, max_block_id) == 84
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_max_block_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodedXLogRecord>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 84;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodedXLogRecord, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodedXLogRecord> as c::expression::OrdinaryField<Field_max_block_id>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).max_block_id) };
#[doc(hidden)]
pub struct Field_blocks;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodedXLogRecord, blocks) == 88);
const _: () = {
    assert!(
        ::core::mem::size_of::<
            crate::__pgrx_c_bindings::__IncompleteArrayField<
                crate::__pgrx_c_bindings::DecodedBkpBlock,
            >,
        >() == 0
    );
    assert!(
        ::core::mem::align_of::<
            crate::__pgrx_c_bindings::__IncompleteArrayField<
                crate::__pgrx_c_bindings::DecodedBkpBlock,
            >,
        >() == 8
    );
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_blocks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodedXLogRecord>
{
    type Member = c::expression::CFlexibleArray<
        c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock>,
        crate::__pgrx_c_bindings::__IncompleteArrayField<crate::__pgrx_c_bindings::DecodedBkpBlock>,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodedXLogRecord, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodedXLogRecord> as c::expression::OrdinaryField<Field_blocks>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).blocks) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::DefElem>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::DefElem>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::DefElem {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DefElem, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DefElem>
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
    crate::__pgrx_c_bindings::DefElem,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::DefElem> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_defaction;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DefElem, defaction) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_defaction>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DefElem>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__DefElemAction,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::DefElem,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::DefElem> as c::expression::OrdinaryField<
        Field_defaction,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).defaction) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DefElem, location) == 36);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DefElem>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::DefElem,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::DefElem> as c::expression::OrdinaryField<
        Field_location,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::DefineStmt>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::DefineStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::DefineStmt {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DefineStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DefineStmt>
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
const _: Projection<crate::__pgrx_c_bindings::DefineStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::DefineStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DefineStmt, kind) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_kind>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DefineStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ObjectType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DefineStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::DefineStmt> as c::expression::OrdinaryField<Field_kind>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).kind) };
#[doc(hidden)]
pub struct Field_oldstyle;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DefineStmt, oldstyle) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldstyle>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DefineStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DefineStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::DefineStmt> as c::expression::OrdinaryField<Field_oldstyle>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oldstyle) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DefineStmt, defnames) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_defnames>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DefineStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DefineStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::DefineStmt> as c::expression::OrdinaryField<Field_defnames>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).defnames) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DefineStmt, args) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_args>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DefineStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DefineStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::DefineStmt> as c::expression::OrdinaryField<Field_args>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).args) };
#[doc(hidden)]
pub struct Field_definition;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DefineStmt, definition) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_definition>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DefineStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DefineStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::DefineStmt> as c::expression::OrdinaryField<Field_definition>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).definition) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DefineStmt, if_not_exists) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_if_not_exists>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DefineStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DefineStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::DefineStmt> as c::expression::OrdinaryField<Field_if_not_exists>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).if_not_exists) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DefineStmt, replace) == 41);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_replace>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DefineStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 41;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DefineStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::DefineStmt> as c::expression::OrdinaryField<Field_replace>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).replace) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::DeleteStmt>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::DeleteStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::DeleteStmt {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DeleteStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DeleteStmt>
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
const _: Projection<crate::__pgrx_c_bindings::DeleteStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::DeleteStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_usingClause;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DeleteStmt, usingClause) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_usingClause>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DeleteStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DeleteStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::DeleteStmt> as c::expression::OrdinaryField<Field_usingClause>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).usingClause) };
#[doc(hidden)]
pub struct Field_returningList;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DeleteStmt, returningList) == 32);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_returningList>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DeleteStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DeleteStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::DeleteStmt> as c::expression::OrdinaryField<Field_returningList>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).returningList) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::DimensionInfo>() == 20);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::DimensionInfo>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::DimensionInfo {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DimensionInfo, nvalues) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nvalues>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DimensionInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DimensionInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::DimensionInfo> as c::expression::OrdinaryField<Field_nvalues>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nvalues) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DimensionInfo, nbytes) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nbytes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DimensionInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DimensionInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::DimensionInfo> as c::expression::OrdinaryField<Field_nbytes>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nbytes) };
#[doc(hidden)]
pub struct Field_nbytes_aligned;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DimensionInfo, nbytes_aligned) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nbytes_aligned>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DimensionInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DimensionInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::DimensionInfo> as c::expression::OrdinaryField<Field_nbytes_aligned>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nbytes_aligned) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DimensionInfo, typlen) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typlen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DimensionInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DimensionInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::DimensionInfo> as c::expression::OrdinaryField<Field_typlen>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typlen) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DimensionInfo, typbyval) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typbyval>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DimensionInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DimensionInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::DimensionInfo> as c::expression::OrdinaryField<Field_typbyval>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typbyval) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::DiscardStmt>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::DiscardStmt>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::DiscardStmt {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DiscardStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DiscardStmt>
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
const _: Projection<crate::__pgrx_c_bindings::DiscardStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::DiscardStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DiscardStmt, target) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_target>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DiscardStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__DiscardMode,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DiscardStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::DiscardStmt> as c::expression::OrdinaryField<Field_target>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).target) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::DoStmt>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::DoStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::DoStmt {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DoStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DoStmt>
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
    crate::__pgrx_c_bindings::DoStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::DoStmt> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DoStmt, args) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_args>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DoStmt>
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
    crate::__pgrx_c_bindings::DoStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::DoStmt> as c::expression::OrdinaryField<
        Field_args,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).args) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::DomainConstraintRef>() == 64);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::DomainConstraintRef>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::DomainConstraintRef {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::DomainConstraintRef, constraints) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_constraints>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DomainConstraintRef>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DomainConstraintRef, <<c::expression::CRecord<crate::__pgrx_c_bindings::DomainConstraintRef> as c::expression::OrdinaryField<Field_constraints>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).constraints) };
#[doc(hidden)]
pub struct Field_need_exprstate;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::DomainConstraintRef, need_exprstate) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_need_exprstate>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DomainConstraintRef>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DomainConstraintRef, <<c::expression::CRecord<crate::__pgrx_c_bindings::DomainConstraintRef> as c::expression::OrdinaryField<Field_need_exprstate>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).need_exprstate) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::DomainConstraintState>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::DomainConstraintState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::DomainConstraintState {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DomainConstraintState, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DomainConstraintState>
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
const _: Projection<crate::__pgrx_c_bindings::DomainConstraintState, <<c::expression::CRecord<crate::__pgrx_c_bindings::DomainConstraintState> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_constrainttype;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::DomainConstraintState, constrainttype) == 4
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_constrainttype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DomainConstraintState>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__DomainConstraintType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DomainConstraintState, <<c::expression::CRecord<crate::__pgrx_c_bindings::DomainConstraintState> as c::expression::OrdinaryField<Field_constrainttype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).constrainttype) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::DropOwnedStmt>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::DropOwnedStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::DropOwnedStmt {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DropOwnedStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DropOwnedStmt>
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
const _: Projection<crate::__pgrx_c_bindings::DropOwnedStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::DropOwnedStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DropOwnedStmt, roles) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_roles>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DropOwnedStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DropOwnedStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::DropOwnedStmt> as c::expression::OrdinaryField<Field_roles>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).roles) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DropOwnedStmt, behavior) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_behavior>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DropOwnedStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__DropBehavior,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DropOwnedStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::DropOwnedStmt> as c::expression::OrdinaryField<Field_behavior>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).behavior) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::DropReplicationSlotCmd>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::DropReplicationSlotCmd>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::DropReplicationSlotCmd {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DropReplicationSlotCmd, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DropReplicationSlotCmd>
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
const _: Projection<crate::__pgrx_c_bindings::DropReplicationSlotCmd, <<c::expression::CRecord<crate::__pgrx_c_bindings::DropReplicationSlotCmd> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_wait;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DropReplicationSlotCmd, wait) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wait>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DropReplicationSlotCmd>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DropReplicationSlotCmd, <<c::expression::CRecord<crate::__pgrx_c_bindings::DropReplicationSlotCmd> as c::expression::OrdinaryField<Field_wait>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wait) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::DropRoleStmt>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::DropRoleStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::DropRoleStmt {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DropRoleStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DropRoleStmt>
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
const _: Projection<crate::__pgrx_c_bindings::DropRoleStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::DropRoleStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DropRoleStmt, roles) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_roles>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DropRoleStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DropRoleStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::DropRoleStmt> as c::expression::OrdinaryField<Field_roles>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).roles) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DropRoleStmt, missing_ok) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_missing_ok>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DropRoleStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DropRoleStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::DropRoleStmt> as c::expression::OrdinaryField<Field_missing_ok>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).missing_ok) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::DropStmt>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::DropStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::DropStmt {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DropStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DropStmt>
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
    crate::__pgrx_c_bindings::DropStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::DropStmt> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DropStmt, objects) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_objects>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DropStmt>
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
    crate::__pgrx_c_bindings::DropStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::DropStmt> as c::expression::OrdinaryField<
        Field_objects,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).objects) };
#[doc(hidden)]
pub struct Field_removeType;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DropStmt, removeType) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_removeType>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DropStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ObjectType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::DropStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::DropStmt> as c::expression::OrdinaryField<
        Field_removeType,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).removeType) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DropStmt, behavior) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_behavior>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DropStmt>
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
const _: Projection<
    crate::__pgrx_c_bindings::DropStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::DropStmt> as c::expression::OrdinaryField<
        Field_behavior,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).behavior) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DropStmt, missing_ok) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_missing_ok>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DropStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::DropStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::DropStmt> as c::expression::OrdinaryField<
        Field_missing_ok,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).missing_ok) };
#[doc(hidden)]
pub struct Field_concurrent;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DropStmt, concurrent) == 25);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_concurrent>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DropStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 25;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::DropStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::DropStmt> as c::expression::OrdinaryField<
        Field_concurrent,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).concurrent) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::DropSubscriptionStmt>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::DropSubscriptionStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::DropSubscriptionStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DropSubscriptionStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DropSubscriptionStmt>
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
const _: Projection<crate::__pgrx_c_bindings::DropSubscriptionStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::DropSubscriptionStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::DropSubscriptionStmt, missing_ok) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_missing_ok>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DropSubscriptionStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DropSubscriptionStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::DropSubscriptionStmt> as c::expression::OrdinaryField<Field_missing_ok>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).missing_ok) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::DropSubscriptionStmt, behavior) == 20
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_behavior>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DropSubscriptionStmt>
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
const _: Projection<crate::__pgrx_c_bindings::DropSubscriptionStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::DropSubscriptionStmt> as c::expression::OrdinaryField<Field_behavior>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).behavior) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::DropTableSpaceStmt>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::DropTableSpaceStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::DropTableSpaceStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DropTableSpaceStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DropTableSpaceStmt>
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
const _: Projection<crate::__pgrx_c_bindings::DropTableSpaceStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::DropTableSpaceStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::DropTableSpaceStmt, missing_ok) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_missing_ok>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DropTableSpaceStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DropTableSpaceStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::DropTableSpaceStmt> as c::expression::OrdinaryField<Field_missing_ok>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).missing_ok) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::DropUserMappingStmt>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::DropUserMappingStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::DropUserMappingStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DropUserMappingStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DropUserMappingStmt>
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
const _: Projection<crate::__pgrx_c_bindings::DropUserMappingStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::DropUserMappingStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::DropUserMappingStmt, missing_ok) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_missing_ok>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DropUserMappingStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DropUserMappingStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::DropUserMappingStmt> as c::expression::OrdinaryField<Field_missing_ok>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).missing_ok) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::DropdbStmt>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::DropdbStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::DropdbStmt {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DropdbStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DropdbStmt>
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
const _: Projection<crate::__pgrx_c_bindings::DropdbStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::DropdbStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DropdbStmt, missing_ok) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_missing_ok>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DropdbStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DropdbStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::DropdbStmt> as c::expression::OrdinaryField<Field_missing_ok>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).missing_ok) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DropdbStmt, options) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DropdbStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DropdbStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::DropdbStmt> as c::expression::OrdinaryField<Field_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
#[doc(hidden)]
pub struct Field_epqParam;
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::EPQState>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::EPQState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::EPQState {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EPQState, epqParam) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_epqParam>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EPQState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::EPQState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::EPQState> as c::expression::OrdinaryField<
        Field_epqParam,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).epqParam) };
#[doc(hidden)]
pub struct Field_arowMarks;
#[cfg(feature = "pg15")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EPQState, arowMarks) == 40);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_arowMarks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EPQState>
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
    crate::__pgrx_c_bindings::EPQState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::EPQState> as c::expression::OrdinaryField<
        Field_arowMarks,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).arowMarks) };
#[doc(hidden)]
pub struct Field_origslot;
#[cfg(feature = "pg15")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EPQState, origslot) == 48);
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::TupleTableSlot>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::TupleTableSlot>() == 8);
};
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_origslot>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EPQState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::EPQState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::EPQState> as c::expression::OrdinaryField<
        Field_origslot,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).origslot) };
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::EState>() == 296);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::EState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::EState {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
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
    crate::__pgrx_c_bindings::EState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::EState> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_es_direction;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_direction) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_direction>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ScanDirection,
        c::CInt,
        i32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::EState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::EState> as c::expression::OrdinaryField<
        Field_es_direction,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).es_direction) };
#[doc(hidden)]
pub struct Field_es_range_table;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_range_table) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_range_table>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
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
    crate::__pgrx_c_bindings::EState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::EState> as c::expression::OrdinaryField<
        Field_es_range_table,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).es_range_table) };
#[doc(hidden)]
pub struct Field_es_range_table_size;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_range_table_size) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_range_table_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::EState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::EState> as c::expression::OrdinaryField<
        Field_es_range_table_size,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).es_range_table_size) };
#[doc(hidden)]
pub struct Field_es_output_cid;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_output_cid) == 80);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_output_cid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::EState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::EState> as c::expression::OrdinaryField<
        Field_es_output_cid,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).es_output_cid) };
#[doc(hidden)]
pub struct Field_es_opened_result_relations;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_opened_result_relations) == 96
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_opened_result_relations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::EState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::EState> as c::expression::OrdinaryField<
        Field_es_opened_result_relations,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).es_opened_result_relations) };
#[doc(hidden)]
pub struct Field_es_tuple_routing_result_relations;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_tuple_routing_result_relations)
        == 112
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_tuple_routing_result_relations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
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
    crate::__pgrx_c_bindings::EState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::EState> as c::expression::OrdinaryField<
        Field_es_tuple_routing_result_relations,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).es_tuple_routing_result_relations) };
#[doc(hidden)]
pub struct Field_es_trig_target_relations;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_trig_target_relations) == 120
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_trig_target_relations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::EState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::EState> as c::expression::OrdinaryField<
        Field_es_trig_target_relations,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).es_trig_target_relations) };
#[doc(hidden)]
pub struct Field_es_tupleTable;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_tupleTable) == 160);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_tupleTable>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::EState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::EState> as c::expression::OrdinaryField<
        Field_es_tupleTable,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).es_tupleTable) };
#[doc(hidden)]
pub struct Field_es_processed;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_processed) == 168);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_processed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 168;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::EState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::EState> as c::expression::OrdinaryField<
        Field_es_processed,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).es_processed) };
#[doc(hidden)]
pub struct Field_es_top_eflags;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_top_eflags) == 176);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_top_eflags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 176;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::EState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::EState> as c::expression::OrdinaryField<
        Field_es_top_eflags,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).es_top_eflags) };
#[doc(hidden)]
pub struct Field_es_instrument;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_instrument) == 180);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_instrument>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 180;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::EState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::EState> as c::expression::OrdinaryField<
        Field_es_instrument,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).es_instrument) };
#[doc(hidden)]
pub struct Field_es_finished;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_finished) == 184);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_finished>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 184;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::EState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::EState> as c::expression::OrdinaryField<
        Field_es_finished,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).es_finished) };
#[doc(hidden)]
pub struct Field_es_exprcontexts;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_exprcontexts) == 192);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_exprcontexts>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 192;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::EState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::EState> as c::expression::OrdinaryField<
        Field_es_exprcontexts,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).es_exprcontexts) };
#[doc(hidden)]
pub struct Field_es_subplanstates;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_subplanstates) == 200);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_subplanstates>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 200;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::EState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::EState> as c::expression::OrdinaryField<
        Field_es_subplanstates,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).es_subplanstates) };
#[doc(hidden)]
pub struct Field_es_auxmodifytables;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_auxmodifytables) == 208);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_auxmodifytables>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 208;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::EState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::EState> as c::expression::OrdinaryField<
        Field_es_auxmodifytables,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).es_auxmodifytables) };
#[doc(hidden)]
pub struct Field_es_per_tuple_exprcontext;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_per_tuple_exprcontext) == 216
);
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::ExprContext>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::ExprContext>() == 8);
};
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_per_tuple_exprcontext>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::ExprContext>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 216;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::EState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::EState> as c::expression::OrdinaryField<
        Field_es_per_tuple_exprcontext,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).es_per_tuple_exprcontext) };
#[doc(hidden)]
pub struct Field_es_use_parallel_mode;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_use_parallel_mode) == 232);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_use_parallel_mode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 232;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::EState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::EState> as c::expression::OrdinaryField<
        Field_es_use_parallel_mode,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).es_use_parallel_mode) };
#[doc(hidden)]
pub struct Field_es_jit_flags;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_jit_flags) == 248);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_jit_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 248;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::EState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::EState> as c::expression::OrdinaryField<
        Field_es_jit_flags,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).es_jit_flags) };
#[doc(hidden)]
pub struct Field_es_insert_pending_result_relations;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_insert_pending_result_relations)
        == 272
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_insert_pending_result_relations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 272;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::EState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::EState> as c::expression::OrdinaryField<
        Field_es_insert_pending_result_relations,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).es_insert_pending_result_relations) };
#[doc(hidden)]
pub struct Field_es_insert_pending_modifytables;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_insert_pending_modifytables)
        == 280
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_insert_pending_modifytables>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 280;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::EState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::EState> as c::expression::OrdinaryField<
        Field_es_insert_pending_modifytables,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).es_insert_pending_modifytables) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_es_resultrelinfo_extra;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EState, es_resultrelinfo_extra) == 288
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_es_resultrelinfo_extra>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 288;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::EState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::EState> as c::expression::OrdinaryField<
        Field_es_resultrelinfo_extra,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).es_resultrelinfo_extra) };
#[doc(hidden)]
pub struct Field_total_edges;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Edge>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Edge>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::Edge {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Edge, total_edges) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_total_edges>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Edge>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Edge,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Edge> as c::expression::OrdinaryField<
        Field_total_edges,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).total_edges) };
#[doc(hidden)]
pub struct Field_unused_edges;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Edge, unused_edges) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_unused_edges>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Edge>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Edge,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Edge> as c::expression::OrdinaryField<
        Field_unused_edges,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).unused_edges) };
#[doc(hidden)]
pub struct Field_reliddesc;
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::EphemeralNamedRelationMetadataData>()
            == 40
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::EphemeralNamedRelationMetadataData>()
            == 8
    );
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::EphemeralNamedRelationMetadataData {}
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::EphemeralNamedRelationMetadataData,
        reliddesc
    ) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_reliddesc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EphemeralNamedRelationMetadataData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::EphemeralNamedRelationMetadataData, <<c::expression::CRecord<crate::__pgrx_c_bindings::EphemeralNamedRelationMetadataData> as c::expression::OrdinaryField<Field_reliddesc>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).reliddesc) };
#[doc(hidden)]
pub struct Field_enrtype;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EphemeralNamedRelationMetadataData, enrtype)
        == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_enrtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EphemeralNamedRelationMetadataData>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__EphemeralNameRelationType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::EphemeralNamedRelationMetadataData, <<c::expression::CRecord<crate::__pgrx_c_bindings::EphemeralNamedRelationMetadataData> as c::expression::OrdinaryField<Field_enrtype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).enrtype) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::EquivalenceClass>() == 80);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::EquivalenceClass>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::EquivalenceClass {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EquivalenceClass, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceClass>
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
const _: Projection<crate::__pgrx_c_bindings::EquivalenceClass, <<c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceClass> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_ec_opfamilies;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EquivalenceClass, ec_opfamilies) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ec_opfamilies>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceClass>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::EquivalenceClass, <<c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceClass> as c::expression::OrdinaryField<Field_ec_opfamilies>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ec_opfamilies) };
#[doc(hidden)]
pub struct Field_ec_collation;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EquivalenceClass, ec_collation) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ec_collation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceClass>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::EquivalenceClass, <<c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceClass> as c::expression::OrdinaryField<Field_ec_collation>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ec_collation) };
#[doc(hidden)]
pub struct Field_ec_members;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EquivalenceClass, ec_members) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ec_members>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceClass>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::EquivalenceClass, <<c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceClass> as c::expression::OrdinaryField<Field_ec_members>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ec_members) };
#[doc(hidden)]
pub struct Field_ec_sources;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EquivalenceClass, ec_sources) == 32);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ec_sources>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceClass>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::EquivalenceClass, <<c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceClass> as c::expression::OrdinaryField<Field_ec_sources>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ec_sources) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
#[doc(hidden)]
pub struct Field_ec_derives;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EquivalenceClass, ec_derives) == 40);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ec_derives>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceClass>
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
const _: Projection<crate::__pgrx_c_bindings::EquivalenceClass, <<c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceClass> as c::expression::OrdinaryField<Field_ec_derives>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ec_derives) };
#[doc(hidden)]
pub struct Field_ec_has_const;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EquivalenceClass, ec_has_const) == 56
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ec_has_const>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceClass>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::EquivalenceClass, <<c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceClass> as c::expression::OrdinaryField<Field_ec_has_const>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ec_has_const) };
#[doc(hidden)]
pub struct Field_ec_has_volatile;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EquivalenceClass, ec_has_volatile) == 57
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ec_has_volatile>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceClass>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 57;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::EquivalenceClass, <<c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceClass> as c::expression::OrdinaryField<Field_ec_has_volatile>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ec_has_volatile) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_ec_below_outer_join;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EquivalenceClass, ec_below_outer_join) == 58
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ec_below_outer_join>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceClass>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 58;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::EquivalenceClass, <<c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceClass> as c::expression::OrdinaryField<Field_ec_below_outer_join>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ec_below_outer_join) };
#[doc(hidden)]
pub struct Field_ec_broken;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EquivalenceClass, ec_broken) == 59);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ec_broken>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceClass>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 59;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::EquivalenceClass, <<c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceClass> as c::expression::OrdinaryField<Field_ec_broken>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ec_broken) };
#[doc(hidden)]
pub struct Field_ec_sortref;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EquivalenceClass, ec_sortref) == 60);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ec_sortref>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceClass>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 60;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::EquivalenceClass, <<c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceClass> as c::expression::OrdinaryField<Field_ec_sortref>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ec_sortref) };
#[doc(hidden)]
pub struct Field_ec_min_security;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EquivalenceClass, ec_min_security) == 64
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ec_min_security>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceClass>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::EquivalenceClass, <<c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceClass> as c::expression::OrdinaryField<Field_ec_min_security>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ec_min_security) };
#[doc(hidden)]
pub struct Field_ec_max_security;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EquivalenceClass, ec_max_security) == 68
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ec_max_security>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceClass>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 68;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::EquivalenceClass, <<c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceClass> as c::expression::OrdinaryField<Field_ec_max_security>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ec_max_security) };
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::EquivalenceMember>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::EquivalenceMember>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::EquivalenceMember {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EquivalenceMember, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceMember>
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
const _: Projection<crate::__pgrx_c_bindings::EquivalenceMember, <<c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceMember> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_em_is_const;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EquivalenceMember, em_is_const) == 32
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_em_is_const>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceMember>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::EquivalenceMember, <<c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceMember> as c::expression::OrdinaryField<Field_em_is_const>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).em_is_const) };
#[doc(hidden)]
pub struct Field_em_is_child;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EquivalenceMember, em_is_child) == 33
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_em_is_child>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceMember>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 33;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::EquivalenceMember, <<c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceMember> as c::expression::OrdinaryField<Field_em_is_child>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).em_is_child) };
#[doc(hidden)]
pub struct Field_em_datatype;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EquivalenceMember, em_datatype) == 36
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_em_datatype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceMember>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::EquivalenceMember, <<c::expression::CRecord<crate::__pgrx_c_bindings::EquivalenceMember> as c::expression::OrdinaryField<Field_em_datatype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).em_datatype) };
#[doc(hidden)]
pub struct Field_elevel;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ErrorData>() == 184);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ErrorData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ErrorData {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ErrorData, elevel) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_elevel>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ErrorData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ErrorData,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ErrorData> as c::expression::OrdinaryField<
        Field_elevel,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).elevel) };
#[doc(hidden)]
pub struct Field_output_to_server;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ErrorData, output_to_server) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_output_to_server>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ErrorData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ErrorData,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ErrorData> as c::expression::OrdinaryField<
        Field_output_to_server,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).output_to_server) };
#[doc(hidden)]
pub struct Field_output_to_client;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ErrorData, output_to_client) == 5);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_output_to_client>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ErrorData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 5;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ErrorData,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ErrorData> as c::expression::OrdinaryField<
        Field_output_to_client,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).output_to_client) };
#[doc(hidden)]
pub struct Field_hide_stmt;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ErrorData, hide_stmt) == 6);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hide_stmt>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ErrorData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 6;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ErrorData,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ErrorData> as c::expression::OrdinaryField<
        Field_hide_stmt,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hide_stmt) };
#[doc(hidden)]
pub struct Field_hide_ctx;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ErrorData, hide_ctx) == 7);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hide_ctx>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ErrorData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 7;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ErrorData,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ErrorData> as c::expression::OrdinaryField<
        Field_hide_ctx,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hide_ctx) };
#[doc(hidden)]
pub struct Field_lineno;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ErrorData, lineno) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lineno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ErrorData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ErrorData,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ErrorData> as c::expression::OrdinaryField<
        Field_lineno,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lineno) };
#[doc(hidden)]
pub struct Field_sqlerrcode;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ErrorData, sqlerrcode) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sqlerrcode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ErrorData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ErrorData,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ErrorData> as c::expression::OrdinaryField<
        Field_sqlerrcode,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sqlerrcode) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ErrorData, context) == 88);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_context>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ErrorData>
{
    type Member = c::expression::CPointer<
        c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ErrorData,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ErrorData> as c::expression::OrdinaryField<
        Field_context,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).context) };
#[doc(hidden)]
pub struct Field_cursorpos;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ErrorData, cursorpos) == 152);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cursorpos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ErrorData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ErrorData,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ErrorData> as c::expression::OrdinaryField<
        Field_cursorpos,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cursorpos) };
#[doc(hidden)]
pub struct Field_internalpos;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ErrorData, internalpos) == 156);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_internalpos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ErrorData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 156;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ErrorData,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ErrorData> as c::expression::OrdinaryField<
        Field_internalpos,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).internalpos) };
#[doc(hidden)]
pub struct Field_saved_errno;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ErrorData, saved_errno) == 168);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_saved_errno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ErrorData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 168;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ErrorData,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ErrorData> as c::expression::OrdinaryField<
        Field_saved_errno,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).saved_errno) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::EstimationInfo>() == 4);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::EstimationInfo>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::EstimationInfo {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EstimationInfo, flags) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EstimationInfo>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::EstimationInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::EstimationInfo> as c::expression::OrdinaryField<Field_flags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).flags) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::EventTriggerData>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::EventTriggerData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::EventTriggerData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EventTriggerData, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EventTriggerData>
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
const _: Projection<crate::__pgrx_c_bindings::EventTriggerData, <<c::expression::CRecord<crate::__pgrx_c_bindings::EventTriggerData> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EventTriggerData, tag) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tag>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EventTriggerData>
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
const _: Projection<crate::__pgrx_c_bindings::EventTriggerData, <<c::expression::CRecord<crate::__pgrx_c_bindings::EventTriggerData> as c::expression::OrdinaryField<Field_tag>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tag) };
#[doc(hidden)]
pub struct Field_ctidAttNo;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ExecAuxRowMark>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ExecAuxRowMark>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ExecAuxRowMark {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExecAuxRowMark, ctidAttNo) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ctidAttNo>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExecAuxRowMark>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExecAuxRowMark, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExecAuxRowMark> as c::expression::OrdinaryField<Field_ctidAttNo>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ctidAttNo) };
#[doc(hidden)]
pub struct Field_toidAttNo;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExecAuxRowMark, toidAttNo) == 10);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_toidAttNo>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExecAuxRowMark>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 10;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExecAuxRowMark, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExecAuxRowMark> as c::expression::OrdinaryField<Field_toidAttNo>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).toidAttNo) };
#[doc(hidden)]
pub struct Field_wholeAttNo;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExecAuxRowMark, wholeAttNo) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wholeAttNo>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExecAuxRowMark>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExecAuxRowMark, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExecAuxRowMark> as c::expression::OrdinaryField<Field_wholeAttNo>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wholeAttNo) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ExecPhraseData>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ExecPhraseData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ExecPhraseData {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExecPhraseData, npos) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_npos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExecPhraseData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExecPhraseData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExecPhraseData> as c::expression::OrdinaryField<Field_npos>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).npos) };
#[doc(hidden)]
pub struct Field_allocated;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExecPhraseData, allocated) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_allocated>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExecPhraseData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExecPhraseData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExecPhraseData> as c::expression::OrdinaryField<Field_allocated>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).allocated) };
#[doc(hidden)]
pub struct Field_negate;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExecPhraseData, negate) == 5);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_negate>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExecPhraseData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 5;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExecPhraseData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExecPhraseData> as c::expression::OrdinaryField<Field_negate>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).negate) };
#[doc(hidden)]
pub struct Field_width;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExecPhraseData, width) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_width>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExecPhraseData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExecPhraseData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExecPhraseData> as c::expression::OrdinaryField<Field_width>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).width) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ExecRowMark>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ExecRowMark>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ExecRowMark {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExecRowMark, relid) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExecRowMark>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExecRowMark, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExecRowMark> as c::expression::OrdinaryField<Field_relid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relid) };
#[doc(hidden)]
pub struct Field_rti;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExecRowMark, rti) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rti>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExecRowMark>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExecRowMark, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExecRowMark> as c::expression::OrdinaryField<Field_rti>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rti) };
#[doc(hidden)]
pub struct Field_prti;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExecRowMark, prti) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_prti>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExecRowMark>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExecRowMark, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExecRowMark> as c::expression::OrdinaryField<Field_prti>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).prti) };
#[doc(hidden)]
pub struct Field_rowmarkId;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExecRowMark, rowmarkId) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rowmarkId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExecRowMark>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExecRowMark, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExecRowMark> as c::expression::OrdinaryField<Field_rowmarkId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rowmarkId) };
#[doc(hidden)]
pub struct Field_markType;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExecRowMark, markType) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_markType>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExecRowMark>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__RowMarkType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExecRowMark, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExecRowMark> as c::expression::OrdinaryField<Field_markType>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).markType) };
#[doc(hidden)]
pub struct Field_strength;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExecRowMark, strength) == 28);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_strength>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExecRowMark>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__LockClauseStrength,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExecRowMark, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExecRowMark> as c::expression::OrdinaryField<Field_strength>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).strength) };
#[doc(hidden)]
pub struct Field_waitPolicy;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExecRowMark, waitPolicy) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_waitPolicy>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExecRowMark>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__LockWaitPolicy,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExecRowMark, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExecRowMark> as c::expression::OrdinaryField<Field_waitPolicy>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).waitPolicy) };
#[doc(hidden)]
pub struct Field_ermActive;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExecRowMark, ermActive) == 36);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ermActive>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExecRowMark>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExecRowMark, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExecRowMark> as c::expression::OrdinaryField<Field_ermActive>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ermActive) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ExecuteStmt>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ExecuteStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ExecuteStmt {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExecuteStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExecuteStmt>
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
const _: Projection<crate::__pgrx_c_bindings::ExecuteStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExecuteStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExecuteStmt, params) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_params>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExecuteStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExecuteStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExecuteStmt> as c::expression::OrdinaryField<Field_params>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).params) };
#[doc(hidden)]
pub struct Field_hdr;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ExpandedArrayHeader>() == 136);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ExpandedArrayHeader>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ExpandedArrayHeader {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExpandedArrayHeader, hdr) == 0);
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ExpandedObjectHeader>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ExpandedObjectHeader>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hdr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedArrayHeader>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedObjectHeader>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExpandedArrayHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedArrayHeader> as c::expression::OrdinaryField<Field_hdr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hdr) };
#[doc(hidden)]
pub struct Field_ea_magic;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExpandedArrayHeader, ea_magic) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ea_magic>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedArrayHeader>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExpandedArrayHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedArrayHeader> as c::expression::OrdinaryField<Field_ea_magic>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ea_magic) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExpandedArrayHeader, ndims) == 52);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ndims>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedArrayHeader>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExpandedArrayHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedArrayHeader> as c::expression::OrdinaryField<Field_ndims>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ndims) };
#[doc(hidden)]
pub struct Field_dims;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExpandedArrayHeader, dims) == 56);
const _: () = {
    assert!(::core::mem::size_of::<*mut i32>() == 8);
    assert!(::core::mem::align_of::<*mut i32>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dims>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedArrayHeader>
{
    type Member = c::expression::CPointer<c::CInt, c::expression::ReadWrite>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExpandedArrayHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedArrayHeader> as c::expression::OrdinaryField<Field_dims>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dims) };
#[doc(hidden)]
pub struct Field_lbound;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExpandedArrayHeader, lbound) == 64);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lbound>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedArrayHeader>
{
    type Member = c::expression::CPointer<c::CInt, c::expression::ReadWrite>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExpandedArrayHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedArrayHeader> as c::expression::OrdinaryField<Field_lbound>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lbound) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ExpandedArrayHeader, element_type) == 72
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_element_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedArrayHeader>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExpandedArrayHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedArrayHeader> as c::expression::OrdinaryField<Field_element_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).element_type) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExpandedArrayHeader, typlen) == 76);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typlen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedArrayHeader>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 76;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExpandedArrayHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedArrayHeader> as c::expression::OrdinaryField<Field_typlen>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typlen) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExpandedArrayHeader, typbyval) == 78);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typbyval>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedArrayHeader>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 78;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExpandedArrayHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedArrayHeader> as c::expression::OrdinaryField<Field_typbyval>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typbyval) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExpandedArrayHeader, typalign) == 79);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typalign>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedArrayHeader>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 79;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExpandedArrayHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedArrayHeader> as c::expression::OrdinaryField<Field_typalign>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typalign) };
#[doc(hidden)]
pub struct Field_dvalues;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExpandedArrayHeader, dvalues) == 80);
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::Datum>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::Datum>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dvalues>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedArrayHeader>
{
    type Member = c::expression::CPointer<
        c::expression::CIntegerStorage<c::CUnsignedLong, crate::__pgrx_c_bindings::Datum>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExpandedArrayHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedArrayHeader> as c::expression::OrdinaryField<Field_dvalues>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dvalues) };
#[doc(hidden)]
pub struct Field_dnulls;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExpandedArrayHeader, dnulls) == 88);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dnulls>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedArrayHeader>
{
    type Member = c::expression::CPointer<c::CBool, c::expression::ReadWrite>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExpandedArrayHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedArrayHeader> as c::expression::OrdinaryField<Field_dnulls>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dnulls) };
#[doc(hidden)]
pub struct Field_dvalueslen;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ExpandedArrayHeader, dvalueslen) == 96
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dvalueslen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedArrayHeader>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExpandedArrayHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedArrayHeader> as c::expression::OrdinaryField<Field_dvalueslen>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dvalueslen) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExpandedArrayHeader, nelems) == 100);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nelems>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedArrayHeader>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 100;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExpandedArrayHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedArrayHeader> as c::expression::OrdinaryField<Field_nelems>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nelems) };
#[doc(hidden)]
pub struct Field_flat_size;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ExpandedArrayHeader, flat_size) == 104
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_flat_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedArrayHeader>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExpandedArrayHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedArrayHeader> as c::expression::OrdinaryField<Field_flat_size>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).flat_size) };
#[doc(hidden)]
pub struct Field_fvalue;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExpandedArrayHeader, fvalue) == 112);
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::ArrayType>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::ArrayType>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fvalue>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedArrayHeader>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::ArrayType>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExpandedArrayHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedArrayHeader> as c::expression::OrdinaryField<Field_fvalue>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fvalue) };
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ExpandedObjectHeader {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExpandedObjectHeader, vl_len_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vl_len_>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedObjectHeader>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExpandedObjectHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedObjectHeader> as c::expression::OrdinaryField<Field_vl_len_>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).vl_len_) };
#[doc(hidden)]
pub struct Field_eoh_context;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ExpandedObjectHeader, eoh_context) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_eoh_context>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedObjectHeader>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::MemoryContextData>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExpandedObjectHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedObjectHeader> as c::expression::OrdinaryField<Field_eoh_context>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).eoh_context) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
pub struct Field_eoh_rw_ptr;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ExpandedObjectHeader, eoh_rw_ptr) == 24
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<[::core::ffi::c_char; 10]>() == 10);
    assert!(::core::mem::align_of::<[::core::ffi::c_char; 10]>() == 1);
};
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_eoh_rw_ptr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedObjectHeader>
{
    type Member =
        c::expression::CArray<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, 10>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExpandedObjectHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedObjectHeader> as c::expression::OrdinaryField<Field_eoh_rw_ptr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).eoh_rw_ptr) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
pub struct Field_eoh_ro_ptr;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ExpandedObjectHeader, eoh_ro_ptr) == 34
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_eoh_ro_ptr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedObjectHeader>
{
    type Member =
        c::expression::CArray<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, 10>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 34;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExpandedObjectHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedObjectHeader> as c::expression::OrdinaryField<Field_eoh_ro_ptr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).eoh_ro_ptr) };
#[doc(hidden)]
pub struct Field_fnumber;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ExpandedRecordFieldInfo>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ExpandedRecordFieldInfo>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ExpandedRecordFieldInfo {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ExpandedRecordFieldInfo, fnumber) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fnumber>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedRecordFieldInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExpandedRecordFieldInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedRecordFieldInfo> as c::expression::OrdinaryField<Field_fnumber>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fnumber) };
#[doc(hidden)]
pub struct Field_ftypeid;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ExpandedRecordFieldInfo, ftypeid) == 4
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ftypeid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedRecordFieldInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExpandedRecordFieldInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedRecordFieldInfo> as c::expression::OrdinaryField<Field_ftypeid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ftypeid) };
#[doc(hidden)]
pub struct Field_ftypmod;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ExpandedRecordFieldInfo, ftypmod) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ftypmod>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedRecordFieldInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExpandedRecordFieldInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedRecordFieldInfo> as c::expression::OrdinaryField<Field_ftypmod>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ftypmod) };
#[doc(hidden)]
pub struct Field_fcollation;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ExpandedRecordFieldInfo, fcollation) == 12
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fcollation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedRecordFieldInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExpandedRecordFieldInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedRecordFieldInfo> as c::expression::OrdinaryField<Field_fcollation>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fcollation) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ExpandedRecordHeader>() == 208);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ExpandedRecordHeader>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ExpandedRecordHeader {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExpandedRecordHeader, hdr) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hdr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedRecordHeader>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedObjectHeader>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExpandedRecordHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedRecordHeader> as c::expression::OrdinaryField<Field_hdr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hdr) };
#[doc(hidden)]
pub struct Field_er_magic;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ExpandedRecordHeader, er_magic) == 48
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_er_magic>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedRecordHeader>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExpandedRecordHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedRecordHeader> as c::expression::OrdinaryField<Field_er_magic>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).er_magic) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExpandedRecordHeader, flags) == 52);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedRecordHeader>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExpandedRecordHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedRecordHeader> as c::expression::OrdinaryField<Field_flags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).flags) };
#[doc(hidden)]
pub struct Field_er_decltypeid;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ExpandedRecordHeader, er_decltypeid) == 56
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_er_decltypeid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedRecordHeader>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExpandedRecordHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedRecordHeader> as c::expression::OrdinaryField<Field_er_decltypeid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).er_decltypeid) };
#[doc(hidden)]
pub struct Field_er_typeid;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ExpandedRecordHeader, er_typeid) == 60
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_er_typeid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedRecordHeader>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 60;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExpandedRecordHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedRecordHeader> as c::expression::OrdinaryField<Field_er_typeid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).er_typeid) };
#[doc(hidden)]
pub struct Field_er_typmod;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ExpandedRecordHeader, er_typmod) == 64
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_er_typmod>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedRecordHeader>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExpandedRecordHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedRecordHeader> as c::expression::OrdinaryField<Field_er_typmod>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).er_typmod) };
#[doc(hidden)]
pub struct Field_er_tupdesc_id;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ExpandedRecordHeader, er_tupdesc_id) == 80
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_er_tupdesc_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedRecordHeader>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExpandedRecordHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedRecordHeader> as c::expression::OrdinaryField<Field_er_tupdesc_id>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).er_tupdesc_id) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExpandedRecordHeader, dvalues) == 88);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dvalues>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedRecordHeader>
{
    type Member = c::expression::CPointer<
        c::expression::CIntegerStorage<c::CUnsignedLong, crate::__pgrx_c_bindings::Datum>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExpandedRecordHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedRecordHeader> as c::expression::OrdinaryField<Field_dvalues>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dvalues) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExpandedRecordHeader, dnulls) == 96);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dnulls>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedRecordHeader>
{
    type Member = c::expression::CPointer<c::CBool, c::expression::ReadWrite>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExpandedRecordHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedRecordHeader> as c::expression::OrdinaryField<Field_dnulls>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dnulls) };
#[doc(hidden)]
pub struct Field_nfields;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ExpandedRecordHeader, nfields) == 104
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nfields>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedRecordHeader>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExpandedRecordHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedRecordHeader> as c::expression::OrdinaryField<Field_nfields>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nfields) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ExpandedRecordHeader, flat_size) == 112
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_flat_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedRecordHeader>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExpandedRecordHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedRecordHeader> as c::expression::OrdinaryField<Field_flat_size>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).flat_size) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ExpandedRecordHeader, data_len) == 120
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_data_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedRecordHeader>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExpandedRecordHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedRecordHeader> as c::expression::OrdinaryField<Field_data_len>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).data_len) };
#[doc(hidden)]
pub struct Field_hoff;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExpandedRecordHeader, hoff) == 128);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hoff>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedRecordHeader>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExpandedRecordHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedRecordHeader> as c::expression::OrdinaryField<Field_hoff>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hoff) };
#[doc(hidden)]
pub struct Field_hasnull;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ExpandedRecordHeader, hasnull) == 132
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasnull>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedRecordHeader>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 132;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExpandedRecordHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedRecordHeader> as c::expression::OrdinaryField<Field_hasnull>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hasnull) };
#[doc(hidden)]
pub struct Field_verbose;
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ExplainState>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ExplainState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ExplainState {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExplainState, verbose) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_verbose>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExplainState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState> as c::expression::OrdinaryField<Field_verbose>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).verbose) };
#[doc(hidden)]
pub struct Field_analyze;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExplainState, analyze) == 9);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_analyze>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 9;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExplainState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState> as c::expression::OrdinaryField<Field_analyze>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).analyze) };
#[doc(hidden)]
pub struct Field_costs;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExplainState, costs) == 10);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_costs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 10;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExplainState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState> as c::expression::OrdinaryField<Field_costs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).costs) };
#[doc(hidden)]
pub struct Field_buffers;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExplainState, buffers) == 11);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_buffers>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 11;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExplainState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState> as c::expression::OrdinaryField<Field_buffers>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).buffers) };
#[doc(hidden)]
pub struct Field_wal;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExplainState, wal) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wal>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExplainState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState> as c::expression::OrdinaryField<Field_wal>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wal) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExplainState, timing) == 13);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_timing>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 13;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExplainState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState> as c::expression::OrdinaryField<Field_timing>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).timing) };
#[doc(hidden)]
pub struct Field_summary;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExplainState, summary) == 14);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_summary>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 14;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExplainState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState> as c::expression::OrdinaryField<Field_summary>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).summary) };
#[doc(hidden)]
pub struct Field_settings;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExplainState, settings) == 15);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_settings>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 15;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExplainState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState> as c::expression::OrdinaryField<Field_settings>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).settings) };
#[doc(hidden)]
pub struct Field_format;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExplainState, format) == 16);
#[cfg(feature = "pg15")]
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
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExplainState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState> as c::expression::OrdinaryField<Field_format>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).format) };
#[doc(hidden)]
pub struct Field_indent;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExplainState, indent) == 20);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indent>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExplainState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState> as c::expression::OrdinaryField<Field_indent>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indent) };
#[doc(hidden)]
pub struct Field_grouping_stack;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExplainState, grouping_stack) == 24);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_grouping_stack>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExplainState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState> as c::expression::OrdinaryField<Field_grouping_stack>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).grouping_stack) };
#[doc(hidden)]
pub struct Field_rtable;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExplainState, rtable) == 40);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rtable>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExplainState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState> as c::expression::OrdinaryField<Field_rtable>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rtable) };
#[doc(hidden)]
pub struct Field_rtable_names;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExplainState, rtable_names) == 48);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rtable_names>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExplainState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState> as c::expression::OrdinaryField<Field_rtable_names>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rtable_names) };
#[doc(hidden)]
pub struct Field_deparse_cxt;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExplainState, deparse_cxt) == 56);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_deparse_cxt>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExplainState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState> as c::expression::OrdinaryField<Field_deparse_cxt>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).deparse_cxt) };
#[doc(hidden)]
pub struct Field_hide_workers;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExplainState, hide_workers) == 72);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hide_workers>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExplainState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState> as c::expression::OrdinaryField<Field_hide_workers>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hide_workers) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ExplainStmt>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ExplainStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ExplainStmt {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExplainStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExplainStmt>
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
const _: Projection<crate::__pgrx_c_bindings::ExplainStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExplainStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExplainStmt, options) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExplainStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExplainStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExplainStmt> as c::expression::OrdinaryField<Field_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
#[doc(hidden)]
pub struct Field_num_workers;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ExplainWorkersState>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ExplainWorkersState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ExplainWorkersState {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ExplainWorkersState, num_workers) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_workers>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExplainWorkersState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExplainWorkersState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExplainWorkersState> as c::expression::OrdinaryField<Field_num_workers>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_workers) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Expr>() == 4);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Expr>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::Expr {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Expr, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Expr>
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
    crate::__pgrx_c_bindings::Expr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Expr> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ExprContext>() == 128);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ExprContext>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ExprContext {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExprContext, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprContext>
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
const _: Projection<crate::__pgrx_c_bindings::ExprContext, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprContext> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_ecxt_per_tuple_memory;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ExprContext, ecxt_per_tuple_memory) == 40
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ecxt_per_tuple_memory>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprContext>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::MemoryContextData>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprContext, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprContext> as c::expression::OrdinaryField<Field_ecxt_per_tuple_memory>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ecxt_per_tuple_memory) };
#[doc(hidden)]
pub struct Field_caseValue_datum;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExprContext, caseValue_datum) == 80);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_caseValue_datum>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprContext>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, crate::__pgrx_c_bindings::Datum>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprContext, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprContext> as c::expression::OrdinaryField<Field_caseValue_datum>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).caseValue_datum) };
#[doc(hidden)]
pub struct Field_caseValue_isNull;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExprContext, caseValue_isNull) == 88);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_caseValue_isNull>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprContext>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprContext, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprContext> as c::expression::OrdinaryField<Field_caseValue_isNull>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).caseValue_isNull) };
#[doc(hidden)]
pub struct Field_domainValue_datum;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ExprContext, domainValue_datum) == 96
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_domainValue_datum>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprContext>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, crate::__pgrx_c_bindings::Datum>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprContext, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprContext> as c::expression::OrdinaryField<Field_domainValue_datum>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).domainValue_datum) };
#[doc(hidden)]
pub struct Field_domainValue_isNull;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ExprContext, domainValue_isNull) == 104
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_domainValue_isNull>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprContext>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprContext, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprContext> as c::expression::OrdinaryField<Field_domainValue_isNull>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).domainValue_isNull) };
#[doc(hidden)]
pub struct Field_arg;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ExprContext_CB>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ExprContext_CB>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ExprContext_CB {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExprContext_CB, arg) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_arg>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprContext_CB>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, crate::__pgrx_c_bindings::Datum>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprContext_CB, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprContext_CB> as c::expression::OrdinaryField<Field_arg>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).arg) };
#[doc(hidden)]
pub struct Field_tupdesc_id;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalRowtypeCache>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalRowtypeCache>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ExprEvalRowtypeCache {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ExprEvalRowtypeCache, tupdesc_id) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tupdesc_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalRowtypeCache>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalRowtypeCache, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalRowtypeCache> as c::expression::OrdinaryField<Field_tupdesc_id>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tupdesc_id) };
#[doc(hidden)]
pub struct Field_last_var;
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_1>(
        ) == 24
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_1>(
        ) == 8
    );
};
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_1
{
}
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_1,
        last_var
    ) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_last_var>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_1>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_1, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_1> as c::expression::OrdinaryField<Field_last_var>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).last_var) };
#[doc(hidden)]
pub struct Field_fixed;
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_1,
        fixed
    ) == 4
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fixed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_1>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_1, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_1> as c::expression::OrdinaryField<Field_fixed>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fixed) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_2>(
        ) == 8
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_2>(
        ) == 4
    );
};
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_2
{
}
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_2,
        attnum
    ) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attnum>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_2>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_2, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_2> as c::expression::OrdinaryField<Field_attnum>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attnum) };
#[doc(hidden)]
pub struct Field_vartype;
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_2,
        vartype
    ) == 4
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vartype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_2>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_2, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_2> as c::expression::OrdinaryField<Field_vartype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).vartype) };
#[doc(hidden)]
pub struct Field_first;
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_3>(
        ) == 32
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_3>(
        ) == 8
    );
};
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_3
{
}
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_3,
        first
    ) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_first>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_3>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_3, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_3> as c::expression::OrdinaryField<Field_first>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).first) };
#[doc(hidden)]
pub struct Field_slow;
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_3,
        slow
    ) == 9
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_slow>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_3>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 9;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_3, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_3> as c::expression::OrdinaryField<Field_slow>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).slow) };
#[doc(hidden)]
pub struct Field_resultnum;
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_4>(
        ) == 8
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_4>(
        ) == 4
    );
};
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_4
{
}
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_4,
        resultnum
    ) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resultnum>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_4>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_4, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_4> as c::expression::OrdinaryField<Field_resultnum>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).resultnum) };
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_4,
        attnum
    ) == 4
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attnum>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_4>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_4, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_4> as c::expression::OrdinaryField<Field_attnum>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attnum) };
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_5>(
        ) == 4
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_5>(
        ) == 4
    );
};
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_5
{
}
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_5,
        resultnum
    ) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resultnum>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_5>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_5, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_5> as c::expression::OrdinaryField<Field_resultnum>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).resultnum) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_6>(
        ) == 16
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_6>(
        ) == 8
    );
};
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_6
{
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_6,
        value
    ) == 0
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_value>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_6>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, crate::__pgrx_c_bindings::Datum>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_6, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_6> as c::expression::OrdinaryField<Field_value>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).value) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_6,
        isnull
    ) == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isnull>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_6>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_6, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_6> as c::expression::OrdinaryField<Field_isnull>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isnull) };
#[doc(hidden)]
pub struct Field_nargs;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_7>(
        ) == 32
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_7>(
        ) == 8
    );
};
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_7
{
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_7,
        nargs
    ) == 24
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nargs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_7>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_7, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_7> as c::expression::OrdinaryField<Field_nargs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nargs) };
#[doc(hidden)]
pub struct Field_make_ro;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_7,
        make_ro
    ) == 28
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_make_ro>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_7>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_7, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_7> as c::expression::OrdinaryField<Field_make_ro>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).make_ro) };
#[doc(hidden)]
pub struct Field_jumpdone;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_8>(
        ) == 16
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_8>(
        ) == 8
    );
};
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_8
{
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_8,
        jumpdone
    ) == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_jumpdone>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_8>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_8, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_8> as c::expression::OrdinaryField<Field_jumpdone>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).jumpdone) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_9>(
        ) == 4
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_9>(
        ) == 4
    );
};
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_9
{
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_9,
        jumpdone
    ) == 0
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_jumpdone>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_9>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_9, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_9> as c::expression::OrdinaryField<Field_jumpdone>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).jumpdone) };
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_10>(
        ) == 4
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_10>(
        ) == 4
    );
};
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_10
{
}
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_10,
        jumpdone
    ) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_jumpdone>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_10>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_10, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_10> as c::expression::OrdinaryField<Field_jumpdone>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).jumpdone) };
#[doc(hidden)]
pub struct Field_paramid;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_12>(
        ) == 8
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_12>(
        ) == 4
    );
};
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_12
{
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_12,
        paramid
    ) == 0
);
