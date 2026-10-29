// Shared generated C macro support.

const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_execstate, retval) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_retval>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_execstate>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, crate::__pgrx_c_bindings::Datum>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_execstate, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_execstate> as c::expression::OrdinaryField<Field_retval>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).retval) };
#[doc(hidden)]
pub struct Field_retisnull;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_execstate, retisnull) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_retisnull>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_execstate>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_execstate, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_execstate> as c::expression::OrdinaryField<Field_retisnull>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).retisnull) };
#[doc(hidden)]
pub struct Field_rettype;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_execstate, rettype) == 36);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rettype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_execstate>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_execstate, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_execstate> as c::expression::OrdinaryField<Field_rettype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rettype) };
#[doc(hidden)]
pub struct Field_fn_rettype;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_execstate, fn_rettype) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fn_rettype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_execstate>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_execstate, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_execstate> as c::expression::OrdinaryField<Field_fn_rettype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fn_rettype) };
#[doc(hidden)]
pub struct Field_retistuple;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_execstate, retistuple) == 44);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_retistuple>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_execstate>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_execstate, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_execstate> as c::expression::OrdinaryField<Field_retistuple>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).retistuple) };
#[doc(hidden)]
pub struct Field_retisset;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_execstate, retisset) == 45);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_retisset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_execstate>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 45;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_execstate, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_execstate> as c::expression::OrdinaryField<Field_retisset>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).retisset) };
#[doc(hidden)]
pub struct Field_readonly_func;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_execstate, readonly_func) == 46
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_readonly_func>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_execstate>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 46;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_execstate, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_execstate> as c::expression::OrdinaryField<Field_readonly_func>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).readonly_func) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_execstate, atomic) == 47);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_atomic>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_execstate>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 47;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_execstate, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_execstate> as c::expression::OrdinaryField<Field_atomic>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).atomic) };
#[doc(hidden)]
pub struct Field_found_varno;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_execstate, found_varno) == 104
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_found_varno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_execstate>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_execstate, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_execstate> as c::expression::OrdinaryField<Field_found_varno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).found_varno) };
#[doc(hidden)]
pub struct Field_ndatums;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_execstate, ndatums) == 108);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ndatums>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_execstate>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 108;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_execstate, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_execstate> as c::expression::OrdinaryField<Field_ndatums>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ndatums) };
#[doc(hidden)]
pub struct Field_eval_processed;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_execstate, eval_processed) == 200
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_eval_processed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_execstate>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 200;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_execstate, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_execstate> as c::expression::OrdinaryField<Field_eval_processed>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).eval_processed) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_expr>() == 120);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_expr>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_expr {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_expr, parseMode) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parseMode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_expr>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_RawParseMode,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_expr, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_expr> as c::expression::OrdinaryField<Field_parseMode>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).parseMode) };
#[doc(hidden)]
pub struct Field_expr_simple_type;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_expr, expr_simple_type) == 56
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_expr_simple_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_expr>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_expr, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_expr> as c::expression::OrdinaryField<Field_expr_simple_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).expr_simple_type) };
#[doc(hidden)]
pub struct Field_expr_simple_typmod;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_expr, expr_simple_typmod) == 60
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_expr_simple_typmod>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_expr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 60;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_expr, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_expr> as c::expression::OrdinaryField<Field_expr_simple_typmod>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).expr_simple_typmod) };
#[doc(hidden)]
pub struct Field_expr_simple_mutable;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_expr, expr_simple_mutable) == 64
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_expr_simple_mutable>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_expr>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_expr, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_expr> as c::expression::OrdinaryField<Field_expr_simple_mutable>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).expr_simple_mutable) };
#[doc(hidden)]
pub struct Field_target_param;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_expr, target_param) == 68);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_target_param>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_expr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 68;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_expr, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_expr> as c::expression::OrdinaryField<Field_target_param>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).target_param) };
#[doc(hidden)]
pub struct Field_expr_simple_plan_lxid;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_expr, expr_simple_plan_lxid) == 96
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_expr_simple_plan_lxid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_expr>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_expr, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_expr> as c::expression::OrdinaryField<Field_expr_simple_plan_lxid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).expr_simple_plan_lxid) };
#[doc(hidden)]
pub struct Field_expr_simple_in_use;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_expr, expr_simple_in_use) == 112
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_expr_simple_in_use>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_expr>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_expr, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_expr> as c::expression::OrdinaryField<Field_expr_simple_in_use>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).expr_simple_in_use) };
#[doc(hidden)]
pub struct Field_expr_simple_lxid;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_expr, expr_simple_lxid) == 116
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_expr_simple_lxid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_expr>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 116;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_expr, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_expr> as c::expression::OrdinaryField<Field_expr_simple_lxid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).expr_simple_lxid) };
#[doc(hidden)]
pub struct Field_funcOid;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_func_hashkey>() == 416);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_func_hashkey>() == 4);
};
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_func_hashkey {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_func_hashkey, funcOid) == 0);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_funcOid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_func_hashkey>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_func_hashkey, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_func_hashkey> as c::expression::OrdinaryField<Field_funcOid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).funcOid) };
#[doc(hidden)]
pub struct Field_isTrigger;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_func_hashkey, isTrigger) == 4
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isTrigger>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_func_hashkey>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_func_hashkey, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_func_hashkey> as c::expression::OrdinaryField<Field_isTrigger>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isTrigger) };
#[doc(hidden)]
pub struct Field_isEventTrigger;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_func_hashkey, isEventTrigger) == 5
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isEventTrigger>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_func_hashkey>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 5;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_func_hashkey, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_func_hashkey> as c::expression::OrdinaryField<Field_isEventTrigger>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isEventTrigger) };
#[doc(hidden)]
pub struct Field_trigOid;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_func_hashkey, trigOid) == 8);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_trigOid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_func_hashkey>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_func_hashkey, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_func_hashkey> as c::expression::OrdinaryField<Field_trigOid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).trigOid) };
#[doc(hidden)]
pub struct Field_inputCollation;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_func_hashkey, inputCollation) == 12
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inputCollation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_func_hashkey>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_func_hashkey, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_func_hashkey> as c::expression::OrdinaryField<Field_inputCollation>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).inputCollation) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_function>() == 552);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_function>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_function {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, fn_oid) == 8);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fn_oid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_function, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function> as c::expression::OrdinaryField<Field_fn_oid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fn_oid) };
#[doc(hidden)]
pub struct Field_fn_xmin;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, fn_xmin) == 12);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fn_xmin>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_function, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function> as c::expression::OrdinaryField<Field_fn_xmin>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fn_xmin) };
#[doc(hidden)]
pub struct Field_fn_is_trigger;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, fn_is_trigger) == 24
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fn_is_trigger>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PLpgSQL_trigtype,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_function, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function> as c::expression::OrdinaryField<Field_fn_is_trigger>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fn_is_trigger) };
#[doc(hidden)]
pub struct Field_fn_input_collation;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, fn_input_collation) == 28
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fn_input_collation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_function, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function> as c::expression::OrdinaryField<Field_fn_input_collation>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fn_input_collation) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, fn_rettype) == 48);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fn_rettype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_function, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function> as c::expression::OrdinaryField<Field_fn_rettype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fn_rettype) };
#[doc(hidden)]
pub struct Field_fn_rettyplen;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, fn_rettyplen) == 52
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fn_rettyplen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_function, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function> as c::expression::OrdinaryField<Field_fn_rettyplen>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fn_rettyplen) };
#[doc(hidden)]
pub struct Field_fn_retbyval;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, fn_retbyval) == 56);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fn_retbyval>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_function, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function> as c::expression::OrdinaryField<Field_fn_retbyval>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fn_retbyval) };
#[doc(hidden)]
pub struct Field_fn_retistuple;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, fn_retistuple) == 57
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fn_retistuple>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 57;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_function, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function> as c::expression::OrdinaryField<Field_fn_retistuple>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fn_retistuple) };
#[doc(hidden)]
pub struct Field_fn_retisdomain;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, fn_retisdomain) == 58
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fn_retisdomain>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 58;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_function, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function> as c::expression::OrdinaryField<Field_fn_retisdomain>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fn_retisdomain) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, fn_retset) == 59);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fn_retset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 59;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_function, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function> as c::expression::OrdinaryField<Field_fn_retset>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fn_retset) };
#[doc(hidden)]
pub struct Field_fn_readonly;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, fn_readonly) == 60);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fn_readonly>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 60;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_function, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function> as c::expression::OrdinaryField<Field_fn_readonly>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fn_readonly) };
#[doc(hidden)]
pub struct Field_fn_prokind;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, fn_prokind) == 61);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fn_prokind>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 61;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_function, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function> as c::expression::OrdinaryField<Field_fn_prokind>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fn_prokind) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, fn_nargs) == 64);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fn_nargs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_function, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function> as c::expression::OrdinaryField<Field_fn_nargs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fn_nargs) };
#[doc(hidden)]
pub struct Field_out_param_varno;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, out_param_varno) == 468
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_out_param_varno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 468;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_function, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function> as c::expression::OrdinaryField<Field_out_param_varno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).out_param_varno) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, found_varno) == 472
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_found_varno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 472;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_function, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function> as c::expression::OrdinaryField<Field_found_varno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).found_varno) };
#[doc(hidden)]
pub struct Field_new_varno;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, new_varno) == 476);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_new_varno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 476;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_function, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function> as c::expression::OrdinaryField<Field_new_varno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).new_varno) };
#[doc(hidden)]
pub struct Field_old_varno;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, old_varno) == 480);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_old_varno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 480;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_function, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function> as c::expression::OrdinaryField<Field_old_varno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).old_varno) };
#[doc(hidden)]
pub struct Field_resolve_option;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, resolve_option) == 484
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resolve_option>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PLpgSQL_resolve_option,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 484;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_function, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function> as c::expression::OrdinaryField<Field_resolve_option>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).resolve_option) };
#[doc(hidden)]
pub struct Field_print_strict_params;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, print_strict_params) == 488
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_print_strict_params>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 488;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_function, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function> as c::expression::OrdinaryField<Field_print_strict_params>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).print_strict_params) };
#[doc(hidden)]
pub struct Field_extra_warnings;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, extra_warnings) == 492
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_extra_warnings>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 492;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_function, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function> as c::expression::OrdinaryField<Field_extra_warnings>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).extra_warnings) };
#[doc(hidden)]
pub struct Field_extra_errors;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, extra_errors) == 496
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_extra_errors>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 496;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_function, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function> as c::expression::OrdinaryField<Field_extra_errors>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).extra_errors) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, ndatums) == 500);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ndatums>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 500;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_function, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function> as c::expression::OrdinaryField<Field_ndatums>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ndatums) };
#[doc(hidden)]
pub struct Field_copiable_size;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, copiable_size) == 512
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_copiable_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 512;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_function, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function> as c::expression::OrdinaryField<Field_copiable_size>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).copiable_size) };
#[doc(hidden)]
pub struct Field_nstatements;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, nstatements) == 528
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nstatements>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 528;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_function, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function> as c::expression::OrdinaryField<Field_nstatements>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nstatements) };
#[doc(hidden)]
pub struct Field_requires_procedure_resowner;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::PLpgSQL_function,
        requires_procedure_resowner
    ) == 532
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_requires_procedure_resowner>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 532;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_function, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function> as c::expression::OrdinaryField<Field_requires_procedure_resowner>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).requires_procedure_resowner) };
#[doc(hidden)]
pub struct Field_use_count;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, use_count) == 544);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_use_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 544;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_function, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function> as c::expression::OrdinaryField<Field_use_count>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).use_count) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_if_elsif>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_if_elsif>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_if_elsif {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_if_elsif, lineno) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lineno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_if_elsif>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_if_elsif, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_if_elsif> as c::expression::OrdinaryField<Field_lineno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lineno) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_if_elsif, stmts) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmts>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_if_elsif>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_if_elsif, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_if_elsif> as c::expression::OrdinaryField<Field_stmts>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stmts) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_nsitem>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_nsitem>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_nsitem {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_nsitem, itemtype) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_itemtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_nsitem>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PLpgSQL_nsitem_type,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_nsitem, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_nsitem> as c::expression::OrdinaryField<Field_itemtype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).itemtype) };
#[doc(hidden)]
pub struct Field_itemno;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_nsitem, itemno) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_itemno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_nsitem>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_nsitem, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_nsitem> as c::expression::OrdinaryField<Field_itemno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).itemno) };
#[doc(hidden)]
pub struct Field_opt_type;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_raise_option>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_raise_option>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_raise_option {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_raise_option, opt_type) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_opt_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_raise_option>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PLpgSQL_raise_option_type,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_raise_option, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_raise_option> as c::expression::OrdinaryField<Field_opt_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).opt_type) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_rec>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_rec>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_rec {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_rec, dtype) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_rec>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PLpgSQL_datum_type,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_rec, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_rec> as c::expression::OrdinaryField<Field_dtype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dtype) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_rec, dno) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_rec>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_rec, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_rec> as c::expression::OrdinaryField<Field_dno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dno) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_rec, lineno) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lineno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_rec>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_rec, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_rec> as c::expression::OrdinaryField<Field_lineno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lineno) };
#[doc(hidden)]
pub struct Field_isconst;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_rec, isconst) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isconst>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_rec>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_rec, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_rec> as c::expression::OrdinaryField<Field_isconst>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isconst) };
#[doc(hidden)]
pub struct Field_notnull;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_rec, notnull) == 21);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_notnull>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_rec>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 21;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_rec, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_rec> as c::expression::OrdinaryField<Field_notnull>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).notnull) };
#[doc(hidden)]
pub struct Field_rectypeid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_rec, rectypeid) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rectypeid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_rec>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_rec, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_rec> as c::expression::OrdinaryField<Field_rectypeid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rectypeid) };
#[doc(hidden)]
pub struct Field_firstfield;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_rec, firstfield) == 44);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_firstfield>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_rec>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_rec, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_rec> as c::expression::OrdinaryField<Field_firstfield>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).firstfield) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_recfield>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_recfield>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_recfield {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_recfield, dtype) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_recfield>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PLpgSQL_datum_type,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_recfield, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_recfield> as c::expression::OrdinaryField<Field_dtype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dtype) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_recfield, dno) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_recfield>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_recfield, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_recfield> as c::expression::OrdinaryField<Field_dno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dno) };
#[doc(hidden)]
pub struct Field_recparentno;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_recfield, recparentno) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_recparentno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_recfield>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_recfield, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_recfield> as c::expression::OrdinaryField<Field_recparentno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).recparentno) };
#[doc(hidden)]
pub struct Field_nextfield;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_recfield, nextfield) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nextfield>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_recfield>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_recfield, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_recfield> as c::expression::OrdinaryField<Field_nextfield>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nextfield) };
#[doc(hidden)]
pub struct Field_rectupledescid;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_recfield, rectupledescid) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rectupledescid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_recfield>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_recfield, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_recfield> as c::expression::OrdinaryField<Field_rectupledescid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rectupledescid) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_row>() == 64);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_row>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_row {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_row, dtype) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_row>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PLpgSQL_datum_type,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_row, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_row> as c::expression::OrdinaryField<Field_dtype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dtype) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_row, dno) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_row>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_row, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_row> as c::expression::OrdinaryField<Field_dno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dno) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_row, lineno) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lineno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_row>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_row, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_row> as c::expression::OrdinaryField<Field_lineno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lineno) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_row, isconst) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isconst>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_row>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_row, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_row> as c::expression::OrdinaryField<Field_isconst>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isconst) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_row, notnull) == 21);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_notnull>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_row>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 21;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_row, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_row> as c::expression::OrdinaryField<Field_notnull>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).notnull) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_row, nfields) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nfields>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_row>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_row, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_row> as c::expression::OrdinaryField<Field_nfields>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nfields) };
#[doc(hidden)]
pub struct Field_cmd_type;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_stmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt, cmd_type) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cmd_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PLpgSQL_stmt_type,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt> as c::expression::OrdinaryField<Field_cmd_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cmd_type) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt, lineno) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lineno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt> as c::expression::OrdinaryField<Field_lineno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lineno) };
#[doc(hidden)]
pub struct Field_stmtid;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt, stmtid) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmtid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt> as c::expression::OrdinaryField<Field_stmtid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stmtid) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_assert>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_assert>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_stmt_assert {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_assert, cmd_type) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cmd_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_assert>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PLpgSQL_stmt_type,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_assert, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_assert> as c::expression::OrdinaryField<Field_cmd_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cmd_type) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_assert, lineno) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lineno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_assert>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_assert, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_assert> as c::expression::OrdinaryField<Field_lineno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lineno) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_assert, stmtid) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmtid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_assert>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_assert, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_assert> as c::expression::OrdinaryField<Field_stmtid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stmtid) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_assign>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_assign>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_stmt_assign {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_assign, cmd_type) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cmd_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_assign>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PLpgSQL_stmt_type,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_assign, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_assign> as c::expression::OrdinaryField<Field_cmd_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cmd_type) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_assign, lineno) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lineno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_assign>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_assign, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_assign> as c::expression::OrdinaryField<Field_lineno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lineno) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_assign, stmtid) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmtid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_assign>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_assign, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_assign> as c::expression::OrdinaryField<Field_stmtid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stmtid) };
#[doc(hidden)]
pub struct Field_varno;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_assign, varno) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_varno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_assign>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_assign, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_assign> as c::expression::OrdinaryField<Field_varno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).varno) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_block>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_block>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_stmt_block {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_block, cmd_type) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cmd_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_block>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PLpgSQL_stmt_type,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_block, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_block> as c::expression::OrdinaryField<Field_cmd_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cmd_type) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_block, lineno) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lineno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_block>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_block, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_block> as c::expression::OrdinaryField<Field_lineno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lineno) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_block, stmtid) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmtid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_block>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_block, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_block> as c::expression::OrdinaryField<Field_stmtid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stmtid) };
#[doc(hidden)]
pub struct Field_body;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_block, body) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_body>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_block>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_block, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_block> as c::expression::OrdinaryField<Field_body>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).body) };
#[doc(hidden)]
pub struct Field_n_initvars;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_block, n_initvars) == 32
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_n_initvars>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_block>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_block, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_block> as c::expression::OrdinaryField<Field_n_initvars>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).n_initvars) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_call>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_call>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_stmt_call {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_call, cmd_type) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cmd_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_call>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PLpgSQL_stmt_type,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_call, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_call> as c::expression::OrdinaryField<Field_cmd_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cmd_type) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_call, lineno) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lineno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_call>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_call, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_call> as c::expression::OrdinaryField<Field_lineno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lineno) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_call, stmtid) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmtid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_call>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_call, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_call> as c::expression::OrdinaryField<Field_stmtid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stmtid) };
#[doc(hidden)]
pub struct Field_is_call;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_call, is_call) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_call>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_call>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_call, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_call> as c::expression::OrdinaryField<Field_is_call>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_call) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_case>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_case>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_stmt_case {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_case, cmd_type) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cmd_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_case>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PLpgSQL_stmt_type,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_case, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_case> as c::expression::OrdinaryField<Field_cmd_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cmd_type) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_case, lineno) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lineno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_case>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_case, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_case> as c::expression::OrdinaryField<Field_lineno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lineno) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_case, stmtid) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmtid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_case>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_case, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_case> as c::expression::OrdinaryField<Field_stmtid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stmtid) };
#[doc(hidden)]
pub struct Field_t_varno;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_case, t_varno) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t_varno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_case>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_case, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_case> as c::expression::OrdinaryField<Field_t_varno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t_varno) };
#[doc(hidden)]
pub struct Field_case_when_list;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_case, case_when_list) == 32
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_case_when_list>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_case>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_case, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_case> as c::expression::OrdinaryField<Field_case_when_list>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).case_when_list) };
#[doc(hidden)]
pub struct Field_have_else;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_case, have_else) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_have_else>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_case>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_case, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_case> as c::expression::OrdinaryField<Field_have_else>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).have_else) };
#[doc(hidden)]
pub struct Field_else_stmts;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_case, else_stmts) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_else_stmts>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_case>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_case, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_case> as c::expression::OrdinaryField<Field_else_stmts>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).else_stmts) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_close>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_close>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_stmt_close {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_close, cmd_type) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cmd_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_close>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PLpgSQL_stmt_type,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_close, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_close> as c::expression::OrdinaryField<Field_cmd_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cmd_type) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_close, lineno) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lineno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_close>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_close, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_close> as c::expression::OrdinaryField<Field_lineno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lineno) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_close, stmtid) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmtid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_close>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_close, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_close> as c::expression::OrdinaryField<Field_stmtid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stmtid) };
#[doc(hidden)]
pub struct Field_curvar;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_close, curvar) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_curvar>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_close>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_close, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_close> as c::expression::OrdinaryField<Field_curvar>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).curvar) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_commit>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_commit>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_stmt_commit {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_commit, cmd_type) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cmd_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_commit>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PLpgSQL_stmt_type,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_commit, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_commit> as c::expression::OrdinaryField<Field_cmd_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cmd_type) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_commit, lineno) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lineno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_commit>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_commit, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_commit> as c::expression::OrdinaryField<Field_lineno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lineno) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_commit, stmtid) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmtid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_commit>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_commit, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_commit> as c::expression::OrdinaryField<Field_stmtid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stmtid) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_commit, chain) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_chain>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_commit>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_commit, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_commit> as c::expression::OrdinaryField<Field_chain>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).chain) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_dynexecute>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_dynexecute>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_stmt_dynexecute {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_dynexecute, cmd_type) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cmd_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_dynexecute>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PLpgSQL_stmt_type,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_dynexecute, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_dynexecute> as c::expression::OrdinaryField<Field_cmd_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cmd_type) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_dynexecute, lineno) == 4
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lineno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_dynexecute>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_dynexecute, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_dynexecute> as c::expression::OrdinaryField<Field_lineno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lineno) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_dynexecute, stmtid) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmtid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_dynexecute>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_dynexecute, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_dynexecute> as c::expression::OrdinaryField<Field_stmtid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stmtid) };
#[doc(hidden)]
pub struct Field_into;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_dynexecute, into) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_into>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_dynexecute>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_dynexecute, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_dynexecute> as c::expression::OrdinaryField<Field_into>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).into) };
#[doc(hidden)]
pub struct Field_strict;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_dynexecute, strict) == 25
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_strict>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_dynexecute>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 25;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_dynexecute, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_dynexecute> as c::expression::OrdinaryField<Field_strict>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).strict) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_dynexecute, params) == 40
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_params>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_dynexecute>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_dynexecute, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_dynexecute> as c::expression::OrdinaryField<Field_params>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).params) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_dynfors>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_dynfors>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_stmt_dynfors {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_dynfors, cmd_type) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cmd_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_dynfors>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PLpgSQL_stmt_type,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_dynfors, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_dynfors> as c::expression::OrdinaryField<Field_cmd_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cmd_type) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_dynfors, lineno) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lineno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_dynfors>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_dynfors, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_dynfors> as c::expression::OrdinaryField<Field_lineno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lineno) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_dynfors, stmtid) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmtid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_dynfors>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_dynfors, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_dynfors> as c::expression::OrdinaryField<Field_stmtid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stmtid) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_dynfors, body) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_body>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_dynfors>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_dynfors, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_dynfors> as c::expression::OrdinaryField<Field_body>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).body) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_dynfors, params) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_params>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_dynfors>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_dynfors, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_dynfors> as c::expression::OrdinaryField<Field_params>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).params) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_execsql>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_execsql>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_stmt_execsql {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_execsql, cmd_type) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cmd_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_execsql>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PLpgSQL_stmt_type,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_execsql, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_execsql> as c::expression::OrdinaryField<Field_cmd_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cmd_type) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_execsql, lineno) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lineno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_execsql>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_execsql, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_execsql> as c::expression::OrdinaryField<Field_lineno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lineno) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_execsql, stmtid) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmtid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_execsql>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_execsql, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_execsql> as c::expression::OrdinaryField<Field_stmtid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stmtid) };
#[doc(hidden)]
pub struct Field_mod_stmt;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_execsql, mod_stmt) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mod_stmt>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_execsql>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_execsql, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_execsql> as c::expression::OrdinaryField<Field_mod_stmt>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mod_stmt) };
#[doc(hidden)]
pub struct Field_mod_stmt_set;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_execsql, mod_stmt_set) == 25
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mod_stmt_set>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_execsql>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 25;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_execsql, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_execsql> as c::expression::OrdinaryField<Field_mod_stmt_set>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mod_stmt_set) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_execsql, into) == 26);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_into>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_execsql>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 26;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_execsql, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_execsql> as c::expression::OrdinaryField<Field_into>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).into) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_execsql, strict) == 27);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_strict>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_execsql>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 27;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_execsql, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_execsql> as c::expression::OrdinaryField<Field_strict>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).strict) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_exit>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_exit>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_stmt_exit {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_exit, cmd_type) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cmd_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_exit>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PLpgSQL_stmt_type,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_exit, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_exit> as c::expression::OrdinaryField<Field_cmd_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cmd_type) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_exit, lineno) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lineno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_exit>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_exit, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_exit> as c::expression::OrdinaryField<Field_lineno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lineno) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_exit, stmtid) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmtid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_exit>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_exit, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_exit> as c::expression::OrdinaryField<Field_stmtid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stmtid) };
#[doc(hidden)]
pub struct Field_is_exit;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_exit, is_exit) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_exit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_exit>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_exit, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_exit> as c::expression::OrdinaryField<Field_is_exit>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_exit) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_fetch>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_fetch>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_stmt_fetch {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_fetch, cmd_type) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cmd_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_fetch>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PLpgSQL_stmt_type,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_fetch, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_fetch> as c::expression::OrdinaryField<Field_cmd_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cmd_type) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_fetch, lineno) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lineno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_fetch>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_fetch, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_fetch> as c::expression::OrdinaryField<Field_lineno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lineno) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_fetch, stmtid) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmtid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_fetch>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_fetch, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_fetch> as c::expression::OrdinaryField<Field_stmtid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stmtid) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_fetch, curvar) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_curvar>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_fetch>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_fetch, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_fetch> as c::expression::OrdinaryField<Field_curvar>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).curvar) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_fetch, direction) == 28);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_direction>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_fetch>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__FetchDirection,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_fetch, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_fetch> as c::expression::OrdinaryField<Field_direction>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).direction) };
#[doc(hidden)]
pub struct Field_how_many;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_fetch, how_many) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_how_many>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_fetch>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_fetch, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_fetch> as c::expression::OrdinaryField<Field_how_many>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).how_many) };
#[doc(hidden)]
pub struct Field_is_move;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_fetch, is_move) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_move>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_fetch>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_fetch, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_fetch> as c::expression::OrdinaryField<Field_is_move>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_move) };
#[doc(hidden)]
pub struct Field_returns_multiple_rows;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_fetch, returns_multiple_rows)
        == 49
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_returns_multiple_rows>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_fetch>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 49;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_fetch, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_fetch> as c::expression::OrdinaryField<Field_returns_multiple_rows>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).returns_multiple_rows) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_forc>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_forc>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_stmt_forc {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_forc, cmd_type) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cmd_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_forc>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PLpgSQL_stmt_type,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_forc, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_forc> as c::expression::OrdinaryField<Field_cmd_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cmd_type) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_forc, lineno) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lineno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_forc>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_forc, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_forc> as c::expression::OrdinaryField<Field_lineno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lineno) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_forc, stmtid) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmtid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_forc>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_forc, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_forc> as c::expression::OrdinaryField<Field_stmtid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stmtid) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_forc, body) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_body>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_forc>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_forc, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_forc> as c::expression::OrdinaryField<Field_body>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).body) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_forc, curvar) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_curvar>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_forc>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_forc, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_forc> as c::expression::OrdinaryField<Field_curvar>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).curvar) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_foreach_a>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_foreach_a>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_stmt_foreach_a {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_foreach_a, cmd_type) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cmd_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_foreach_a>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PLpgSQL_stmt_type,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_foreach_a, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_foreach_a> as c::expression::OrdinaryField<Field_cmd_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cmd_type) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_foreach_a, lineno) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lineno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_foreach_a>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_foreach_a, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_foreach_a> as c::expression::OrdinaryField<Field_lineno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lineno) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_foreach_a, stmtid) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmtid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_foreach_a>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_foreach_a, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_foreach_a> as c::expression::OrdinaryField<Field_stmtid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stmtid) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_foreach_a, varno) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_varno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_foreach_a>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_foreach_a, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_foreach_a> as c::expression::OrdinaryField<Field_varno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).varno) };
#[doc(hidden)]
pub struct Field_slice;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_foreach_a, slice) == 28);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_slice>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_foreach_a>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_foreach_a, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_foreach_a> as c::expression::OrdinaryField<Field_slice>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).slice) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_foreach_a, body) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_body>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_foreach_a>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_foreach_a, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_foreach_a> as c::expression::OrdinaryField<Field_body>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).body) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_fori>() == 72);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_fori>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_stmt_fori {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_fori, cmd_type) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cmd_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_fori>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PLpgSQL_stmt_type,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_fori, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_fori> as c::expression::OrdinaryField<Field_cmd_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cmd_type) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_fori, lineno) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lineno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_fori>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_fori, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_fori> as c::expression::OrdinaryField<Field_lineno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lineno) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_fori, stmtid) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmtid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_fori>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_fori, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_fori> as c::expression::OrdinaryField<Field_stmtid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stmtid) };
#[doc(hidden)]
pub struct Field_reverse;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_fori, reverse) == 56);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_reverse>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_fori>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_fori, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_fori> as c::expression::OrdinaryField<Field_reverse>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).reverse) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_fori, body) == 64);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_body>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_fori>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_fori, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_fori> as c::expression::OrdinaryField<Field_body>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).body) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_forq>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_forq>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_stmt_forq {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_forq, cmd_type) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cmd_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_forq>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PLpgSQL_stmt_type,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_forq, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_forq> as c::expression::OrdinaryField<Field_cmd_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cmd_type) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_forq, lineno) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lineno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_forq>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_forq, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_forq> as c::expression::OrdinaryField<Field_lineno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lineno) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_forq, stmtid) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmtid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_forq>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_forq, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_forq> as c::expression::OrdinaryField<Field_stmtid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stmtid) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_forq, body) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_body>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_forq>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_forq, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_forq> as c::expression::OrdinaryField<Field_body>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).body) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_fors>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_fors>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_stmt_fors {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_fors, cmd_type) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cmd_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_fors>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PLpgSQL_stmt_type,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_fors, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_fors> as c::expression::OrdinaryField<Field_cmd_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cmd_type) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_fors, lineno) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lineno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_fors>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_fors, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_fors> as c::expression::OrdinaryField<Field_lineno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lineno) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_fors, stmtid) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmtid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_fors>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_fors, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_fors> as c::expression::OrdinaryField<Field_stmtid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stmtid) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_fors, body) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_body>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_fors>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_fors, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_fors> as c::expression::OrdinaryField<Field_body>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).body) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_getdiag>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_getdiag>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_stmt_getdiag {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_getdiag, cmd_type) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cmd_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_getdiag>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PLpgSQL_stmt_type,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_getdiag, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_getdiag> as c::expression::OrdinaryField<Field_cmd_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cmd_type) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_getdiag, lineno) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lineno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_getdiag>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_getdiag, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_getdiag> as c::expression::OrdinaryField<Field_lineno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lineno) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_getdiag, stmtid) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmtid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_getdiag>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_getdiag, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_getdiag> as c::expression::OrdinaryField<Field_stmtid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stmtid) };
#[doc(hidden)]
pub struct Field_is_stacked;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_getdiag, is_stacked) == 12
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_stacked>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_getdiag>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_getdiag, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_getdiag> as c::expression::OrdinaryField<Field_is_stacked>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_stacked) };
#[doc(hidden)]
pub struct Field_diag_items;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_getdiag, diag_items) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_diag_items>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_getdiag>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_getdiag, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_getdiag> as c::expression::OrdinaryField<Field_diag_items>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).diag_items) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_if>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_if>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_stmt_if {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_if, cmd_type) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cmd_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_if>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PLpgSQL_stmt_type,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_if, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_if> as c::expression::OrdinaryField<Field_cmd_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cmd_type) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_if, lineno) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lineno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_if>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_if, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_if> as c::expression::OrdinaryField<Field_lineno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lineno) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_if, stmtid) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmtid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_if>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_if, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_if> as c::expression::OrdinaryField<Field_stmtid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stmtid) };
#[doc(hidden)]
pub struct Field_then_body;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_if, then_body) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_then_body>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_if>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_if, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_if> as c::expression::OrdinaryField<Field_then_body>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).then_body) };
#[doc(hidden)]
pub struct Field_elsif_list;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_if, elsif_list) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_elsif_list>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_if>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_if, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_if> as c::expression::OrdinaryField<Field_elsif_list>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).elsif_list) };
#[doc(hidden)]
pub struct Field_else_body;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_if, else_body) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_else_body>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_if>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_if, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_if> as c::expression::OrdinaryField<Field_else_body>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).else_body) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_loop>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_loop>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_stmt_loop {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_loop, cmd_type) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cmd_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_loop>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PLpgSQL_stmt_type,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_loop, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_loop> as c::expression::OrdinaryField<Field_cmd_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cmd_type) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_loop, lineno) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lineno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_loop>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_loop, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_loop> as c::expression::OrdinaryField<Field_lineno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lineno) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_loop, stmtid) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmtid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_loop>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_loop, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_loop> as c::expression::OrdinaryField<Field_stmtid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stmtid) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_loop, body) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_body>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_loop>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_loop, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_loop> as c::expression::OrdinaryField<Field_body>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).body) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_open>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_open>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_stmt_open {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_open, cmd_type) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cmd_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_open>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PLpgSQL_stmt_type,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_open, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_open> as c::expression::OrdinaryField<Field_cmd_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cmd_type) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_open, lineno) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lineno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_open>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_open, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_open> as c::expression::OrdinaryField<Field_lineno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lineno) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_open, stmtid) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmtid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_open>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_open, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_open> as c::expression::OrdinaryField<Field_stmtid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stmtid) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_open, curvar) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_curvar>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_open>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_open, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_open> as c::expression::OrdinaryField<Field_curvar>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).curvar) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_open, cursor_options) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cursor_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_open>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_open, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_open> as c::expression::OrdinaryField<Field_cursor_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cursor_options) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_open, params) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_params>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_open>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_open, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_open> as c::expression::OrdinaryField<Field_params>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).params) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_perform>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_perform>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_stmt_perform {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_perform, cmd_type) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cmd_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_perform>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PLpgSQL_stmt_type,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_perform, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_perform> as c::expression::OrdinaryField<Field_cmd_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cmd_type) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_perform, lineno) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lineno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_perform>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_perform, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_perform> as c::expression::OrdinaryField<Field_lineno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lineno) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_perform, stmtid) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmtid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_perform>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_perform, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_perform> as c::expression::OrdinaryField<Field_stmtid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stmtid) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_raise>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_raise>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_stmt_raise {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_raise, cmd_type) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cmd_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_raise>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PLpgSQL_stmt_type,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_raise, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_raise> as c::expression::OrdinaryField<Field_cmd_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cmd_type) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_raise, lineno) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lineno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_raise>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_raise, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_raise> as c::expression::OrdinaryField<Field_lineno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lineno) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_raise, stmtid) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmtid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_raise>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_raise, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_raise> as c::expression::OrdinaryField<Field_stmtid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stmtid) };
#[doc(hidden)]
pub struct Field_elog_level;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_raise, elog_level) == 12
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_elog_level>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_raise>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_raise, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_raise> as c::expression::OrdinaryField<Field_elog_level>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).elog_level) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_raise, params) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_params>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_raise>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_raise, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_raise> as c::expression::OrdinaryField<Field_params>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).params) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_raise, options) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_raise>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_raise, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_raise> as c::expression::OrdinaryField<Field_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_return>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_return>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_stmt_return {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_return, cmd_type) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cmd_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_return>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PLpgSQL_stmt_type,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_return, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_return> as c::expression::OrdinaryField<Field_cmd_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cmd_type) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_return, lineno) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lineno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_return>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_return, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_return> as c::expression::OrdinaryField<Field_lineno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lineno) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_return, stmtid) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmtid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_return>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_return, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_return> as c::expression::OrdinaryField<Field_stmtid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stmtid) };
#[doc(hidden)]
pub struct Field_retvarno;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_return, retvarno) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_retvarno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_return>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_return, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_return> as c::expression::OrdinaryField<Field_retvarno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).retvarno) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_return_next>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_return_next>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_stmt_return_next {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_return_next, cmd_type) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cmd_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_return_next>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PLpgSQL_stmt_type,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_return_next, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_return_next> as c::expression::OrdinaryField<Field_cmd_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cmd_type) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_return_next, lineno) == 4
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lineno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_return_next>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_return_next, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_return_next> as c::expression::OrdinaryField<Field_lineno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lineno) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_return_next, stmtid) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmtid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_return_next>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_return_next, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_return_next> as c::expression::OrdinaryField<Field_stmtid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stmtid) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_return_next, retvarno) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_retvarno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_return_next>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_return_next, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_return_next> as c::expression::OrdinaryField<Field_retvarno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).retvarno) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_return_query>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_return_query>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_stmt_return_query {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_return_query, cmd_type) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cmd_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_return_query>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PLpgSQL_stmt_type,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_return_query, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_return_query> as c::expression::OrdinaryField<Field_cmd_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cmd_type) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_return_query, lineno) == 4
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lineno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_return_query>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_return_query, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_return_query> as c::expression::OrdinaryField<Field_lineno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lineno) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_return_query, stmtid) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmtid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_return_query>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_return_query, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_return_query> as c::expression::OrdinaryField<Field_stmtid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stmtid) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_return_query, params) == 32
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_params>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_return_query>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_return_query, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_return_query> as c::expression::OrdinaryField<Field_params>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).params) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_rollback>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_rollback>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_stmt_rollback {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_rollback, cmd_type) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cmd_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_rollback>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PLpgSQL_stmt_type,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_rollback, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_rollback> as c::expression::OrdinaryField<Field_cmd_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cmd_type) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_rollback, lineno) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lineno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_rollback>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_rollback, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_rollback> as c::expression::OrdinaryField<Field_lineno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lineno) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_rollback, stmtid) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmtid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_rollback>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_rollback, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_rollback> as c::expression::OrdinaryField<Field_stmtid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stmtid) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_rollback, chain) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_chain>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_rollback>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_rollback, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_rollback> as c::expression::OrdinaryField<Field_chain>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).chain) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_while>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_stmt_while>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_stmt_while {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_while, cmd_type) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cmd_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_while>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PLpgSQL_stmt_type,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_while, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_while> as c::expression::OrdinaryField<Field_cmd_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cmd_type) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_while, lineno) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lineno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_while>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_while, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_while> as c::expression::OrdinaryField<Field_lineno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lineno) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_while, stmtid) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmtid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_while>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_while, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_while> as c::expression::OrdinaryField<Field_stmtid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stmtid) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_stmt_while, body) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_body>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_while>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_stmt_while, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_stmt_while> as c::expression::OrdinaryField<Field_body>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).body) };
#[doc(hidden)]
pub struct Field_typoid;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_type>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_type>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_type {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_type, typoid) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typoid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_type>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_type, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_type> as c::expression::OrdinaryField<Field_typoid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typoid) };
#[doc(hidden)]
pub struct Field_ttype;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_type, ttype) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ttype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_type>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PLpgSQL_type_type,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_type, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_type> as c::expression::OrdinaryField<Field_ttype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ttype) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_type, typlen) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typlen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_type>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_type, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_type> as c::expression::OrdinaryField<Field_typlen>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typlen) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_type, typbyval) == 18);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typbyval>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_type>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 18;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_type, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_type> as c::expression::OrdinaryField<Field_typbyval>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typbyval) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_type, typtype) == 19);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_type>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 19;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_type, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_type> as c::expression::OrdinaryField<Field_typtype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typtype) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_type, collation) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_collation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_type>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_type, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_type> as c::expression::OrdinaryField<Field_collation>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).collation) };
#[doc(hidden)]
pub struct Field_typisarray;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_type, typisarray) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typisarray>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_type>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_type, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_type> as c::expression::OrdinaryField<Field_typisarray>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typisarray) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_type, atttypmod) == 28);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_atttypmod>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_type>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_type, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_type> as c::expression::OrdinaryField<Field_atttypmod>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).atttypmod) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_type, tupdesc_id) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tupdesc_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_type>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_type, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_type> as c::expression::OrdinaryField<Field_tupdesc_id>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tupdesc_id) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_var>() == 72);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_var>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_var {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_var, dtype) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_var>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PLpgSQL_datum_type,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_var, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_var> as c::expression::OrdinaryField<Field_dtype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dtype) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_var, dno) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_var>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_var, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_var> as c::expression::OrdinaryField<Field_dno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dno) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_var, lineno) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lineno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_var>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_var, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_var> as c::expression::OrdinaryField<Field_lineno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lineno) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_var, isconst) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isconst>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_var>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_var, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_var> as c::expression::OrdinaryField<Field_isconst>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isconst) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_var, notnull) == 21);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_notnull>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_var>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 21;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_var, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_var> as c::expression::OrdinaryField<Field_notnull>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).notnull) };
#[doc(hidden)]
pub struct Field_cursor_explicit_argrow;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_var, cursor_explicit_argrow) == 48
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cursor_explicit_argrow>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_var>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_var, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_var> as c::expression::OrdinaryField<Field_cursor_explicit_argrow>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cursor_explicit_argrow) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_var, cursor_options) == 52);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cursor_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_var>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_var, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_var> as c::expression::OrdinaryField<Field_cursor_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cursor_options) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_var, value) == 56);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_value>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_var>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, crate::__pgrx_c_bindings::Datum>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_var, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_var> as c::expression::OrdinaryField<Field_value>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).value) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_var, isnull) == 64);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isnull>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_var>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_var, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_var> as c::expression::OrdinaryField<Field_isnull>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isnull) };
#[doc(hidden)]
pub struct Field_freeval;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_var, freeval) == 65);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_freeval>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_var>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 65;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_var, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_var> as c::expression::OrdinaryField<Field_freeval>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).freeval) };
#[doc(hidden)]
pub struct Field_promise;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_var, promise) == 68);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_promise>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_var>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PLpgSQL_promise_type,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 68;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_var, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_var> as c::expression::OrdinaryField<Field_promise>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).promise) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_variable>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_variable>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_variable {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_variable, dtype) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_variable>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PLpgSQL_datum_type,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_variable, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_variable> as c::expression::OrdinaryField<Field_dtype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dtype) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_variable, dno) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_variable>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_variable, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_variable> as c::expression::OrdinaryField<Field_dno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dno) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_variable, lineno) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lineno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_variable>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_variable, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_variable> as c::expression::OrdinaryField<Field_lineno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lineno) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_variable, isconst) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isconst>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_variable>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_variable, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_variable> as c::expression::OrdinaryField<Field_isconst>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isconst) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_variable, notnull) == 21);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_notnull>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_variable>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 21;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_variable, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_variable> as c::expression::OrdinaryField<Field_notnull>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).notnull) };
#[doc(hidden)]
pub struct Field_quoted;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLwdatum>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLwdatum>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLwdatum {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLwdatum, quoted) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_quoted>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLwdatum>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PLwdatum,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PLwdatum> as c::expression::OrdinaryField<
        Field_quoted,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).quoted) };
#[doc(hidden)]
pub struct Field_idents;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLwdatum, idents) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_idents>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLwdatum>
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
    crate::__pgrx_c_bindings::PLwdatum,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PLwdatum> as c::expression::OrdinaryField<
        Field_idents,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).idents) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLword>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLword>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLword {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLword, quoted) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_quoted>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLword>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PLword,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PLword> as c::expression::OrdinaryField<
        Field_quoted,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).quoted) };
#[doc(hidden)]
pub struct Field_commitSeqNo;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PREDICATELOCK>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PREDICATELOCK>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PREDICATELOCK {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PREDICATELOCK, commitSeqNo) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_commitSeqNo>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PREDICATELOCK>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PREDICATELOCK, <<c::expression::CRecord<crate::__pgrx_c_bindings::PREDICATELOCK> as c::expression::OrdinaryField<Field_commitSeqNo>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).commitSeqNo) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PREDICATELOCKTARGETTAG>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PREDICATELOCKTARGETTAG>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PREDICATELOCKTARGETTAG {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PREDICATELOCKTARGETTAG, locktag_field1) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_locktag_field1>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PREDICATELOCKTARGETTAG>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PREDICATELOCKTARGETTAG, <<c::expression::CRecord<crate::__pgrx_c_bindings::PREDICATELOCKTARGETTAG> as c::expression::OrdinaryField<Field_locktag_field1>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).locktag_field1) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PREDICATELOCKTARGETTAG, locktag_field2) == 4
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_locktag_field2>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PREDICATELOCKTARGETTAG>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PREDICATELOCKTARGETTAG, <<c::expression::CRecord<crate::__pgrx_c_bindings::PREDICATELOCKTARGETTAG> as c::expression::OrdinaryField<Field_locktag_field2>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).locktag_field2) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PREDICATELOCKTARGETTAG, locktag_field3) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_locktag_field3>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PREDICATELOCKTARGETTAG>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PREDICATELOCKTARGETTAG, <<c::expression::CRecord<crate::__pgrx_c_bindings::PREDICATELOCKTARGETTAG> as c::expression::OrdinaryField<Field_locktag_field3>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).locktag_field3) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PREDICATELOCKTARGETTAG, locktag_field4) == 12
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_locktag_field4>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PREDICATELOCKTARGETTAG>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PREDICATELOCKTARGETTAG, <<c::expression::CRecord<crate::__pgrx_c_bindings::PREDICATELOCKTARGETTAG> as c::expression::OrdinaryField<Field_locktag_field4>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).locktag_field4) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PROCLOCK>() == 64);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PROCLOCK>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PROCLOCK {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PROCLOCK, tag) == 0);
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PROCLOCKTAG>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PROCLOCKTAG>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tag>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PROCLOCK>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::PROCLOCKTAG>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PROCLOCK,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PROCLOCK> as c::expression::OrdinaryField<
        Field_tag,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tag) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PROCLOCK, holdMask) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_holdMask>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PROCLOCK>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PROCLOCK,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PROCLOCK> as c::expression::OrdinaryField<
        Field_holdMask,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).holdMask) };
#[doc(hidden)]
pub struct Field_releaseMask;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PROCLOCK, releaseMask) == 28);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_releaseMask>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PROCLOCK>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PROCLOCK,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PROCLOCK> as c::expression::OrdinaryField<
        Field_releaseMask,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).releaseMask) };
#[doc(hidden)]
pub struct Field_myLock;
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PROCLOCKTAG {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PROCLOCKTAG, myLock) == 0);
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::LOCK>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::LOCK>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_myLock>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PROCLOCKTAG>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::LOCK>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PROCLOCKTAG, <<c::expression::CRecord<crate::__pgrx_c_bindings::PROCLOCKTAG> as c::expression::OrdinaryField<Field_myLock>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).myLock) };
#[doc(hidden)]
pub struct Field_allProcs;
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PROC_HDR>() == 104);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PROC_HDR>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PROC_HDR {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PROC_HDR, allProcs) == 0);
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::PGPROC>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::PGPROC>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_allProcs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PROC_HDR>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PROC_HDR,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PROC_HDR> as c::expression::OrdinaryField<
        Field_allProcs,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).allProcs) };
#[doc(hidden)]
pub struct Field_allProcCount;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PROC_HDR, allProcCount) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_allProcCount>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PROC_HDR>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PROC_HDR,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PROC_HDR> as c::expression::OrdinaryField<
        Field_allProcCount,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).allProcCount) };
#[doc(hidden)]
pub struct Field_spins_per_delay;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PROC_HDR, spins_per_delay) == 96);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_spins_per_delay>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PROC_HDR>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PROC_HDR,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PROC_HDR> as c::expression::OrdinaryField<
        Field_spins_per_delay,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).spins_per_delay) };
#[doc(hidden)]
pub struct Field_startupBufferPinWaitBufId;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PROC_HDR, startupBufferPinWaitBufId) == 100
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_startupBufferPinWaitBufId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PROC_HDR>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 100;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PROC_HDR,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PROC_HDR> as c::expression::OrdinaryField<
        Field_startupBufferPinWaitBufId,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).startupBufferPinWaitBufId) };
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PROC_QUEUE>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PROC_QUEUE>() == 8);
};
#[cfg(feature = "pg15")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PROC_QUEUE {}
#[cfg(feature = "pg15")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PROC_QUEUE, size) == 16);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PROC_QUEUE>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PROC_QUEUE, <<c::expression::CRecord<crate::__pgrx_c_bindings::PROC_QUEUE> as c::expression::OrdinaryField<Field_size>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).size) };
#[doc(hidden)]
pub struct Field_pd_lsn;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PageHeaderData>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PageHeaderData>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PageHeaderData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PageHeaderData, pd_lsn) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pd_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PageHeaderData>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::PageXLogRecPtr>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PageHeaderData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PageHeaderData> as c::expression::OrdinaryField<Field_pd_lsn>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pd_lsn) };
#[doc(hidden)]
pub struct Field_pd_checksum;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PageHeaderData, pd_checksum) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pd_checksum>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PageHeaderData>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PageHeaderData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PageHeaderData> as c::expression::OrdinaryField<Field_pd_checksum>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pd_checksum) };
#[doc(hidden)]
pub struct Field_pd_flags;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PageHeaderData, pd_flags) == 10);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pd_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PageHeaderData>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 10;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PageHeaderData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PageHeaderData> as c::expression::OrdinaryField<Field_pd_flags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pd_flags) };
#[doc(hidden)]
pub struct Field_pd_lower;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PageHeaderData, pd_lower) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pd_lower>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PageHeaderData>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PageHeaderData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PageHeaderData> as c::expression::OrdinaryField<Field_pd_lower>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pd_lower) };
#[doc(hidden)]
pub struct Field_pd_upper;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PageHeaderData, pd_upper) == 14);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pd_upper>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PageHeaderData>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 14;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PageHeaderData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PageHeaderData> as c::expression::OrdinaryField<Field_pd_upper>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pd_upper) };
#[doc(hidden)]
pub struct Field_pd_special;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PageHeaderData, pd_special) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pd_special>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PageHeaderData>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PageHeaderData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PageHeaderData> as c::expression::OrdinaryField<Field_pd_special>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pd_special) };
#[doc(hidden)]
pub struct Field_pd_pagesize_version;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PageHeaderData, pd_pagesize_version) == 18
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pd_pagesize_version>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PageHeaderData>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 18;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PageHeaderData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PageHeaderData> as c::expression::OrdinaryField<Field_pd_pagesize_version>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pd_pagesize_version) };
#[doc(hidden)]
pub struct Field_pd_prune_xid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PageHeaderData, pd_prune_xid) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pd_prune_xid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PageHeaderData>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PageHeaderData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PageHeaderData> as c::expression::OrdinaryField<Field_pd_prune_xid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pd_prune_xid) };
#[doc(hidden)]
pub struct Field_pd_linp;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PageHeaderData, pd_linp) == 24);
impl c::expression::OffsetField<Field_pd_linp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PageHeaderData>
{
    type Member = ();
    const OFFSET: usize =
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::PageHeaderData, pd_linp);
}
const _: () = {
    assert!(
        ::core::mem::size_of::<
            crate::__pgrx_c_bindings::__IncompleteArrayField<crate::__pgrx_c_bindings::ItemIdData>,
        >() == 0
    );
    assert!(
        ::core::mem::align_of::<
            crate::__pgrx_c_bindings::__IncompleteArrayField<crate::__pgrx_c_bindings::ItemIdData>,
        >() == 4
    );
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pd_linp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PageHeaderData>
{
    type Member = c::expression::CFlexibleArray<
        c::expression::CRecord<crate::__pgrx_c_bindings::ItemIdData>,
        crate::__pgrx_c_bindings::__IncompleteArrayField<crate::__pgrx_c_bindings::ItemIdData>,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PageHeaderData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PageHeaderData> as c::expression::OrdinaryField<Field_pd_linp>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pd_linp) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
pub struct Field_tbmiterator;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ParallelBitmapHeapState>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ParallelBitmapHeapState>() == 8);
};
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ParallelBitmapHeapState {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelBitmapHeapState, tbmiterator) == 0
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tbmiterator>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelBitmapHeapState>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelBitmapHeapState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelBitmapHeapState> as c::expression::OrdinaryField<Field_tbmiterator>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tbmiterator) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
#[doc(hidden)]
pub struct Field_prefetch_iterator;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelBitmapHeapState, prefetch_iterator)
        == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_prefetch_iterator>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelBitmapHeapState>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelBitmapHeapState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelBitmapHeapState> as c::expression::OrdinaryField<Field_prefetch_iterator>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).prefetch_iterator) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelBitmapHeapState, mutex) == 16
);
