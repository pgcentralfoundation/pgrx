// Shared generated C macro support.

impl c::sealed::Sealed
    for Signature_struct__MemoryContextData__0p__0l__struct__MemoryContextData__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_struct__MemoryContextData__0p__0l__struct__MemoryContextData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::MemoryContextData,
            )
                -> *mut crate::__pgrx_c_bindings::MemoryContextData,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
            )
                -> *mut crate::__pgrx_c_bindings::MemoryContextData,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
            )
                -> *mut crate::__pgrx_c_bindings::MemoryContextData,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__MemoryContextData__0p__0l__void__0p__0r;
impl c::sealed::Sealed for Signature_struct__MemoryContextData__0p__0l__void__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_struct__MemoryContextData__0p__0l__void__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
            )
                -> *mut crate::__pgrx_c_bindings::MemoryContextData,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
            )
                -> *mut crate::__pgrx_c_bindings::MinimalTupleData,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
            )
                -> *mut crate::__pgrx_c_bindings::MinimalTupleData,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__MinimalTupleData__0p__0l__struct__TupleTableSlot__0p__0r;
impl c::sealed::Sealed
    for Signature_struct__MinimalTupleData__0p__0l__struct__TupleTableSlot__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_struct__MinimalTupleData__0p__0l__struct__TupleTableSlot__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
            )
                -> *mut crate::__pgrx_c_bindings::MinimalTupleData,
        >,
    >;
}
const _: () = assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::TupleTableSlot>() == 8);
const _: () =
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::TupleTableSlot>() == 8);
const _: () =
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::MinimalTupleData>() == 8);
const _: () =
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::MinimalTupleData>() == 8);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<
    A0: c::expression::ImplicitTo<
            c::expression::CPointer<
                c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot>,
                c::expression::ReadWrite,
            >,
        >,
> c::expression::Call<(A0,)>
    for Signature_struct__MinimalTupleData__0p__0l__struct__TupleTableSlot__0p__0r
{
    type Output = <c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::MinimalTupleData>,
        c::expression::ReadWrite,
    > as c::expression::CType>::Value;
    unsafe fn call(pointer: Self::Pointer, args: (A0,)) -> Self::Output {
        let function = pointer.expect("C indirect call requires a non-null function pointer");
        const {
            assert!(!::core::mem::needs_drop::<A0>());
        }
        const _: () =
            assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::TupleTableSlot>());
        let native0 = <c::expression::CPointer<
            c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot>,
            c::expression::ReadWrite,
        > as c::expression::CType>::into_storage(c::expression::implicit::<
            c::expression::CPointer<
                c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot>,
                c::expression::ReadWrite,
            >,
            _,
        >(args.0));
        // SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
        let result = unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0)) };
        <c::expression::CPointer<
            c::expression::CRecord<crate::__pgrx_c_bindings::MinimalTupleData>,
            c::expression::ReadWrite,
        > as c::expression::CType>::from_storage(result)
    }
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::CustomScan,
            ) -> *mut crate::__pgrx_c_bindings::Node,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::CustomScan,
            ) -> *mut crate::__pgrx_c_bindings::Node,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__Node__0p__0l__struct__CustomScan__0p__0r;
impl c::sealed::Sealed for Signature_struct__Node__0p__0l__struct__CustomScan__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_struct__Node__0p__0l__struct__CustomScan__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::CustomScan,
            ) -> *mut crate::__pgrx_c_bindings::Node,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ParseState,
                *mut crate::__pgrx_c_bindings::ColumnRef,
            ) -> *mut crate::__pgrx_c_bindings::Node,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ParseState,
                *mut crate::__pgrx_c_bindings::ColumnRef,
            ) -> *mut crate::__pgrx_c_bindings::Node,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__Node__0p__0l__struct__ParseState__0p__0c__struct__ColumnRef__0p__0r;
impl c::sealed::Sealed
    for Signature_struct__Node__0p__0l__struct__ParseState__0p__0c__struct__ColumnRef__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_struct__Node__0p__0l__struct__ParseState__0p__0c__struct__ColumnRef__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ParseState,
                *mut crate::__pgrx_c_bindings::ColumnRef,
            ) -> *mut crate::__pgrx_c_bindings::Node,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ParseState,
                *mut crate::__pgrx_c_bindings::ColumnRef,
                *mut crate::__pgrx_c_bindings::Node,
            ) -> *mut crate::__pgrx_c_bindings::Node,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ParseState,
                *mut crate::__pgrx_c_bindings::ColumnRef,
                *mut crate::__pgrx_c_bindings::Node,
            ) -> *mut crate::__pgrx_c_bindings::Node,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__Node__0p__0l__struct__ParseState__0p__0c__struct__ColumnRef__0p__0c__struct__Node__0p__0r;
impl c::sealed::Sealed for Signature_struct__Node__0p__0l__struct__ParseState__0p__0c__struct__ColumnRef__0p__0c__struct__Node__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_struct__Node__0p__0l__struct__ParseState__0p__0c__struct__ColumnRef__0p__0c__struct__Node__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ParseState, *mut crate::__pgrx_c_bindings::ColumnRef, *mut crate::__pgrx_c_bindings::Node) -> *mut crate::__pgrx_c_bindings::Node>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ParseState,
                *mut crate::__pgrx_c_bindings::Param,
                crate::__pgrx_c_bindings::Oid,
                i32,
                i32,
            ) -> *mut crate::__pgrx_c_bindings::Node,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ParseState,
                *mut crate::__pgrx_c_bindings::Param,
                crate::__pgrx_c_bindings::Oid,
                i32,
                i32,
            ) -> *mut crate::__pgrx_c_bindings::Node,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__Node__0p__0l__struct__ParseState__0p__0c__struct__Param__0p__0c__unsigned__int__0c__int__0c__int__0r;
impl c::sealed::Sealed for Signature_struct__Node__0p__0l__struct__ParseState__0p__0c__struct__Param__0p__0c__unsigned__int__0c__int__0c__int__0r {}
impl c::expression::NativeFunctionSignature for Signature_struct__Node__0p__0l__struct__ParseState__0p__0c__struct__Param__0p__0c__unsigned__int__0c__int__0c__int__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ParseState, *mut crate::__pgrx_c_bindings::Param, crate::__pgrx_c_bindings::Oid, i32, i32) -> *mut crate::__pgrx_c_bindings::Node>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ParseState,
                *mut crate::__pgrx_c_bindings::ParamRef,
            ) -> *mut crate::__pgrx_c_bindings::Node,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ParseState,
                *mut crate::__pgrx_c_bindings::ParamRef,
            ) -> *mut crate::__pgrx_c_bindings::Node,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__Node__0p__0l__struct__ParseState__0p__0c__struct__ParamRef__0p__0r;
impl c::sealed::Sealed
    for Signature_struct__Node__0p__0l__struct__ParseState__0p__0c__struct__ParamRef__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_struct__Node__0p__0l__struct__ParseState__0p__0c__struct__ParamRef__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ParseState,
                *mut crate::__pgrx_c_bindings::ParamRef,
            ) -> *mut crate::__pgrx_c_bindings::Node,
        >,
    >;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::Var,
                *mut crate::__pgrx_c_bindings::replace_rte_variables_context,
            ) -> *mut crate::__pgrx_c_bindings::Node,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::Var,
                *mut crate::__pgrx_c_bindings::replace_rte_variables_context,
            ) -> *mut crate::__pgrx_c_bindings::Node,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__Node__0p__0l__struct__Var__0p__0c__struct__replace_rte_variables_context__0p__0r;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::sealed::Sealed for Signature_struct__Node__0p__0l__struct__Var__0p__0c__struct__replace_rte_variables_context__0p__0r {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::NativeFunctionSignature for Signature_struct__Node__0p__0l__struct__Var__0p__0c__struct__replace_rte_variables_context__0p__0r {
 type Physical = PhysicalFunction_C_unwind_2<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::Var, *mut crate::__pgrx_c_bindings::replace_rte_variables_context) -> *mut crate::__pgrx_c_bindings::Node>>;
}
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_void,
            ) -> *mut crate::__pgrx_c_bindings::Node,
        >,
    >() == 8
);
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_void,
            ) -> *mut crate::__pgrx_c_bindings::Node,
        >,
    >() == 8
);
#[cfg(feature = "pg15")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__Node__0p__0l__const__void__0p__0r;
#[cfg(feature = "pg15")]
impl c::sealed::Sealed for Signature_struct__Node__0p__0l__const__void__0p__0r {}
#[cfg(feature = "pg15")]
impl c::expression::NativeFunctionSignature
    for Signature_struct__Node__0p__0l__const__void__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_void,
            ) -> *mut crate::__pgrx_c_bindings::Node,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::proclist_head,
                usize,
            ) -> *mut crate::__pgrx_c_bindings::PGPROC,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::proclist_head,
                usize,
            ) -> *mut crate::__pgrx_c_bindings::PGPROC,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__PGPROC__0p__0l__struct__proclist_head__0p__0c__unsigned__long__0r;
impl c::sealed::Sealed
    for Signature_struct__PGPROC__0p__0l__struct__proclist_head__0p__0c__unsigned__long__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_struct__PGPROC__0p__0l__struct__proclist_head__0p__0c__unsigned__long__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::proclist_head,
                usize,
            ) -> *mut crate::__pgrx_c_bindings::PGPROC,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ParamListInfoData,
                i32,
                bool,
                *mut crate::__pgrx_c_bindings::ParamExternData,
            )
                -> *mut crate::__pgrx_c_bindings::ParamExternData,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ParamListInfoData,
                i32,
                bool,
                *mut crate::__pgrx_c_bindings::ParamExternData,
            )
                -> *mut crate::__pgrx_c_bindings::ParamExternData,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__ParamExternData__0p__0l__struct__ParamListInfoData__0p__0c__int__0c__0x5f426f6f6c__0c__struct__ParamExternData__0p__0r;
impl c::sealed::Sealed for Signature_struct__ParamExternData__0p__0l__struct__ParamListInfoData__0p__0c__int__0c__0x5f426f6f6c__0c__struct__ParamExternData__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_struct__ParamExternData__0p__0l__struct__ParamListInfoData__0p__0c__int__0c__0x5f426f6f6c__0c__struct__ParamExternData__0p__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ParamListInfoData, i32, bool, *mut crate::__pgrx_c_bindings::ParamExternData) -> *mut crate::__pgrx_c_bindings::ParamExternData>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::List,
            ) -> *mut crate::__pgrx_c_bindings::PathTarget,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::List,
            ) -> *mut crate::__pgrx_c_bindings::PathTarget,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__PathTarget__0p__0l__struct__List__0p__0r;
impl c::sealed::Sealed for Signature_struct__PathTarget__0p__0l__struct__List__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_struct__PathTarget__0p__0l__struct__List__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::List,
            ) -> *mut crate::__pgrx_c_bindings::PathTarget,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerInfo,
                *mut crate::__pgrx_c_bindings::PathTarget,
            ) -> *mut crate::__pgrx_c_bindings::PathTarget,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerInfo,
                *mut crate::__pgrx_c_bindings::PathTarget,
            ) -> *mut crate::__pgrx_c_bindings::PathTarget,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__PathTarget__0p__0l__struct__PlannerInfo__0p__0c__struct__PathTarget__0p__0r;
impl c::sealed::Sealed for Signature_struct__PathTarget__0p__0l__struct__PlannerInfo__0p__0c__struct__PathTarget__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_struct__PathTarget__0p__0l__struct__PlannerInfo__0p__0c__struct__PathTarget__0p__0r {
 type Physical = PhysicalFunction_C_unwind_2<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PlannerInfo, *mut crate::__pgrx_c_bindings::PathTarget) -> *mut crate::__pgrx_c_bindings::PathTarget>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerInfo,
                *mut crate::__pgrx_c_bindings::RelOptInfo,
                *mut crate::__pgrx_c_bindings::CustomPath,
                *mut crate::__pgrx_c_bindings::List,
                *mut crate::__pgrx_c_bindings::List,
                *mut crate::__pgrx_c_bindings::List,
            ) -> *mut crate::__pgrx_c_bindings::Plan,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerInfo,
                *mut crate::__pgrx_c_bindings::RelOptInfo,
                *mut crate::__pgrx_c_bindings::CustomPath,
                *mut crate::__pgrx_c_bindings::List,
                *mut crate::__pgrx_c_bindings::List,
                *mut crate::__pgrx_c_bindings::List,
            ) -> *mut crate::__pgrx_c_bindings::Plan,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__Plan__0p__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__CustomPath__0p__0c__struct__List__0p__0c__struct__List__0p__0c__struct__List__0p__0r;
impl c::sealed::Sealed for Signature_struct__Plan__0p__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__CustomPath__0p__0c__struct__List__0p__0c__struct__List__0p__0c__struct__List__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_struct__Plan__0p__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__CustomPath__0p__0c__struct__List__0p__0c__struct__List__0p__0c__struct__List__0p__0r {
 type Physical = PhysicalFunction_C_unwind_6<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PlannerInfo, *mut crate::__pgrx_c_bindings::RelOptInfo, *mut crate::__pgrx_c_bindings::CustomPath, *mut crate::__pgrx_c_bindings::List, *mut crate::__pgrx_c_bindings::List, *mut crate::__pgrx_c_bindings::List) -> *mut crate::__pgrx_c_bindings::Plan>>;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::Query,
                *const ::core::ffi::c_char,
                i32,
                *mut crate::__pgrx_c_bindings::ParamListInfoData,
            ) -> *mut crate::__pgrx_c_bindings::PlannedStmt,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::Query,
                *const ::core::ffi::c_char,
                i32,
                *mut crate::__pgrx_c_bindings::ParamListInfoData,
            ) -> *mut crate::__pgrx_c_bindings::PlannedStmt,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__PlannedStmt__0p__0l__struct__Query__0p__0c__const__char__0p__0c__int__0c__struct__ParamListInfoData__0p__0r;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::sealed::Sealed for Signature_struct__PlannedStmt__0p__0l__struct__Query__0p__0c__const__char__0p__0c__int__0c__struct__ParamListInfoData__0p__0r {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::NativeFunctionSignature for Signature_struct__PlannedStmt__0p__0l__struct__Query__0p__0c__const__char__0p__0c__int__0c__struct__ParamListInfoData__0p__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::Query, *const ::core::ffi::c_char, i32, *mut crate::__pgrx_c_bindings::ParamListInfoData) -> *mut crate::__pgrx_c_bindings::PlannedStmt>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RBTreeIterator,
            ) -> *mut crate::__pgrx_c_bindings::RBTNode,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RBTreeIterator,
            ) -> *mut crate::__pgrx_c_bindings::RBTNode,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__RBTNode__0p__0l__struct__RBTreeIterator__0p__0r;
impl c::sealed::Sealed for Signature_struct__RBTNode__0p__0l__struct__RBTreeIterator__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_struct__RBTNode__0p__0l__struct__RBTreeIterator__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RBTreeIterator,
            ) -> *mut crate::__pgrx_c_bindings::RBTNode,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
            ) -> *mut crate::__pgrx_c_bindings::RBTNode,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
            ) -> *mut crate::__pgrx_c_bindings::RBTNode,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__RBTNode__0p__0l__void__0p__0r;
impl c::sealed::Sealed for Signature_struct__RBTNode__0p__0l__void__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_struct__RBTNode__0p__0l__void__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
            ) -> *mut crate::__pgrx_c_bindings::RBTNode,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerInfo,
                i32,
                *mut crate::__pgrx_c_bindings::List,
            ) -> *mut crate::__pgrx_c_bindings::RelOptInfo,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerInfo,
                i32,
                *mut crate::__pgrx_c_bindings::List,
            ) -> *mut crate::__pgrx_c_bindings::RelOptInfo,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__RelOptInfo__0p__0l__struct__PlannerInfo__0p__0c__int__0c__struct__List__0p__0r;
impl c::sealed::Sealed for Signature_struct__RelOptInfo__0p__0l__struct__PlannerInfo__0p__0c__int__0c__struct__List__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_struct__RelOptInfo__0p__0l__struct__PlannerInfo__0p__0c__int__0c__struct__List__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PlannerInfo, i32, *mut crate::__pgrx_c_bindings::List) -> *mut crate::__pgrx_c_bindings::RelOptInfo>>;
}
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerInfo,
                *mut crate::__pgrx_c_bindings::Expr,
                bool,
                bool,
                bool,
                u32,
                *mut crate::__pgrx_c_bindings::Bitmapset,
                *mut crate::__pgrx_c_bindings::Bitmapset,
                *mut crate::__pgrx_c_bindings::Bitmapset,
            ) -> *mut crate::__pgrx_c_bindings::RestrictInfo,
        >,
    >() == 8
);
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerInfo,
                *mut crate::__pgrx_c_bindings::Expr,
                bool,
                bool,
                bool,
                u32,
                *mut crate::__pgrx_c_bindings::Bitmapset,
                *mut crate::__pgrx_c_bindings::Bitmapset,
                *mut crate::__pgrx_c_bindings::Bitmapset,
            ) -> *mut crate::__pgrx_c_bindings::RestrictInfo,
        >,
    >() == 8
);
#[cfg(feature = "pg15")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__RestrictInfo__0p__0l__struct__PlannerInfo__0p__0c__struct__Expr__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__unsigned__int__0c__struct__Bitmapset__0p__0c__struct__Bitmapset__0p__0c__struct__Bitmapset__0p__0r;
#[cfg(feature = "pg15")]
impl c::sealed::Sealed for Signature_struct__RestrictInfo__0p__0l__struct__PlannerInfo__0p__0c__struct__Expr__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__unsigned__int__0c__struct__Bitmapset__0p__0c__struct__Bitmapset__0p__0c__struct__Bitmapset__0p__0r {}
#[cfg(feature = "pg15")]
impl c::expression::NativeFunctionSignature for Signature_struct__RestrictInfo__0p__0l__struct__PlannerInfo__0p__0c__struct__Expr__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__unsigned__int__0c__struct__Bitmapset__0p__0c__struct__Bitmapset__0p__0c__struct__Bitmapset__0p__0r {
 type Physical = PhysicalFunction_C_unwind_9<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PlannerInfo, *mut crate::__pgrx_c_bindings::Expr, bool, bool, bool, u32, *mut crate::__pgrx_c_bindings::Bitmapset, *mut crate::__pgrx_c_bindings::Bitmapset, *mut crate::__pgrx_c_bindings::Bitmapset) -> *mut crate::__pgrx_c_bindings::RestrictInfo>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u8,
            ) -> ::core::mem::MaybeUninit<
                crate::__pgrx_c_bindings::RmgrData,
            >,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u8,
            ) -> ::core::mem::MaybeUninit<
                crate::__pgrx_c_bindings::RmgrData,
            >,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__RmgrData__0l__unsigned__char__0r;
impl c::sealed::Sealed for Signature_struct__RmgrData__0l__unsigned__char__0r {}
impl c::expression::NativeFunctionSignature for Signature_struct__RmgrData__0l__unsigned__char__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u8,
            ) -> ::core::mem::MaybeUninit<
                crate::__pgrx_c_bindings::RmgrData,
            >,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
            )
                -> *mut crate::__pgrx_c_bindings::SMgrRelationData,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
            )
                -> *mut crate::__pgrx_c_bindings::SMgrRelationData,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__SMgrRelationData__0p__0l__struct__RelationData__0p__0r;
impl c::sealed::Sealed
    for Signature_struct__SMgrRelationData__0p__0l__struct__RelationData__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_struct__SMgrRelationData__0p__0l__struct__RelationData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
            )
                -> *mut crate::__pgrx_c_bindings::SMgrRelationData,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                i32,
                *mut crate::__pgrx_c_bindings::ScanKeyData,
                *mut crate::__pgrx_c_bindings::ParallelTableScanDescData,
                u32,
            )
                -> *mut crate::__pgrx_c_bindings::TableScanDescData,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                i32,
                *mut crate::__pgrx_c_bindings::ScanKeyData,
                *mut crate::__pgrx_c_bindings::ParallelTableScanDescData,
                u32,
            )
                -> *mut crate::__pgrx_c_bindings::TableScanDescData,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0c__int__0c__struct__ScanKeyData__0p__0c__struct__ParallelTableScanDescData__0p__0c__unsigned__int__0r;
impl c::sealed::Sealed for Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0c__int__0c__struct__ScanKeyData__0p__0c__struct__ParallelTableScanDescData__0p__0c__unsigned__int__0r {}
impl c::expression::NativeFunctionSignature for Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0c__int__0c__struct__ScanKeyData__0p__0c__struct__ParallelTableScanDescData__0p__0c__unsigned__int__0r {
 type Physical = PhysicalFunction_C_unwind_6<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::SnapshotData, i32, *mut crate::__pgrx_c_bindings::ScanKeyData, *mut crate::__pgrx_c_bindings::ParallelTableScanDescData, u32) -> *mut crate::__pgrx_c_bindings::TableScanDescData>>;
}
#[cfg(any(feature = "pg15", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
            )
                -> *mut crate::__pgrx_c_bindings::TableScanDescData,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
            )
                -> *mut crate::__pgrx_c_bindings::TableScanDescData,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0r;
#[cfg(any(feature = "pg15", feature = "pg19"))]
impl c::sealed::Sealed
    for Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0r
{
}
#[cfg(any(feature = "pg15", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
            )
                -> *mut crate::__pgrx_c_bindings::TableScanDescData,
        >,
    >;
}
#[cfg(any(feature = "pg15", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::SnapshotData,
            )
                -> *mut crate::__pgrx_c_bindings::TableScanDescData,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::SnapshotData,
            )
                -> *mut crate::__pgrx_c_bindings::TableScanDescData,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0r;
#[cfg(any(feature = "pg15", feature = "pg19"))]
impl c::sealed::Sealed for Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0r {}
#[cfg(any(feature = "pg15", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0r {
 type Physical = PhysicalFunction_C_unwind_2<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::SnapshotData) -> *mut crate::__pgrx_c_bindings::TableScanDescData>>;
}
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                i32,
                *mut crate::__pgrx_c_bindings::ScanKeyData,
            )
                -> *mut crate::__pgrx_c_bindings::TableScanDescData,
        >,
    >() == 8
);
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                i32,
                *mut crate::__pgrx_c_bindings::ScanKeyData,
            )
                -> *mut crate::__pgrx_c_bindings::TableScanDescData,
        >,
    >() == 8
);
#[cfg(feature = "pg15")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0c__int__0c__struct__ScanKeyData__0p__0r;
#[cfg(feature = "pg15")]
impl c::sealed::Sealed for Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0c__int__0c__struct__ScanKeyData__0p__0r {}
#[cfg(feature = "pg15")]
impl c::expression::NativeFunctionSignature for Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0c__int__0c__struct__ScanKeyData__0p__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::SnapshotData, i32, *mut crate::__pgrx_c_bindings::ScanKeyData) -> *mut crate::__pgrx_c_bindings::TableScanDescData>>;
}
#[cfg(any(feature = "pg15", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                i32,
                *mut crate::__pgrx_c_bindings::ScanKeyData,
                bool,
                bool,
            )
                -> *mut crate::__pgrx_c_bindings::TableScanDescData,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                i32,
                *mut crate::__pgrx_c_bindings::ScanKeyData,
                bool,
                bool,
            )
                -> *mut crate::__pgrx_c_bindings::TableScanDescData,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0c__int__0c__struct__ScanKeyData__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0r;
#[cfg(any(feature = "pg15", feature = "pg19"))]
impl c::sealed::Sealed for Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0c__int__0c__struct__ScanKeyData__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0r {}
#[cfg(any(feature = "pg15", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0c__int__0c__struct__ScanKeyData__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0r {
 type Physical = PhysicalFunction_C_unwind_6<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::SnapshotData, i32, *mut crate::__pgrx_c_bindings::ScanKeyData, bool, bool) -> *mut crate::__pgrx_c_bindings::TableScanDescData>>;
}
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                i32,
                *mut crate::__pgrx_c_bindings::ScanKeyData,
                bool,
                bool,
                bool,
            )
                -> *mut crate::__pgrx_c_bindings::TableScanDescData,
        >,
    >() == 8
);
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                i32,
                *mut crate::__pgrx_c_bindings::ScanKeyData,
                bool,
                bool,
                bool,
            )
                -> *mut crate::__pgrx_c_bindings::TableScanDescData,
        >,
    >() == 8
);
#[cfg(feature = "pg15")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0c__int__0c__struct__ScanKeyData__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0r;
#[cfg(feature = "pg15")]
impl c::sealed::Sealed for Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0c__int__0c__struct__ScanKeyData__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0r {}
#[cfg(feature = "pg15")]
impl c::expression::NativeFunctionSignature for Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0c__int__0c__struct__ScanKeyData__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0r {
 type Physical = PhysicalFunction_C_unwind_7<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::SnapshotData, i32, *mut crate::__pgrx_c_bindings::ScanKeyData, bool, bool, bool) -> *mut crate::__pgrx_c_bindings::TableScanDescData>>;
}
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                *mut crate::__pgrx_c_bindings::ItemPointerData,
                *mut crate::__pgrx_c_bindings::ItemPointerData,
            )
                -> *mut crate::__pgrx_c_bindings::TableScanDescData,
        >,
    >() == 8
);
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                *mut crate::__pgrx_c_bindings::ItemPointerData,
                *mut crate::__pgrx_c_bindings::ItemPointerData,
            )
                -> *mut crate::__pgrx_c_bindings::TableScanDescData,
        >,
    >() == 8
);
#[cfg(feature = "pg15")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0c__struct__ItemPointerData__0p__0c__struct__ItemPointerData__0p__0r;
#[cfg(feature = "pg15")]
impl c::sealed::Sealed for Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0c__struct__ItemPointerData__0p__0c__struct__ItemPointerData__0p__0r {}
#[cfg(feature = "pg15")]
impl c::expression::NativeFunctionSignature for Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0c__struct__ItemPointerData__0p__0c__struct__ItemPointerData__0p__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::SnapshotData, *mut crate::__pgrx_c_bindings::ItemPointerData, *mut crate::__pgrx_c_bindings::ItemPointerData) -> *mut crate::__pgrx_c_bindings::TableScanDescData>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ExpandedRecordHeader,
            )
                -> *mut crate::__pgrx_c_bindings::TupleDescData,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ExpandedRecordHeader,
            )
                -> *mut crate::__pgrx_c_bindings::TupleDescData,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__TupleDescData__0p__0l__struct__ExpandedRecordHeader__0p__0r;
impl c::sealed::Sealed
    for Signature_struct__TupleDescData__0p__0l__struct__ExpandedRecordHeader__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_struct__TupleDescData__0p__0l__struct__ExpandedRecordHeader__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ExpandedRecordHeader,
            )
                -> *mut crate::__pgrx_c_bindings::TupleDescData,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::tuplehash_hash,
                *mut crate::__pgrx_c_bindings::tuplehash_iterator,
            )
                -> *mut crate::__pgrx_c_bindings::TupleHashEntryData,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::tuplehash_hash,
                *mut crate::__pgrx_c_bindings::tuplehash_iterator,
            )
                -> *mut crate::__pgrx_c_bindings::TupleHashEntryData,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__TupleHashEntryData__0p__0l__struct__tuplehash_hash__0p__0c__struct__tuplehash_iterator__0p__0r;
impl c::sealed::Sealed for Signature_struct__TupleHashEntryData__0p__0l__struct__tuplehash_hash__0p__0c__struct__tuplehash_iterator__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_struct__TupleHashEntryData__0p__0l__struct__tuplehash_hash__0p__0c__struct__tuplehash_iterator__0p__0r {
 type Physical = PhysicalFunction_C_unwind_2<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::tuplehash_hash, *mut crate::__pgrx_c_bindings::tuplehash_iterator) -> *mut crate::__pgrx_c_bindings::TupleHashEntryData>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::CustomScanState,
            )
                -> *mut crate::__pgrx_c_bindings::TupleTableSlot,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::CustomScanState,
            )
                -> *mut crate::__pgrx_c_bindings::TupleTableSlot,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__TupleTableSlot__0p__0l__struct__CustomScanState__0p__0r;
impl c::sealed::Sealed
    for Signature_struct__TupleTableSlot__0p__0l__struct__CustomScanState__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_struct__TupleTableSlot__0p__0l__struct__CustomScanState__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::CustomScanState,
            )
                -> *mut crate::__pgrx_c_bindings::TupleTableSlot,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::EState,
                *mut crate::__pgrx_c_bindings::ResultRelInfo,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
            )
                -> *mut crate::__pgrx_c_bindings::TupleTableSlot,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::EState,
                *mut crate::__pgrx_c_bindings::ResultRelInfo,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
            )
                -> *mut crate::__pgrx_c_bindings::TupleTableSlot,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__TupleTableSlot__0p__0l__struct__EState__0p__0c__struct__ResultRelInfo__0p__0c__struct__TupleTableSlot__0p__0c__struct__TupleTableSlot__0p__0r;
impl c::sealed::Sealed for Signature_struct__TupleTableSlot__0p__0l__struct__EState__0p__0c__struct__ResultRelInfo__0p__0c__struct__TupleTableSlot__0p__0c__struct__TupleTableSlot__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_struct__TupleTableSlot__0p__0l__struct__EState__0p__0c__struct__ResultRelInfo__0p__0c__struct__TupleTableSlot__0p__0c__struct__TupleTableSlot__0p__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::EState, *mut crate::__pgrx_c_bindings::ResultRelInfo, *mut crate::__pgrx_c_bindings::TupleTableSlot, *mut crate::__pgrx_c_bindings::TupleTableSlot) -> *mut crate::__pgrx_c_bindings::TupleTableSlot>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ForeignScanState,
            )
                -> *mut crate::__pgrx_c_bindings::TupleTableSlot,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ForeignScanState,
            )
                -> *mut crate::__pgrx_c_bindings::TupleTableSlot,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__TupleTableSlot__0p__0l__struct__ForeignScanState__0p__0r;
impl c::sealed::Sealed
    for Signature_struct__TupleTableSlot__0p__0l__struct__ForeignScanState__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_struct__TupleTableSlot__0p__0l__struct__ForeignScanState__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ForeignScanState,
            )
                -> *mut crate::__pgrx_c_bindings::TupleTableSlot,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlanState,
            )
                -> *mut crate::__pgrx_c_bindings::TupleTableSlot,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlanState,
            )
                -> *mut crate::__pgrx_c_bindings::TupleTableSlot,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__TupleTableSlot__0p__0l__struct__PlanState__0p__0r;
impl c::sealed::Sealed for Signature_struct__TupleTableSlot__0p__0l__struct__PlanState__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_struct__TupleTableSlot__0p__0l__struct__PlanState__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlanState,
            )
                -> *mut crate::__pgrx_c_bindings::TupleTableSlot,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ScanState,
            )
                -> *mut crate::__pgrx_c_bindings::TupleTableSlot,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ScanState,
            )
                -> *mut crate::__pgrx_c_bindings::TupleTableSlot,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__TupleTableSlot__0p__0l__struct__ScanState__0p__0r;
impl c::sealed::Sealed for Signature_struct__TupleTableSlot__0p__0l__struct__ScanState__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_struct__TupleTableSlot__0p__0l__struct__ScanState__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ScanState,
            )
                -> *mut crate::__pgrx_c_bindings::TupleTableSlot,
        >,
    >;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ProjectionInfo,
            )
                -> *mut crate::__pgrx_c_bindings::TupleTableSlot,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ProjectionInfo,
            )
                -> *mut crate::__pgrx_c_bindings::TupleTableSlot,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__TupleTableSlot__0p__0l__struct__ProjectionInfo__0p__0r;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
impl c::sealed::Sealed
    for Signature_struct__TupleTableSlot__0p__0l__struct__ProjectionInfo__0p__0r
{
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
impl c::expression::NativeFunctionSignature
    for Signature_struct__TupleTableSlot__0p__0l__struct__ProjectionInfo__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ProjectionInfo,
            )
                -> *mut crate::__pgrx_c_bindings::TupleTableSlot,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
            )
                -> *mut crate::__pgrx_c_bindings::TupleTableSlot,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
            )
                -> *mut crate::__pgrx_c_bindings::TupleTableSlot,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__TupleTableSlot__0p__0l__struct__TupleTableSlot__0p__0c__struct__TupleTableSlot__0p__0r;
impl c::sealed::Sealed for Signature_struct__TupleTableSlot__0p__0l__struct__TupleTableSlot__0p__0c__struct__TupleTableSlot__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_struct__TupleTableSlot__0p__0l__struct__TupleTableSlot__0p__0c__struct__TupleTableSlot__0p__0r {
 type Physical = PhysicalFunction_C_unwind_2<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TupleTableSlot, *mut crate::__pgrx_c_bindings::TupleTableSlot) -> *mut crate::__pgrx_c_bindings::TupleTableSlot>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::EState,
                *mut crate::__pgrx_c_bindings::ResultRelInfo,
                *mut *mut crate::__pgrx_c_bindings::TupleTableSlot,
                *mut *mut crate::__pgrx_c_bindings::TupleTableSlot,
                *mut i32,
            )
                -> *mut *mut crate::__pgrx_c_bindings::TupleTableSlot,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::EState,
                *mut crate::__pgrx_c_bindings::ResultRelInfo,
                *mut *mut crate::__pgrx_c_bindings::TupleTableSlot,
                *mut *mut crate::__pgrx_c_bindings::TupleTableSlot,
                *mut i32,
            )
                -> *mut *mut crate::__pgrx_c_bindings::TupleTableSlot,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__TupleTableSlot__0p__0p__0l__struct__EState__0p__0c__struct__ResultRelInfo__0p__0c__struct__TupleTableSlot__0p__0p__0c__struct__TupleTableSlot__0p__0p__0c__int__0p__0r;
impl c::sealed::Sealed for Signature_struct__TupleTableSlot__0p__0p__0l__struct__EState__0p__0c__struct__ResultRelInfo__0p__0c__struct__TupleTableSlot__0p__0p__0c__struct__TupleTableSlot__0p__0p__0c__int__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_struct__TupleTableSlot__0p__0p__0l__struct__EState__0p__0c__struct__ResultRelInfo__0p__0c__struct__TupleTableSlot__0p__0p__0c__struct__TupleTableSlot__0p__0p__0c__int__0p__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::EState, *mut crate::__pgrx_c_bindings::ResultRelInfo, *mut *mut crate::__pgrx_c_bindings::TupleTableSlot, *mut *mut crate::__pgrx_c_bindings::TupleTableSlot, *mut i32) -> *mut *mut crate::__pgrx_c_bindings::TupleTableSlot>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::WalReceiverConn,
                *const ::core::ffi::c_char,
                i32,
                *const crate::__pgrx_c_bindings::Oid,
            )
                -> *mut crate::__pgrx_c_bindings::WalRcvExecResult,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::WalReceiverConn,
                *const ::core::ffi::c_char,
                i32,
                *const crate::__pgrx_c_bindings::Oid,
            )
                -> *mut crate::__pgrx_c_bindings::WalRcvExecResult,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__WalRcvExecResult__0p__0l__struct__WalReceiverConn__0p__0c__const__char__0p__0c__int__0c__const__unsigned__int__0p__0r;
impl c::sealed::Sealed for Signature_struct__WalRcvExecResult__0p__0l__struct__WalReceiverConn__0p__0c__const__char__0p__0c__int__0c__const__unsigned__int__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_struct__WalRcvExecResult__0p__0l__struct__WalReceiverConn__0p__0c__const__char__0p__0c__int__0c__const__unsigned__int__0p__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::WalReceiverConn, *const ::core::ffi::c_char, i32, *const crate::__pgrx_c_bindings::Oid) -> *mut crate::__pgrx_c_bindings::WalRcvExecResult>>;
}
const _: () =
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::WalReceiverConn>() == 8);
const _: () =
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::WalReceiverConn>() == 8);
const _: () = assert!(::core::mem::size_of::<*const ::core::ffi::c_char>() == 8);
const _: () = assert!(::core::mem::align_of::<*const ::core::ffi::c_char>() == 8);
const _: () = assert!(::core::mem::size_of::<i32>() == 4);
const _: () = assert!(::core::mem::align_of::<i32>() == 4);
const _: () = assert!(::core::mem::size_of::<*const crate::__pgrx_c_bindings::Oid>() == 8);
const _: () = assert!(::core::mem::align_of::<*const crate::__pgrx_c_bindings::Oid>() == 8);
const _: () =
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::WalRcvExecResult>() == 8);
const _: () =
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::WalRcvExecResult>() == 8);
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<A0: c::expression::ImplicitTo<c::expression::CPointer<c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>, c::expression::ReadWrite>>, A1: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadOnly>>, A2: c::expression::ImplicitTo<c::CInt>, A3: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>, c::expression::ReadOnly>>> c::expression::Call<(A0,A1,A2,A3,)> for Signature_struct__WalRcvExecResult__0p__0l__struct__WalReceiverConn__0p__0c__const__char__0p__0c__int__0c__const__unsigned__int__0p__0r {
 type Output = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvExecResult>, c::expression::ReadWrite> as c::expression::CType>::Value;
 unsafe fn call(pointer: Self::Pointer, args: (A0,A1,A2,A3,)) -> Self::Output {
let function = pointer.expect("C indirect call requires a non-null function pointer");
const { assert!(!::core::mem::needs_drop::<A0>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::WalReceiverConn>());
let native0 = <c::expression::CPointer<c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>, c::expression::ReadWrite>, _>(args.0));
const { assert!(!::core::mem::needs_drop::<A1>()); }
const _: () = assert!(!::core::mem::needs_drop::<*const ::core::ffi::c_char>());
let native1 = <c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadOnly> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadOnly>, _>(args.1));
const { assert!(!::core::mem::needs_drop::<A2>()); }
const _: () = assert!(!::core::mem::needs_drop::<i32>());
let native2 = <c::CInt as c::expression::CType>::into_storage(c::expression::implicit::<c::CInt, _>(args.2));
const { assert!(!::core::mem::needs_drop::<A3>()); }
const _: () = assert!(!::core::mem::needs_drop::<*const crate::__pgrx_c_bindings::Oid>());
let native3 = <c::expression::CPointer<c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>, c::expression::ReadOnly> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>, c::expression::ReadOnly>, _>(args.3));
// SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
let result = unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0, native1, native2, native3)) };
<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvExecResult>, c::expression::ReadWrite> as c::expression::CType>::from_storage(result)
}
}
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
                bool,
                *const ::core::ffi::c_char,
                *mut *mut ::core::ffi::c_char,
            )
                -> *mut crate::__pgrx_c_bindings::WalReceiverConn,
        >,
    >() == 8
);
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
                bool,
                *const ::core::ffi::c_char,
                *mut *mut ::core::ffi::c_char,
            )
                -> *mut crate::__pgrx_c_bindings::WalReceiverConn,
        >,
    >() == 8
);
#[cfg(feature = "pg15")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__WalReceiverConn__0p__0l__const__char__0p__0c__0x5f426f6f6c__0c__const__char__0p__0c__char__0p__0p__0r;
#[cfg(feature = "pg15")]
impl c::sealed::Sealed for Signature_struct__WalReceiverConn__0p__0l__const__char__0p__0c__0x5f426f6f6c__0c__const__char__0p__0c__char__0p__0p__0r {}
#[cfg(feature = "pg15")]
impl c::expression::NativeFunctionSignature for Signature_struct__WalReceiverConn__0p__0l__const__char__0p__0c__0x5f426f6f6c__0c__const__char__0p__0c__char__0p__0p__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_char, bool, *const ::core::ffi::c_char, *mut *mut ::core::ffi::c_char) -> *mut crate::__pgrx_c_bindings::WalReceiverConn>>;
}
const _: () = assert!(::core::mem::size_of::<*const ::core::ffi::c_char>() == 8);
const _: () = assert!(::core::mem::align_of::<*const ::core::ffi::c_char>() == 8);
const _: () = assert!(::core::mem::size_of::<bool>() == 1);
const _: () = assert!(::core::mem::align_of::<bool>() == 1);
const _: () = assert!(::core::mem::size_of::<*const ::core::ffi::c_char>() == 8);
const _: () = assert!(::core::mem::align_of::<*const ::core::ffi::c_char>() == 8);
const _: () = assert!(::core::mem::size_of::<*mut *mut ::core::ffi::c_char>() == 8);
const _: () = assert!(::core::mem::align_of::<*mut *mut ::core::ffi::c_char>() == 8);
const _: () =
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::WalReceiverConn>() == 8);
const _: () =
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::WalReceiverConn>() == 8);
#[cfg(feature = "pg15")]
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<A0: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadOnly>>, A1: c::expression::ImplicitTo<c::CBool>, A2: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadOnly>>, A3: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadWrite>, c::expression::ReadWrite>>> c::expression::Call<(A0,A1,A2,A3,)> for Signature_struct__WalReceiverConn__0p__0l__const__char__0p__0c__0x5f426f6f6c__0c__const__char__0p__0c__char__0p__0p__0r {
 type Output = <c::expression::CPointer<c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>, c::expression::ReadWrite> as c::expression::CType>::Value;
 unsafe fn call(pointer: Self::Pointer, args: (A0,A1,A2,A3,)) -> Self::Output {
let function = pointer.expect("C indirect call requires a non-null function pointer");
const { assert!(!::core::mem::needs_drop::<A0>()); }
const _: () = assert!(!::core::mem::needs_drop::<*const ::core::ffi::c_char>());
let native0 = <c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadOnly> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadOnly>, _>(args.0));
const { assert!(!::core::mem::needs_drop::<A1>()); }
const _: () = assert!(!::core::mem::needs_drop::<bool>());
let native1 = <c::CBool as c::expression::CType>::into_storage(c::expression::implicit::<c::CBool, _>(args.1));
const { assert!(!::core::mem::needs_drop::<A2>()); }
const _: () = assert!(!::core::mem::needs_drop::<*const ::core::ffi::c_char>());
let native2 = <c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadOnly> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadOnly>, _>(args.2));
const { assert!(!::core::mem::needs_drop::<A3>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut *mut ::core::ffi::c_char>());
let native3 = <c::expression::CPointer<c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadWrite>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadWrite>, c::expression::ReadWrite>, _>(args.3));
// SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
let result = unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0, native1, native2, native3)) };
<c::expression::CPointer<c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>, c::expression::ReadWrite> as c::expression::CType>::from_storage(result)
}
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                i32,
                i32,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::catclist,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                i32,
                i32,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::catclist,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__catclist__0p__0l__int__0c__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::sealed::Sealed for Signature_struct__catclist__0p__0l__int__0c__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::NativeFunctionSignature for Signature_struct__catclist__0p__0l__int__0c__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(i32, i32, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum) -> *mut crate::__pgrx_c_bindings::catclist>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::dlist_head,
            ) -> *mut crate::__pgrx_c_bindings::dlist_node,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::dlist_head,
            ) -> *mut crate::__pgrx_c_bindings::dlist_node,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__dlist_node__0p__0l__struct__dlist_head__0p__0r;
impl c::sealed::Sealed for Signature_struct__dlist_node__0p__0l__struct__dlist_head__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_struct__dlist_node__0p__0l__struct__dlist_head__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::dlist_head,
            ) -> *mut crate::__pgrx_c_bindings::dlist_node,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::dlist_head,
                *mut crate::__pgrx_c_bindings::dlist_node,
            ) -> *mut crate::__pgrx_c_bindings::dlist_node,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::dlist_head,
                *mut crate::__pgrx_c_bindings::dlist_node,
            ) -> *mut crate::__pgrx_c_bindings::dlist_node,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__dlist_node__0p__0l__struct__dlist_head__0p__0c__struct__dlist_node__0p__0r;
impl c::sealed::Sealed
    for Signature_struct__dlist_node__0p__0l__struct__dlist_head__0p__0c__struct__dlist_node__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_struct__dlist_node__0p__0l__struct__dlist_head__0p__0c__struct__dlist_node__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::dlist_head,
                *mut crate::__pgrx_c_bindings::dlist_node,
            ) -> *mut crate::__pgrx_c_bindings::dlist_node,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::slist_head,
            ) -> *mut crate::__pgrx_c_bindings::slist_node,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::slist_head,
            ) -> *mut crate::__pgrx_c_bindings::slist_node,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__slist_node__0p__0l__struct__slist_head__0p__0r;
impl c::sealed::Sealed for Signature_struct__slist_node__0p__0l__struct__slist_head__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_struct__slist_node__0p__0l__struct__slist_head__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::slist_head,
            ) -> *mut crate::__pgrx_c_bindings::slist_node,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::slist_head,
                *mut crate::__pgrx_c_bindings::slist_node,
            ) -> *mut crate::__pgrx_c_bindings::slist_node,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::slist_head,
                *mut crate::__pgrx_c_bindings::slist_node,
            ) -> *mut crate::__pgrx_c_bindings::slist_node,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__slist_node__0p__0l__struct__slist_head__0p__0c__struct__slist_node__0p__0r;
impl c::sealed::Sealed
    for Signature_struct__slist_node__0p__0l__struct__slist_head__0p__0c__struct__slist_node__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_struct__slist_node__0p__0l__struct__slist_head__0p__0c__struct__slist_node__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::slist_head,
                *mut crate::__pgrx_c_bindings::slist_node,
            ) -> *mut crate::__pgrx_c_bindings::slist_node,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
                bool,
            ) -> *mut crate::__pgrx_c_bindings::varlena,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
                bool,
            ) -> *mut crate::__pgrx_c_bindings::varlena,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__varlena__0p__0l__unsigned__long__0c__0x5f426f6f6c__0r;
impl c::sealed::Sealed for Signature_struct__varlena__0p__0l__unsigned__long__0c__0x5f426f6f6c__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_struct__varlena__0p__0l__unsigned__long__0c__0x5f426f6f6c__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
                bool,
            ) -> *mut crate::__pgrx_c_bindings::varlena,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
            ) -> *mut crate::__pgrx_c_bindings::varlena,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
            ) -> *mut crate::__pgrx_c_bindings::varlena,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__varlena__0p__0l__const__char__0p__0r;
impl c::sealed::Sealed for Signature_struct__varlena__0p__0l__const__char__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_struct__varlena__0p__0l__const__char__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
            ) -> *mut crate::__pgrx_c_bindings::varlena,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::FmgrInfo,
            ) -> *mut crate::__pgrx_c_bindings::varlena,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::FmgrInfo,
            ) -> *mut crate::__pgrx_c_bindings::varlena,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__varlena__0p__0l__struct__FmgrInfo__0p__0r;
impl c::sealed::Sealed for Signature_struct__varlena__0p__0l__struct__FmgrInfo__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_struct__varlena__0p__0l__struct__FmgrInfo__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::FmgrInfo,
            ) -> *mut crate::__pgrx_c_bindings::varlena,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::varlena,
            ) -> *mut crate::__pgrx_c_bindings::varlena,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::varlena,
            ) -> *mut crate::__pgrx_c_bindings::varlena,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__varlena__0p__0l__struct__varlena__0p__0r;
impl c::sealed::Sealed for Signature_struct__varlena__0p__0l__struct__varlena__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_struct__varlena__0p__0l__struct__varlena__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::varlena,
            ) -> *mut crate::__pgrx_c_bindings::varlena,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::varlena,
                i32,
                i32,
            ) -> *mut crate::__pgrx_c_bindings::varlena,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::varlena,
                i32,
                i32,
            ) -> *mut crate::__pgrx_c_bindings::varlena,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__varlena__0p__0l__struct__varlena__0p__0c__int__0c__int__0r;
impl c::sealed::Sealed
    for Signature_struct__varlena__0p__0l__struct__varlena__0p__0c__int__0c__int__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_struct__varlena__0p__0l__struct__varlena__0p__0c__int__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::varlena,
                i32,
                i32,
            ) -> *mut crate::__pgrx_c_bindings::varlena,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::AnyArrayType,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::AnyArrayType,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_union__AnyArrayType__0p__0l__unsigned__long__0r;
impl c::sealed::Sealed for Signature_union__AnyArrayType__0p__0l__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_union__AnyArrayType__0p__0l__unsigned__long__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::AnyArrayType,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::List,
            ) -> *mut crate::__pgrx_c_bindings::ListCell,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::List,
            ) -> *mut crate::__pgrx_c_bindings::ListCell,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_union__ListCell__0p__0l__const__struct__List__0p__0r;
impl c::sealed::Sealed for Signature_union__ListCell__0p__0l__const__struct__List__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_union__ListCell__0p__0l__const__struct__List__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::List,
            ) -> *mut crate::__pgrx_c_bindings::ListCell,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::List,
                *const crate::__pgrx_c_bindings::ListCell,
            ) -> *mut crate::__pgrx_c_bindings::ListCell,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::List,
                *const crate::__pgrx_c_bindings::ListCell,
            ) -> *mut crate::__pgrx_c_bindings::ListCell,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_union__ListCell__0p__0l__const__struct__List__0p__0c__const__union__ListCell__0p__0r;
impl c::sealed::Sealed for Signature_union__ListCell__0p__0l__const__struct__List__0p__0c__const__union__ListCell__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_union__ListCell__0p__0l__const__struct__List__0p__0c__const__union__ListCell__0p__0r {
 type Physical = PhysicalFunction_C_unwind_2<::core::option::Option<unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::List, *const crate::__pgrx_c_bindings::ListCell) -> *mut crate::__pgrx_c_bindings::ListCell>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::List,
                i32,
            ) -> *mut crate::__pgrx_c_bindings::ListCell,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::List,
                i32,
            ) -> *mut crate::__pgrx_c_bindings::ListCell,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_union__ListCell__0p__0l__const__struct__List__0p__0c__int__0r;
impl c::sealed::Sealed for Signature_union__ListCell__0p__0l__const__struct__List__0p__0c__int__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_union__ListCell__0p__0l__const__struct__List__0p__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::List,
                i32,
            ) -> *mut crate::__pgrx_c_bindings::ListCell,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                u32,
                *mut i32,
            ) -> u8,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                u32,
                *mut i32,
            ) -> u8,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__char__0l__struct__RelationData__0p__0c__unsigned__int__0c__int__0p__0r;
impl c::sealed::Sealed
    for Signature_unsigned__char__0l__struct__RelationData__0p__0c__unsigned__int__0c__int__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__char__0l__struct__RelationData__0p__0c__unsigned__int__0c__int__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                u32,
                *mut i32,
            ) -> u8,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const ::core::ffi::c_void, usize) -> u32,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const ::core::ffi::c_void, usize) -> u32,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__int__0l__const__void__0p__0c__unsigned__long__0r;
impl c::sealed::Sealed for Signature_unsigned__int__0l__const__void__0p__0c__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__int__0l__const__void__0p__0c__unsigned__long__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const ::core::ffi::c_void, usize) -> u32,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_void,
                usize,
                *mut ::core::ffi::c_void,
            ) -> u32,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_void,
                usize,
                *mut ::core::ffi::c_void,
            ) -> u32,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__int__0l__const__void__0p__0c__unsigned__long__0c__void__0p__0r;
impl c::sealed::Sealed
    for Signature_unsigned__int__0l__const__void__0p__0c__unsigned__long__0c__void__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__int__0l__const__void__0p__0c__unsigned__long__0c__void__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_void,
                usize,
                *mut ::core::ffi::c_void,
            ) -> u32,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::GinBtreeData,
                *mut ::core::ffi::c_char,
            ) -> u32,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::GinBtreeData,
                *mut ::core::ffi::c_char,
            ) -> u32,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__int__0l__struct__GinBtreeData__0p__0c__char__0p__0r;
impl c::sealed::Sealed for Signature_unsigned__int__0l__struct__GinBtreeData__0p__0c__char__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__int__0l__struct__GinBtreeData__0p__0c__char__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::GinBtreeData,
                *mut ::core::ffi::c_char,
            ) -> u32,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::GinBtreeData,
                *mut crate::__pgrx_c_bindings::GinBtreeStack,
            ) -> u32,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::GinBtreeData,
                *mut crate::__pgrx_c_bindings::GinBtreeStack,
            ) -> u32,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__int__0l__struct__GinBtreeData__0p__0c__struct__GinBtreeStack__0p__0r;
impl c::sealed::Sealed
    for Signature_unsigned__int__0l__struct__GinBtreeData__0p__0c__struct__GinBtreeStack__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__int__0l__struct__GinBtreeData__0p__0c__struct__GinBtreeStack__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::GinBtreeData,
                *mut crate::__pgrx_c_bindings::GinBtreeStack,
            ) -> u32,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
            ) -> crate::__pgrx_c_bindings::Oid,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
            ) -> crate::__pgrx_c_bindings::Oid,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__int__0l__struct__RelationData__0p__0r;
impl c::sealed::Sealed for Signature_unsigned__int__0l__struct__RelationData__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__int__0l__struct__RelationData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
            ) -> crate::__pgrx_c_bindings::Oid,
        >,
    >;
}
const _: () = assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::RelationData>() == 8);
const _: () = assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::RelationData>() == 8);
const _: () = assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Oid>() == 4);
const _: () = assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Oid>() == 4);
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<
    A0: c::expression::ImplicitTo<
            c::expression::CPointer<
                c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>,
                c::expression::ReadWrite,
            >,
        >,
> c::expression::Call<(A0,)> for Signature_unsigned__int__0l__struct__RelationData__0p__0r
{
    type Output = <c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid> as c::expression::CType>::Value;
    unsafe fn call(pointer: Self::Pointer, args: (A0,)) -> Self::Output {
        let function = pointer.expect("C indirect call requires a non-null function pointer");
        const {
            assert!(!::core::mem::needs_drop::<A0>());
        }
        const _: () =
            assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::RelationData>());
        let native0 = <c::expression::CPointer<
            c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>,
            c::expression::ReadWrite,
        > as c::expression::CType>::into_storage(c::expression::implicit::<
            c::expression::CPointer<
                c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>,
                c::expression::ReadWrite,
            >,
            _,
        >(args.0));
        // SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
        let result = unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0)) };
        <c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid> as c::expression::CType>::from_storage(result)
    }
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::TM_IndexDeleteOp,
            ) -> crate::__pgrx_c_bindings::TransactionId,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::TM_IndexDeleteOp,
            ) -> crate::__pgrx_c_bindings::TransactionId,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__int__0l__struct__RelationData__0p__0c__struct__TM_IndexDeleteOp__0p__0r;
impl c::sealed::Sealed
    for Signature_unsigned__int__0l__struct__RelationData__0p__0c__struct__TM_IndexDeleteOp__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__int__0l__struct__RelationData__0p__0c__struct__TM_IndexDeleteOp__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::TM_IndexDeleteOp,
            ) -> crate::__pgrx_c_bindings::TransactionId,
        >,
    >;
}
const _: () = assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::RelationData>() == 8);
const _: () = assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::RelationData>() == 8);
const _: () =
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::TM_IndexDeleteOp>() == 8);
const _: () =
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::TM_IndexDeleteOp>() == 8);
const _: () = assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TransactionId>() == 4);
const _: () = assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TransactionId>() == 4);
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<
    A0: c::expression::ImplicitTo<
            c::expression::CPointer<
                c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>,
                c::expression::ReadWrite,
            >,
        >,
    A1: c::expression::ImplicitTo<
            c::expression::CPointer<
                c::expression::CRecord<crate::__pgrx_c_bindings::TM_IndexDeleteOp>,
                c::expression::ReadWrite,
            >,
        >,
> c::expression::Call<(A0, A1)>
    for Signature_unsigned__int__0l__struct__RelationData__0p__0c__struct__TM_IndexDeleteOp__0p__0r
{
    type Output = <c::expression::CIntegerStorage<
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::TransactionId,
    > as c::expression::CType>::Value;
    unsafe fn call(pointer: Self::Pointer, args: (A0, A1)) -> Self::Output {
        let function = pointer.expect("C indirect call requires a non-null function pointer");
        const {
            assert!(!::core::mem::needs_drop::<A0>());
        }
        const _: () =
            assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::RelationData>());
        let native0 = <c::expression::CPointer<
            c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>,
            c::expression::ReadWrite,
        > as c::expression::CType>::into_storage(c::expression::implicit::<
            c::expression::CPointer<
                c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>,
                c::expression::ReadWrite,
            >,
            _,
        >(args.0));
        const {
            assert!(!::core::mem::needs_drop::<A1>());
        }
        const _: () =
            assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::TM_IndexDeleteOp>());
        let native1 = <c::expression::CPointer<
            c::expression::CRecord<crate::__pgrx_c_bindings::TM_IndexDeleteOp>,
            c::expression::ReadWrite,
        > as c::expression::CType>::into_storage(c::expression::implicit::<
            c::expression::CPointer<
                c::expression::CRecord<crate::__pgrx_c_bindings::TM_IndexDeleteOp>,
                c::expression::ReadWrite,
            >,
            _,
        >(args.1));
        // SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
        let result =
            unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0, native1)) };
        <c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId> as c::expression::CType>::from_storage(result)
    }
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::SampleScanState, u32) -> u32,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::SampleScanState, u32) -> u32,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__int__0l__struct__SampleScanState__0p__0c__unsigned__int__0r;
impl c::sealed::Sealed
    for Signature_unsigned__int__0l__struct__SampleScanState__0p__0c__unsigned__int__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__int__0l__struct__SampleScanState__0p__0c__unsigned__int__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::SampleScanState, u32) -> u32,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn(u32) -> u32>>() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn(u32) -> u32>>() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__int__0l__unsigned__int__0r;
impl c::sealed::Sealed for Signature_unsigned__int__0l__unsigned__int__0r {}
impl c::expression::NativeFunctionSignature for Signature_unsigned__int__0l__unsigned__int__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(u32) -> u32>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(u32, *const ::core::ffi::c_void, usize) -> u32,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(u32, *const ::core::ffi::c_void, usize) -> u32,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__int__0l__unsigned__int__0c__const__void__0p__0c__unsigned__long__0r;
impl c::sealed::Sealed
    for Signature_unsigned__int__0l__unsigned__int__0c__const__void__0p__0c__unsigned__long__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__int__0l__unsigned__int__0c__const__void__0p__0c__unsigned__long__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(u32, *const ::core::ffi::c_void, usize) -> u32,
        >,
    >;
}
const _: () = assert!(::core::mem::size_of::<u32>() == 4);
const _: () = assert!(::core::mem::align_of::<u32>() == 4);
const _: () = assert!(::core::mem::size_of::<*const ::core::ffi::c_void>() == 8);
const _: () = assert!(::core::mem::align_of::<*const ::core::ffi::c_void>() == 8);
const _: () = assert!(::core::mem::size_of::<usize>() == 8);
const _: () = assert!(::core::mem::align_of::<usize>() == 8);
const _: () = assert!(::core::mem::size_of::<u32>() == 4);
const _: () = assert!(::core::mem::align_of::<u32>() == 4);
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<
    A0: c::expression::ImplicitTo<c::CUnsignedInt>,
    A1: c::expression::ImplicitTo<
            c::expression::CPointer<c::expression::CVoid, c::expression::ReadOnly>,
        >,
    A2: c::expression::ImplicitTo<c::expression::CIntegerStorage<c::CUnsignedLong, usize>>,
> c::expression::Call<(A0, A1, A2)>
    for Signature_unsigned__int__0l__unsigned__int__0c__const__void__0p__0c__unsigned__long__0r
{
    type Output = <c::CUnsignedInt as c::expression::CType>::Value;
    unsafe fn call(pointer: Self::Pointer, args: (A0, A1, A2)) -> Self::Output {
        let function = pointer.expect("C indirect call requires a non-null function pointer");
        const {
            assert!(!::core::mem::needs_drop::<A0>());
        }
        const _: () = assert!(!::core::mem::needs_drop::<u32>());
        let native0 =
            <c::CUnsignedInt as c::expression::CType>::into_storage(c::expression::implicit::<
                c::CUnsignedInt,
                _,
            >(args.0));
        const {
            assert!(!::core::mem::needs_drop::<A1>());
        }
        const _: () = assert!(!::core::mem::needs_drop::<*const ::core::ffi::c_void>());
        let native1 = <c::expression::CPointer<c::expression::CVoid, c::expression::ReadOnly> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CVoid, c::expression::ReadOnly>, _>(args.1));
        const {
            assert!(!::core::mem::needs_drop::<A2>());
        }
        const _: () = assert!(!::core::mem::needs_drop::<usize>());
        let native2 = <c::expression::CIntegerStorage<c::CUnsignedLong, usize> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CIntegerStorage<c::CUnsignedLong, usize>, _>(args.2));
        // SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
        let result = unsafe {
            crate::ffi::pg_guard_ffi_boundary(move || function(native0, native1, native2))
        };
        <c::CUnsignedInt as c::expression::CType>::from_storage(result)
    }
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Datum) -> u32>,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Datum) -> u32>,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__int__0l__unsigned__long__0r;
impl c::sealed::Sealed for Signature_unsigned__int__0l__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature for Signature_unsigned__int__0l__unsigned__long__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Datum) -> u32>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::List,
                i32,
            ) -> crate::__pgrx_c_bindings::Oid,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::List,
                i32,
            ) -> crate::__pgrx_c_bindings::Oid,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__int__0l__const__struct__List__0p__0c__int__0r;
impl c::sealed::Sealed for Signature_unsigned__int__0l__const__struct__List__0p__0c__int__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__int__0l__const__struct__List__0p__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::List,
                i32,
            ) -> crate::__pgrx_c_bindings::Oid,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::RangeVar,
                i32,
                u32,
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *const crate::__pgrx_c_bindings::RangeVar,
                        crate::__pgrx_c_bindings::Oid,
                        crate::__pgrx_c_bindings::Oid,
                        *mut ::core::ffi::c_void,
                    ) -> (),
                >,
                *mut ::core::ffi::c_void,
            ) -> crate::__pgrx_c_bindings::Oid,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::RangeVar,
                i32,
                u32,
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *const crate::__pgrx_c_bindings::RangeVar,
                        crate::__pgrx_c_bindings::Oid,
                        crate::__pgrx_c_bindings::Oid,
                        *mut ::core::ffi::c_void,
                    ) -> (),
                >,
                *mut ::core::ffi::c_void,
            ) -> crate::__pgrx_c_bindings::Oid,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__int__0l__const__struct__RangeVar__0p__0c__int__0c__unsigned__int__0c__void__0l__0p__0r__0l__const__struct__RangeVar__0p__0c__unsigned__int__0c__unsigned__int__0c__void__0p__0r__0c__void__0p__0r;
impl c::sealed::Sealed for Signature_unsigned__int__0l__const__struct__RangeVar__0p__0c__int__0c__unsigned__int__0c__void__0l__0p__0r__0l__const__struct__RangeVar__0p__0c__unsigned__int__0c__unsigned__int__0c__void__0p__0r__0c__void__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_unsigned__int__0l__const__struct__RangeVar__0p__0c__int__0c__unsigned__int__0c__void__0l__0p__0r__0l__const__struct__RangeVar__0p__0c__unsigned__int__0c__unsigned__int__0c__void__0p__0r__0c__void__0p__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::RangeVar, i32, u32, ::core::option::Option<unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::RangeVar, crate::__pgrx_c_bindings::Oid, crate::__pgrx_c_bindings::Oid, *mut ::core::ffi::c_void) -> ()>, *mut ::core::ffi::c_void) -> crate::__pgrx_c_bindings::Oid>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const u8, i32) -> u32>,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const u8, i32) -> u32>,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__int__0l__const__unsigned__char__0p__0c__int__0r;
impl c::sealed::Sealed for Signature_unsigned__int__0l__const__unsigned__char__0p__0c__int__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__int__0l__const__unsigned__char__0p__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const u8, i32) -> u32>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                i32,
                i16,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Oid,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                i32,
                i16,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Oid,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__int__0l__int__0c__short__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::sealed::Sealed for Signature_unsigned__int__0l__int__0c__short__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::NativeFunctionSignature for Signature_unsigned__int__0l__int__0c__short__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_6<::core::option::Option<unsafe extern "C-unwind" fn(i32, i16, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum) -> crate::__pgrx_c_bindings::Oid>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                i32,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> u32,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                i32,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> u32,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__int__0l__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::sealed::Sealed for Signature_unsigned__int__0l__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::NativeFunctionSignature for Signature_unsigned__int__0l__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(i32, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum) -> u32>>;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::HeapTupleHeaderData,
            ) -> crate::__pgrx_c_bindings::TransactionId,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::HeapTupleHeaderData,
            ) -> crate::__pgrx_c_bindings::TransactionId,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__int__0l__struct__HeapTupleHeaderData__0p__0r;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
impl c::sealed::Sealed for Signature_unsigned__int__0l__struct__HeapTupleHeaderData__0p__0r {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__int__0l__struct__HeapTupleHeaderData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::HeapTupleHeaderData,
            ) -> crate::__pgrx_c_bindings::TransactionId,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::IndexTupleData) -> u32,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::IndexTupleData) -> u32,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__int__0l__struct__IndexTupleData__0p__0r;
impl c::sealed::Sealed for Signature_unsigned__int__0l__struct__IndexTupleData__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__int__0l__struct__IndexTupleData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::IndexTupleData) -> u32,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, i32) -> u32,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, i32) -> u32,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__int__0l__struct__RelationData__0p__0c__enum__ForkNumber__0r;
impl c::sealed::Sealed
    for Signature_unsigned__int__0l__struct__RelationData__0p__0c__enum__ForkNumber__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__int__0l__struct__RelationData__0p__0c__enum__ForkNumber__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, i32) -> u32,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::pg_atomic_uint32) -> u32,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::pg_atomic_uint32) -> u32,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__int__0l__volatile__struct__pg_atomic_uint32__0p__0r;
impl c::sealed::Sealed for Signature_unsigned__int__0l__volatile__struct__pg_atomic_uint32__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__int__0l__volatile__struct__pg_atomic_uint32__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::pg_atomic_uint32) -> u32,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::pg_atomic_uint32,
                i32,
            ) -> u32,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::pg_atomic_uint32,
                i32,
            ) -> u32,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__int__0l__volatile__struct__pg_atomic_uint32__0p__0c__int__0r;
impl c::sealed::Sealed
    for Signature_unsigned__int__0l__volatile__struct__pg_atomic_uint32__0p__0c__int__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__int__0l__volatile__struct__pg_atomic_uint32__0p__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::pg_atomic_uint32,
                i32,
            ) -> u32,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::pg_atomic_uint32,
                u32,
            ) -> u32,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::pg_atomic_uint32,
                u32,
            ) -> u32,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__int__0l__volatile__struct__pg_atomic_uint32__0p__0c__unsigned__int__0r;
impl c::sealed::Sealed
    for Signature_unsigned__int__0l__volatile__struct__pg_atomic_uint32__0p__0c__unsigned__int__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__int__0l__volatile__struct__pg_atomic_uint32__0p__0c__unsigned__int__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::pg_atomic_uint32,
                u32,
            ) -> u32,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
                *mut ::core::ffi::c_void,
            ) -> usize,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
                *mut ::core::ffi::c_void,
            ) -> usize,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__const__char__0p__0c__void__0p__0r;
impl c::sealed::Sealed for Signature_unsigned__long__0l__const__char__0p__0c__void__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__long__0l__const__char__0p__0c__void__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
                *mut ::core::ffi::c_void,
            ) -> usize,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::CustomScanState,
                *mut crate::__pgrx_c_bindings::ParallelContext,
            ) -> usize,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::CustomScanState,
                *mut crate::__pgrx_c_bindings::ParallelContext,
            ) -> usize,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__struct__CustomScanState__0p__0c__struct__ParallelContext__0p__0r;
impl c::sealed::Sealed for Signature_unsigned__long__0l__struct__CustomScanState__0p__0c__struct__ParallelContext__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__struct__CustomScanState__0p__0c__struct__ParallelContext__0p__0r {
 type Physical = PhysicalFunction_C_unwind_2<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::CustomScanState, *mut crate::__pgrx_c_bindings::ParallelContext) -> usize>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ExpandedObjectHeader,
            ) -> usize,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ExpandedObjectHeader,
            ) -> usize,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__struct__ExpandedObjectHeader__0p__0r;
impl c::sealed::Sealed for Signature_unsigned__long__0l__struct__ExpandedObjectHeader__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__long__0l__struct__ExpandedObjectHeader__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ExpandedObjectHeader,
            ) -> usize,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ExprState,
                *mut crate::__pgrx_c_bindings::ExprContext,
                *mut bool,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ExprState,
                *mut crate::__pgrx_c_bindings::ExprContext,
                *mut bool,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__struct__ExprState__0p__0c__struct__ExprContext__0p__0c__0x5f426f6f6c__0p__0r;
impl c::sealed::Sealed for Signature_unsigned__long__0l__struct__ExprState__0p__0c__struct__ExprContext__0p__0c__0x5f426f6f6c__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__struct__ExprState__0p__0c__struct__ExprContext__0p__0c__0x5f426f6f6c__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ExprState, *mut crate::__pgrx_c_bindings::ExprContext, *mut bool) -> crate::__pgrx_c_bindings::Datum>>;
}
const _: () = assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::ExprState>() == 8);
const _: () = assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::ExprState>() == 8);
const _: () = assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::ExprContext>() == 8);
const _: () = assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::ExprContext>() == 8);
#[cfg(any(feature = "pg15", feature = "pg19"))]
const _: () = assert!(::core::mem::size_of::<*mut bool>() == 8);
#[cfg(any(feature = "pg15", feature = "pg19"))]
const _: () = assert!(::core::mem::align_of::<*mut bool>() == 8);
const _: () = assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Datum>() == 8);
const _: () = assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Datum>() == 8);
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<A0: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::ExprState>, c::expression::ReadWrite>>, A1: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::ExprContext>, c::expression::ReadWrite>>, A2: c::expression::ImplicitTo<c::expression::CPointer<c::CBool, c::expression::ReadWrite>>> c::expression::Call<(A0,A1,A2,)> for Signature_unsigned__long__0l__struct__ExprState__0p__0c__struct__ExprContext__0p__0c__0x5f426f6f6c__0p__0r {
 type Output = <c::expression::CIntegerStorage<c::CUnsignedLong, crate::__pgrx_c_bindings::Datum> as c::expression::CType>::Value;
 unsafe fn call(pointer: Self::Pointer, args: (A0,A1,A2,)) -> Self::Output {
let function = pointer.expect("C indirect call requires a non-null function pointer");
const { assert!(!::core::mem::needs_drop::<A0>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::ExprState>());
let native0 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::ExprState>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::ExprState>, c::expression::ReadWrite>, _>(args.0));
const { assert!(!::core::mem::needs_drop::<A1>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::ExprContext>());
let native1 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::ExprContext>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::ExprContext>, c::expression::ReadWrite>, _>(args.1));
const { assert!(!::core::mem::needs_drop::<A2>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut bool>());
let native2 = <c::expression::CPointer<c::CBool, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::CBool, c::expression::ReadWrite>, _>(args.2));
// SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
let result = unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0, native1, native2)) };
<c::expression::CIntegerStorage<c::CUnsignedLong, crate::__pgrx_c_bindings::Datum> as c::expression::CType>::from_storage(result)
}
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ForeignScanState,
                *mut crate::__pgrx_c_bindings::ParallelContext,
            ) -> usize,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ForeignScanState,
                *mut crate::__pgrx_c_bindings::ParallelContext,
            ) -> usize,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__struct__ForeignScanState__0p__0c__struct__ParallelContext__0p__0r;
impl c::sealed::Sealed for Signature_unsigned__long__0l__struct__ForeignScanState__0p__0c__struct__ParallelContext__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__struct__ForeignScanState__0p__0c__struct__ParallelContext__0p__0r {
 type Physical = PhysicalFunction_C_unwind_2<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ForeignScanState, *mut crate::__pgrx_c_bindings::ParallelContext) -> usize>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::FunctionCallInfoBaseData,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::FunctionCallInfoBaseData,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__struct__FunctionCallInfoBaseData__0p__0r;
impl c::sealed::Sealed for Signature_unsigned__long__0l__struct__FunctionCallInfoBaseData__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__long__0l__struct__FunctionCallInfoBaseData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::FunctionCallInfoBaseData,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >;
}
const _: () =
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::FunctionCallInfoBaseData>() == 8);
const _: () = assert!(
    ::core::mem::align_of::<*mut crate::__pgrx_c_bindings::FunctionCallInfoBaseData>() == 8
);
const _: () = assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Datum>() == 8);
const _: () = assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Datum>() == 8);
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<
    A0: c::expression::ImplicitTo<
            c::expression::CPointer<
                c::expression::CRecord<crate::__pgrx_c_bindings::FunctionCallInfoBaseData>,
                c::expression::ReadWrite,
            >,
        >,
> c::expression::Call<(A0,)>
    for Signature_unsigned__long__0l__struct__FunctionCallInfoBaseData__0p__0r
{
    type Output = <c::expression::CIntegerStorage<c::CUnsignedLong, crate::__pgrx_c_bindings::Datum> as c::expression::CType>::Value;
    unsafe fn call(pointer: Self::Pointer, args: (A0,)) -> Self::Output {
        let function = pointer.expect("C indirect call requires a non-null function pointer");
        const {
            assert!(!::core::mem::needs_drop::<A0>());
        }
        const _: () = assert!(!::core::mem::needs_drop::<
            *mut crate::__pgrx_c_bindings::FunctionCallInfoBaseData,
        >());
        let native0 = <c::expression::CPointer<
            c::expression::CRecord<crate::__pgrx_c_bindings::FunctionCallInfoBaseData>,
            c::expression::ReadWrite,
        > as c::expression::CType>::into_storage(c::expression::implicit::<
            c::expression::CPointer<
                c::expression::CRecord<crate::__pgrx_c_bindings::FunctionCallInfoBaseData>,
                c::expression::ReadWrite,
            >,
            _,
        >(args.0));
        // SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
        let result = unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0)) };
        <c::expression::CIntegerStorage<c::CUnsignedLong, crate::__pgrx_c_bindings::Datum> as c::expression::CType>::from_storage(result)
    }
}
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::MemoryContextData,
                *mut ::core::ffi::c_void,
            ) -> usize,
        >,
    >() == 8
);
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::MemoryContextData,
                *mut ::core::ffi::c_void,
            ) -> usize,
        >,
    >() == 8
);
#[cfg(feature = "pg15")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__struct__MemoryContextData__0p__0c__void__0p__0r;
#[cfg(feature = "pg15")]
impl c::sealed::Sealed
    for Signature_unsigned__long__0l__struct__MemoryContextData__0p__0c__void__0p__0r
{
}
#[cfg(feature = "pg15")]
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__long__0l__struct__MemoryContextData__0p__0c__void__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::MemoryContextData,
                *mut ::core::ffi::c_void,
            ) -> usize,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PLpgSQL_execstate,
                crate::__pgrx_c_bindings::Datum,
                *mut bool,
                crate::__pgrx_c_bindings::Oid,
                i32,
                crate::__pgrx_c_bindings::Oid,
                i32,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PLpgSQL_execstate,
                crate::__pgrx_c_bindings::Datum,
                *mut bool,
                crate::__pgrx_c_bindings::Oid,
                i32,
                crate::__pgrx_c_bindings::Oid,
                i32,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__struct__PLpgSQL_execstate__0p__0c__unsigned__long__0c__0x5f426f6f6c__0p__0c__unsigned__int__0c__int__0c__unsigned__int__0c__int__0r;
impl c::sealed::Sealed for Signature_unsigned__long__0l__struct__PLpgSQL_execstate__0p__0c__unsigned__long__0c__0x5f426f6f6c__0p__0c__unsigned__int__0c__int__0c__unsigned__int__0c__int__0r {}
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__struct__PLpgSQL_execstate__0p__0c__unsigned__long__0c__0x5f426f6f6c__0p__0c__unsigned__int__0c__int__0c__unsigned__int__0c__int__0r {
 type Physical = PhysicalFunction_C_unwind_7<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PLpgSQL_execstate, crate::__pgrx_c_bindings::Datum, *mut bool, crate::__pgrx_c_bindings::Oid, i32, crate::__pgrx_c_bindings::Oid, i32) -> crate::__pgrx_c_bindings::Datum>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData) -> usize,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData) -> usize,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__struct__RelationData__0p__0r;
impl c::sealed::Sealed for Signature_unsigned__long__0l__struct__RelationData__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__long__0l__struct__RelationData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData) -> usize,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, i32) -> u64,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, i32) -> u64,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__struct__RelationData__0p__0c__enum__ForkNumber__0r;
impl c::sealed::Sealed
    for Signature_unsigned__long__0l__struct__RelationData__0p__0c__enum__ForkNumber__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__long__0l__struct__RelationData__0p__0c__enum__ForkNumber__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, i32) -> u64,
        >,
    >;
}
const _: () = assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::RelationData>() == 8);
const _: () = assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::RelationData>() == 8);
const _: () = assert!(::core::mem::size_of::<i32>() == 4);
const _: () = assert!(::core::mem::align_of::<i32>() == 4);
const _: () = assert!(::core::mem::size_of::<u64>() == 8);
const _: () = assert!(::core::mem::align_of::<u64>() == 8);
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<
    A0: c::expression::ImplicitTo<
            c::expression::CPointer<
                c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>,
                c::expression::ReadWrite,
            >,
        >,
    A1: c::expression::ImplicitTo<
            c::expression::CEnumObject<
                crate::__pgrx_c_generated::EnumIdentity_enum__ForkNumber,
                c::CInt,
                i32,
            >,
        >,
> c::expression::Call<(A0, A1)>
    for Signature_unsigned__long__0l__struct__RelationData__0p__0c__enum__ForkNumber__0r
{
    type Output = <c::CUnsignedLong as c::expression::CType>::Value;
    unsafe fn call(pointer: Self::Pointer, args: (A0, A1)) -> Self::Output {
        let function = pointer.expect("C indirect call requires a non-null function pointer");
        const {
            assert!(!::core::mem::needs_drop::<A0>());
        }
        const _: () =
            assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::RelationData>());
        let native0 = <c::expression::CPointer<
            c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>,
            c::expression::ReadWrite,
        > as c::expression::CType>::into_storage(c::expression::implicit::<
            c::expression::CPointer<
                c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>,
                c::expression::ReadWrite,
            >,
            _,
        >(args.0));
        const {
            assert!(!::core::mem::needs_drop::<A1>());
        }
        const _: () = assert!(!::core::mem::needs_drop::<i32>());
        let native1 = <c::expression::CEnumObject<
            crate::__pgrx_c_generated::EnumIdentity_enum__ForkNumber,
            c::CInt,
            i32,
        > as c::expression::CType>::into_storage(c::expression::implicit::<
            c::expression::CEnumObject<
                crate::__pgrx_c_generated::EnumIdentity_enum__ForkNumber,
                c::CInt,
                i32,
            >,
            _,
        >(args.1));
        // SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
        let result =
            unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0, native1)) };
        <c::CUnsignedLong as c::expression::CType>::from_storage(result)
    }
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::ParallelTableScanDescData,
            ) -> usize,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::ParallelTableScanDescData,
            ) -> usize,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__struct__RelationData__0p__0c__struct__ParallelTableScanDescData__0p__0r;
impl c::sealed::Sealed for Signature_unsigned__long__0l__struct__RelationData__0p__0c__struct__ParallelTableScanDescData__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__struct__RelationData__0p__0c__struct__ParallelTableScanDescData__0p__0r {
 type Physical = PhysicalFunction_C_unwind_2<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::ParallelTableScanDescData) -> usize>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableFuncScanState,
                i32,
                crate::__pgrx_c_bindings::Oid,
                i32,
                *mut bool,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableFuncScanState,
                i32,
                crate::__pgrx_c_bindings::Oid,
                i32,
                *mut bool,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__struct__TableFuncScanState__0p__0c__int__0c__unsigned__int__0c__int__0c__0x5f426f6f6c__0p__0r;
impl c::sealed::Sealed for Signature_unsigned__long__0l__struct__TableFuncScanState__0p__0c__int__0c__unsigned__int__0c__int__0c__0x5f426f6f6c__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__struct__TableFuncScanState__0p__0c__int__0c__unsigned__int__0c__int__0c__0x5f426f6f6c__0p__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TableFuncScanState, i32, crate::__pgrx_c_bindings::Oid, i32, *mut bool) -> crate::__pgrx_c_bindings::Datum>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                i32,
                *mut bool,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                i32,
                *mut bool,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__struct__TupleTableSlot__0p__0c__int__0c__0x5f426f6f6c__0p__0r;
impl c::sealed::Sealed
    for Signature_unsigned__long__0l__struct__TupleTableSlot__0p__0c__int__0c__0x5f426f6f6c__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__long__0l__struct__TupleTableSlot__0p__0c__int__0c__0x5f426f6f6c__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                i32,
                *mut bool,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::VacAttrStats,
                i32,
                *mut bool,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::VacAttrStats,
                i32,
                *mut bool,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__struct__VacAttrStats__0p__0c__int__0c__0x5f426f6f6c__0p__0r;
impl c::sealed::Sealed
    for Signature_unsigned__long__0l__struct__VacAttrStats__0p__0c__int__0c__0x5f426f6f6c__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__long__0l__struct__VacAttrStats__0p__0c__int__0c__0x5f426f6f6c__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::VacAttrStats,
                i32,
                *mut bool,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
                *mut crate::__pgrx_c_bindings::SortSupportData,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
                *mut crate::__pgrx_c_bindings::SortSupportData,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__unsigned__long__0c__struct__SortSupportData__0p__0r;
impl c::sealed::Sealed
    for Signature_unsigned__long__0l__unsigned__long__0c__struct__SortSupportData__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__long__0l__unsigned__long__0c__struct__SortSupportData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
                *mut crate::__pgrx_c_bindings::SortSupportData,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn() -> usize>>() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn() -> usize>>() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg18"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__void__0r;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg18"))]
impl c::sealed::Sealed for Signature_unsigned__long__0l__void__0r {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg18"))]
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__void__0r {
    type Physical =
        PhysicalFunction_C_unwind_0<::core::option::Option<unsafe extern "C-unwind" fn() -> usize>>;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_char) -> usize>,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_char) -> usize>,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__char__0p__0r;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
impl c::sealed::Sealed for Signature_unsigned__long__0l__char__0p__0r {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__char__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_char) -> usize>,
    >;
}
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn(*const i8) -> u64>>()
        == 8
);
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn(*const i8) -> u64>>(
    ) == 8
);
#[cfg(feature = "pg15")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__const__char__0p__0r;
#[cfg(feature = "pg15")]
impl c::sealed::Sealed for Signature_unsigned__long__0l__const__char__0p__0r {}
#[cfg(feature = "pg15")]
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__const__char__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const i8) -> u64>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const i8, *mut *mut i8, i32) -> u64>,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const i8, *mut *mut i8, i32) -> u64>,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__const__char__0p__0c__char__0p__0p__0c__int__0r;
impl c::sealed::Sealed
    for Signature_unsigned__long__0l__const__char__0p__0c__char__0p__0p__0c__int__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__long__0l__const__char__0p__0c__char__0p__0p__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const i8, *mut *mut i8, i32) -> u64>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const i8, *const i8) -> u64>,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const i8, *const i8) -> u64>,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__const__char__0p__0c__const__char__0p__0r;
impl c::sealed::Sealed for Signature_unsigned__long__0l__const__char__0p__0c__const__char__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__long__0l__const__char__0p__0c__const__char__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const i8, *const i8) -> u64>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const u8, i32, u64) -> u64>,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const u8, i32, u64) -> u64>,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__const__unsigned__char__0p__0c__int__0c__unsigned__long__0r;
impl c::sealed::Sealed
    for Signature_unsigned__long__0l__const__unsigned__char__0p__0c__int__0c__unsigned__long__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__long__0l__const__unsigned__char__0p__0c__int__0c__unsigned__long__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const u8, i32, u64) -> u64>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(f64) -> crate::__pgrx_c_bindings::Datum>,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(f64) -> crate::__pgrx_c_bindings::Datum>,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__double__0r;
impl c::sealed::Sealed for Signature_unsigned__long__0l__double__0r {}
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__double__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(f64) -> crate::__pgrx_c_bindings::Datum>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(f32) -> crate::__pgrx_c_bindings::Datum>,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(f32) -> crate::__pgrx_c_bindings::Datum>,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__float__0r;
impl c::sealed::Sealed for Signature_unsigned__long__0l__float__0r {}
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__float__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(f32) -> crate::__pgrx_c_bindings::Datum>,
    >;
}
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn(i32) -> usize>>()
        == 8
);
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn(i32) -> usize>>()
        == 8
);
#[cfg(feature = "pg15")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__int__0r;
#[cfg(feature = "pg15")]
impl c::sealed::Sealed for Signature_unsigned__long__0l__int__0r {}
#[cfg(feature = "pg15")]
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__int__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(i32) -> usize>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ExpandedRecordHeader,
                i32,
                *mut bool,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ExpandedRecordHeader,
                i32,
                *mut bool,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__struct__ExpandedRecordHeader__0p__0c__int__0c__0x5f426f6f6c__0p__0r;
impl c::sealed::Sealed for Signature_unsigned__long__0l__struct__ExpandedRecordHeader__0p__0c__int__0c__0x5f426f6f6c__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__struct__ExpandedRecordHeader__0p__0c__int__0c__0x5f426f6f6c__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ExpandedRecordHeader, i32, *mut bool) -> crate::__pgrx_c_bindings::Datum>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::FmgrInfo,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::FmgrInfo,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__struct__FmgrInfo__0p__0c__unsigned__int__0c__unsigned__long__0r;
impl c::sealed::Sealed for Signature_unsigned__long__0l__struct__FmgrInfo__0p__0c__unsigned__int__0c__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__struct__FmgrInfo__0p__0c__unsigned__int__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::FmgrInfo, crate::__pgrx_c_bindings::Oid, crate::__pgrx_c_bindings::Datum) -> crate::__pgrx_c_bindings::Datum>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::FmgrInfo,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::FmgrInfo,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__struct__FmgrInfo__0p__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0r;
impl c::sealed::Sealed for Signature_unsigned__long__0l__struct__FmgrInfo__0p__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__struct__FmgrInfo__0p__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::FmgrInfo, crate::__pgrx_c_bindings::Oid, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum) -> crate::__pgrx_c_bindings::Datum>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::FmgrInfo,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::FmgrInfo,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__struct__FmgrInfo__0p__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r;
impl c::sealed::Sealed for Signature_unsigned__long__0l__struct__FmgrInfo__0p__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__struct__FmgrInfo__0p__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::FmgrInfo, crate::__pgrx_c_bindings::Oid, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum) -> crate::__pgrx_c_bindings::Datum>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::FmgrInfo,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::FmgrInfo,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__struct__FmgrInfo__0p__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r;
impl c::sealed::Sealed for Signature_unsigned__long__0l__struct__FmgrInfo__0p__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__struct__FmgrInfo__0p__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_6<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::FmgrInfo, crate::__pgrx_c_bindings::Oid, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum) -> crate::__pgrx_c_bindings::Datum>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::FmgrInfo,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::FmgrInfo,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__struct__FmgrInfo__0p__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r;
impl c::sealed::Sealed for Signature_unsigned__long__0l__struct__FmgrInfo__0p__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__struct__FmgrInfo__0p__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_7<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::FmgrInfo, crate::__pgrx_c_bindings::Oid, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum) -> crate::__pgrx_c_bindings::Datum>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::FmgrInfo,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::FmgrInfo,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__struct__FmgrInfo__0p__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r;
impl c::sealed::Sealed for Signature_unsigned__long__0l__struct__FmgrInfo__0p__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__struct__FmgrInfo__0p__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_8<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::FmgrInfo, crate::__pgrx_c_bindings::Oid, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum) -> crate::__pgrx_c_bindings::Datum>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::FmgrInfo,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::FmgrInfo,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__struct__FmgrInfo__0p__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r;
impl c::sealed::Sealed for Signature_unsigned__long__0l__struct__FmgrInfo__0p__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__struct__FmgrInfo__0p__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_9<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::FmgrInfo, crate::__pgrx_c_bindings::Oid, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum) -> crate::__pgrx_c_bindings::Datum>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::FmgrInfo,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::FmgrInfo,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__struct__FmgrInfo__0p__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r;
impl c::sealed::Sealed for Signature_unsigned__long__0l__struct__FmgrInfo__0p__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__struct__FmgrInfo__0p__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_10<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::FmgrInfo, crate::__pgrx_c_bindings::Oid, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum) -> crate::__pgrx_c_bindings::Datum>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::FmgrInfo,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::FmgrInfo,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__struct__FmgrInfo__0p__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r;
impl c::sealed::Sealed for Signature_unsigned__long__0l__struct__FmgrInfo__0p__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__struct__FmgrInfo__0p__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_11<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::FmgrInfo, crate::__pgrx_c_bindings::Oid, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum) -> crate::__pgrx_c_bindings::Datum>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::HeapTupleData,
                i32,
                *mut crate::__pgrx_c_bindings::TupleDescData,
                *mut bool,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::HeapTupleData,
                i32,
                *mut crate::__pgrx_c_bindings::TupleDescData,
                *mut bool,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__struct__HeapTupleData__0p__0c__int__0c__struct__TupleDescData__0p__0c__0x5f426f6f6c__0p__0r;
impl c::sealed::Sealed for Signature_unsigned__long__0l__struct__HeapTupleData__0p__0c__int__0c__struct__TupleDescData__0p__0c__0x5f426f6f6c__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__struct__HeapTupleData__0p__0c__int__0c__struct__TupleDescData__0p__0c__0x5f426f6f6c__0p__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::HeapTupleData, i32, *mut crate::__pgrx_c_bindings::TupleDescData, *mut bool) -> crate::__pgrx_c_bindings::Datum>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::HeapTupleHeaderData,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::HeapTupleHeaderData,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__struct__HeapTupleHeaderData__0p__0r;
impl c::sealed::Sealed for Signature_unsigned__long__0l__struct__HeapTupleHeaderData__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__long__0l__struct__HeapTupleHeaderData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::HeapTupleHeaderData,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TupleDescData,
                i32,
                *mut bool,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TupleDescData,
                i32,
                *mut bool,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__struct__TupleDescData__0p__0c__int__0c__0x5f426f6f6c__0p__0r;
impl c::sealed::Sealed
    for Signature_unsigned__long__0l__struct__TupleDescData__0p__0c__int__0c__0x5f426f6f6c__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__long__0l__struct__TupleDescData__0p__0c__int__0c__0x5f426f6f6c__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TupleDescData,
                i32,
                *mut bool,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                i16,
                *mut bool,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                i16,
                *mut bool,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__struct__TupleTableSlot__0p__0c__short__0c__0x5f426f6f6c__0p__0r;
impl c::sealed::Sealed for Signature_unsigned__long__0l__struct__TupleTableSlot__0p__0c__short__0c__0x5f426f6f6c__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__struct__TupleTableSlot__0p__0c__short__0c__0x5f426f6f6c__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TupleTableSlot, i16, *mut bool) -> crate::__pgrx_c_bindings::Datum>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::dsa_area, usize, i32) -> u64,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::dsa_area, usize, i32) -> u64,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__struct__dsa_area__0p__0c__unsigned__long__0c__int__0r;
impl c::sealed::Sealed
    for Signature_unsigned__long__0l__struct__dsa_area__0p__0c__unsigned__long__0c__int__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__long__0l__struct__dsa_area__0p__0c__unsigned__long__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::dsa_area, usize, i32) -> u64,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Oid,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Oid,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__unsigned__int__0c__unsigned__int__0r;
impl c::sealed::Sealed for Signature_unsigned__long__0l__unsigned__int__0c__unsigned__int__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__long__0l__unsigned__int__0c__unsigned__int__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Oid,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__unsigned__int__0c__unsigned__int__0c__unsigned__long__0r;
impl c::sealed::Sealed
    for Signature_unsigned__long__0l__unsigned__int__0c__unsigned__int__0c__unsigned__long__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__long__0l__unsigned__int__0c__unsigned__int__0c__unsigned__long__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__unsigned__int__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0r;
impl c::sealed::Sealed for Signature_unsigned__long__0l__unsigned__int__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__unsigned__int__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Oid, crate::__pgrx_c_bindings::Oid, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum) -> crate::__pgrx_c_bindings::Datum>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__unsigned__int__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r;
impl c::sealed::Sealed for Signature_unsigned__long__0l__unsigned__int__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__unsigned__int__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Oid, crate::__pgrx_c_bindings::Oid, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum) -> crate::__pgrx_c_bindings::Datum>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__unsigned__int__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r;
impl c::sealed::Sealed for Signature_unsigned__long__0l__unsigned__int__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__unsigned__int__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_6<::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Oid, crate::__pgrx_c_bindings::Oid, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum) -> crate::__pgrx_c_bindings::Datum>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__unsigned__int__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r;
impl c::sealed::Sealed for Signature_unsigned__long__0l__unsigned__int__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__unsigned__int__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_7<::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Oid, crate::__pgrx_c_bindings::Oid, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum) -> crate::__pgrx_c_bindings::Datum>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__unsigned__int__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r;
impl c::sealed::Sealed for Signature_unsigned__long__0l__unsigned__int__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__unsigned__int__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_8<::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Oid, crate::__pgrx_c_bindings::Oid, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum) -> crate::__pgrx_c_bindings::Datum>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__unsigned__int__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r;
impl c::sealed::Sealed for Signature_unsigned__long__0l__unsigned__int__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__unsigned__int__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_9<::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Oid, crate::__pgrx_c_bindings::Oid, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum) -> crate::__pgrx_c_bindings::Datum>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__unsigned__int__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r;
impl c::sealed::Sealed for Signature_unsigned__long__0l__unsigned__int__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__unsigned__int__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_10<::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Oid, crate::__pgrx_c_bindings::Oid, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum) -> crate::__pgrx_c_bindings::Datum>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__unsigned__int__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r;
impl c::sealed::Sealed for Signature_unsigned__long__0l__unsigned__int__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__unsigned__int__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_11<::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Oid, crate::__pgrx_c_bindings::Oid, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum) -> crate::__pgrx_c_bindings::Datum>>;
}
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn(u32, u64) -> u64>>()
        == 8
);
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn(u32, u64) -> u64>>()
        == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__unsigned__int__0c__unsigned__long__0r;
impl c::sealed::Sealed for Signature_unsigned__long__0l__unsigned__int__0c__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__long__0l__unsigned__int__0c__unsigned__long__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<unsafe extern "C-unwind" fn(u32, u64) -> u64>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::FunctionCallInfoBaseData,
                    )
                        -> crate::__pgrx_c_bindings::Datum,
                >,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::FunctionCallInfoBaseData,
                    )
                        -> crate::__pgrx_c_bindings::Datum,
                >,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__unsigned__long__0l__0p__0r__0l__struct__FunctionCallInfoBaseData__0p__0r__0c__unsigned__int__0c__unsigned__long__0r;
impl c::sealed::Sealed for Signature_unsigned__long__0l__unsigned__long__0l__0p__0r__0l__struct__FunctionCallInfoBaseData__0p__0r__0c__unsigned__int__0c__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__unsigned__long__0l__0p__0r__0l__struct__FunctionCallInfoBaseData__0p__0r__0c__unsigned__int__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::FunctionCallInfoBaseData) -> crate::__pgrx_c_bindings::Datum>, crate::__pgrx_c_bindings::Oid, crate::__pgrx_c_bindings::Datum) -> crate::__pgrx_c_bindings::Datum>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::FunctionCallInfoBaseData,
                    )
                        -> crate::__pgrx_c_bindings::Datum,
                >,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::FunctionCallInfoBaseData,
                    )
                        -> crate::__pgrx_c_bindings::Datum,
                >,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__unsigned__long__0l__0p__0r__0l__struct__FunctionCallInfoBaseData__0p__0r__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0r;
impl c::sealed::Sealed for Signature_unsigned__long__0l__unsigned__long__0l__0p__0r__0l__struct__FunctionCallInfoBaseData__0p__0r__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__unsigned__long__0l__0p__0r__0l__struct__FunctionCallInfoBaseData__0p__0r__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::FunctionCallInfoBaseData) -> crate::__pgrx_c_bindings::Datum>, crate::__pgrx_c_bindings::Oid, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum) -> crate::__pgrx_c_bindings::Datum>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::FunctionCallInfoBaseData,
                    )
                        -> crate::__pgrx_c_bindings::Datum,
                >,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::FunctionCallInfoBaseData,
                    )
                        -> crate::__pgrx_c_bindings::Datum,
                >,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__unsigned__long__0l__0p__0r__0l__struct__FunctionCallInfoBaseData__0p__0r__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r;
impl c::sealed::Sealed for Signature_unsigned__long__0l__unsigned__long__0l__0p__0r__0l__struct__FunctionCallInfoBaseData__0p__0r__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__unsigned__long__0l__0p__0r__0l__struct__FunctionCallInfoBaseData__0p__0r__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::FunctionCallInfoBaseData) -> crate::__pgrx_c_bindings::Datum>, crate::__pgrx_c_bindings::Oid, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum) -> crate::__pgrx_c_bindings::Datum>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::FunctionCallInfoBaseData,
                    )
                        -> crate::__pgrx_c_bindings::Datum,
                >,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::FunctionCallInfoBaseData,
                    )
                        -> crate::__pgrx_c_bindings::Datum,
                >,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__unsigned__long__0l__0p__0r__0l__struct__FunctionCallInfoBaseData__0p__0r__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r;
impl c::sealed::Sealed for Signature_unsigned__long__0l__unsigned__long__0l__0p__0r__0l__struct__FunctionCallInfoBaseData__0p__0r__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__unsigned__long__0l__0p__0r__0l__struct__FunctionCallInfoBaseData__0p__0r__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_6<::core::option::Option<unsafe extern "C-unwind" fn(::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::FunctionCallInfoBaseData) -> crate::__pgrx_c_bindings::Datum>, crate::__pgrx_c_bindings::Oid, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum) -> crate::__pgrx_c_bindings::Datum>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::FunctionCallInfoBaseData,
                    )
                        -> crate::__pgrx_c_bindings::Datum,
                >,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::FunctionCallInfoBaseData,
                    )
                        -> crate::__pgrx_c_bindings::Datum,
                >,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__unsigned__long__0l__0p__0r__0l__struct__FunctionCallInfoBaseData__0p__0r__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r;
impl c::sealed::Sealed for Signature_unsigned__long__0l__unsigned__long__0l__0p__0r__0l__struct__FunctionCallInfoBaseData__0p__0r__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__unsigned__long__0l__0p__0r__0l__struct__FunctionCallInfoBaseData__0p__0r__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_7<::core::option::Option<unsafe extern "C-unwind" fn(::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::FunctionCallInfoBaseData) -> crate::__pgrx_c_bindings::Datum>, crate::__pgrx_c_bindings::Oid, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum) -> crate::__pgrx_c_bindings::Datum>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::FunctionCallInfoBaseData,
                    )
                        -> crate::__pgrx_c_bindings::Datum,
                >,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::FunctionCallInfoBaseData,
                    )
                        -> crate::__pgrx_c_bindings::Datum,
                >,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__unsigned__long__0l__0p__0r__0l__struct__FunctionCallInfoBaseData__0p__0r__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r;
impl c::sealed::Sealed for Signature_unsigned__long__0l__unsigned__long__0l__0p__0r__0l__struct__FunctionCallInfoBaseData__0p__0r__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__unsigned__long__0l__0p__0r__0l__struct__FunctionCallInfoBaseData__0p__0r__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_8<::core::option::Option<unsafe extern "C-unwind" fn(::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::FunctionCallInfoBaseData) -> crate::__pgrx_c_bindings::Datum>, crate::__pgrx_c_bindings::Oid, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum) -> crate::__pgrx_c_bindings::Datum>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::FunctionCallInfoBaseData,
                    )
                        -> crate::__pgrx_c_bindings::Datum,
                >,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::FunctionCallInfoBaseData,
                    )
                        -> crate::__pgrx_c_bindings::Datum,
                >,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__unsigned__long__0l__0p__0r__0l__struct__FunctionCallInfoBaseData__0p__0r__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r;
impl c::sealed::Sealed for Signature_unsigned__long__0l__unsigned__long__0l__0p__0r__0l__struct__FunctionCallInfoBaseData__0p__0r__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__unsigned__long__0l__0p__0r__0l__struct__FunctionCallInfoBaseData__0p__0r__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_9<::core::option::Option<unsafe extern "C-unwind" fn(::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::FunctionCallInfoBaseData) -> crate::__pgrx_c_bindings::Datum>, crate::__pgrx_c_bindings::Oid, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum) -> crate::__pgrx_c_bindings::Datum>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::FunctionCallInfoBaseData,
                    )
                        -> crate::__pgrx_c_bindings::Datum,
                >,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::FunctionCallInfoBaseData,
                    )
                        -> crate::__pgrx_c_bindings::Datum,
                >,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__unsigned__long__0l__0p__0r__0l__struct__FunctionCallInfoBaseData__0p__0r__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r;
impl c::sealed::Sealed for Signature_unsigned__long__0l__unsigned__long__0l__0p__0r__0l__struct__FunctionCallInfoBaseData__0p__0r__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__unsigned__long__0l__0p__0r__0l__struct__FunctionCallInfoBaseData__0p__0r__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_10<::core::option::Option<unsafe extern "C-unwind" fn(::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::FunctionCallInfoBaseData) -> crate::__pgrx_c_bindings::Datum>, crate::__pgrx_c_bindings::Oid, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum) -> crate::__pgrx_c_bindings::Datum>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::FunctionCallInfoBaseData,
                    )
                        -> crate::__pgrx_c_bindings::Datum,
                >,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::FunctionCallInfoBaseData,
                    )
                        -> crate::__pgrx_c_bindings::Datum,
                >,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__unsigned__long__0l__0p__0r__0l__struct__FunctionCallInfoBaseData__0p__0r__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r;
impl c::sealed::Sealed for Signature_unsigned__long__0l__unsigned__long__0l__0p__0r__0l__struct__FunctionCallInfoBaseData__0p__0r__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__unsigned__long__0l__0p__0r__0l__struct__FunctionCallInfoBaseData__0p__0r__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_11<::core::option::Option<unsafe extern "C-unwind" fn(::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::FunctionCallInfoBaseData) -> crate::__pgrx_c_bindings::Datum>, crate::__pgrx_c_bindings::Oid, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum) -> crate::__pgrx_c_bindings::Datum>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::pg_atomic_uint64) -> u64,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::pg_atomic_uint64) -> u64,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__volatile__struct__pg_atomic_uint64__0p__0r;
impl c::sealed::Sealed
    for Signature_unsigned__long__0l__volatile__struct__pg_atomic_uint64__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__long__0l__volatile__struct__pg_atomic_uint64__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::pg_atomic_uint64) -> u64,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::pg_atomic_uint64,
                i64,
            ) -> u64,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::pg_atomic_uint64,
                i64,
            ) -> u64,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__volatile__struct__pg_atomic_uint64__0p__0c__long__0r;
impl c::sealed::Sealed
    for Signature_unsigned__long__0l__volatile__struct__pg_atomic_uint64__0p__0c__long__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__long__0l__volatile__struct__pg_atomic_uint64__0p__0c__long__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::pg_atomic_uint64,
                i64,
            ) -> u64,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::pg_atomic_uint64,
                u64,
            ) -> u64,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::pg_atomic_uint64,
                u64,
            ) -> u64,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__volatile__struct__pg_atomic_uint64__0p__0c__unsigned__long__0r;
impl c::sealed::Sealed
    for Signature_unsigned__long__0l__volatile__struct__pg_atomic_uint64__0p__0c__unsigned__long__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__long__0l__volatile__struct__pg_atomic_uint64__0p__0c__unsigned__long__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::pg_atomic_uint64,
                u64,
            ) -> u64,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::GinBtreeData,
                *mut ::core::ffi::c_char,
                u32,
                u16,
            ) -> u16,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::GinBtreeData,
                *mut ::core::ffi::c_char,
                u32,
                u16,
            ) -> u16,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__short__0l__struct__GinBtreeData__0p__0c__char__0p__0c__unsigned__int__0c__unsigned__short__0r;
impl c::sealed::Sealed for Signature_unsigned__short__0l__struct__GinBtreeData__0p__0c__char__0p__0c__unsigned__int__0c__unsigned__short__0r {}
impl c::expression::NativeFunctionSignature for Signature_unsigned__short__0l__struct__GinBtreeData__0p__0c__char__0p__0c__unsigned__int__0c__unsigned__short__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::GinBtreeData, *mut ::core::ffi::c_char, u32, u16) -> u16>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::SampleScanState,
                u32,
                u16,
            ) -> u16,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::SampleScanState,
                u32,
                u16,
            ) -> u16,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__short__0l__struct__SampleScanState__0p__0c__unsigned__int__0c__unsigned__short__0r;
impl c::sealed::Sealed for Signature_unsigned__short__0l__struct__SampleScanState__0p__0c__unsigned__int__0c__unsigned__short__0r {}
impl c::expression::NativeFunctionSignature for Signature_unsigned__short__0l__struct__SampleScanState__0p__0c__unsigned__int__0c__unsigned__short__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::SampleScanState, u32, u16) -> u16>>;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_char,
                *mut ::core::ffi::c_char,
                usize,
                u16,
                i32,
            ) -> u16,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_char,
                *mut ::core::ffi::c_char,
                usize,
                u16,
                i32,
            ) -> u16,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__short__0l__char__0p__0c__char__0p__0c__unsigned__long__0c__unsigned__short__0c__int__0r;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::sealed::Sealed for Signature_unsigned__short__0l__char__0p__0c__char__0p__0c__unsigned__long__0c__unsigned__short__0c__int__0r {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::NativeFunctionSignature for Signature_unsigned__short__0l__char__0p__0c__char__0p__0c__unsigned__long__0c__unsigned__short__0c__int__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_char, *mut ::core::ffi::c_char, usize, u16, i32) -> u16>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::IndexTupleData) -> u16,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::IndexTupleData) -> u16,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__short__0l__struct__IndexTupleData__0p__0r;
impl c::sealed::Sealed for Signature_unsigned__short__0l__struct__IndexTupleData__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__short__0l__struct__IndexTupleData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::IndexTupleData) -> u16,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(u32, bool, bool, *mut ::core::ffi::c_void) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(u32, bool, bool, *mut ::core::ffi::c_void) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__ResourceReleasePhase__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__void__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__ResourceReleasePhase__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__void__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__ResourceReleasePhase__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__void__0p__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(u32, bool, bool, *mut ::core::ffi::c_void) -> ()>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::SharedInvalidationMessage,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::SharedInvalidationMessage,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__SharedInvalidationMessage__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__SharedInvalidationMessage__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__SharedInvalidationMessage__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::SharedInvalidationMessage,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(u32, u32, u32, *mut ::core::ffi::c_void) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(u32, u32, u32, *mut ::core::ffi::c_void) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__SubXactEvent__0c__unsigned__int__0c__unsigned__int__0c__void__0p__0r;
impl c::sealed::Sealed
    for Signature_void__0l__SubXactEvent__0c__unsigned__int__0c__unsigned__int__0c__void__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__SubXactEvent__0c__unsigned__int__0c__unsigned__int__0c__void__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_4<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(u32, u32, u32, *mut ::core::ffi::c_void) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(u32, *mut ::core::ffi::c_void) -> ()>,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(u32, *mut ::core::ffi::c_void) -> ()>,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__XactEvent__0c__void__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__XactEvent__0c__void__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__XactEvent__0c__void__0p__0r {
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<unsafe extern "C-unwind" fn(u32, *mut ::core::ffi::c_void) -> ()>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(bool, *mut ::core::ffi::c_void) -> ()>,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(bool, *mut ::core::ffi::c_void) -> ()>,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__0x5f426f6f6c__0c__void__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__0x5f426f6f6c__0c__void__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__0x5f426f6f6c__0c__void__0p__0r {
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<unsafe extern "C-unwind" fn(bool, *mut ::core::ffi::c_void) -> ()>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_char, u32) -> ()>,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_char, u32) -> ()>,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__char__0p__0c__unsigned__int__0r;
impl c::sealed::Sealed for Signature_void__0l__char__0p__0c__unsigned__int__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__char__0p__0c__unsigned__int__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_char, u32) -> ()>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_char) -> ()>,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_char) -> ()>,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__const__char__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__const__char__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__const__char__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_char) -> ()>,
    >;
}
const _: () = assert!(::core::mem::size_of::<*const ::core::ffi::c_char>() == 8);
const _: () = assert!(::core::mem::align_of::<*const ::core::ffi::c_char>() == 8);
#[cfg(feature = "pg15")]
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<
    A0: c::expression::ImplicitTo<
            c::expression::CPointer<
                c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
                c::expression::ReadOnly,
            >,
        >,
> c::expression::Call<(A0,)> for Signature_void__0l__const__char__0p__0r
{
    type Output = ();
    unsafe fn call(pointer: Self::Pointer, args: (A0,)) -> Self::Output {
        let function = pointer.expect("C indirect call requires a non-null function pointer");
        const {
            assert!(!::core::mem::needs_drop::<A0>());
        }
        const _: () = assert!(!::core::mem::needs_drop::<*const ::core::ffi::c_char>());
        let native0 = <c::expression::CPointer<
            c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
            c::expression::ReadOnly,
        > as c::expression::CType>::into_storage(c::expression::implicit::<
            c::expression::CPointer<
                c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
                c::expression::ReadOnly,
            >,
            _,
        >(args.0));
        // SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
        let result = unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0)) };
        result
    }
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                u32,
                crate::__pgrx_c_bindings::Datum,
                bool,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                u32,
                crate::__pgrx_c_bindings::Datum,
                bool,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__const__char__0p__0c__const__char__0p__0c__enum__PasswordType__0c__unsigned__long__0c__0x5f426f6f6c__0r;
impl c::sealed::Sealed for Signature_void__0l__const__char__0p__0c__const__char__0p__0c__enum__PasswordType__0c__unsigned__long__0c__0x5f426f6f6c__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__const__char__0p__0c__const__char__0p__0c__enum__PasswordType__0c__unsigned__long__0c__0x5f426f6f6c__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_char, *const ::core::ffi::c_char, u32, crate::__pgrx_c_bindings::Datum, bool) -> ()>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const ::core::ffi::c_char, *mut ::core::ffi::c_void) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const ::core::ffi::c_char, *mut ::core::ffi::c_void) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__const__char__0p__0c__void__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__const__char__0p__0c__void__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__const__char__0p__0c__void__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const ::core::ffi::c_char, *mut ::core::ffi::c_void) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::ObjectAddress,
                *const ::core::ffi::c_char,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::ObjectAddress,
                *const ::core::ffi::c_char,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__const__struct__ObjectAddress__0p__0c__const__char__0p__0r;
impl c::sealed::Sealed
    for Signature_void__0l__const__struct__ObjectAddress__0p__0c__const__char__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__const__struct__ObjectAddress__0p__0c__const__char__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::ObjectAddress,
                *const ::core::ffi::c_char,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::RangeVar,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Oid,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::RangeVar,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Oid,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__const__struct__RangeVar__0p__0c__unsigned__int__0c__unsigned__int__0c__void__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__const__struct__RangeVar__0p__0c__unsigned__int__0c__unsigned__int__0c__void__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__const__struct__RangeVar__0p__0c__unsigned__int__0c__unsigned__int__0c__void__0p__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::RangeVar, crate::__pgrx_c_bindings::Oid, crate::__pgrx_c_bindings::Oid, *mut ::core::ffi::c_void) -> ()>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::SubscriptingRef,
                *mut crate::__pgrx_c_bindings::SubscriptingRefState,
                *mut crate::__pgrx_c_bindings::SubscriptExecSteps,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::SubscriptingRef,
                *mut crate::__pgrx_c_bindings::SubscriptingRefState,
                *mut crate::__pgrx_c_bindings::SubscriptExecSteps,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__const__struct__SubscriptingRef__0p__0c__struct__SubscriptingRefState__0p__0c__struct__SubscriptExecSteps__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__const__struct__SubscriptingRef__0p__0c__struct__SubscriptingRefState__0p__0c__struct__SubscriptExecSteps__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__const__struct__SubscriptingRef__0p__0c__struct__SubscriptingRefState__0p__0c__struct__SubscriptExecSteps__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::SubscriptingRef, *mut crate::__pgrx_c_bindings::SubscriptingRefState, *mut crate::__pgrx_c_bindings::SubscriptExecSteps) -> ()>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(f64, *mut ::core::ffi::c_void) -> ()>,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(f64, *mut ::core::ffi::c_void) -> ()>,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__double__0c__void__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__double__0c__void__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__double__0c__void__0p__0r {
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<unsafe extern "C-unwind" fn(f64, *mut ::core::ffi::c_void) -> ()>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                *mut crate::__pgrx_c_bindings::FmgrInfo,
                *mut crate::__pgrx_c_bindings::Datum,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                *mut crate::__pgrx_c_bindings::FmgrInfo,
                *mut crate::__pgrx_c_bindings::Datum,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__enum__FmgrHookEventType__0c__struct__FmgrInfo__0p__0c__unsigned__long__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__enum__FmgrHookEventType__0c__struct__FmgrInfo__0p__0c__unsigned__long__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__enum__FmgrHookEventType__0c__struct__FmgrInfo__0p__0c__unsigned__long__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(u32, *mut crate::__pgrx_c_bindings::FmgrInfo, *mut crate::__pgrx_c_bindings::Datum) -> ()>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                crate::__pgrx_c_bindings::Oid,
                *const ::core::ffi::c_char,
                i32,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                crate::__pgrx_c_bindings::Oid,
                *const ::core::ffi::c_char,
                i32,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__enum__ObjectAccessType__0c__unsigned__int__0c__const__char__0p__0c__int__0c__void__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__enum__ObjectAccessType__0c__unsigned__int__0c__const__char__0p__0c__int__0c__void__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__enum__ObjectAccessType__0c__unsigned__int__0c__const__char__0p__0c__int__0c__void__0p__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(u32, crate::__pgrx_c_bindings::Oid, *const ::core::ffi::c_char, i32, *mut ::core::ffi::c_void) -> ()>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Oid,
                i32,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Oid,
                i32,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__enum__ObjectAccessType__0c__unsigned__int__0c__unsigned__int__0c__int__0c__void__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__enum__ObjectAccessType__0c__unsigned__int__0c__unsigned__int__0c__int__0c__void__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__enum__ObjectAccessType__0c__unsigned__int__0c__unsigned__int__0c__int__0c__void__0p__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(u32, crate::__pgrx_c_bindings::Oid, crate::__pgrx_c_bindings::Oid, i32, *mut ::core::ffi::c_void) -> ()>>;
}
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn(i32) -> ()>>() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn(i32) -> ()>>() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__int__0r;
impl c::sealed::Sealed for Signature_void__0l__int__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__int__0r {
    type Physical =
        PhysicalFunction_C_unwind_1<::core::option::Option<unsafe extern "C-unwind" fn(i32) -> ()>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                i32,
                u32,
                crate::__pgrx_c_bindings::Oid,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                i32,
                u32,
                crate::__pgrx_c_bindings::Oid,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__int__0c__unsigned__int__0c__unsigned__int__0c__void__0p__0r;
impl c::sealed::Sealed
    for Signature_void__0l__int__0c__unsigned__int__0c__unsigned__int__0c__void__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__int__0c__unsigned__int__0c__unsigned__int__0c__void__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_4<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                i32,
                u32,
                crate::__pgrx_c_bindings::Oid,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(i32, crate::__pgrx_c_bindings::Datum) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(i32, crate::__pgrx_c_bindings::Datum) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__int__0c__unsigned__long__0r;
impl c::sealed::Sealed for Signature_void__0l__int__0c__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__int__0c__unsigned__long__0r {
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(i32, crate::__pgrx_c_bindings::Datum) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(i32, *mut ::core::ffi::c_void) -> ()>,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(i32, *mut ::core::ffi::c_void) -> ()>,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__int__0c__void__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__int__0c__void__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__int__0c__void__0p__0r {
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<unsafe extern "C-unwind" fn(i32, *mut ::core::ffi::c_void) -> ()>,
    >;
}
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ArchiveModuleCallbacks,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ArchiveModuleCallbacks,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg15")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__ArchiveModuleCallbacks__0p__0r;
#[cfg(feature = "pg15")]
impl c::sealed::Sealed for Signature_void__0l__struct__ArchiveModuleCallbacks__0p__0r {}
#[cfg(feature = "pg15")]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__ArchiveModuleCallbacks__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ArchiveModuleCallbacks,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::AsyncRequest) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::AsyncRequest) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__AsyncRequest__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__AsyncRequest__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__AsyncRequest__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::AsyncRequest) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::BrinDesc,
                crate::__pgrx_c_bindings::Datum,
                *mut crate::__pgrx_c_bindings::Datum,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::BrinDesc,
                crate::__pgrx_c_bindings::Datum,
                *mut crate::__pgrx_c_bindings::Datum,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__BrinDesc__0p__0c__unsigned__long__0c__unsigned__long__0p__0r;
impl c::sealed::Sealed
    for Signature_void__0l__struct__BrinDesc__0p__0c__unsigned__long__0c__unsigned__long__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__BrinDesc__0p__0c__unsigned__long__0c__unsigned__long__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::BrinDesc,
                crate::__pgrx_c_bindings::Datum,
                *mut crate::__pgrx_c_bindings::Datum,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::CustomScanState) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::CustomScanState) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__CustomScanState__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__CustomScanState__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__CustomScanState__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::CustomScanState) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::CustomScanState,
                *mut crate::__pgrx_c_bindings::EState,
                i32,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::CustomScanState,
                *mut crate::__pgrx_c_bindings::EState,
                i32,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__CustomScanState__0p__0c__struct__EState__0p__0c__int__0r;
impl c::sealed::Sealed
    for Signature_void__0l__struct__CustomScanState__0p__0c__struct__EState__0p__0c__int__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__CustomScanState__0p__0c__struct__EState__0p__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::CustomScanState,
                *mut crate::__pgrx_c_bindings::EState,
                i32,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::CustomScanState,
                *mut crate::__pgrx_c_bindings::List,
                *mut crate::__pgrx_c_bindings::ExplainState,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::CustomScanState,
                *mut crate::__pgrx_c_bindings::List,
                *mut crate::__pgrx_c_bindings::ExplainState,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__CustomScanState__0p__0c__struct__List__0p__0c__struct__ExplainState__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__CustomScanState__0p__0c__struct__List__0p__0c__struct__ExplainState__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__CustomScanState__0p__0c__struct__List__0p__0c__struct__ExplainState__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::CustomScanState, *mut crate::__pgrx_c_bindings::List, *mut crate::__pgrx_c_bindings::ExplainState) -> ()>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::CustomScanState,
                *mut crate::__pgrx_c_bindings::ParallelContext,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::CustomScanState,
                *mut crate::__pgrx_c_bindings::ParallelContext,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__CustomScanState__0p__0c__struct__ParallelContext__0p__0c__void__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__CustomScanState__0p__0c__struct__ParallelContext__0p__0c__void__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__CustomScanState__0p__0c__struct__ParallelContext__0p__0c__void__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::CustomScanState, *mut crate::__pgrx_c_bindings::ParallelContext, *mut ::core::ffi::c_void) -> ()>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::CustomScanState,
                *mut crate::__pgrx_c_bindings::shm_toc,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::CustomScanState,
                *mut crate::__pgrx_c_bindings::shm_toc,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__CustomScanState__0p__0c__struct__shm_toc__0p__0c__void__0p__0r;
impl c::sealed::Sealed
    for Signature_void__0l__struct__CustomScanState__0p__0c__struct__shm_toc__0p__0c__void__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__CustomScanState__0p__0c__struct__shm_toc__0p__0c__void__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::CustomScanState,
                *mut crate::__pgrx_c_bindings::shm_toc,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::EState,
                *mut crate::__pgrx_c_bindings::ExecRowMark,
                crate::__pgrx_c_bindings::Datum,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                *mut bool,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::EState,
                *mut crate::__pgrx_c_bindings::ExecRowMark,
                crate::__pgrx_c_bindings::Datum,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                *mut bool,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__EState__0p__0c__struct__ExecRowMark__0p__0c__unsigned__long__0c__struct__TupleTableSlot__0p__0c__0x5f426f6f6c__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__EState__0p__0c__struct__ExecRowMark__0p__0c__unsigned__long__0c__struct__TupleTableSlot__0p__0c__0x5f426f6f6c__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__EState__0p__0c__struct__ExecRowMark__0p__0c__unsigned__long__0c__struct__TupleTableSlot__0p__0c__0x5f426f6f6c__0p__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::EState, *mut crate::__pgrx_c_bindings::ExecRowMark, crate::__pgrx_c_bindings::Datum, *mut crate::__pgrx_c_bindings::TupleTableSlot, *mut bool) -> ()>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::EState,
                *mut crate::__pgrx_c_bindings::ResultRelInfo,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::EState,
                *mut crate::__pgrx_c_bindings::ResultRelInfo,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__EState__0p__0c__struct__ResultRelInfo__0p__0r;
impl c::sealed::Sealed
    for Signature_void__0l__struct__EState__0p__0c__struct__ResultRelInfo__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__EState__0p__0c__struct__ResultRelInfo__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::EState,
                *mut crate::__pgrx_c_bindings::ResultRelInfo,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ErrorData) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ErrorData) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__ErrorData__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__ErrorData__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__ErrorData__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ErrorData) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ExpandedObjectHeader,
                *mut ::core::ffi::c_void,
                usize,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ExpandedObjectHeader,
                *mut ::core::ffi::c_void,
                usize,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__ExpandedObjectHeader__0p__0c__void__0p__0c__unsigned__long__0r;
impl c::sealed::Sealed
    for Signature_void__0l__struct__ExpandedObjectHeader__0p__0c__void__0p__0c__unsigned__long__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__ExpandedObjectHeader__0p__0c__void__0p__0c__unsigned__long__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ExpandedObjectHeader,
                *mut ::core::ffi::c_void,
                usize,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ExprState,
                *mut crate::__pgrx_c_bindings::ExprEvalStep,
                *mut crate::__pgrx_c_bindings::ExprContext,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ExprState,
                *mut crate::__pgrx_c_bindings::ExprEvalStep,
                *mut crate::__pgrx_c_bindings::ExprContext,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__ExprState__0p__0c__struct__ExprEvalStep__0p__0c__struct__ExprContext__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__ExprState__0p__0c__struct__ExprEvalStep__0p__0c__struct__ExprContext__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__ExprState__0p__0c__struct__ExprEvalStep__0p__0c__struct__ExprContext__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ExprState, *mut crate::__pgrx_c_bindings::ExprEvalStep, *mut crate::__pgrx_c_bindings::ExprContext) -> ()>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ExtensibleNode) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ExtensibleNode) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__ExtensibleNode__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__ExtensibleNode__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__ExtensibleNode__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ExtensibleNode) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ExtensibleNode,
                *const crate::__pgrx_c_bindings::ExtensibleNode,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ExtensibleNode,
                *const crate::__pgrx_c_bindings::ExtensibleNode,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__ExtensibleNode__0p__0c__const__struct__ExtensibleNode__0p__0r;
impl c::sealed::Sealed
    for Signature_void__0l__struct__ExtensibleNode__0p__0c__const__struct__ExtensibleNode__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__ExtensibleNode__0p__0c__const__struct__ExtensibleNode__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ExtensibleNode,
                *const crate::__pgrx_c_bindings::ExtensibleNode,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ForeignScanState) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ForeignScanState) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__ForeignScanState__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__ForeignScanState__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__ForeignScanState__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ForeignScanState) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ForeignScanState, i32) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ForeignScanState, i32) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__ForeignScanState__0p__0c__int__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__ForeignScanState__0p__0c__int__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__ForeignScanState__0p__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ForeignScanState, i32) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ForeignScanState,
                *mut crate::__pgrx_c_bindings::ExplainState,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ForeignScanState,
                *mut crate::__pgrx_c_bindings::ExplainState,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__ForeignScanState__0p__0c__struct__ExplainState__0p__0r;
impl c::sealed::Sealed
    for Signature_void__0l__struct__ForeignScanState__0p__0c__struct__ExplainState__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__ForeignScanState__0p__0c__struct__ExplainState__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ForeignScanState,
                *mut crate::__pgrx_c_bindings::ExplainState,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ForeignScanState,
                *mut crate::__pgrx_c_bindings::ParallelContext,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ForeignScanState,
                *mut crate::__pgrx_c_bindings::ParallelContext,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__ForeignScanState__0p__0c__struct__ParallelContext__0p__0c__void__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__ForeignScanState__0p__0c__struct__ParallelContext__0p__0c__void__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__ForeignScanState__0p__0c__struct__ParallelContext__0p__0c__void__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ForeignScanState, *mut crate::__pgrx_c_bindings::ParallelContext, *mut ::core::ffi::c_void) -> ()>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ForeignScanState,
                *mut crate::__pgrx_c_bindings::shm_toc,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ForeignScanState,
                *mut crate::__pgrx_c_bindings::shm_toc,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__ForeignScanState__0p__0c__struct__shm_toc__0p__0c__void__0p__0r;
impl c::sealed::Sealed
    for Signature_void__0l__struct__ForeignScanState__0p__0c__struct__shm_toc__0p__0c__void__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__ForeignScanState__0p__0c__struct__shm_toc__0p__0c__void__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ForeignScanState,
                *mut crate::__pgrx_c_bindings::shm_toc,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::GinBtreeData,
                *mut ::core::ffi::c_char,
                u32,
                *mut ::core::ffi::c_char,
                u32,
                *mut ::core::ffi::c_char,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::GinBtreeData,
                *mut ::core::ffi::c_char,
                u32,
                *mut ::core::ffi::c_char,
                u32,
                *mut ::core::ffi::c_char,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__GinBtreeData__0p__0c__char__0p__0c__unsigned__int__0c__char__0p__0c__unsigned__int__0c__char__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__GinBtreeData__0p__0c__char__0p__0c__unsigned__int__0c__char__0p__0c__unsigned__int__0c__char__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__GinBtreeData__0p__0c__char__0p__0c__unsigned__int__0c__char__0p__0c__unsigned__int__0c__char__0p__0r {
 type Physical = PhysicalFunction_C_unwind_6<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::GinBtreeData, *mut ::core::ffi::c_char, u32, *mut ::core::ffi::c_char, u32, *mut ::core::ffi::c_char) -> ()>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::GinBtreeData,
                i32,
                *mut crate::__pgrx_c_bindings::GinBtreeStack,
                *mut ::core::ffi::c_void,
                u32,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::GinBtreeData,
                i32,
                *mut crate::__pgrx_c_bindings::GinBtreeStack,
                *mut ::core::ffi::c_void,
                u32,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__GinBtreeData__0p__0c__int__0c__struct__GinBtreeStack__0p__0c__void__0p__0c__unsigned__int__0c__void__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__GinBtreeData__0p__0c__int__0c__struct__GinBtreeStack__0p__0c__void__0p__0c__unsigned__int__0c__void__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__GinBtreeData__0p__0c__int__0c__struct__GinBtreeStack__0p__0c__void__0p__0c__unsigned__int__0c__void__0p__0r {
 type Physical = PhysicalFunction_C_unwind_6<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::GinBtreeData, i32, *mut crate::__pgrx_c_bindings::GinBtreeStack, *mut ::core::ffi::c_void, u32, *mut ::core::ffi::c_void) -> ()>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::HeapTupleData) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::HeapTupleData) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__HeapTupleData__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__HeapTupleData__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__HeapTupleData__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::HeapTupleData) -> (),
        >,
    >;
}
const _: () = assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::HeapTupleData>() == 8);
const _: () = assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::HeapTupleData>() == 8);
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<
    A0: c::expression::ImplicitTo<
            c::expression::CPointer<
                c::expression::CRecord<crate::__pgrx_c_bindings::HeapTupleData>,
                c::expression::ReadWrite,
            >,
        >,
> c::expression::Call<(A0,)> for Signature_void__0l__struct__HeapTupleData__0p__0r
{
    type Output = ();
    unsafe fn call(pointer: Self::Pointer, args: (A0,)) -> Self::Output {
        let function = pointer.expect("C indirect call requires a non-null function pointer");
        const {
            assert!(!::core::mem::needs_drop::<A0>());
        }
        const _: () =
            assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::HeapTupleData>());
        let native0 = <c::expression::CPointer<
            c::expression::CRecord<crate::__pgrx_c_bindings::HeapTupleData>,
            c::expression::ReadWrite,
        > as c::expression::CType>::into_storage(c::expression::implicit::<
            c::expression::CPointer<
                c::expression::CRecord<crate::__pgrx_c_bindings::HeapTupleData>,
                c::expression::ReadWrite,
            >,
            _,
        >(args.0));
        // SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
        let result = unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0)) };
        result
    }
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::IndexFetchTableData) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::IndexFetchTableData) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__IndexFetchTableData__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__IndexFetchTableData__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__IndexFetchTableData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::IndexFetchTableData) -> (),
        >,
    >;
}
const _: () =
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::IndexFetchTableData>() == 8);
const _: () =
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::IndexFetchTableData>() == 8);
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<
    A0: c::expression::ImplicitTo<
            c::expression::CPointer<
                c::expression::CRecord<crate::__pgrx_c_bindings::IndexFetchTableData>,
                c::expression::ReadWrite,
            >,
        >,
> c::expression::Call<(A0,)> for Signature_void__0l__struct__IndexFetchTableData__0p__0r
{
    type Output = ();
    unsafe fn call(pointer: Self::Pointer, args: (A0,)) -> Self::Output {
        let function = pointer.expect("C indirect call requires a non-null function pointer");
        const {
            assert!(!::core::mem::needs_drop::<A0>());
        }
        const _: () = assert!(!::core::mem::needs_drop::<
            *mut crate::__pgrx_c_bindings::IndexFetchTableData,
        >());
        let native0 = <c::expression::CPointer<
            c::expression::CRecord<crate::__pgrx_c_bindings::IndexFetchTableData>,
            c::expression::ReadWrite,
        > as c::expression::CType>::into_storage(c::expression::implicit::<
            c::expression::CPointer<
                c::expression::CRecord<crate::__pgrx_c_bindings::IndexFetchTableData>,
                c::expression::ReadWrite,
            >,
            _,
        >(args.0));
        // SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
        let result = unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0)) };
        result
    }
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::IndexScanDescData) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::IndexScanDescData) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__IndexScanDescData__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__IndexScanDescData__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__IndexScanDescData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::IndexScanDescData) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::IndexScanDescData,
                *mut crate::__pgrx_c_bindings::ScanKeyData,
                i32,
                *mut crate::__pgrx_c_bindings::ScanKeyData,
                i32,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::IndexScanDescData,
                *mut crate::__pgrx_c_bindings::ScanKeyData,
                i32,
                *mut crate::__pgrx_c_bindings::ScanKeyData,
                i32,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__IndexScanDescData__0p__0c__struct__ScanKeyData__0p__0c__int__0c__struct__ScanKeyData__0p__0c__int__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__IndexScanDescData__0p__0c__struct__ScanKeyData__0p__0c__int__0c__struct__ScanKeyData__0p__0c__int__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__IndexScanDescData__0p__0c__struct__ScanKeyData__0p__0c__int__0c__struct__ScanKeyData__0p__0c__int__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::IndexScanDescData, *mut crate::__pgrx_c_bindings::ScanKeyData, i32, *mut crate::__pgrx_c_bindings::ScanKeyData, i32) -> ()>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::JitContext) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::JitContext) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__JitContext__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__JitContext__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__JitContext__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::JitContext) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::JitProviderCallbacks) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::JitProviderCallbacks) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__JitProviderCallbacks__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__JitProviderCallbacks__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__JitProviderCallbacks__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::JitProviderCallbacks) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::LWLock,
                u32,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::LWLock,
                u32,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__LWLock__0p__0c__enum__LWLockMode__0c__void__0p__0r;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::sealed::Sealed
    for Signature_void__0l__struct__LWLock__0p__0c__enum__LWLockMode__0c__void__0p__0r
{
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__LWLock__0p__0c__enum__LWLockMode__0c__void__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::LWLock,
                u32,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::List, u32, bool) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::List, u32, bool) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__List__0p__0c__enum__DropBehavior__0c__0x5f426f6f6c__0r;
impl c::sealed::Sealed
    for Signature_void__0l__struct__List__0p__0c__enum__DropBehavior__0c__0x5f426f6f6c__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__List__0p__0c__enum__DropBehavior__0c__0x5f426f6f6c__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::List, u32, bool) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::LogicalDecodingContext,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::LogicalDecodingContext,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__LogicalDecodingContext__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__LogicalDecodingContext__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__LogicalDecodingContext__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::LogicalDecodingContext,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::LogicalDecodingContext,
                *mut crate::__pgrx_c_bindings::OutputPluginOptions,
                bool,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::LogicalDecodingContext,
                *mut crate::__pgrx_c_bindings::OutputPluginOptions,
                bool,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__OutputPluginOptions__0p__0c__0x5f426f6f6c__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__OutputPluginOptions__0p__0c__0x5f426f6f6c__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__OutputPluginOptions__0p__0c__0x5f426f6f6c__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::LogicalDecodingContext, *mut crate::__pgrx_c_bindings::OutputPluginOptions, bool) -> ()>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::LogicalDecodingContext,
                *mut crate::__pgrx_c_bindings::ReorderBufferTXN,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::LogicalDecodingContext,
                *mut crate::__pgrx_c_bindings::ReorderBufferTXN,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__ReorderBufferTXN__0p__0r;
impl c::sealed::Sealed
    for Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__ReorderBufferTXN__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__ReorderBufferTXN__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::LogicalDecodingContext,
                *mut crate::__pgrx_c_bindings::ReorderBufferTXN,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::LogicalDecodingContext,
                *mut crate::__pgrx_c_bindings::ReorderBufferTXN,
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::ReorderBufferChange,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::LogicalDecodingContext,
                *mut crate::__pgrx_c_bindings::ReorderBufferTXN,
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::ReorderBufferChange,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__ReorderBufferTXN__0p__0c__struct__RelationData__0p__0c__struct__ReorderBufferChange__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__ReorderBufferTXN__0p__0c__struct__RelationData__0p__0c__struct__ReorderBufferChange__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__ReorderBufferTXN__0p__0c__struct__RelationData__0p__0c__struct__ReorderBufferChange__0p__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::LogicalDecodingContext, *mut crate::__pgrx_c_bindings::ReorderBufferTXN, *mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::ReorderBufferChange) -> ()>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::LogicalDecodingContext,
                *mut crate::__pgrx_c_bindings::ReorderBufferTXN,
                u64,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::LogicalDecodingContext,
                *mut crate::__pgrx_c_bindings::ReorderBufferTXN,
                u64,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::LogicalDecodingContext, *mut crate::__pgrx_c_bindings::ReorderBufferTXN, u64) -> ()>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::LogicalDecodingContext,
                *mut crate::__pgrx_c_bindings::ReorderBufferTXN,
                u64,
                bool,
                *const ::core::ffi::c_char,
                usize,
                *const ::core::ffi::c_char,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::LogicalDecodingContext,
                *mut crate::__pgrx_c_bindings::ReorderBufferTXN,
                u64,
                bool,
                *const ::core::ffi::c_char,
                usize,
                *const ::core::ffi::c_char,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0c__0x5f426f6f6c__0c__const__char__0p__0c__unsigned__long__0c__const__char__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0c__0x5f426f6f6c__0c__const__char__0p__0c__unsigned__long__0c__const__char__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0c__0x5f426f6f6c__0c__const__char__0p__0c__unsigned__long__0c__const__char__0p__0r {
 type Physical = PhysicalFunction_C_unwind_7<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::LogicalDecodingContext, *mut crate::__pgrx_c_bindings::ReorderBufferTXN, u64, bool, *const ::core::ffi::c_char, usize, *const ::core::ffi::c_char) -> ()>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::LogicalDecodingContext,
                *mut crate::__pgrx_c_bindings::ReorderBufferTXN,
                u64,
                i64,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::LogicalDecodingContext,
                *mut crate::__pgrx_c_bindings::ReorderBufferTXN,
                u64,
                i64,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0c__long__0r;
