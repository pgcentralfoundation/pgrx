// Shared generated C macro support.

use crate::__pgrx_c_macros as c;
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::GinBtreeData,
                i32,
                *mut crate::__pgrx_c_bindings::GinBtreeStack,
                *mut ::core::ffi::c_void,
                u32,
                *mut *mut ::core::ffi::c_void,
                *mut *mut ::core::ffi::c_char,
                *mut *mut ::core::ffi::c_char,
            ) -> u32,
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
                *mut *mut ::core::ffi::c_void,
                *mut *mut ::core::ffi::c_char,
                *mut *mut ::core::ffi::c_char,
            ) -> u32,
        >,
    >() == 8
);
#[doc(hidden)]
pub struct PhysicalFunction_C_unwind_8<P>(::core::marker::PhantomData<P>);
impl<P> c::sealed::Sealed for PhysicalFunction_C_unwind_8<P> {}
impl<A0, A1, A2, A3, A4, A5, A6, A7, R> c::expression::PhysicalFunctionPointer
    for PhysicalFunction_C_unwind_8<
        ::core::option::Option<unsafe extern "C-unwind" fn(A0, A1, A2, A3, A4, A5, A6, A7) -> R>,
    >
{
    type Pointer =
        ::core::option::Option<unsafe extern "C-unwind" fn(A0, A1, A2, A3, A4, A5, A6, A7) -> R>;
    fn null() -> Self::Pointer {
        None
    }
    fn address(pointer: Self::Pointer) -> *const () {
        pointer.map_or(::core::ptr::null(), |function| function as *const ())
    }
}
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_GinPlaceToPageRC__0l__struct__GinBtreeData__0p__0c__int__0c__struct__GinBtreeStack__0p__0c__void__0p__0c__unsigned__int__0c__void__0p__0p__0c__char__0p__0p__0c__char__0p__0p__0r;
impl c::sealed::Sealed for Signature_GinPlaceToPageRC__0l__struct__GinBtreeData__0p__0c__int__0c__struct__GinBtreeStack__0p__0c__void__0p__0c__unsigned__int__0c__void__0p__0p__0c__char__0p__0p__0c__char__0p__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_GinPlaceToPageRC__0l__struct__GinBtreeData__0p__0c__int__0c__struct__GinBtreeStack__0p__0c__void__0p__0c__unsigned__int__0c__void__0p__0p__0c__char__0p__0p__0c__char__0p__0p__0r {
 type Physical = PhysicalFunction_C_unwind_8<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::GinBtreeData, i32, *mut crate::__pgrx_c_bindings::GinBtreeStack, *mut ::core::ffi::c_void, u32, *mut *mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_char, *mut *mut ::core::ffi::c_char) -> u32>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
                *mut crate::__pgrx_c_bindings::QueryOperand,
                *mut crate::__pgrx_c_bindings::ExecPhraseData,
            ) -> u32,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
                *mut crate::__pgrx_c_bindings::QueryOperand,
                *mut crate::__pgrx_c_bindings::ExecPhraseData,
            ) -> u32,
        >,
    >() == 8
);
#[doc(hidden)]
pub struct PhysicalFunction_C_unwind_3<P>(::core::marker::PhantomData<P>);
impl<P> c::sealed::Sealed for PhysicalFunction_C_unwind_3<P> {}
impl<A0, A1, A2, R> c::expression::PhysicalFunctionPointer
    for PhysicalFunction_C_unwind_3<
        ::core::option::Option<unsafe extern "C-unwind" fn(A0, A1, A2) -> R>,
    >
{
    type Pointer = ::core::option::Option<unsafe extern "C-unwind" fn(A0, A1, A2) -> R>;
    fn null() -> Self::Pointer {
        None
    }
    fn address(pointer: Self::Pointer) -> *const () {
        pointer.map_or(::core::ptr::null(), |function| function as *const ())
    }
}
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_TSTernaryValue__0l__void__0p__0c__QueryOperand__0p__0c__struct__ExecPhraseData__0p__0r;
impl c::sealed::Sealed for Signature_TSTernaryValue__0l__void__0p__0c__QueryOperand__0p__0c__struct__ExecPhraseData__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_TSTernaryValue__0l__void__0p__0c__QueryOperand__0p__0c__struct__ExecPhraseData__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void, *mut crate::__pgrx_c_bindings::QueryOperand, *mut crate::__pgrx_c_bindings::ExecPhraseData) -> u32>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut bool, *mut *mut ::core::ffi::c_void, u32) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut bool, *mut *mut ::core::ffi::c_void, u32) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__0x5f426f6f6c__0p__0c__void__0p__0p__0c__GucSource__0r;
impl c::sealed::Sealed
    for Signature_0x5f426f6f6c__0l__0x5f426f6f6c__0p__0c__void__0p__0p__0c__GucSource__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__0x5f426f6f6c__0p__0c__void__0p__0p__0c__GucSource__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut bool, *mut *mut ::core::ffi::c_void, u32) -> bool,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut *mut ::core::ffi::c_char,
                *mut *mut ::core::ffi::c_void,
                u32,
            ) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut *mut ::core::ffi::c_char,
                *mut *mut ::core::ffi::c_void,
                u32,
            ) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__char__0p__0p__0c__void__0p__0p__0c__GucSource__0r;
impl c::sealed::Sealed
    for Signature_0x5f426f6f6c__0l__char__0p__0p__0c__void__0p__0p__0c__GucSource__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__char__0p__0p__0c__void__0p__0p__0c__GucSource__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut *mut ::core::ffi::c_char,
                *mut *mut ::core::ffi::c_void,
                u32,
            ) -> bool,
        >,
    >;
}
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
            ) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
pub struct PhysicalFunction_C_unwind_2<P>(::core::marker::PhantomData<P>);
impl<P> c::sealed::Sealed for PhysicalFunction_C_unwind_2<P> {}
impl<A0, A1, R> c::expression::PhysicalFunctionPointer
    for PhysicalFunction_C_unwind_2<
        ::core::option::Option<unsafe extern "C-unwind" fn(A0, A1) -> R>,
    >
{
    type Pointer = ::core::option::Option<unsafe extern "C-unwind" fn(A0, A1) -> R>;
    fn null() -> Self::Pointer {
        None
    }
    fn address(pointer: Self::Pointer) -> *const () {
        pointer.map_or(::core::ptr::null(), |function| function as *const ())
    }
}
#[cfg(feature = "pg15")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__const__char__0p__0c__const__char__0p__0r;
#[cfg(feature = "pg15")]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__const__char__0p__0c__const__char__0p__0r {}
#[cfg(feature = "pg15")]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__const__char__0p__0c__const__char__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
            ) -> bool,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::ExtensibleNode,
                *const crate::__pgrx_c_bindings::ExtensibleNode,
            ) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::ExtensibleNode,
                *const crate::__pgrx_c_bindings::ExtensibleNode,
            ) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__const__struct__ExtensibleNode__0p__0c__const__struct__ExtensibleNode__0p__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__const__struct__ExtensibleNode__0p__0c__const__struct__ExtensibleNode__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__const__struct__ExtensibleNode__0p__0c__const__struct__ExtensibleNode__0p__0r {
 type Physical = PhysicalFunction_C_unwind_2<::core::option::Option<unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::ExtensibleNode, *const crate::__pgrx_c_bindings::ExtensibleNode) -> bool>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut f64, *mut *mut ::core::ffi::c_void, u32) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut f64, *mut *mut ::core::ffi::c_void, u32) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__double__0p__0c__void__0p__0p__0c__GucSource__0r;
impl c::sealed::Sealed
    for Signature_0x5f426f6f6c__0l__double__0p__0c__void__0p__0p__0c__GucSource__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__double__0p__0c__void__0p__0p__0c__GucSource__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut f64, *mut *mut ::core::ffi::c_void, u32) -> bool,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut i32, *mut *mut ::core::ffi::c_void, u32) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut i32, *mut *mut ::core::ffi::c_void, u32) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__int__0p__0c__void__0p__0p__0c__GucSource__0r;
impl c::sealed::Sealed
    for Signature_0x5f426f6f6c__0l__int__0p__0c__void__0p__0p__0c__GucSource__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__int__0p__0c__void__0p__0p__0c__GucSource__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut i32, *mut *mut ::core::ffi::c_void, u32) -> bool,
        >,
    >;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn(i32, i32) -> bool>>()
        == 8
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn(i32, i32) -> bool>>(
    ) == 8
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__int__0c__int__0r;
#[cfg(any(feature = "pg15", feature = "pg16"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__int__0c__int__0r {}
#[cfg(any(feature = "pg15", feature = "pg16"))]
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__int__0c__int__0r {
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<unsafe extern "C-unwind" fn(i32, i32) -> bool>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                i32,
                *mut crate::__pgrx_c_bindings::SortSupportData,
            ) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                i32,
                *mut crate::__pgrx_c_bindings::SortSupportData,
            ) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__int__0c__struct__SortSupportData__0p__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__int__0c__struct__SortSupportData__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__int__0c__struct__SortSupportData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                i32,
                *mut crate::__pgrx_c_bindings::SortSupportData,
            ) -> bool,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::AppendState) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::AppendState) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
pub struct PhysicalFunction_C_unwind_1<P>(::core::marker::PhantomData<P>);
impl<P> c::sealed::Sealed for PhysicalFunction_C_unwind_1<P> {}
impl<A0, R> c::expression::PhysicalFunctionPointer
    for PhysicalFunction_C_unwind_1<::core::option::Option<unsafe extern "C-unwind" fn(A0) -> R>>
{
    type Pointer = ::core::option::Option<unsafe extern "C-unwind" fn(A0) -> R>;
    fn null() -> Self::Pointer {
        None
    }
    fn address(pointer: Self::Pointer) -> *const () {
        pointer.map_or(::core::ptr::null(), |function| function as *const ())
    }
}
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__AppendState__0p__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__AppendState__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__struct__AppendState__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::AppendState) -> bool,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ExprState) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ExprState) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__ExprState__0p__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__ExprState__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__struct__ExprState__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ExprState) -> bool,
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
            ) -> bool,
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
            ) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__ExprState__0p__0c__struct__ExprEvalStep__0p__0c__struct__ExprContext__0p__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__ExprState__0p__0c__struct__ExprEvalStep__0p__0c__struct__ExprContext__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__struct__ExprState__0p__0c__struct__ExprEvalStep__0p__0c__struct__ExprContext__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ExprState, *mut crate::__pgrx_c_bindings::ExprEvalStep, *mut crate::__pgrx_c_bindings::ExprContext) -> bool>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ForeignPath) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ForeignPath) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__ForeignPath__0p__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__ForeignPath__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__struct__ForeignPath__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ForeignPath) -> bool,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ForeignScanState,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
            ) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ForeignScanState,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
            ) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__ForeignScanState__0p__0c__struct__TupleTableSlot__0p__0r;
impl c::sealed::Sealed
    for Signature_0x5f426f6f6c__0l__struct__ForeignScanState__0p__0c__struct__TupleTableSlot__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__struct__ForeignScanState__0p__0c__struct__TupleTableSlot__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ForeignScanState,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
            ) -> bool,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::GinBtreeData,
                *mut ::core::ffi::c_char,
            ) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::GinBtreeData,
                *mut ::core::ffi::c_char,
            ) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__GinBtreeData__0p__0c__char__0p__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__GinBtreeData__0p__0c__char__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__struct__GinBtreeData__0p__0c__char__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::GinBtreeData,
                *mut ::core::ffi::c_char,
            ) -> bool,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::GinBtreeData,
                *mut crate::__pgrx_c_bindings::GinBtreeStack,
            ) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::GinBtreeData,
                *mut crate::__pgrx_c_bindings::GinBtreeStack,
            ) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__GinBtreeData__0p__0c__struct__GinBtreeStack__0p__0r;
impl c::sealed::Sealed
    for Signature_0x5f426f6f6c__0l__struct__GinBtreeData__0p__0c__struct__GinBtreeStack__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__struct__GinBtreeData__0p__0c__struct__GinBtreeStack__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::GinBtreeData,
                *mut crate::__pgrx_c_bindings::GinBtreeStack,
            ) -> bool,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::GinScanKeyData) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::GinScanKeyData) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__GinScanKeyData__0p__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__GinScanKeyData__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__struct__GinScanKeyData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::GinScanKeyData) -> bool,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::IndexFetchTableData,
                *mut crate::__pgrx_c_bindings::ItemPointerData,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                *mut bool,
                *mut bool,
            ) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::IndexFetchTableData,
                *mut crate::__pgrx_c_bindings::ItemPointerData,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                *mut bool,
                *mut bool,
            ) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
pub struct PhysicalFunction_C_unwind_6<P>(::core::marker::PhantomData<P>);
impl<P> c::sealed::Sealed for PhysicalFunction_C_unwind_6<P> {}
impl<A0, A1, A2, A3, A4, A5, R> c::expression::PhysicalFunctionPointer
    for PhysicalFunction_C_unwind_6<
        ::core::option::Option<unsafe extern "C-unwind" fn(A0, A1, A2, A3, A4, A5) -> R>,
    >
{
    type Pointer = ::core::option::Option<unsafe extern "C-unwind" fn(A0, A1, A2, A3, A4, A5) -> R>;
    fn null() -> Self::Pointer {
        None
    }
    fn address(pointer: Self::Pointer) -> *const () {
        pointer.map_or(::core::ptr::null(), |function| function as *const ())
    }
}
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__IndexFetchTableData__0p__0c__struct__ItemPointerData__0p__0c__struct__SnapshotData__0p__0c__struct__TupleTableSlot__0p__0c__0x5f426f6f6c__0p__0c__0x5f426f6f6c__0p__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__IndexFetchTableData__0p__0c__struct__ItemPointerData__0p__0c__struct__SnapshotData__0p__0c__struct__TupleTableSlot__0p__0c__0x5f426f6f6c__0p__0c__0x5f426f6f6c__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__struct__IndexFetchTableData__0p__0c__struct__ItemPointerData__0p__0c__struct__SnapshotData__0p__0c__struct__TupleTableSlot__0p__0c__0x5f426f6f6c__0p__0c__0x5f426f6f6c__0p__0r {
 type Physical = PhysicalFunction_C_unwind_6<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::IndexFetchTableData, *mut crate::__pgrx_c_bindings::ItemPointerData, *mut crate::__pgrx_c_bindings::SnapshotData, *mut crate::__pgrx_c_bindings::TupleTableSlot, *mut bool, *mut bool) -> bool>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::IndexScanDescData,
                i32,
            ) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::IndexScanDescData,
                i32,
            ) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__IndexScanDescData__0p__0c__enum__ScanDirection__0r;
impl c::sealed::Sealed
    for Signature_0x5f426f6f6c__0l__struct__IndexScanDescData__0p__0c__enum__ScanDirection__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__struct__IndexScanDescData__0p__0c__enum__ScanDirection__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::IndexScanDescData,
                i32,
            ) -> bool,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ItemPointerData,
                *mut ::core::ffi::c_void,
            ) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ItemPointerData,
                *mut ::core::ffi::c_void,
            ) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__ItemPointerData__0p__0c__void__0p__0r;
impl c::sealed::Sealed
    for Signature_0x5f426f6f6c__0l__struct__ItemPointerData__0p__0c__void__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__struct__ItemPointerData__0p__0c__void__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ItemPointerData,
                *mut ::core::ffi::c_void,
            ) -> bool,
        >,
    >;
}
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::List, bool) -> bool,
        >,
    >() == 8
);
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::List, bool) -> bool,
        >,
    >() == 8
);
#[cfg(feature = "pg15")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__List__0p__0c__0x5f426f6f6c__0r;
#[cfg(feature = "pg15")]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__List__0p__0c__0x5f426f6f6c__0r {}
#[cfg(feature = "pg15")]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__struct__List__0p__0c__0x5f426f6f6c__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::List, bool) -> bool,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::LogicalDecodingContext,
                crate::__pgrx_c_bindings::TransactionId,
                *const ::core::ffi::c_char,
            ) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::LogicalDecodingContext,
                crate::__pgrx_c_bindings::TransactionId,
                *const ::core::ffi::c_char,
            ) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__LogicalDecodingContext__0p__0c__unsigned__int__0c__const__char__0p__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__LogicalDecodingContext__0p__0c__unsigned__int__0c__const__char__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__struct__LogicalDecodingContext__0p__0c__unsigned__int__0c__const__char__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::LogicalDecodingContext, crate::__pgrx_c_bindings::TransactionId, *const ::core::ffi::c_char) -> bool>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::LogicalDecodingContext,
                u16,
            ) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::LogicalDecodingContext,
                u16,
            ) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__LogicalDecodingContext__0p__0c__unsigned__short__0r;
impl c::sealed::Sealed
    for Signature_0x5f426f6f6c__0l__struct__LogicalDecodingContext__0p__0c__unsigned__short__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__struct__LogicalDecodingContext__0p__0c__unsigned__short__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::LogicalDecodingContext,
                u16,
            ) -> bool,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::MemoryContextData) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::MemoryContextData) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__MemoryContextData__0p__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__MemoryContextData__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__struct__MemoryContextData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::MemoryContextData) -> bool,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerInfo,
                *mut crate::__pgrx_c_bindings::ModifyTable,
                u32,
                i32,
            ) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerInfo,
                *mut crate::__pgrx_c_bindings::ModifyTable,
                u32,
                i32,
            ) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
pub struct PhysicalFunction_C_unwind_4<P>(::core::marker::PhantomData<P>);
impl<P> c::sealed::Sealed for PhysicalFunction_C_unwind_4<P> {}
impl<A0, A1, A2, A3, R> c::expression::PhysicalFunctionPointer
    for PhysicalFunction_C_unwind_4<
        ::core::option::Option<unsafe extern "C-unwind" fn(A0, A1, A2, A3) -> R>,
    >
{
    type Pointer = ::core::option::Option<unsafe extern "C-unwind" fn(A0, A1, A2, A3) -> R>;
    fn null() -> Self::Pointer {
        None
    }
    fn address(pointer: Self::Pointer) -> *const () {
        pointer.map_or(::core::ptr::null(), |function| function as *const ())
    }
}
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__PlannerInfo__0p__0c__struct__ModifyTable__0p__0c__unsigned__int__0c__int__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__PlannerInfo__0p__0c__struct__ModifyTable__0p__0c__unsigned__int__0c__int__0r {}
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__struct__PlannerInfo__0p__0c__struct__ModifyTable__0p__0c__unsigned__int__0c__int__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PlannerInfo, *mut crate::__pgrx_c_bindings::ModifyTable, u32, i32) -> bool>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerInfo,
                *mut crate::__pgrx_c_bindings::RangeTblEntry,
                i16,
                *mut crate::__pgrx_c_bindings::VariableStatData,
            ) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerInfo,
                *mut crate::__pgrx_c_bindings::RangeTblEntry,
                i16,
                *mut crate::__pgrx_c_bindings::VariableStatData,
            ) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__PlannerInfo__0p__0c__struct__RangeTblEntry__0p__0c__short__0c__struct__VariableStatData__0p__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__PlannerInfo__0p__0c__struct__RangeTblEntry__0p__0c__short__0c__struct__VariableStatData__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__struct__PlannerInfo__0p__0c__struct__RangeTblEntry__0p__0c__short__0c__struct__VariableStatData__0p__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PlannerInfo, *mut crate::__pgrx_c_bindings::RangeTblEntry, i16, *mut crate::__pgrx_c_bindings::VariableStatData) -> bool>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerInfo,
                *mut crate::__pgrx_c_bindings::RelOptInfo,
                *mut crate::__pgrx_c_bindings::EquivalenceClass,
                *mut crate::__pgrx_c_bindings::EquivalenceMember,
                *mut ::core::ffi::c_void,
            ) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerInfo,
                *mut crate::__pgrx_c_bindings::RelOptInfo,
                *mut crate::__pgrx_c_bindings::EquivalenceClass,
                *mut crate::__pgrx_c_bindings::EquivalenceMember,
                *mut ::core::ffi::c_void,
            ) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
pub struct PhysicalFunction_C_unwind_5<P>(::core::marker::PhantomData<P>);
impl<P> c::sealed::Sealed for PhysicalFunction_C_unwind_5<P> {}
impl<A0, A1, A2, A3, A4, R> c::expression::PhysicalFunctionPointer
    for PhysicalFunction_C_unwind_5<
        ::core::option::Option<unsafe extern "C-unwind" fn(A0, A1, A2, A3, A4) -> R>,
    >
{
    type Pointer = ::core::option::Option<unsafe extern "C-unwind" fn(A0, A1, A2, A3, A4) -> R>;
    fn null() -> Self::Pointer {
        None
    }
    fn address(pointer: Self::Pointer) -> *const () {
        pointer.map_or(::core::ptr::null(), |function| function as *const ())
    }
}
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__EquivalenceClass__0p__0c__struct__EquivalenceMember__0p__0c__void__0p__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__EquivalenceClass__0p__0c__struct__EquivalenceMember__0p__0c__void__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__EquivalenceClass__0p__0c__struct__EquivalenceMember__0p__0c__void__0p__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PlannerInfo, *mut crate::__pgrx_c_bindings::RelOptInfo, *mut crate::__pgrx_c_bindings::EquivalenceClass, *mut crate::__pgrx_c_bindings::EquivalenceMember, *mut ::core::ffi::c_void) -> bool>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerInfo,
                *mut crate::__pgrx_c_bindings::RelOptInfo,
                *mut crate::__pgrx_c_bindings::RangeTblEntry,
            ) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerInfo,
                *mut crate::__pgrx_c_bindings::RelOptInfo,
                *mut crate::__pgrx_c_bindings::RangeTblEntry,
            ) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__RangeTblEntry__0p__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__RangeTblEntry__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__RangeTblEntry__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PlannerInfo, *mut crate::__pgrx_c_bindings::RelOptInfo, *mut crate::__pgrx_c_bindings::RangeTblEntry) -> bool>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerInfo,
                crate::__pgrx_c_bindings::Oid,
                i16,
                *mut crate::__pgrx_c_bindings::VariableStatData,
            ) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerInfo,
                crate::__pgrx_c_bindings::Oid,
                i16,
                *mut crate::__pgrx_c_bindings::VariableStatData,
            ) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__PlannerInfo__0p__0c__unsigned__int__0c__short__0c__struct__VariableStatData__0p__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__PlannerInfo__0p__0c__unsigned__int__0c__short__0c__struct__VariableStatData__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__struct__PlannerInfo__0p__0c__unsigned__int__0c__short__0c__struct__VariableStatData__0p__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PlannerInfo, crate::__pgrx_c_bindings::Oid, i16, *mut crate::__pgrx_c_bindings::VariableStatData) -> bool>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__RelationData__0p__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__RelationData__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__struct__RelationData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData) -> bool,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::RelationData,
                        i32,
                        *mut *mut crate::__pgrx_c_bindings::HeapTupleData,
                        i32,
                        *mut f64,
                        *mut f64,
                    ) -> i32,
                >,
                *mut u32,
            ) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::RelationData,
                        i32,
                        *mut *mut crate::__pgrx_c_bindings::HeapTupleData,
                        i32,
                        *mut f64,
                        *mut f64,
                    ) -> i32,
                >,
                *mut u32,
            ) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__RelationData__0p__0c__int__0l__0p__0p__0r__0l__struct__RelationData__0p__0c__int__0c__struct__HeapTupleData__0p__0p__0c__int__0c__double__0p__0c__double__0p__0r__0c__unsigned__int__0p__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__RelationData__0p__0c__int__0l__0p__0p__0r__0l__struct__RelationData__0p__0c__int__0c__struct__HeapTupleData__0p__0p__0c__int__0c__double__0p__0c__double__0p__0r__0c__unsigned__int__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__struct__RelationData__0p__0c__int__0l__0p__0p__0r__0l__struct__RelationData__0p__0c__int__0c__struct__HeapTupleData__0p__0p__0c__int__0c__double__0p__0c__double__0p__0r__0c__unsigned__int__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut ::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, i32, *mut *mut crate::__pgrx_c_bindings::HeapTupleData, i32, *mut f64, *mut f64) -> i32>, *mut u32) -> bool>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, i32) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, i32) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__RelationData__0p__0c__int__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__RelationData__0p__0c__int__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__struct__RelationData__0p__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, i32) -> bool,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::ItemPointerData,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
            ) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::ItemPointerData,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
            ) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__RelationData__0p__0c__struct__ItemPointerData__0p__0c__struct__SnapshotData__0p__0c__struct__TupleTableSlot__0p__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__RelationData__0p__0c__struct__ItemPointerData__0p__0c__struct__SnapshotData__0p__0c__struct__TupleTableSlot__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__struct__RelationData__0p__0c__struct__ItemPointerData__0p__0c__struct__SnapshotData__0p__0c__struct__TupleTableSlot__0p__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::ItemPointerData, *mut crate::__pgrx_c_bindings::SnapshotData, *mut crate::__pgrx_c_bindings::TupleTableSlot) -> bool>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                *mut crate::__pgrx_c_bindings::SnapshotData,
            ) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                *mut crate::__pgrx_c_bindings::SnapshotData,
            ) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__RelationData__0p__0c__struct__TupleTableSlot__0p__0c__struct__SnapshotData__0p__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__RelationData__0p__0c__struct__TupleTableSlot__0p__0c__struct__SnapshotData__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__struct__RelationData__0p__0c__struct__TupleTableSlot__0p__0c__struct__SnapshotData__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::TupleTableSlot, *mut crate::__pgrx_c_bindings::SnapshotData) -> bool>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::Datum,
                *mut bool,
                *mut crate::__pgrx_c_bindings::ItemPointerData,
                *mut crate::__pgrx_c_bindings::RelationData,
                u32,
                bool,
                *mut crate::__pgrx_c_bindings::IndexInfo,
            ) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::Datum,
                *mut bool,
                *mut crate::__pgrx_c_bindings::ItemPointerData,
                *mut crate::__pgrx_c_bindings::RelationData,
                u32,
                bool,
                *mut crate::__pgrx_c_bindings::IndexInfo,
            ) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__RelationData__0p__0c__unsigned__long__0p__0c__0x5f426f6f6c__0p__0c__struct__ItemPointerData__0p__0c__struct__RelationData__0p__0c__enum__IndexUniqueCheck__0c__0x5f426f6f6c__0c__struct__IndexInfo__0p__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__RelationData__0p__0c__unsigned__long__0p__0c__0x5f426f6f6c__0p__0c__struct__ItemPointerData__0p__0c__struct__RelationData__0p__0c__enum__IndexUniqueCheck__0c__0x5f426f6f6c__0c__struct__IndexInfo__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__struct__RelationData__0p__0c__unsigned__long__0p__0c__0x5f426f6f6c__0p__0c__struct__ItemPointerData__0p__0c__struct__RelationData__0p__0c__enum__IndexUniqueCheck__0c__0x5f426f6f6c__0c__struct__IndexInfo__0p__0r {
 type Physical = PhysicalFunction_C_unwind_8<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::Datum, *mut bool, *mut crate::__pgrx_c_bindings::ItemPointerData, *mut crate::__pgrx_c_bindings::RelationData, u32, bool, *mut crate::__pgrx_c_bindings::IndexInfo) -> bool>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ScanState,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
            ) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ScanState,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
            ) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__ScanState__0p__0c__struct__TupleTableSlot__0p__0r;
impl c::sealed::Sealed
    for Signature_0x5f426f6f6c__0l__struct__ScanState__0p__0c__struct__TupleTableSlot__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__struct__ScanState__0p__0c__struct__TupleTableSlot__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ScanState,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
            ) -> bool,
        >,
    >;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::SlruCtlData,
                *mut ::core::ffi::c_char,
                i32,
                *mut ::core::ffi::c_void,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::SlruCtlData,
                *mut ::core::ffi::c_char,
                i32,
                *mut ::core::ffi::c_void,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__SlruCtlData__0p__0c__char__0p__0c__int__0c__void__0p__0r;
#[cfg(any(feature = "pg15", feature = "pg16"))]
impl c::sealed::Sealed
    for Signature_0x5f426f6f6c__0l__struct__SlruCtlData__0p__0c__char__0p__0c__int__0c__void__0p__0r
{
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__struct__SlruCtlData__0p__0c__char__0p__0c__int__0c__void__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_4<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::SlruCtlData,
                *mut ::core::ffi::c_char,
                i32,
                *mut ::core::ffi::c_void,
            ) -> bool,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TableFuncScanState) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TableFuncScanState) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__TableFuncScanState__0p__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__TableFuncScanState__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__struct__TableFuncScanState__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TableFuncScanState) -> bool,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableScanDescData,
                i32,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
            ) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableScanDescData,
                i32,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
            ) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__enum__ScanDirection__0c__struct__TupleTableSlot__0p__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__enum__ScanDirection__0c__struct__TupleTableSlot__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__enum__ScanDirection__0c__struct__TupleTableSlot__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TableScanDescData, i32, *mut crate::__pgrx_c_bindings::TupleTableSlot) -> bool>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableScanDescData,
                *mut crate::__pgrx_c_bindings::ItemPointerData,
            ) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableScanDescData,
                *mut crate::__pgrx_c_bindings::ItemPointerData,
            ) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__struct__ItemPointerData__0p__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__struct__ItemPointerData__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__struct__ItemPointerData__0p__0r {
 type Physical = PhysicalFunction_C_unwind_2<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TableScanDescData, *mut crate::__pgrx_c_bindings::ItemPointerData) -> bool>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableScanDescData,
                *mut crate::__pgrx_c_bindings::SampleScanState,
            ) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableScanDescData,
                *mut crate::__pgrx_c_bindings::SampleScanState,
            ) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__struct__SampleScanState__0p__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__struct__SampleScanState__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__struct__SampleScanState__0p__0r {
 type Physical = PhysicalFunction_C_unwind_2<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TableScanDescData, *mut crate::__pgrx_c_bindings::SampleScanState) -> bool>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableScanDescData,
                *mut crate::__pgrx_c_bindings::SampleScanState,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
            ) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableScanDescData,
                *mut crate::__pgrx_c_bindings::SampleScanState,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
            ) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__struct__SampleScanState__0p__0c__struct__TupleTableSlot__0p__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__struct__SampleScanState__0p__0c__struct__TupleTableSlot__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__struct__SampleScanState__0p__0c__struct__TupleTableSlot__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TableScanDescData, *mut crate::__pgrx_c_bindings::SampleScanState, *mut crate::__pgrx_c_bindings::TupleTableSlot) -> bool>>;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableScanDescData,
                *mut crate::__pgrx_c_bindings::TBMIterateResult,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableScanDescData,
                *mut crate::__pgrx_c_bindings::TBMIterateResult,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__struct__TBMIterateResult__0p__0r;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__struct__TBMIterateResult__0p__0r {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__struct__TBMIterateResult__0p__0r {
 type Physical = PhysicalFunction_C_unwind_2<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TableScanDescData, *mut crate::__pgrx_c_bindings::TBMIterateResult) -> bool>>;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableScanDescData,
                *mut crate::__pgrx_c_bindings::TBMIterateResult,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableScanDescData,
                *mut crate::__pgrx_c_bindings::TBMIterateResult,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__struct__TBMIterateResult__0p__0c__struct__TupleTableSlot__0p__0r;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__struct__TBMIterateResult__0p__0c__struct__TupleTableSlot__0p__0r {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__struct__TBMIterateResult__0p__0c__struct__TupleTableSlot__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TableScanDescData, *mut crate::__pgrx_c_bindings::TBMIterateResult, *mut crate::__pgrx_c_bindings::TupleTableSlot) -> bool>>;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableScanDescData,
                crate::__pgrx_c_bindings::TransactionId,
                *mut f64,
                *mut f64,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableScanDescData,
                crate::__pgrx_c_bindings::TransactionId,
                *mut f64,
                *mut f64,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__unsigned__int__0c__double__0p__0c__double__0p__0c__struct__TupleTableSlot__0p__0r;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__unsigned__int__0c__double__0p__0c__double__0p__0c__struct__TupleTableSlot__0p__0r {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__unsigned__int__0c__double__0p__0c__double__0p__0c__struct__TupleTableSlot__0p__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TableScanDescData, crate::__pgrx_c_bindings::TransactionId, *mut f64, *mut f64, *mut crate::__pgrx_c_bindings::TupleTableSlot) -> bool>>;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableScanDescData,
                u32,
                *mut crate::__pgrx_c_bindings::BufferAccessStrategyData,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableScanDescData,
                u32,
                *mut crate::__pgrx_c_bindings::BufferAccessStrategyData,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__unsigned__int__0c__struct__BufferAccessStrategyData__0p__0r;
#[cfg(any(feature = "pg15", feature = "pg16"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__unsigned__int__0c__struct__BufferAccessStrategyData__0p__0r {}
#[cfg(any(feature = "pg15", feature = "pg16"))]
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__unsigned__int__0c__struct__BufferAccessStrategyData__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TableScanDescData, u32, *mut crate::__pgrx_c_bindings::BufferAccessStrategyData) -> bool>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                *mut crate::__pgrx_c_bindings::_DestReceiver,
            ) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                *mut crate::__pgrx_c_bindings::_DestReceiver,
            ) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__TupleTableSlot__0p__0c__struct__0x5f446573745265636569766572__0p__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__TupleTableSlot__0p__0c__struct__0x5f446573745265636569766572__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__struct__TupleTableSlot__0p__0c__struct__0x5f446573745265636569766572__0p__0r {
 type Physical = PhysicalFunction_C_unwind_2<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TupleTableSlot, *mut crate::__pgrx_c_bindings::_DestReceiver) -> bool>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::WalReceiverConn,
                *const crate::__pgrx_c_bindings::WalRcvStreamOptions,
            ) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::WalReceiverConn,
                *const crate::__pgrx_c_bindings::WalRcvStreamOptions,
            ) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__WalReceiverConn__0p__0c__const__WalRcvStreamOptions__0p__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__WalReceiverConn__0p__0c__const__WalRcvStreamOptions__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__struct__WalReceiverConn__0p__0c__const__WalRcvStreamOptions__0p__0r {
 type Physical = PhysicalFunction_C_unwind_2<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::WalReceiverConn, *const crate::__pgrx_c_bindings::WalRcvStreamOptions) -> bool>>;
}
const _: () =
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::WalReceiverConn>() == 8);
const _: () =
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::WalReceiverConn>() == 8);
const _: () =
    assert!(::core::mem::size_of::<*const crate::__pgrx_c_bindings::WalRcvStreamOptions>() == 8);
const _: () =
    assert!(::core::mem::align_of::<*const crate::__pgrx_c_bindings::WalRcvStreamOptions>() == 8);
const _: () = assert!(::core::mem::size_of::<bool>() == 1);
const _: () = assert!(::core::mem::align_of::<bool>() == 1);
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<A0: c::expression::ImplicitTo<c::expression::CPointer<c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>, c::expression::ReadWrite>>, A1: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvStreamOptions>, c::expression::ReadOnly>>> c::expression::Call<(A0,A1,)> for Signature_0x5f426f6f6c__0l__struct__WalReceiverConn__0p__0c__const__WalRcvStreamOptions__0p__0r {
 type Output = <c::CBool as c::expression::CType>::Value;
 unsafe fn call(pointer: Self::Pointer, args: (A0,A1,)) -> Self::Output {
let function = pointer.expect("C indirect call requires a non-null function pointer");
const { assert!(!::core::mem::needs_drop::<A0>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::WalReceiverConn>());
let native0 = <c::expression::CPointer<c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>, c::expression::ReadWrite>, _>(args.0));
const { assert!(!::core::mem::needs_drop::<A1>()); }
const _: () = assert!(!::core::mem::needs_drop::<*const crate::__pgrx_c_bindings::WalRcvStreamOptions>());
let native1 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvStreamOptions>, c::expression::ReadOnly> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvStreamOptions>, c::expression::ReadOnly>, _>(args.1));
// SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
let result = unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0, native1)) };
<c::CBool as c::expression::CType>::from_storage(result)
}
}
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn(*mut u8, i32) -> bool>>(
    ) == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*mut u8, i32) -> bool>,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__unsigned__char__0p__0c__int__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__unsigned__char__0p__0c__int__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__unsigned__char__0p__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<unsafe extern "C-unwind" fn(*mut u8, i32) -> bool>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Oid) -> bool>,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Oid) -> bool>,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__unsigned__int__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__unsigned__int__0r {}
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__unsigned__int__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Oid) -> bool>,
    >;
}
const _: () = assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Oid>() == 4);
const _: () = assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Oid>() == 4);
const _: () = assert!(::core::mem::size_of::<bool>() == 1);
const _: () = assert!(::core::mem::align_of::<bool>() == 1);
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<
    A0: c::expression::ImplicitTo<
            c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>,
        >,
> c::expression::Call<(A0,)> for Signature_0x5f426f6f6c__0l__unsigned__int__0r
{
    type Output = <c::CBool as c::expression::CType>::Value;
    unsafe fn call(pointer: Self::Pointer, args: (A0,)) -> Self::Output {
        let function = pointer.expect("C indirect call requires a non-null function pointer");
        const {
            assert!(!::core::mem::needs_drop::<A0>());
        }
        const _: () = assert!(!::core::mem::needs_drop::<crate::__pgrx_c_bindings::Oid>());
        let native0 = <c::expression::CIntegerStorage<
            c::CUnsignedInt,
            crate::__pgrx_c_bindings::Oid,
        > as c::expression::CType>::into_storage(c::expression::implicit::<
            c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>,
            _,
        >(args.0));
        // SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
        let result = unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0)) };
        <c::CBool as c::expression::CType>::from_storage(result)
    }
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                i32,
                u32,
                *const ::core::ffi::c_char,
                *mut bool,
                *mut bool,
            ) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                i32,
                u32,
                *const ::core::ffi::c_char,
                *mut bool,
                *mut bool,
            ) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__unsigned__int__0c__int__0c__enum__IndexAMProperty__0c__const__char__0p__0c__0x5f426f6f6c__0p__0c__0x5f426f6f6c__0p__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__unsigned__int__0c__int__0c__enum__IndexAMProperty__0c__const__char__0p__0c__0x5f426f6f6c__0p__0c__0x5f426f6f6c__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__unsigned__int__0c__int__0c__enum__IndexAMProperty__0c__const__char__0p__0c__0x5f426f6f6c__0p__0c__0x5f426f6f6c__0p__0r {
 type Physical = PhysicalFunction_C_unwind_6<::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Oid, i32, u32, *const ::core::ffi::c_char, *mut bool, *mut bool) -> bool>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                *mut ::core::ffi::c_void,
            ) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                *mut ::core::ffi::c_void,
            ) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__unsigned__int__0c__void__0p__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__unsigned__int__0c__void__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__unsigned__int__0c__void__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                *mut ::core::ffi::c_void,
            ) -> bool,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__unsigned__long__0c__unsigned__long__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__unsigned__long__0c__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__unsigned__long__0c__unsigned__long__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
            ) -> bool,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn() -> bool>>() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn() -> bool>>() == 8
);
#[doc(hidden)]
pub struct PhysicalFunction_C_unwind_0<P>(::core::marker::PhantomData<P>);
impl<P> c::sealed::Sealed for PhysicalFunction_C_unwind_0<P> {}
impl<R> c::expression::PhysicalFunctionPointer
    for PhysicalFunction_C_unwind_0<::core::option::Option<unsafe extern "C-unwind" fn() -> R>>
{
    type Pointer = ::core::option::Option<unsafe extern "C-unwind" fn() -> R>;
    fn null() -> Self::Pointer {
        None
    }
    fn address(pointer: Self::Pointer) -> *const () {
        pointer.map_or(::core::ptr::null(), |function| function as *const ())
    }
}
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__void__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__void__0r {}
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__void__0r {
    type Physical =
        PhysicalFunction_C_unwind_0<::core::option::Option<unsafe extern "C-unwind" fn() -> bool>>;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_char) -> bool>,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_char) -> bool>,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__char__0p__0r;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__char__0p__0r {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__char__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_char) -> bool>,
    >;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut ::core::ffi::c_char, u32, i32) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut ::core::ffi::c_char, u32, i32) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__char__0p__0c__unsigned__int__0c__int__0r;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__char__0p__0c__unsigned__int__0c__int__0r {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__char__0p__0c__unsigned__int__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut ::core::ffi::c_char, u32, i32) -> bool,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_char) -> bool>,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_char) -> bool>,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__const__char__0p__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__const__char__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__const__char__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_char) -> bool>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const ::core::ffi::c_char, bool) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const ::core::ffi::c_char, bool) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__const__char__0p__0c__0x5f426f6f6c__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__const__char__0p__0c__0x5f426f6f6c__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__const__char__0p__0c__0x5f426f6f6c__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const ::core::ffi::c_char, bool) -> bool,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::Bitmapset,
                *const crate::__pgrx_c_bindings::Bitmapset,
            ) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::Bitmapset,
                *const crate::__pgrx_c_bindings::Bitmapset,
            ) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__const__struct__Bitmapset__0p__0c__const__struct__Bitmapset__0p__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__const__struct__Bitmapset__0p__0c__const__struct__Bitmapset__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__const__struct__Bitmapset__0p__0c__const__struct__Bitmapset__0p__0r {
 type Physical = PhysicalFunction_C_unwind_2<::core::option::Option<unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::Bitmapset, *const crate::__pgrx_c_bindings::Bitmapset) -> bool>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_void) -> bool>,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_void) -> bool>,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__const__void__0p__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__const__void__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__const__void__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_void) -> bool>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn(f64, f64) -> bool>>()
        == 8
);
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn(f64, f64) -> bool>>(
    ) == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__double__0c__double__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__double__0c__double__0r {}
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__double__0c__double__0r {
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<unsafe extern "C-unwind" fn(f64, f64) -> bool>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn(f32, f32) -> bool>>()
        == 8
);
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn(f32, f32) -> bool>>(
    ) == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__float__0c__float__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__float__0c__float__0r {}
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__float__0c__float__0r {
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<unsafe extern "C-unwind" fn(f32, f32) -> bool>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn(i32) -> bool>>() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn(i32) -> bool>>()
        == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__int__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__int__0r {}
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__int__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(i32) -> bool>,
    >;
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
            ) -> bool,
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
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(i32, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum) -> bool>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ExprState,
                *mut crate::__pgrx_c_bindings::ExprContext,
            ) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ExprState,
                *mut crate::__pgrx_c_bindings::ExprContext,
            ) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__ExprState__0p__0c__struct__ExprContext__0p__0r;
impl c::sealed::Sealed
    for Signature_0x5f426f6f6c__0l__struct__ExprState__0p__0c__struct__ExprContext__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__struct__ExprState__0p__0c__struct__ExprContext__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ExprState,
                *mut crate::__pgrx_c_bindings::ExprContext,
            ) -> bool,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::FmgrInfo) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::FmgrInfo) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__FmgrInfo__0p__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__FmgrInfo__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__struct__FmgrInfo__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::FmgrInfo) -> bool,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::IndexTupleData) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::IndexTupleData) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__IndexTupleData__0p__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__IndexTupleData__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__struct__IndexTupleData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::IndexTupleData) -> bool,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelOptInfo) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelOptInfo) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__RelOptInfo__0p__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__RelOptInfo__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__struct__RelOptInfo__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelOptInfo) -> bool,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TupleTableSlot, i32) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TupleTableSlot, i32) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__TupleTableSlot__0p__0c__int__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__TupleTableSlot__0p__0c__int__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__struct__TupleTableSlot__0p__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TupleTableSlot, i32) -> bool,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::XLogReaderState) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::XLogReaderState) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__XLogReaderState__0p__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__XLogReaderState__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__struct__XLogReaderState__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::XLogReaderState) -> bool,
        >,
    >;
}
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::dlist_head) -> bool,
        >,
    >() == 8
);
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::dlist_head) -> bool,
        >,
    >() == 8
);
#[cfg(feature = "pg15")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__dlist_head__0p__0r;
#[cfg(feature = "pg15")]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__dlist_head__0p__0r {}
#[cfg(feature = "pg15")]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__struct__dlist_head__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::dlist_head) -> bool,
        >,
    >;
}
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::dlist_head,
                *mut crate::__pgrx_c_bindings::dlist_node,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::dlist_head,
                *mut crate::__pgrx_c_bindings::dlist_node,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(feature = "pg15")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__dlist_head__0p__0c__struct__dlist_node__0p__0r;
#[cfg(feature = "pg15")]
impl c::sealed::Sealed
    for Signature_0x5f426f6f6c__0l__struct__dlist_head__0p__0c__struct__dlist_node__0p__0r
{
}
#[cfg(feature = "pg15")]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__struct__dlist_head__0p__0c__struct__dlist_node__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::dlist_head,
                *mut crate::__pgrx_c_bindings::dlist_node,
            ) -> bool,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::pg_prng_state) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::pg_prng_state) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__pg_prng_state__0p__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__pg_prng_state__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__struct__pg_prng_state__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::pg_prng_state) -> bool,
        >,
    >;
}
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::proclist_head) -> bool,
        >,
    >() == 8
);
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::proclist_head) -> bool,
        >,
    >() == 8
);
#[cfg(feature = "pg15")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__proclist_head__0p__0r;
#[cfg(feature = "pg15")]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__proclist_head__0p__0r {}
#[cfg(feature = "pg15")]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__struct__proclist_head__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::proclist_head) -> bool,
        >,
    >;
}
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::proclist_head,
                i32,
                usize,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::proclist_head,
                i32,
                usize,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(feature = "pg15")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__proclist_head__0p__0c__int__0c__unsigned__long__0r;
#[cfg(feature = "pg15")]
impl c::sealed::Sealed
    for Signature_0x5f426f6f6c__0l__struct__proclist_head__0p__0c__int__0c__unsigned__long__0r
{
}
#[cfg(feature = "pg15")]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__struct__proclist_head__0p__0c__int__0c__unsigned__long__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::proclist_head,
                i32,
                usize,
            ) -> bool,
        >,
    >;
}
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::slist_head) -> bool,
        >,
    >() == 8
);
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::slist_head) -> bool,
        >,
    >() == 8
);
#[cfg(feature = "pg15")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__slist_head__0p__0r;
#[cfg(feature = "pg15")]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__slist_head__0p__0r {}
#[cfg(feature = "pg15")]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__struct__slist_head__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::slist_head) -> bool,
        >,
    >;
}
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::slist_head,
                *mut crate::__pgrx_c_bindings::slist_node,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::slist_head,
                *mut crate::__pgrx_c_bindings::slist_node,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(feature = "pg15")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__slist_head__0p__0c__struct__slist_node__0p__0r;
#[cfg(feature = "pg15")]
impl c::sealed::Sealed
    for Signature_0x5f426f6f6c__0l__struct__slist_head__0p__0c__struct__slist_node__0p__0r
{
}
#[cfg(feature = "pg15")]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__struct__slist_head__0p__0c__struct__slist_node__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::slist_head,
                *mut crate::__pgrx_c_bindings::slist_node,
            ) -> bool,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn(u8) -> bool>>() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn(u8) -> bool>>() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__unsigned__char__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__unsigned__char__0r {}
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__unsigned__char__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(u8) -> bool>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Oid, bool) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Oid, bool) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__unsigned__int__0c__0x5f426f6f6c__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__unsigned__int__0c__0x5f426f6f6c__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__unsigned__int__0c__0x5f426f6f6c__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Oid, bool) -> bool,
        >,
    >;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::TransactionId,
                crate::__pgrx_c_bindings::TransactionId,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::TransactionId,
                crate::__pgrx_c_bindings::TransactionId,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__unsigned__int__0c__unsigned__int__0r;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__unsigned__int__0c__unsigned__int__0r {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__unsigned__int__0c__unsigned__int__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::TransactionId,
                crate::__pgrx_c_bindings::TransactionId,
            ) -> bool,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void, usize) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void, usize) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__void__0p__0c__unsigned__long__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__void__0p__0c__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__void__0p__0c__unsigned__long__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void, usize) -> bool,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::pg_atomic_flag) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::pg_atomic_flag) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__volatile__struct__pg_atomic_flag__0p__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__volatile__struct__pg_atomic_flag__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__volatile__struct__pg_atomic_flag__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::pg_atomic_flag) -> bool,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::pg_atomic_uint32,
                *mut u32,
                u32,
            ) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::pg_atomic_uint32,
                *mut u32,
                u32,
            ) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__volatile__struct__pg_atomic_uint32__0p__0c__unsigned__int__0p__0c__unsigned__int__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__volatile__struct__pg_atomic_uint32__0p__0c__unsigned__int__0p__0c__unsigned__int__0r {}
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__volatile__struct__pg_atomic_uint32__0p__0c__unsigned__int__0p__0c__unsigned__int__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::pg_atomic_uint32, *mut u32, u32) -> bool>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::pg_atomic_uint64,
                *mut u64,
                u64,
            ) -> bool,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::pg_atomic_uint64,
                *mut u64,
                u64,
            ) -> bool,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__volatile__struct__pg_atomic_uint64__0p__0c__unsigned__long__0p__0c__unsigned__long__0r;
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__volatile__struct__pg_atomic_uint64__0p__0c__unsigned__long__0p__0c__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__volatile__struct__pg_atomic_uint64__0p__0c__unsigned__long__0p__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::pg_atomic_uint64, *mut u64, u64) -> bool>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::GinScanKeyData,
            ) -> ::core::ffi::c_char,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::GinScanKeyData,
            ) -> ::core::ffi::c_char,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_char__0l__struct__GinScanKeyData__0p__0r;
impl c::sealed::Sealed for Signature_char__0l__struct__GinScanKeyData__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_char__0l__struct__GinScanKeyData__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::GinScanKeyData,
            ) -> ::core::ffi::c_char,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::RangeType) -> i8,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::RangeType) -> i8,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_char__0l__const__RangeType__0p__0r;
impl c::sealed::Sealed for Signature_char__0l__const__RangeType__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_char__0l__const__RangeType__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::RangeType) -> i8,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Oid) -> i8>,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Oid) -> i8>,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_char__0l__unsigned__int__0r;
impl c::sealed::Sealed for Signature_char__0l__unsigned__int__0r {}
impl c::expression::NativeFunctionSignature for Signature_char__0l__unsigned__int__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Oid) -> i8>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const ::core::ffi::c_char) -> *mut ::core::ffi::c_char,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const ::core::ffi::c_char) -> *mut ::core::ffi::c_char,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_char__0p__0l__const__char__0p__0r;
impl c::sealed::Sealed for Signature_char__0p__0l__const__char__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_char__0p__0l__const__char__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const ::core::ffi::c_char) -> *mut ::core::ffi::c_char,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(i64) -> *mut ::core::ffi::c_char>,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(i64) -> *mut ::core::ffi::c_char>,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_char__0p__0l__long__0r;
impl c::sealed::Sealed for Signature_char__0p__0l__long__0r {}
impl c::expression::NativeFunctionSignature for Signature_char__0p__0l__long__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(i64) -> *mut ::core::ffi::c_char>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::WalReceiverConn,
            ) -> *mut ::core::ffi::c_char,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::WalReceiverConn,
            ) -> *mut ::core::ffi::c_char,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_char__0p__0l__struct__WalReceiverConn__0p__0r;
impl c::sealed::Sealed for Signature_char__0p__0l__struct__WalReceiverConn__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_char__0p__0l__struct__WalReceiverConn__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::WalReceiverConn,
            ) -> *mut ::core::ffi::c_char,
        >,
    >;
}
const _: () =
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::WalReceiverConn>() == 8);
const _: () =
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::WalReceiverConn>() == 8);
const _: () = assert!(::core::mem::size_of::<*mut ::core::ffi::c_char>() == 8);
const _: () = assert!(::core::mem::align_of::<*mut ::core::ffi::c_char>() == 8);
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<
    A0: c::expression::ImplicitTo<
            c::expression::CPointer<
                c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>,
                c::expression::ReadWrite,
            >,
        >,
> c::expression::Call<(A0,)> for Signature_char__0p__0l__struct__WalReceiverConn__0p__0r
{
    type Output = <c::expression::CPointer<
        c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
        c::expression::ReadWrite,
    > as c::expression::CType>::Value;
    unsafe fn call(pointer: Self::Pointer, args: (A0,)) -> Self::Output {
        let function = pointer.expect("C indirect call requires a non-null function pointer");
        const {
            assert!(!::core::mem::needs_drop::<A0>());
        }
        const _: () =
            assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::WalReceiverConn>());
        let native0 = <c::expression::CPointer<
            c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>,
            c::expression::ReadWrite,
        > as c::expression::CType>::into_storage(c::expression::implicit::<
            c::expression::CPointer<
                c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>,
                c::expression::ReadWrite,
            >,
            _,
        >(args.0));
        // SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
        let result = unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0)) };
        <c::expression::CPointer<
            c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
            c::expression::ReadWrite,
        > as c::expression::CType>::from_storage(result)
    }
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::WalReceiverConn,
                *const ::core::ffi::c_char,
                bool,
                bool,
                u32,
                *mut u64,
            ) -> *mut ::core::ffi::c_char,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::WalReceiverConn,
                *const ::core::ffi::c_char,
                bool,
                bool,
                u32,
                *mut u64,
            ) -> *mut ::core::ffi::c_char,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_char__0p__0l__struct__WalReceiverConn__0p__0c__const__char__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__CRSSnapshotAction__0c__unsigned__long__0p__0r;
#[cfg(any(feature = "pg15", feature = "pg16"))]
impl c::sealed::Sealed for Signature_char__0p__0l__struct__WalReceiverConn__0p__0c__const__char__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__CRSSnapshotAction__0c__unsigned__long__0p__0r {}
#[cfg(any(feature = "pg15", feature = "pg16"))]
impl c::expression::NativeFunctionSignature for Signature_char__0p__0l__struct__WalReceiverConn__0p__0c__const__char__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__CRSSnapshotAction__0c__unsigned__long__0p__0r {
 type Physical = PhysicalFunction_C_unwind_6<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::WalReceiverConn, *const ::core::ffi::c_char, bool, bool, u32, *mut u64) -> *mut ::core::ffi::c_char>>;
}
const _: () =
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::WalReceiverConn>() == 8);
const _: () =
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::WalReceiverConn>() == 8);
const _: () = assert!(::core::mem::size_of::<*const ::core::ffi::c_char>() == 8);
const _: () = assert!(::core::mem::align_of::<*const ::core::ffi::c_char>() == 8);
const _: () = assert!(::core::mem::size_of::<bool>() == 1);
const _: () = assert!(::core::mem::align_of::<bool>() == 1);
const _: () = assert!(::core::mem::size_of::<bool>() == 1);
const _: () = assert!(::core::mem::align_of::<bool>() == 1);
const _: () = assert!(::core::mem::size_of::<u32>() == 4);
const _: () = assert!(::core::mem::align_of::<u32>() == 4);
const _: () = assert!(::core::mem::size_of::<*mut u64>() == 8);
const _: () = assert!(::core::mem::align_of::<*mut u64>() == 8);
const _: () = assert!(::core::mem::size_of::<*mut ::core::ffi::c_char>() == 8);
const _: () = assert!(::core::mem::align_of::<*mut ::core::ffi::c_char>() == 8);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<A0: c::expression::ImplicitTo<c::expression::CPointer<c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>, c::expression::ReadWrite>>, A1: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadOnly>>, A2: c::expression::ImplicitTo<c::CBool>, A3: c::expression::ImplicitTo<c::CBool>, A4: c::expression::ImplicitTo<c::expression::CEnumObject<crate::__pgrx_c_generated::EnumIdentity_CRSSnapshotAction, c::CUnsignedInt, u32>>, A5: c::expression::ImplicitTo<c::expression::CPointer<c::CUnsignedLong, c::expression::ReadWrite>>> c::expression::Call<(A0,A1,A2,A3,A4,A5,)> for Signature_char__0p__0l__struct__WalReceiverConn__0p__0c__const__char__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__CRSSnapshotAction__0c__unsigned__long__0p__0r {
 type Output = <c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadWrite> as c::expression::CType>::Value;
 unsafe fn call(pointer: Self::Pointer, args: (A0,A1,A2,A3,A4,A5,)) -> Self::Output {
let function = pointer.expect("C indirect call requires a non-null function pointer");
const { assert!(!::core::mem::needs_drop::<A0>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::WalReceiverConn>());
let native0 = <c::expression::CPointer<c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>, c::expression::ReadWrite>, _>(args.0));
const { assert!(!::core::mem::needs_drop::<A1>()); }
const _: () = assert!(!::core::mem::needs_drop::<*const ::core::ffi::c_char>());
let native1 = <c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadOnly> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadOnly>, _>(args.1));
const { assert!(!::core::mem::needs_drop::<A2>()); }
const _: () = assert!(!::core::mem::needs_drop::<bool>());
let native2 = <c::CBool as c::expression::CType>::into_storage(c::expression::implicit::<c::CBool, _>(args.2));
const { assert!(!::core::mem::needs_drop::<A3>()); }
const _: () = assert!(!::core::mem::needs_drop::<bool>());
let native3 = <c::CBool as c::expression::CType>::into_storage(c::expression::implicit::<c::CBool, _>(args.3));
const { assert!(!::core::mem::needs_drop::<A4>()); }
const _: () = assert!(!::core::mem::needs_drop::<u32>());
let native4 = <c::expression::CEnumObject<crate::__pgrx_c_generated::EnumIdentity_CRSSnapshotAction, c::CUnsignedInt, u32> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CEnumObject<crate::__pgrx_c_generated::EnumIdentity_CRSSnapshotAction, c::CUnsignedInt, u32>, _>(args.4));
const { assert!(!::core::mem::needs_drop::<A5>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut u64>());
let native5 = <c::expression::CPointer<c::CUnsignedLong, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::CUnsignedLong, c::expression::ReadWrite>, _>(args.5));
// SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
let result = unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0, native1, native2, native3, native4, native5)) };
<c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadWrite> as c::expression::CType>::from_storage(result)
}
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::WalReceiverConn,
                *mut u32,
            ) -> *mut ::core::ffi::c_char,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::WalReceiverConn,
                *mut u32,
            ) -> *mut ::core::ffi::c_char,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_char__0p__0l__struct__WalReceiverConn__0p__0c__unsigned__int__0p__0r;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::sealed::Sealed
    for Signature_char__0p__0l__struct__WalReceiverConn__0p__0c__unsigned__int__0p__0r
{
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::NativeFunctionSignature
    for Signature_char__0p__0l__struct__WalReceiverConn__0p__0c__unsigned__int__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::WalReceiverConn,
                *mut u32,
            ) -> *mut ::core::ffi::c_char,
        >,
    >;
}
const _: () =
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::WalReceiverConn>() == 8);
const _: () =
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::WalReceiverConn>() == 8);
const _: () = assert!(::core::mem::size_of::<*mut u32>() == 8);
const _: () = assert!(::core::mem::align_of::<*mut u32>() == 8);
const _: () = assert!(::core::mem::size_of::<*mut ::core::ffi::c_char>() == 8);
const _: () = assert!(::core::mem::align_of::<*mut ::core::ffi::c_char>() == 8);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<
    A0: c::expression::ImplicitTo<
            c::expression::CPointer<
                c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>,
                c::expression::ReadWrite,
            >,
        >,
    A1: c::expression::ImplicitTo<c::expression::CPointer<c::CUnsignedInt, c::expression::ReadWrite>>,
> c::expression::Call<(A0, A1)>
    for Signature_char__0p__0l__struct__WalReceiverConn__0p__0c__unsigned__int__0p__0r
{
    type Output = <c::expression::CPointer<
        c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
        c::expression::ReadWrite,
    > as c::expression::CType>::Value;
    unsafe fn call(pointer: Self::Pointer, args: (A0, A1)) -> Self::Output {
        let function = pointer.expect("C indirect call requires a non-null function pointer");
        const {
            assert!(!::core::mem::needs_drop::<A0>());
        }
        const _: () =
            assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::WalReceiverConn>());
        let native0 = <c::expression::CPointer<
            c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>,
            c::expression::ReadWrite,
        > as c::expression::CType>::into_storage(c::expression::implicit::<
            c::expression::CPointer<
                c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>,
                c::expression::ReadWrite,
            >,
            _,
        >(args.0));
        const {
            assert!(!::core::mem::needs_drop::<A1>());
        }
        const _: () = assert!(!::core::mem::needs_drop::<*mut u32>());
        let native1 = <c::expression::CPointer<c::CUnsignedInt, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::CUnsignedInt, c::expression::ReadWrite>, _>(args.1));
        // SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
        let result =
            unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0, native1)) };
        <c::expression::CPointer<
            c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
            c::expression::ReadWrite,
        > as c::expression::CType>::from_storage(result)
    }
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const i8, *const i8, i32) -> *mut i8>,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const i8, *const i8, i32) -> *mut i8>,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_char__0p__0l__const__char__0p__0c__const__char__0p__0c__int__0r;
impl c::sealed::Sealed
    for Signature_char__0p__0l__const__char__0p__0c__const__char__0p__0c__int__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_char__0p__0l__const__char__0p__0c__const__char__0p__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const i8, *const i8, i32) -> *mut i8>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::varlena,
            ) -> *mut ::core::ffi::c_char,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::varlena,
            ) -> *mut ::core::ffi::c_char,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_char__0p__0l__const__struct__varlena__0p__0r;
impl c::sealed::Sealed for Signature_char__0p__0l__const__struct__varlena__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_char__0p__0l__const__struct__varlena__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::varlena,
            ) -> *mut ::core::ffi::c_char,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::MemoryContextData,
                *const ::core::ffi::c_char,
            ) -> *mut ::core::ffi::c_char,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::MemoryContextData,
                *const ::core::ffi::c_char,
            ) -> *mut ::core::ffi::c_char,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_char__0p__0l__struct__MemoryContextData__0p__0c__const__char__0p__0r;
impl c::sealed::Sealed
    for Signature_char__0p__0l__struct__MemoryContextData__0p__0c__const__char__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_char__0p__0l__struct__MemoryContextData__0p__0c__const__char__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::MemoryContextData,
                *const ::core::ffi::c_char,
            ) -> *mut ::core::ffi::c_char,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                bool,
            ) -> *mut ::core::ffi::c_char,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                bool,
            ) -> *mut ::core::ffi::c_char,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_char__0p__0l__unsigned__int__0c__0x5f426f6f6c__0r;
impl c::sealed::Sealed for Signature_char__0p__0l__unsigned__int__0c__0x5f426f6f6c__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_char__0p__0l__unsigned__int__0c__0x5f426f6f6c__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                bool,
            ) -> *mut ::core::ffi::c_char,
        >,
    >;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Oid,
                i32,
                i32,
            ) -> *mut ::core::ffi::c_char,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Oid,
                i32,
                i32,
            ) -> *mut ::core::ffi::c_char,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_char__0p__0l__unsigned__int__0c__unsigned__int__0c__unsigned__int__0c__int__0c__enum__ForkNumber__0r;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
impl c::sealed::Sealed for Signature_char__0p__0l__unsigned__int__0c__unsigned__int__0c__unsigned__int__0c__int__0c__enum__ForkNumber__0r {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
impl c::expression::NativeFunctionSignature for Signature_char__0p__0l__unsigned__int__0c__unsigned__int__0c__unsigned__int__0c__int__0c__enum__ForkNumber__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Oid, crate::__pgrx_c_bindings::Oid, crate::__pgrx_c_bindings::Oid, i32, i32) -> *mut ::core::ffi::c_char>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn() -> *const crate::__pgrx_c_bindings::Pg_finfo_record,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn() -> *const crate::__pgrx_c_bindings::Pg_finfo_record,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_const__Pg_finfo_record__0p__0l__void__0r;
impl c::sealed::Sealed for Signature_const__Pg_finfo_record__0p__0l__void__0r {}
impl c::expression::NativeFunctionSignature for Signature_const__Pg_finfo_record__0p__0l__void__0r {
    type Physical = PhysicalFunction_C_unwind_0<
        ::core::option::Option<
            unsafe extern "C-unwind" fn() -> *const crate::__pgrx_c_bindings::Pg_finfo_record,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn() -> *const crate::__pgrx_c_bindings::Pg_magic_struct,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn() -> *const crate::__pgrx_c_bindings::Pg_magic_struct,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_const__Pg_magic_struct__0p__0l__void__0r;
impl c::sealed::Sealed for Signature_const__Pg_magic_struct__0p__0l__void__0r {}
impl c::expression::NativeFunctionSignature for Signature_const__Pg_magic_struct__0p__0l__void__0r {
    type Physical = PhysicalFunction_C_unwind_0<
        ::core::option::Option<
            unsafe extern "C-unwind" fn() -> *const crate::__pgrx_c_bindings::Pg_magic_struct,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(u8) -> *const ::core::ffi::c_char>,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(u8) -> *const ::core::ffi::c_char>,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_const__char__0p__0l__unsigned__char__0r;
impl c::sealed::Sealed for Signature_const__char__0p__0l__unsigned__char__0r {}
impl c::expression::NativeFunctionSignature for Signature_const__char__0p__0l__unsigned__char__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(u8) -> *const ::core::ffi::c_char>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
            ) -> *const ::core::ffi::c_char,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
            ) -> *const ::core::ffi::c_char,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_const__char__0p__0l__unsigned__int__0r;
impl c::sealed::Sealed for Signature_const__char__0p__0l__unsigned__int__0r {}
impl c::expression::NativeFunctionSignature for Signature_const__char__0p__0l__unsigned__int__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
            ) -> *const ::core::ffi::c_char,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn() -> *const ::core::ffi::c_char>,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn() -> *const ::core::ffi::c_char>,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_const__char__0p__0l__void__0r;
impl c::sealed::Sealed for Signature_const__char__0p__0l__void__0r {}
impl c::expression::NativeFunctionSignature for Signature_const__char__0p__0l__void__0r {
    type Physical = PhysicalFunction_C_unwind_0<
        ::core::option::Option<unsafe extern "C-unwind" fn() -> *const ::core::ffi::c_char>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(i8) -> *const ::core::ffi::c_char>,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(i8) -> *const ::core::ffi::c_char>,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_const__char__0p__0l__char__0r;
impl c::sealed::Sealed for Signature_const__char__0p__0l__char__0r {}
impl c::expression::NativeFunctionSignature for Signature_const__char__0p__0l__char__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(i8) -> *const ::core::ffi::c_char>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                i32,
                *const crate::__pgrx_c_bindings::ScanKeywordList,
            ) -> *const ::core::ffi::c_char,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                i32,
                *const crate::__pgrx_c_bindings::ScanKeywordList,
            ) -> *const ::core::ffi::c_char,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_const__char__0p__0l__int__0c__const__struct__ScanKeywordList__0p__0r;
impl c::sealed::Sealed
    for Signature_const__char__0p__0l__int__0c__const__struct__ScanKeywordList__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_const__char__0p__0l__int__0c__const__struct__ScanKeywordList__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                i32,
                *const crate::__pgrx_c_bindings::ScanKeywordList,
            ) -> *const ::core::ffi::c_char,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::Node,
            ) -> *const ::core::ffi::c_char,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::Node,
            ) -> *const ::core::ffi::c_char,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_const__char__0p__0l__struct__Node__0p__0r;
impl c::sealed::Sealed for Signature_const__char__0p__0l__struct__Node__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_const__char__0p__0l__struct__Node__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::Node,
            ) -> *const ::core::ffi::c_char,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
            )
                -> *const crate::__pgrx_c_bindings::TupleTableSlotOps,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
            )
                -> *const crate::__pgrx_c_bindings::TupleTableSlotOps,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_const__struct__TupleTableSlotOps__0p__0l__struct__RelationData__0p__0r;
impl c::sealed::Sealed
    for Signature_const__struct__TupleTableSlotOps__0p__0l__struct__RelationData__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_const__struct__TupleTableSlotOps__0p__0l__struct__RelationData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
            )
                -> *const crate::__pgrx_c_bindings::TupleTableSlotOps,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn() -> *mut *const u16>>(
    ) == 8
);
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn() -> *mut *const u16>>(
    ) == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_const__unsigned__short__0p__0p__0l__void__0r;
impl c::sealed::Sealed for Signature_const__unsigned__short__0p__0p__0l__void__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_const__unsigned__short__0p__0p__0l__void__0r
{
    type Physical = PhysicalFunction_C_unwind_0<
        ::core::option::Option<unsafe extern "C-unwind" fn() -> *mut *const u16>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::IndexInfo,
                bool,
                bool,
                bool,
                u32,
                u32,
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::RelationData,
                        *mut crate::__pgrx_c_bindings::ItemPointerData,
                        *mut crate::__pgrx_c_bindings::Datum,
                        *mut bool,
                        bool,
                        *mut ::core::ffi::c_void,
                    ) -> (),
                >,
                *mut ::core::ffi::c_void,
                *mut crate::__pgrx_c_bindings::TableScanDescData,
            ) -> f64,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::IndexInfo,
                bool,
                bool,
                bool,
                u32,
                u32,
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::RelationData,
                        *mut crate::__pgrx_c_bindings::ItemPointerData,
                        *mut crate::__pgrx_c_bindings::Datum,
                        *mut bool,
                        bool,
                        *mut ::core::ffi::c_void,
                    ) -> (),
                >,
                *mut ::core::ffi::c_void,
                *mut crate::__pgrx_c_bindings::TableScanDescData,
            ) -> f64,
        >,
    >() == 8
);
#[doc(hidden)]
pub struct PhysicalFunction_C_unwind_11<P>(::core::marker::PhantomData<P>);
impl<P> c::sealed::Sealed for PhysicalFunction_C_unwind_11<P> {}
impl<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, R> c::expression::PhysicalFunctionPointer
    for PhysicalFunction_C_unwind_11<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10) -> R,
        >,
    >
{
    type Pointer = ::core::option::Option<
        unsafe extern "C-unwind" fn(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10) -> R,
    >;
    fn null() -> Self::Pointer {
        None
    }
    fn address(pointer: Self::Pointer) -> *const () {
        pointer.map_or(::core::ptr::null(), |function| function as *const ())
    }
}
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_double__0l__struct__RelationData__0p__0c__struct__RelationData__0p__0c__struct__IndexInfo__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__unsigned__int__0c__unsigned__int__0c__void__0l__0p__0r__0l__struct__RelationData__0p__0c__struct__ItemPointerData__0p__0c__unsigned__long__0p__0c__0x5f426f6f6c__0p__0c__0x5f426f6f6c__0c__void__0p__0r__0c__void__0p__0c__struct__TableScanDescData__0p__0r;
impl c::sealed::Sealed for Signature_double__0l__struct__RelationData__0p__0c__struct__RelationData__0p__0c__struct__IndexInfo__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__unsigned__int__0c__unsigned__int__0c__void__0l__0p__0r__0l__struct__RelationData__0p__0c__struct__ItemPointerData__0p__0c__unsigned__long__0p__0c__0x5f426f6f6c__0p__0c__0x5f426f6f6c__0c__void__0p__0r__0c__void__0p__0c__struct__TableScanDescData__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_double__0l__struct__RelationData__0p__0c__struct__RelationData__0p__0c__struct__IndexInfo__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__unsigned__int__0c__unsigned__int__0c__void__0l__0p__0r__0l__struct__RelationData__0p__0c__struct__ItemPointerData__0p__0c__unsigned__long__0p__0c__0x5f426f6f6c__0p__0c__0x5f426f6f6c__0c__void__0p__0r__0c__void__0p__0c__struct__TableScanDescData__0p__0r {
 type Physical = PhysicalFunction_C_unwind_11<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::IndexInfo, bool, bool, bool, u32, u32, ::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::ItemPointerData, *mut crate::__pgrx_c_bindings::Datum, *mut bool, bool, *mut ::core::ffi::c_void) -> ()>, *mut ::core::ffi::c_void, *mut crate::__pgrx_c_bindings::TableScanDescData) -> f64>>;
}
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn(*const i8) -> f64>>()
        == 8
);
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn(*const i8) -> f64>>(
    ) == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_double__0l__const__char__0p__0r;
impl c::sealed::Sealed for Signature_double__0l__const__char__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_double__0l__const__char__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const i8) -> f64>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn(f64) -> f64>>() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn(f64) -> f64>>() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_double__0l__double__0r;
impl c::sealed::Sealed for Signature_double__0l__double__0r {}
impl c::expression::NativeFunctionSignature for Signature_double__0l__double__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(f64) -> f64>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn(f64, f64) -> f64>>()
        == 8
);
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn(f64, f64) -> f64>>()
        == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_double__0l__double__0c__double__0r;
impl c::sealed::Sealed for Signature_double__0l__double__0c__double__0r {}
impl c::expression::NativeFunctionSignature for Signature_double__0l__double__0c__double__0r {
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<unsafe extern "C-unwind" fn(f64, f64) -> f64>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::IndexInfo,
                bool,
                bool,
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::RelationData,
                        *mut crate::__pgrx_c_bindings::ItemPointerData,
                        *mut crate::__pgrx_c_bindings::Datum,
                        *mut bool,
                        bool,
                        *mut ::core::ffi::c_void,
                    ) -> (),
                >,
                *mut ::core::ffi::c_void,
                *mut crate::__pgrx_c_bindings::TableScanDescData,
            ) -> f64,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::IndexInfo,
                bool,
                bool,
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::RelationData,
                        *mut crate::__pgrx_c_bindings::ItemPointerData,
                        *mut crate::__pgrx_c_bindings::Datum,
                        *mut bool,
                        bool,
                        *mut ::core::ffi::c_void,
                    ) -> (),
                >,
                *mut ::core::ffi::c_void,
                *mut crate::__pgrx_c_bindings::TableScanDescData,
            ) -> f64,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_double__0l__struct__RelationData__0p__0c__struct__RelationData__0p__0c__struct__IndexInfo__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__void__0l__0p__0r__0l__struct__RelationData__0p__0c__struct__ItemPointerData__0p__0c__unsigned__long__0p__0c__0x5f426f6f6c__0p__0c__0x5f426f6f6c__0c__void__0p__0r__0c__void__0p__0c__struct__TableScanDescData__0p__0r;
impl c::sealed::Sealed for Signature_double__0l__struct__RelationData__0p__0c__struct__RelationData__0p__0c__struct__IndexInfo__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__void__0l__0p__0r__0l__struct__RelationData__0p__0c__struct__ItemPointerData__0p__0c__unsigned__long__0p__0c__0x5f426f6f6c__0p__0c__0x5f426f6f6c__0c__void__0p__0r__0c__void__0p__0c__struct__TableScanDescData__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_double__0l__struct__RelationData__0p__0c__struct__RelationData__0p__0c__struct__IndexInfo__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__void__0l__0p__0r__0l__struct__RelationData__0p__0c__struct__ItemPointerData__0p__0c__unsigned__long__0p__0c__0x5f426f6f6c__0p__0c__0x5f426f6f6c__0c__void__0p__0r__0c__void__0p__0c__struct__TableScanDescData__0p__0r {
 type Physical = PhysicalFunction_C_unwind_8<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::IndexInfo, bool, bool, ::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::ItemPointerData, *mut crate::__pgrx_c_bindings::Datum, *mut bool, bool, *mut ::core::ffi::c_void) -> ()>, *mut ::core::ffi::c_void, *mut crate::__pgrx_c_bindings::TableScanDescData) -> f64>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Datum) -> f64>,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Datum) -> f64>,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_double__0l__unsigned__long__0r;
impl c::sealed::Sealed for Signature_double__0l__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature for Signature_double__0l__unsigned__long__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Datum) -> f64>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn() -> f64>>() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn() -> f64>>() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_double__0l__void__0r;
impl c::sealed::Sealed for Signature_double__0l__void__0r {}
impl c::expression::NativeFunctionSignature for Signature_double__0l__void__0r {
    type Physical =
        PhysicalFunction_C_unwind_0<::core::option::Option<unsafe extern "C-unwind" fn() -> f64>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RangeTblEntry, u32) -> u32,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RangeTblEntry, u32) -> u32,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_enum__RowMarkType__0l__struct__RangeTblEntry__0p__0c__enum__LockClauseStrength__0r;
impl c::sealed::Sealed
    for Signature_enum__RowMarkType__0l__struct__RangeTblEntry__0p__0c__enum__LockClauseStrength__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_enum__RowMarkType__0l__struct__RangeTblEntry__0p__0c__enum__LockClauseStrength__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RangeTblEntry, u32) -> u32,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::ItemPointerData,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                u32,
                u32,
                u32,
                u8,
                *mut crate::__pgrx_c_bindings::TM_FailureData,
            ) -> u32,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::ItemPointerData,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                u32,
                u32,
                u32,
                u8,
                *mut crate::__pgrx_c_bindings::TM_FailureData,
            ) -> u32,
        >,
    >() == 8
);
#[doc(hidden)]
pub struct PhysicalFunction_C_unwind_9<P>(::core::marker::PhantomData<P>);
impl<P> c::sealed::Sealed for PhysicalFunction_C_unwind_9<P> {}
impl<A0, A1, A2, A3, A4, A5, A6, A7, A8, R> c::expression::PhysicalFunctionPointer
    for PhysicalFunction_C_unwind_9<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(A0, A1, A2, A3, A4, A5, A6, A7, A8) -> R,
        >,
    >
{
    type Pointer = ::core::option::Option<
        unsafe extern "C-unwind" fn(A0, A1, A2, A3, A4, A5, A6, A7, A8) -> R,
    >;
    fn null() -> Self::Pointer {
        None
    }
    fn address(pointer: Self::Pointer) -> *const () {
        pointer.map_or(::core::ptr::null(), |function| function as *const ())
    }
}
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_enum__TM_Result__0l__struct__RelationData__0p__0c__struct__ItemPointerData__0p__0c__struct__SnapshotData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__enum__LockTupleMode__0c__enum__LockWaitPolicy__0c__unsigned__char__0c__struct__TM_FailureData__0p__0r;
impl c::sealed::Sealed for Signature_enum__TM_Result__0l__struct__RelationData__0p__0c__struct__ItemPointerData__0p__0c__struct__SnapshotData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__enum__LockTupleMode__0c__enum__LockWaitPolicy__0c__unsigned__char__0c__struct__TM_FailureData__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_enum__TM_Result__0l__struct__RelationData__0p__0c__struct__ItemPointerData__0p__0c__struct__SnapshotData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__enum__LockTupleMode__0c__enum__LockWaitPolicy__0c__unsigned__char__0c__struct__TM_FailureData__0p__0r {
 type Physical = PhysicalFunction_C_unwind_9<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::ItemPointerData, *mut crate::__pgrx_c_bindings::SnapshotData, *mut crate::__pgrx_c_bindings::TupleTableSlot, u32, u32, u32, u8, *mut crate::__pgrx_c_bindings::TM_FailureData) -> u32>>;
}
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::ItemPointerData,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                u32,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                bool,
                *mut crate::__pgrx_c_bindings::TM_FailureData,
                *mut u32,
                *mut bool,
            ) -> u32,
        >,
    >() == 8
);
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::ItemPointerData,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                u32,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                bool,
                *mut crate::__pgrx_c_bindings::TM_FailureData,
                *mut u32,
                *mut bool,
            ) -> u32,
        >,
    >() == 8
);
#[doc(hidden)]
pub struct PhysicalFunction_C_unwind_10<P>(::core::marker::PhantomData<P>);
impl<P> c::sealed::Sealed for PhysicalFunction_C_unwind_10<P> {}
impl<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, R> c::expression::PhysicalFunctionPointer
    for PhysicalFunction_C_unwind_10<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9) -> R,
        >,
    >
{
    type Pointer = ::core::option::Option<
        unsafe extern "C-unwind" fn(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9) -> R,
    >;
    fn null() -> Self::Pointer {
        None
    }
    fn address(pointer: Self::Pointer) -> *const () {
        pointer.map_or(::core::ptr::null(), |function| function as *const ())
    }
}
#[cfg(feature = "pg15")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_enum__TM_Result__0l__struct__RelationData__0p__0c__struct__ItemPointerData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__struct__SnapshotData__0p__0c__struct__SnapshotData__0p__0c__0x5f426f6f6c__0c__struct__TM_FailureData__0p__0c__enum__LockTupleMode__0p__0c__0x5f426f6f6c__0p__0r;
#[cfg(feature = "pg15")]
impl c::sealed::Sealed for Signature_enum__TM_Result__0l__struct__RelationData__0p__0c__struct__ItemPointerData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__struct__SnapshotData__0p__0c__struct__SnapshotData__0p__0c__0x5f426f6f6c__0c__struct__TM_FailureData__0p__0c__enum__LockTupleMode__0p__0c__0x5f426f6f6c__0p__0r {}
#[cfg(feature = "pg15")]
impl c::expression::NativeFunctionSignature for Signature_enum__TM_Result__0l__struct__RelationData__0p__0c__struct__ItemPointerData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__struct__SnapshotData__0p__0c__struct__SnapshotData__0p__0c__0x5f426f6f6c__0c__struct__TM_FailureData__0p__0c__enum__LockTupleMode__0p__0c__0x5f426f6f6c__0p__0r {
 type Physical = PhysicalFunction_C_unwind_10<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::ItemPointerData, *mut crate::__pgrx_c_bindings::TupleTableSlot, u32, *mut crate::__pgrx_c_bindings::SnapshotData, *mut crate::__pgrx_c_bindings::SnapshotData, bool, *mut crate::__pgrx_c_bindings::TM_FailureData, *mut u32, *mut bool) -> u32>>;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::ItemPointerData,
                u32,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                bool,
                *mut crate::__pgrx_c_bindings::TM_FailureData,
                bool,
            ) -> u32,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::ItemPointerData,
                u32,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                bool,
                *mut crate::__pgrx_c_bindings::TM_FailureData,
                bool,
            ) -> u32,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_enum__TM_Result__0l__struct__RelationData__0p__0c__struct__ItemPointerData__0p__0c__unsigned__int__0c__struct__SnapshotData__0p__0c__struct__SnapshotData__0p__0c__0x5f426f6f6c__0c__struct__TM_FailureData__0p__0c__0x5f426f6f6c__0r;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::sealed::Sealed for Signature_enum__TM_Result__0l__struct__RelationData__0p__0c__struct__ItemPointerData__0p__0c__unsigned__int__0c__struct__SnapshotData__0p__0c__struct__SnapshotData__0p__0c__0x5f426f6f6c__0c__struct__TM_FailureData__0p__0c__0x5f426f6f6c__0r {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::NativeFunctionSignature for Signature_enum__TM_Result__0l__struct__RelationData__0p__0c__struct__ItemPointerData__0p__0c__unsigned__int__0c__struct__SnapshotData__0p__0c__struct__SnapshotData__0p__0c__0x5f426f6f6c__0c__struct__TM_FailureData__0p__0c__0x5f426f6f6c__0r {
 type Physical = PhysicalFunction_C_unwind_8<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::ItemPointerData, u32, *mut crate::__pgrx_c_bindings::SnapshotData, *mut crate::__pgrx_c_bindings::SnapshotData, bool, *mut crate::__pgrx_c_bindings::TM_FailureData, bool) -> u32>>;
}
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn(f32, f32) -> f32>>()
        == 8
);
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn(f32, f32) -> f32>>()
        == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_float__0l__float__0c__float__0r;
impl c::sealed::Sealed for Signature_float__0l__float__0c__float__0r {}
impl c::expression::NativeFunctionSignature for Signature_float__0l__float__0c__float__0r {
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<unsafe extern "C-unwind" fn(f32, f32) -> f32>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Datum) -> f32>,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Datum) -> f32>,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_float__0l__unsigned__long__0r;
impl c::sealed::Sealed for Signature_float__0l__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature for Signature_float__0l__unsigned__long__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Datum) -> f32>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn() -> f32>>() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn() -> f32>>() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_float__0l__void__0r;
impl c::sealed::Sealed for Signature_float__0l__void__0r {}
impl c::expression::NativeFunctionSignature for Signature_float__0l__void__0r {
    type Physical =
        PhysicalFunction_C_unwind_0<::core::option::Option<unsafe extern "C-unwind" fn() -> f32>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::RBTNode,
                *const crate::__pgrx_c_bindings::RBTNode,
                *mut ::core::ffi::c_void,
            ) -> i32,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::RBTNode,
                *const crate::__pgrx_c_bindings::RBTNode,
                *mut ::core::ffi::c_void,
            ) -> i32,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__const__struct__RBTNode__0p__0c__const__struct__RBTNode__0p__0c__void__0p__0r;
impl c::sealed::Sealed for Signature_int__0l__const__struct__RBTNode__0p__0c__const__struct__RBTNode__0p__0c__void__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_int__0l__const__struct__RBTNode__0p__0c__const__struct__RBTNode__0p__0c__void__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::RBTNode, *const crate::__pgrx_c_bindings::RBTNode, *mut ::core::ffi::c_void) -> i32>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::pairingheap_node,
                *const crate::__pgrx_c_bindings::pairingheap_node,
                *mut ::core::ffi::c_void,
            ) -> i32,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::pairingheap_node,
                *const crate::__pgrx_c_bindings::pairingheap_node,
                *mut ::core::ffi::c_void,
            ) -> i32,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__const__struct__pairingheap_node__0p__0c__const__struct__pairingheap_node__0p__0c__void__0p__0r;
impl c::sealed::Sealed for Signature_int__0l__const__struct__pairingheap_node__0p__0c__const__struct__pairingheap_node__0p__0c__void__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_int__0l__const__struct__pairingheap_node__0p__0c__const__struct__pairingheap_node__0p__0c__void__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::pairingheap_node, *const crate::__pgrx_c_bindings::pairingheap_node, *mut ::core::ffi::c_void) -> i32>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::ListCell,
                *const crate::__pgrx_c_bindings::ListCell,
            ) -> i32,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::ListCell,
                *const crate::__pgrx_c_bindings::ListCell,
            ) -> i32,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__const__union__ListCell__0p__0c__const__union__ListCell__0p__0r;
impl c::sealed::Sealed
    for Signature_int__0l__const__union__ListCell__0p__0c__const__union__ListCell__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_int__0l__const__union__ListCell__0p__0c__const__union__ListCell__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::ListCell,
                *const crate::__pgrx_c_bindings::ListCell,
            ) -> i32,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn(*const u8) -> i32>>()
        == 8
);
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn(*const u8) -> i32>>(
    ) == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__const__unsigned__char__0p__0r;
impl c::sealed::Sealed for Signature_int__0l__const__unsigned__char__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_int__0l__const__unsigned__char__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const u8) -> i32>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const u8, i32) -> i32>,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const u8, i32) -> i32>,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__const__unsigned__char__0p__0c__int__0r;
impl c::sealed::Sealed for Signature_int__0l__const__unsigned__char__0p__0c__int__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_int__0l__const__unsigned__char__0p__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const u8, i32) -> i32>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const u8, *mut u32, i32) -> i32>,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const u8, *mut u32, i32) -> i32>,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__const__unsigned__char__0p__0c__unsigned__int__0p__0c__int__0r;
impl c::sealed::Sealed
    for Signature_int__0l__const__unsigned__char__0p__0c__unsigned__int__0p__0c__int__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_int__0l__const__unsigned__char__0p__0c__unsigned__int__0p__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const u8, *mut u32, i32) -> i32>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const u32, *mut u8, i32) -> i32>,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const u32, *mut u8, i32) -> i32>,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__const__unsigned__int__0p__0c__unsigned__char__0p__0c__int__0r;
impl c::sealed::Sealed
    for Signature_int__0l__const__unsigned__int__0p__0c__unsigned__char__0p__0c__int__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_int__0l__const__unsigned__int__0p__0c__unsigned__char__0p__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const u32, *mut u8, i32) -> i32>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
            ) -> i32,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
            ) -> i32,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__const__void__0p__0c__const__void__0p__0r;
impl c::sealed::Sealed for Signature_int__0l__const__void__0p__0c__const__void__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_int__0l__const__void__0p__0c__const__void__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
            ) -> i32,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                usize,
            ) -> i32,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                usize,
            ) -> i32,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__const__void__0p__0c__const__void__0p__0c__unsigned__long__0r;
impl c::sealed::Sealed
    for Signature_int__0l__const__void__0p__0c__const__void__0p__0c__unsigned__long__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_int__0l__const__void__0p__0c__const__void__0p__0c__unsigned__long__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                usize,
            ) -> i32,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                usize,
                *mut ::core::ffi::c_void,
            ) -> i32,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                usize,
                *mut ::core::ffi::c_void,
            ) -> i32,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__const__void__0p__0c__const__void__0p__0c__unsigned__long__0c__void__0p__0r;
impl c::sealed::Sealed for Signature_int__0l__const__void__0p__0c__const__void__0p__0c__unsigned__long__0c__void__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_int__0l__const__void__0p__0c__const__void__0p__0c__unsigned__long__0c__void__0p__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_void, *const ::core::ffi::c_void, usize, *mut ::core::ffi::c_void) -> i32>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
            ) -> i32,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
            ) -> i32,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__const__void__0p__0c__const__void__0p__0c__void__0p__0r;
impl c::sealed::Sealed
    for Signature_int__0l__const__void__0p__0c__const__void__0p__0c__void__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_int__0l__const__void__0p__0c__const__void__0p__0c__void__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
            ) -> i32,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const ::core::ffi::c_void, usize) -> i32,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const ::core::ffi::c_void, usize) -> i32,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__const__void__0p__0c__unsigned__long__0r;
impl c::sealed::Sealed for Signature_int__0l__const__void__0p__0c__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_int__0l__const__void__0p__0c__unsigned__long__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const ::core::ffi::c_void, usize) -> i32,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData) -> i32,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData) -> i32,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__struct__RelationData__0p__0r;
impl c::sealed::Sealed for Signature_int__0l__struct__RelationData__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_int__0l__struct__RelationData__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData) -> i32,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                i32,
                *mut *mut crate::__pgrx_c_bindings::HeapTupleData,
                i32,
                *mut f64,
                *mut f64,
            ) -> i32,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                i32,
                *mut *mut crate::__pgrx_c_bindings::HeapTupleData,
                i32,
                *mut f64,
                *mut f64,
            ) -> i32,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__struct__RelationData__0p__0c__int__0c__struct__HeapTupleData__0p__0p__0c__int__0c__double__0p__0c__double__0p__0r;
impl c::sealed::Sealed for Signature_int__0l__struct__RelationData__0p__0c__int__0c__struct__HeapTupleData__0p__0p__0c__int__0c__double__0p__0c__double__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_int__0l__struct__RelationData__0p__0c__int__0c__struct__HeapTupleData__0p__0p__0c__int__0c__double__0p__0c__double__0p__0r {
 type Physical = PhysicalFunction_C_unwind_6<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, i32, *mut *mut crate::__pgrx_c_bindings::HeapTupleData, i32, *mut f64, *mut f64) -> i32>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ResultRelInfo) -> i32,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ResultRelInfo) -> i32,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__struct__ResultRelInfo__0p__0r;
impl c::sealed::Sealed for Signature_int__0l__struct__ResultRelInfo__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_int__0l__struct__ResultRelInfo__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ResultRelInfo) -> i32,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::WalReceiverConn) -> i32,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::WalReceiverConn) -> i32,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__struct__WalReceiverConn__0p__0r;
impl c::sealed::Sealed for Signature_int__0l__struct__WalReceiverConn__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_int__0l__struct__WalReceiverConn__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::WalReceiverConn) -> i32,
        >,
    >;
}
const _: () =
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::WalReceiverConn>() == 8);
const _: () =
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::WalReceiverConn>() == 8);
const _: () = assert!(::core::mem::size_of::<i32>() == 4);
const _: () = assert!(::core::mem::align_of::<i32>() == 4);
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<
    A0: c::expression::ImplicitTo<
            c::expression::CPointer<
                c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>,
                c::expression::ReadWrite,
            >,
        >,
> c::expression::Call<(A0,)> for Signature_int__0l__struct__WalReceiverConn__0p__0r
{
    type Output = <c::CInt as c::expression::CType>::Value;
    unsafe fn call(pointer: Self::Pointer, args: (A0,)) -> Self::Output {
        let function = pointer.expect("C indirect call requires a non-null function pointer");
        const {
            assert!(!::core::mem::needs_drop::<A0>());
        }
        const _: () =
            assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::WalReceiverConn>());
        let native0 = <c::expression::CPointer<
            c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>,
            c::expression::ReadWrite,
        > as c::expression::CType>::into_storage(c::expression::implicit::<
            c::expression::CPointer<
                c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>,
                c::expression::ReadWrite,
            >,
            _,
        >(args.0));
        // SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
        let result = unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0)) };
        <c::CInt as c::expression::CType>::from_storage(result)
    }
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::WalReceiverConn,
                *mut *mut ::core::ffi::c_char,
                *mut i32,
            ) -> i32,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::WalReceiverConn,
                *mut *mut ::core::ffi::c_char,
                *mut i32,
            ) -> i32,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__struct__WalReceiverConn__0p__0c__char__0p__0p__0c__int__0p__0r;
impl c::sealed::Sealed
    for Signature_int__0l__struct__WalReceiverConn__0p__0c__char__0p__0p__0c__int__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_int__0l__struct__WalReceiverConn__0p__0c__char__0p__0p__0c__int__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::WalReceiverConn,
                *mut *mut ::core::ffi::c_char,
                *mut i32,
            ) -> i32,
        >,
    >;
}
const _: () =
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::WalReceiverConn>() == 8);
const _: () =
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::WalReceiverConn>() == 8);
const _: () = assert!(::core::mem::size_of::<*mut *mut ::core::ffi::c_char>() == 8);
const _: () = assert!(::core::mem::align_of::<*mut *mut ::core::ffi::c_char>() == 8);
const _: () = assert!(::core::mem::size_of::<*mut i32>() == 8);
const _: () = assert!(::core::mem::align_of::<*mut i32>() == 8);
const _: () = assert!(::core::mem::size_of::<i32>() == 4);
const _: () = assert!(::core::mem::align_of::<i32>() == 4);
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<
    A0: c::expression::ImplicitTo<
            c::expression::CPointer<
                c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>,
                c::expression::ReadWrite,
            >,
        >,
    A1: c::expression::ImplicitTo<
            c::expression::CPointer<
                c::expression::CPointer<
                    c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
                    c::expression::ReadWrite,
                >,
                c::expression::ReadWrite,
            >,
        >,
    A2: c::expression::ImplicitTo<c::expression::CPointer<c::CInt, c::expression::ReadWrite>>,
> c::expression::Call<(A0, A1, A2)>
    for Signature_int__0l__struct__WalReceiverConn__0p__0c__char__0p__0p__0c__int__0p__0r
{
    type Output = <c::CInt as c::expression::CType>::Value;
    unsafe fn call(pointer: Self::Pointer, args: (A0, A1, A2)) -> Self::Output {
        let function = pointer.expect("C indirect call requires a non-null function pointer");
        const {
            assert!(!::core::mem::needs_drop::<A0>());
        }
        const _: () =
            assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::WalReceiverConn>());
        let native0 = <c::expression::CPointer<
            c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>,
            c::expression::ReadWrite,
        > as c::expression::CType>::into_storage(c::expression::implicit::<
            c::expression::CPointer<
                c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>,
                c::expression::ReadWrite,
            >,
            _,
        >(args.0));
        const {
            assert!(!::core::mem::needs_drop::<A1>());
        }
        const _: () = assert!(!::core::mem::needs_drop::<*mut *mut ::core::ffi::c_char>());
        let native1 = <c::expression::CPointer<
            c::expression::CPointer<
                c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
                c::expression::ReadWrite,
            >,
            c::expression::ReadWrite,
        > as c::expression::CType>::into_storage(c::expression::implicit::<
            c::expression::CPointer<
                c::expression::CPointer<
                    c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
                    c::expression::ReadWrite,
                >,
                c::expression::ReadWrite,
            >,
            _,
        >(args.1));
        const {
            assert!(!::core::mem::needs_drop::<A2>());
        }
        const _: () = assert!(!::core::mem::needs_drop::<*mut i32>());
        let native2 = <c::expression::CPointer<c::CInt, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::CInt, c::expression::ReadWrite>, _>(args.2));
        // SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
        let result = unsafe {
            crate::ffi::pg_guard_ffi_boundary(move || function(native0, native1, native2))
        };
        <c::CInt as c::expression::CType>::from_storage(result)
    }
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::XLogReaderState,
                u64,
                i32,
                u64,
                *mut ::core::ffi::c_char,
            ) -> i32,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::XLogReaderState,
                u64,
                i32,
                u64,
                *mut ::core::ffi::c_char,
            ) -> i32,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__struct__XLogReaderState__0p__0c__unsigned__long__0c__int__0c__unsigned__long__0c__char__0p__0r;
impl c::sealed::Sealed for Signature_int__0l__struct__XLogReaderState__0p__0c__unsigned__long__0c__int__0c__unsigned__long__0c__char__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_int__0l__struct__XLogReaderState__0p__0c__unsigned__long__0c__int__0c__unsigned__long__0c__char__0p__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::XLogReaderState, u64, i32, u64, *mut ::core::ffi::c_char) -> i32>>;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn(u32) -> i32>>() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn(u32) -> i32>>() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__unsigned__int__0r;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::sealed::Sealed for Signature_int__0l__unsigned__int__0r {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::NativeFunctionSignature for Signature_int__0l__unsigned__int__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(u32) -> i32>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Oid, i16) -> i32,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Oid, i16) -> i32,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__unsigned__int__0c__short__0r;
impl c::sealed::Sealed for Signature_int__0l__unsigned__int__0c__short__0r {}
impl c::expression::NativeFunctionSignature for Signature_int__0l__unsigned__int__0c__short__0r {
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Oid, i16) -> i32,
        >,
    >;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn(u64) -> i32>>() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn(u64) -> i32>>() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__unsigned__long__0r;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::sealed::Sealed for Signature_int__0l__unsigned__long__0r {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::NativeFunctionSignature for Signature_int__0l__unsigned__long__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(u64) -> i32>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                *mut crate::__pgrx_c_bindings::SortSupportData,
            ) -> i32,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                *mut crate::__pgrx_c_bindings::SortSupportData,
            ) -> i32,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__unsigned__long__0c__unsigned__long__0c__struct__SortSupportData__0p__0r;
impl c::sealed::Sealed
    for Signature_int__0l__unsigned__long__0c__unsigned__long__0c__struct__SortSupportData__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_int__0l__unsigned__long__0c__unsigned__long__0c__struct__SortSupportData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Datum,
                *mut crate::__pgrx_c_bindings::SortSupportData,
            ) -> i32,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void, i32, i32) -> i32,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void, i32, i32) -> i32,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__void__0p__0c__int__0c__int__0r;
impl c::sealed::Sealed for Signature_int__0l__void__0p__0c__int__0c__int__0r {}
impl c::expression::NativeFunctionSignature for Signature_int__0l__void__0p__0c__int__0c__int__0r {
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void, i32, i32) -> i32,
        >,
    >;
}
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const i8, *const i8) -> i32>,
    >() == 8
);
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const i8, *const i8) -> i32>,
    >() == 8
);
#[cfg(feature = "pg15")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__const__char__0p__0c__const__char__0p__0r;
#[cfg(feature = "pg15")]
impl c::sealed::Sealed for Signature_int__0l__const__char__0p__0c__const__char__0p__0r {}
#[cfg(feature = "pg15")]
impl c::expression::NativeFunctionSignature
    for Signature_int__0l__const__char__0p__0c__const__char__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const i8, *const i8) -> i32>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::List) -> i32,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::List) -> i32,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__const__struct__List__0p__0r;
impl c::sealed::Sealed for Signature_int__0l__const__struct__List__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_int__0l__const__struct__List__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::List) -> i32,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::List,
                *const crate::__pgrx_c_bindings::ListCell,
            ) -> i32,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::List,
                *const crate::__pgrx_c_bindings::ListCell,
            ) -> i32,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__const__struct__List__0p__0c__const__union__ListCell__0p__0r;
impl c::sealed::Sealed
    for Signature_int__0l__const__struct__List__0p__0c__const__union__ListCell__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_int__0l__const__struct__List__0p__0c__const__union__ListCell__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::List,
                *const crate::__pgrx_c_bindings::ListCell,
            ) -> i32,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::List, i32) -> i32,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::List, i32) -> i32,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__const__struct__List__0p__0c__int__0r;
impl c::sealed::Sealed for Signature_int__0l__const__struct__List__0p__0c__int__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_int__0l__const__struct__List__0p__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::List, i32) -> i32,
        >,
    >;
}
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(i32, *mut crate::__pgrx_c_bindings::timespec) -> i32,
        >,
    >() == 8
);
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(i32, *mut crate::__pgrx_c_bindings::timespec) -> i32,
        >,
    >() == 8
);
#[cfg(feature = "pg15")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__int__0c__struct__timespec__0p__0r;
#[cfg(feature = "pg15")]
impl c::sealed::Sealed for Signature_int__0l__int__0c__struct__timespec__0p__0r {}
#[cfg(feature = "pg15")]
impl c::expression::NativeFunctionSignature
    for Signature_int__0l__int__0c__struct__timespec__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(i32, *mut crate::__pgrx_c_bindings::timespec) -> i32,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn(i64, i64) -> i32>>()
        == 8
);
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn(i64, i64) -> i32>>()
        == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__long__0c__long__0r;
impl c::sealed::Sealed for Signature_int__0l__long__0c__long__0r {}
impl c::expression::NativeFunctionSignature for Signature_int__0l__long__0c__long__0r {
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<unsafe extern "C-unwind" fn(i64, i64) -> i32>,
    >;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ItemPointerData,
                *mut crate::__pgrx_c_bindings::ItemPointerData,
            ) -> i32,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ItemPointerData,
                *mut crate::__pgrx_c_bindings::ItemPointerData,
            ) -> i32,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__struct__ItemPointerData__0p__0c__struct__ItemPointerData__0p__0r;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::sealed::Sealed
    for Signature_int__0l__struct__ItemPointerData__0p__0c__struct__ItemPointerData__0p__0r
{
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::NativeFunctionSignature
    for Signature_int__0l__struct__ItemPointerData__0p__0c__struct__ItemPointerData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ItemPointerData,
                *mut crate::__pgrx_c_bindings::ItemPointerData,
            ) -> i32,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
                bool,
                crate::__pgrx_c_bindings::Datum,
                bool,
                *mut crate::__pgrx_c_bindings::SortSupportData,
            ) -> i32,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
                bool,
                crate::__pgrx_c_bindings::Datum,
                bool,
                *mut crate::__pgrx_c_bindings::SortSupportData,
            ) -> i32,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__unsigned__long__0c__0x5f426f6f6c__0c__unsigned__long__0c__0x5f426f6f6c__0c__struct__SortSupportData__0p__0r;
impl c::sealed::Sealed for Signature_int__0l__unsigned__long__0c__0x5f426f6f6c__0c__unsigned__long__0c__0x5f426f6f6c__0c__struct__SortSupportData__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_int__0l__unsigned__long__0c__0x5f426f6f6c__0c__unsigned__long__0c__0x5f426f6f6c__0c__struct__SortSupportData__0p__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Datum, bool, crate::__pgrx_c_bindings::Datum, bool, *mut crate::__pgrx_c_bindings::SortSupportData) -> i32>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_void,
            ) -> i32,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_void,
            ) -> i32,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__void__0p__0c__const__void__0p__0r;
impl c::sealed::Sealed for Signature_int__0l__void__0p__0c__const__void__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_int__0l__void__0p__0c__const__void__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_void,
            ) -> i32,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                i32,
            ) -> i32,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                i32,
            ) -> i32,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__void__0p__0c__const__void__0p__0c__int__0r;
impl c::sealed::Sealed for Signature_int__0l__void__0p__0c__const__void__0p__0c__int__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_int__0l__void__0p__0c__const__void__0p__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                i32,
            ) -> i32,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn(*mut u8) -> i32>>()
        == 8
);
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn(*mut u8) -> i32>>()
        == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__volatile__unsigned__char__0p__0r;
impl c::sealed::Sealed for Signature_int__0l__volatile__unsigned__char__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_int__0l__volatile__unsigned__char__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(*mut u8) -> i32>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::IndexScanDescData,
                *mut crate::__pgrx_c_bindings::TIDBitmap,
            ) -> i64,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::IndexScanDescData,
                *mut crate::__pgrx_c_bindings::TIDBitmap,
            ) -> i64,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_long__0l__struct__IndexScanDescData__0p__0c__struct__TIDBitmap__0p__0r;
impl c::sealed::Sealed
    for Signature_long__0l__struct__IndexScanDescData__0p__0c__struct__TIDBitmap__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_long__0l__struct__IndexScanDescData__0p__0c__struct__TIDBitmap__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::IndexScanDescData,
                *mut crate::__pgrx_c_bindings::TIDBitmap,
            ) -> i64,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const i8, *mut *mut i8, i32) -> i64>,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const i8, *mut *mut i8, i32) -> i64>,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_long__0l__const__char__0p__0c__char__0p__0p__0c__int__0r;
impl c::sealed::Sealed for Signature_long__0l__const__char__0p__0c__char__0p__0p__0c__int__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_long__0l__const__char__0p__0c__char__0p__0p__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const i8, *mut *mut i8, i32) -> i64>,
    >;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ItemPointerData) -> i64,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ItemPointerData) -> i64,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_long__0l__struct__ItemPointerData__0p__0r;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::sealed::Sealed for Signature_long__0l__struct__ItemPointerData__0p__0r {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::NativeFunctionSignature
    for Signature_long__0l__struct__ItemPointerData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ItemPointerData) -> i64,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            )
                -> *mut crate::__pgrx_c_bindings::ExpandedArrayHeader,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            )
                -> *mut crate::__pgrx_c_bindings::ExpandedArrayHeader,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__ExpandedArrayHeader__0p__0l__unsigned__long__0r;
impl c::sealed::Sealed for Signature_struct__ExpandedArrayHeader__0p__0l__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_struct__ExpandedArrayHeader__0p__0l__unsigned__long__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            )
                -> *mut crate::__pgrx_c_bindings::ExpandedArrayHeader,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
                *mut crate::__pgrx_c_bindings::ArrayMetaState,
            )
                -> *mut crate::__pgrx_c_bindings::ExpandedArrayHeader,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
                *mut crate::__pgrx_c_bindings::ArrayMetaState,
            )
                -> *mut crate::__pgrx_c_bindings::ExpandedArrayHeader,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__ExpandedArrayHeader__0p__0l__unsigned__long__0c__struct__ArrayMetaState__0p__0r;
impl c::sealed::Sealed for Signature_struct__ExpandedArrayHeader__0p__0l__unsigned__long__0c__struct__ArrayMetaState__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_struct__ExpandedArrayHeader__0p__0l__unsigned__long__0c__struct__ArrayMetaState__0p__0r {
 type Physical = PhysicalFunction_C_unwind_2<::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Datum, *mut crate::__pgrx_c_bindings::ArrayMetaState) -> *mut crate::__pgrx_c_bindings::ExpandedArrayHeader>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            )
                -> *mut crate::__pgrx_c_bindings::ExpandedRecordHeader,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            )
                -> *mut crate::__pgrx_c_bindings::ExpandedRecordHeader,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__ExpandedRecordHeader__0p__0l__unsigned__long__0r;
impl c::sealed::Sealed for Signature_struct__ExpandedRecordHeader__0p__0l__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_struct__ExpandedRecordHeader__0p__0l__unsigned__long__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            )
                -> *mut crate::__pgrx_c_bindings::ExpandedRecordHeader,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_void,
            ) -> *mut crate::__pgrx_c_bindings::Expr,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_void,
            ) -> *mut crate::__pgrx_c_bindings::Expr,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__Expr__0p__0l__const__void__0p__0r;
impl c::sealed::Sealed for Signature_struct__Expr__0p__0l__const__void__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_struct__Expr__0p__0l__const__void__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_void,
            ) -> *mut crate::__pgrx_c_bindings::Expr,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::EState,
            ) -> *mut crate::__pgrx_c_bindings::ExprContext,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::EState,
            ) -> *mut crate::__pgrx_c_bindings::ExprContext,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__ExprContext__0p__0l__struct__EState__0p__0r;
impl c::sealed::Sealed for Signature_struct__ExprContext__0p__0l__struct__EState__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_struct__ExprContext__0p__0l__struct__EState__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::EState,
            ) -> *mut crate::__pgrx_c_bindings::ExprContext,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::List,
                *const crate::__pgrx_c_bindings::ListCell,
                *const crate::__pgrx_c_bindings::List,
                *const crate::__pgrx_c_bindings::ListCell,
            ) -> ::core::mem::MaybeUninit<
                crate::__pgrx_c_bindings::ForBothCellState,
            >,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::List,
                *const crate::__pgrx_c_bindings::ListCell,
                *const crate::__pgrx_c_bindings::List,
                *const crate::__pgrx_c_bindings::ListCell,
            ) -> ::core::mem::MaybeUninit<
                crate::__pgrx_c_bindings::ForBothCellState,
            >,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__ForBothCellState__0l__const__struct__List__0p__0c__const__union__ListCell__0p__0c__const__struct__List__0p__0c__const__union__ListCell__0p__0r;
impl c::sealed::Sealed for Signature_struct__ForBothCellState__0l__const__struct__List__0p__0c__const__union__ListCell__0p__0c__const__struct__List__0p__0c__const__union__ListCell__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_struct__ForBothCellState__0l__const__struct__List__0p__0c__const__union__ListCell__0p__0c__const__struct__List__0p__0c__const__union__ListCell__0p__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::List, *const crate::__pgrx_c_bindings::ListCell, *const crate::__pgrx_c_bindings::List, *const crate::__pgrx_c_bindings::ListCell) -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ForBothCellState>>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::List,
                *const crate::__pgrx_c_bindings::ListCell,
            ) -> ::core::mem::MaybeUninit<
                crate::__pgrx_c_bindings::ForEachState,
            >,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::List,
                *const crate::__pgrx_c_bindings::ListCell,
            ) -> ::core::mem::MaybeUninit<
                crate::__pgrx_c_bindings::ForEachState,
            >,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__ForEachState__0l__const__struct__List__0p__0c__const__union__ListCell__0p__0r;
impl c::sealed::Sealed for Signature_struct__ForEachState__0l__const__struct__List__0p__0c__const__union__ListCell__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_struct__ForEachState__0l__const__struct__List__0p__0c__const__union__ListCell__0p__0r {
 type Physical = PhysicalFunction_C_unwind_2<::core::option::Option<unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::List, *const crate::__pgrx_c_bindings::ListCell) -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ForEachState>>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::List,
                i32,
            ) -> ::core::mem::MaybeUninit<
                crate::__pgrx_c_bindings::ForEachState,
            >,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::List,
                i32,
            ) -> ::core::mem::MaybeUninit<
                crate::__pgrx_c_bindings::ForEachState,
            >,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__ForEachState__0l__const__struct__List__0p__0c__int__0r;
impl c::sealed::Sealed
    for Signature_struct__ForEachState__0l__const__struct__List__0p__0c__int__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_struct__ForEachState__0l__const__struct__List__0p__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::List,
                i32,
            ) -> ::core::mem::MaybeUninit<
                crate::__pgrx_c_bindings::ForEachState,
            >,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerInfo,
                *mut crate::__pgrx_c_bindings::RelOptInfo,
                crate::__pgrx_c_bindings::Oid,
                *mut crate::__pgrx_c_bindings::ForeignPath,
                *mut crate::__pgrx_c_bindings::List,
                *mut crate::__pgrx_c_bindings::List,
                *mut crate::__pgrx_c_bindings::Plan,
            ) -> *mut crate::__pgrx_c_bindings::ForeignScan,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerInfo,
                *mut crate::__pgrx_c_bindings::RelOptInfo,
                crate::__pgrx_c_bindings::Oid,
                *mut crate::__pgrx_c_bindings::ForeignPath,
                *mut crate::__pgrx_c_bindings::List,
                *mut crate::__pgrx_c_bindings::List,
                *mut crate::__pgrx_c_bindings::Plan,
            ) -> *mut crate::__pgrx_c_bindings::ForeignScan,
        >,
    >() == 8
);
#[doc(hidden)]
pub struct PhysicalFunction_C_unwind_7<P>(::core::marker::PhantomData<P>);
impl<P> c::sealed::Sealed for PhysicalFunction_C_unwind_7<P> {}
impl<A0, A1, A2, A3, A4, A5, A6, R> c::expression::PhysicalFunctionPointer
    for PhysicalFunction_C_unwind_7<
        ::core::option::Option<unsafe extern "C-unwind" fn(A0, A1, A2, A3, A4, A5, A6) -> R>,
    >
{
    type Pointer =
        ::core::option::Option<unsafe extern "C-unwind" fn(A0, A1, A2, A3, A4, A5, A6) -> R>;
    fn null() -> Self::Pointer {
        None
    }
    fn address(pointer: Self::Pointer) -> *const () {
        pointer.map_or(::core::ptr::null(), |function| function as *const ())
    }
}
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__ForeignScan__0p__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__unsigned__int__0c__struct__ForeignPath__0p__0c__struct__List__0p__0c__struct__List__0p__0c__struct__Plan__0p__0r;
impl c::sealed::Sealed for Signature_struct__ForeignScan__0p__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__unsigned__int__0c__struct__ForeignPath__0p__0c__struct__List__0p__0c__struct__List__0p__0c__struct__Plan__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_struct__ForeignScan__0p__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__unsigned__int__0c__struct__ForeignPath__0p__0c__struct__List__0p__0c__struct__List__0p__0c__struct__Plan__0p__0r {
 type Physical = PhysicalFunction_C_unwind_7<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PlannerInfo, *mut crate::__pgrx_c_bindings::RelOptInfo, crate::__pgrx_c_bindings::Oid, *mut crate::__pgrx_c_bindings::ForeignPath, *mut crate::__pgrx_c_bindings::List, *mut crate::__pgrx_c_bindings::List, *mut crate::__pgrx_c_bindings::Plan) -> *mut crate::__pgrx_c_bindings::ForeignScan>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_char,
            ) -> ::core::mem::MaybeUninit<
                crate::__pgrx_c_bindings::FullTransactionId,
            >,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_char,
            ) -> ::core::mem::MaybeUninit<
                crate::__pgrx_c_bindings::FullTransactionId,
            >,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__FullTransactionId__0l__char__0p__0r;
impl c::sealed::Sealed for Signature_struct__FullTransactionId__0l__char__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_struct__FullTransactionId__0l__char__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_char,
            ) -> ::core::mem::MaybeUninit<
                crate::__pgrx_c_bindings::FullTransactionId,
            >,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>,
            ) -> ::core::mem::MaybeUninit<
                crate::__pgrx_c_bindings::FullTransactionId,
            >,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>,
            ) -> ::core::mem::MaybeUninit<
                crate::__pgrx_c_bindings::FullTransactionId,
            >,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__FullTransactionId__0l__struct__FullTransactionId__0c__struct__FullTransactionId__0r;
impl c::sealed::Sealed for Signature_struct__FullTransactionId__0l__struct__FullTransactionId__0c__struct__FullTransactionId__0r {}
impl c::expression::NativeFunctionSignature for Signature_struct__FullTransactionId__0l__struct__FullTransactionId__0c__struct__FullTransactionId__0r {
 type Physical = PhysicalFunction_C_unwind_2<::core::option::Option<unsafe extern "C-unwind" fn(::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>, ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>) -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                crate::__pgrx_c_bindings::TransactionId,
            ) -> ::core::mem::MaybeUninit<
                crate::__pgrx_c_bindings::FullTransactionId,
            >,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                crate::__pgrx_c_bindings::TransactionId,
            ) -> ::core::mem::MaybeUninit<
                crate::__pgrx_c_bindings::FullTransactionId,
            >,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__FullTransactionId__0l__unsigned__int__0c__unsigned__int__0r;
impl c::sealed::Sealed
    for Signature_struct__FullTransactionId__0l__unsigned__int__0c__unsigned__int__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_struct__FullTransactionId__0l__unsigned__int__0c__unsigned__int__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                crate::__pgrx_c_bindings::TransactionId,
            ) -> ::core::mem::MaybeUninit<
                crate::__pgrx_c_bindings::FullTransactionId,
            >,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u64,
            ) -> ::core::mem::MaybeUninit<
                crate::__pgrx_c_bindings::FullTransactionId,
            >,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u64,
            ) -> ::core::mem::MaybeUninit<
                crate::__pgrx_c_bindings::FullTransactionId,
            >,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__FullTransactionId__0l__unsigned__long__0r;
impl c::sealed::Sealed for Signature_struct__FullTransactionId__0l__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_struct__FullTransactionId__0l__unsigned__long__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u64,
            ) -> ::core::mem::MaybeUninit<
                crate::__pgrx_c_bindings::FullTransactionId,
            >,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::FunctionCallInfoBaseData,
            )
                -> *mut crate::__pgrx_c_bindings::FuncCallContext,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::FunctionCallInfoBaseData,
            )
                -> *mut crate::__pgrx_c_bindings::FuncCallContext,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__FuncCallContext__0p__0l__struct__FunctionCallInfoBaseData__0p__0r;
impl c::sealed::Sealed
    for Signature_struct__FuncCallContext__0p__0l__struct__FunctionCallInfoBaseData__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_struct__FuncCallContext__0p__0l__struct__FunctionCallInfoBaseData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::FunctionCallInfoBaseData,
            )
                -> *mut crate::__pgrx_c_bindings::FuncCallContext,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
            )
                -> *mut crate::__pgrx_c_bindings::HeapTupleData,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
            )
                -> *mut crate::__pgrx_c_bindings::HeapTupleData,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__HeapTupleData__0p__0l__struct__TupleTableSlot__0p__0r;
impl c::sealed::Sealed for Signature_struct__HeapTupleData__0p__0l__struct__TupleTableSlot__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_struct__HeapTupleData__0p__0l__struct__TupleTableSlot__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
            )
                -> *mut crate::__pgrx_c_bindings::HeapTupleData,
        >,
    >;
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
            )
                -> *mut crate::__pgrx_c_bindings::HeapTupleData,
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
            )
                -> *mut crate::__pgrx_c_bindings::HeapTupleData,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__HeapTupleData__0p__0l__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::sealed::Sealed for Signature_struct__HeapTupleData__0p__0l__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::NativeFunctionSignature for Signature_struct__HeapTupleData__0p__0l__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(i32, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum) -> *mut crate::__pgrx_c_bindings::HeapTupleData>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::IndexInfo,
            )
                -> *mut crate::__pgrx_c_bindings::IndexBuildResult,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::IndexInfo,
            )
                -> *mut crate::__pgrx_c_bindings::IndexBuildResult,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__IndexBuildResult__0p__0l__struct__RelationData__0p__0c__struct__RelationData__0p__0c__struct__IndexInfo__0p__0r;
impl c::sealed::Sealed for Signature_struct__IndexBuildResult__0p__0l__struct__RelationData__0p__0c__struct__RelationData__0p__0c__struct__IndexInfo__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_struct__IndexBuildResult__0p__0l__struct__RelationData__0p__0c__struct__RelationData__0p__0c__struct__IndexInfo__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::IndexInfo) -> *mut crate::__pgrx_c_bindings::IndexBuildResult>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::IndexVacuumInfo,
                *mut crate::__pgrx_c_bindings::IndexBulkDeleteResult,
            )
                -> *mut crate::__pgrx_c_bindings::IndexBulkDeleteResult,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::IndexVacuumInfo,
                *mut crate::__pgrx_c_bindings::IndexBulkDeleteResult,
            )
                -> *mut crate::__pgrx_c_bindings::IndexBulkDeleteResult,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__IndexBulkDeleteResult__0p__0l__struct__IndexVacuumInfo__0p__0c__struct__IndexBulkDeleteResult__0p__0r;
impl c::sealed::Sealed for Signature_struct__IndexBulkDeleteResult__0p__0l__struct__IndexVacuumInfo__0p__0c__struct__IndexBulkDeleteResult__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_struct__IndexBulkDeleteResult__0p__0l__struct__IndexVacuumInfo__0p__0c__struct__IndexBulkDeleteResult__0p__0r {
 type Physical = PhysicalFunction_C_unwind_2<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::IndexVacuumInfo, *mut crate::__pgrx_c_bindings::IndexBulkDeleteResult) -> *mut crate::__pgrx_c_bindings::IndexBulkDeleteResult>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::IndexVacuumInfo,
                *mut crate::__pgrx_c_bindings::IndexBulkDeleteResult,
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::ItemPointerData,
                        *mut ::core::ffi::c_void,
                    ) -> bool,
                >,
                *mut ::core::ffi::c_void,
            )
                -> *mut crate::__pgrx_c_bindings::IndexBulkDeleteResult,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::IndexVacuumInfo,
                *mut crate::__pgrx_c_bindings::IndexBulkDeleteResult,
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::ItemPointerData,
                        *mut ::core::ffi::c_void,
                    ) -> bool,
                >,
                *mut ::core::ffi::c_void,
            )
                -> *mut crate::__pgrx_c_bindings::IndexBulkDeleteResult,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__IndexBulkDeleteResult__0p__0l__struct__IndexVacuumInfo__0p__0c__struct__IndexBulkDeleteResult__0p__0c__0x5f426f6f6c__0l__0p__0r__0l__struct__ItemPointerData__0p__0c__void__0p__0r__0c__void__0p__0r;
impl c::sealed::Sealed for Signature_struct__IndexBulkDeleteResult__0p__0l__struct__IndexVacuumInfo__0p__0c__struct__IndexBulkDeleteResult__0p__0c__0x5f426f6f6c__0l__0p__0r__0l__struct__ItemPointerData__0p__0c__void__0p__0r__0c__void__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_struct__IndexBulkDeleteResult__0p__0l__struct__IndexVacuumInfo__0p__0c__struct__IndexBulkDeleteResult__0p__0c__0x5f426f6f6c__0l__0p__0r__0l__struct__ItemPointerData__0p__0c__void__0p__0r__0c__void__0p__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::IndexVacuumInfo, *mut crate::__pgrx_c_bindings::IndexBulkDeleteResult, ::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ItemPointerData, *mut ::core::ffi::c_void) -> bool>, *mut ::core::ffi::c_void) -> *mut crate::__pgrx_c_bindings::IndexBulkDeleteResult>>;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
            )
                -> *mut crate::__pgrx_c_bindings::IndexFetchTableData,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
            )
                -> *mut crate::__pgrx_c_bindings::IndexFetchTableData,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__IndexFetchTableData__0p__0l__struct__RelationData__0p__0r;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::sealed::Sealed
    for Signature_struct__IndexFetchTableData__0p__0l__struct__RelationData__0p__0r
{
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::NativeFunctionSignature
    for Signature_struct__IndexFetchTableData__0p__0l__struct__RelationData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
            )
                -> *mut crate::__pgrx_c_bindings::IndexFetchTableData,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                i32,
                i32,
            )
                -> *mut crate::__pgrx_c_bindings::IndexScanDescData,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                i32,
                i32,
            )
                -> *mut crate::__pgrx_c_bindings::IndexScanDescData,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__IndexScanDescData__0p__0l__struct__RelationData__0p__0c__int__0c__int__0r;
impl c::sealed::Sealed
    for Signature_struct__IndexScanDescData__0p__0l__struct__RelationData__0p__0c__int__0c__int__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_struct__IndexScanDescData__0p__0l__struct__RelationData__0p__0c__int__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                i32,
                i32,
            )
                -> *mut crate::__pgrx_c_bindings::IndexScanDescData,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::IndexTupleData,
            )
                -> *mut crate::__pgrx_c_bindings::ItemPointerData,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::IndexTupleData,
            )
                -> *mut crate::__pgrx_c_bindings::ItemPointerData,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__ItemPointerData__0p__0l__struct__IndexTupleData__0p__0r;
impl c::sealed::Sealed
    for Signature_struct__ItemPointerData__0p__0l__struct__IndexTupleData__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_struct__ItemPointerData__0p__0l__struct__IndexTupleData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::IndexTupleData,
            )
                -> *mut crate::__pgrx_c_bindings::ItemPointerData,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::IndexTupleData,
                i32,
            )
                -> *mut crate::__pgrx_c_bindings::ItemPointerData,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::IndexTupleData,
                i32,
            )
                -> *mut crate::__pgrx_c_bindings::ItemPointerData,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__ItemPointerData__0p__0l__struct__IndexTupleData__0p__0c__int__0r;
impl c::sealed::Sealed
    for Signature_struct__ItemPointerData__0p__0l__struct__IndexTupleData__0p__0c__int__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_struct__ItemPointerData__0p__0l__struct__IndexTupleData__0p__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::IndexTupleData,
                i32,
            )
                -> *mut crate::__pgrx_c_bindings::ItemPointerData,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                *mut crate::__pgrx_c_bindings::RelationData,
            ) -> *mut crate::__pgrx_c_bindings::List,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                *mut crate::__pgrx_c_bindings::RelationData,
            ) -> *mut crate::__pgrx_c_bindings::List,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__List__0p__0l__enum__CmdType__0c__struct__RelationData__0p__0r;
impl c::sealed::Sealed
    for Signature_struct__List__0p__0l__enum__CmdType__0c__struct__RelationData__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_struct__List__0p__0l__enum__CmdType__0c__struct__RelationData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                *mut crate::__pgrx_c_bindings::RelationData,
            ) -> *mut crate::__pgrx_c_bindings::List,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ImportForeignSchemaStmt,
                crate::__pgrx_c_bindings::Oid,
            ) -> *mut crate::__pgrx_c_bindings::List,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ImportForeignSchemaStmt,
                crate::__pgrx_c_bindings::Oid,
            ) -> *mut crate::__pgrx_c_bindings::List,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__List__0p__0l__struct__ImportForeignSchemaStmt__0p__0c__unsigned__int__0r;
impl c::sealed::Sealed
    for Signature_struct__List__0p__0l__struct__ImportForeignSchemaStmt__0p__0c__unsigned__int__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_struct__List__0p__0l__struct__ImportForeignSchemaStmt__0p__0c__unsigned__int__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ImportForeignSchemaStmt,
                crate::__pgrx_c_bindings::Oid,
            ) -> *mut crate::__pgrx_c_bindings::List,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerInfo,
                *mut crate::__pgrx_c_bindings::List,
                *mut crate::__pgrx_c_bindings::RelOptInfo,
            ) -> *mut crate::__pgrx_c_bindings::List,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerInfo,
                *mut crate::__pgrx_c_bindings::List,
                *mut crate::__pgrx_c_bindings::RelOptInfo,
            ) -> *mut crate::__pgrx_c_bindings::List,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__List__0p__0l__struct__PlannerInfo__0p__0c__struct__List__0p__0c__struct__RelOptInfo__0p__0r;
impl c::sealed::Sealed for Signature_struct__List__0p__0l__struct__PlannerInfo__0p__0c__struct__List__0p__0c__struct__RelOptInfo__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_struct__List__0p__0l__struct__PlannerInfo__0p__0c__struct__List__0p__0c__struct__RelOptInfo__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PlannerInfo, *mut crate::__pgrx_c_bindings::List, *mut crate::__pgrx_c_bindings::RelOptInfo) -> *mut crate::__pgrx_c_bindings::List>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerInfo,
                *mut crate::__pgrx_c_bindings::ModifyTable,
                u32,
                i32,
            ) -> *mut crate::__pgrx_c_bindings::List,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerInfo,
                *mut crate::__pgrx_c_bindings::ModifyTable,
                u32,
                i32,
            ) -> *mut crate::__pgrx_c_bindings::List,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__List__0p__0l__struct__PlannerInfo__0p__0c__struct__ModifyTable__0p__0c__unsigned__int__0c__int__0r;
impl c::sealed::Sealed for Signature_struct__List__0p__0l__struct__PlannerInfo__0p__0c__struct__ModifyTable__0p__0c__unsigned__int__0c__int__0r {}
impl c::expression::NativeFunctionSignature for Signature_struct__List__0p__0l__struct__PlannerInfo__0p__0c__struct__ModifyTable__0p__0c__unsigned__int__0c__int__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PlannerInfo, *mut crate::__pgrx_c_bindings::ModifyTable, u32, i32) -> *mut crate::__pgrx_c_bindings::List>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::MemoryContextData,
            )
                -> *mut crate::__pgrx_c_bindings::MemoryContextData,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::MemoryContextData,
            )
                -> *mut crate::__pgrx_c_bindings::MemoryContextData,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__MemoryContextData__0p__0l__struct__MemoryContextData__0p__0r;
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
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_void,
            ) -> *mut crate::__pgrx_c_bindings::Node,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_void,
            ) -> *mut crate::__pgrx_c_bindings::Node,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__Node__0p__0l__const__void__0p__0r;
impl c::sealed::Sealed for Signature_struct__Node__0p__0l__const__void__0p__0r {}
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
                u32,
                *mut crate::__pgrx_c_bindings::EState,
            )
                -> *mut crate::__pgrx_c_bindings::RangeTblEntry,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                *mut crate::__pgrx_c_bindings::EState,
            )
                -> *mut crate::__pgrx_c_bindings::RangeTblEntry,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__RangeTblEntry__0p__0l__unsigned__int__0c__struct__EState__0p__0r;
impl c::sealed::Sealed
    for Signature_struct__RangeTblEntry__0p__0l__unsigned__int__0c__struct__EState__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_struct__RangeTblEntry__0p__0l__unsigned__int__0c__struct__EState__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                *mut crate::__pgrx_c_bindings::EState,
            )
                -> *mut crate::__pgrx_c_bindings::RangeTblEntry,
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
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0r;
impl c::sealed::Sealed
    for Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0r
{
}
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
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0r;
impl c::sealed::Sealed for Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0r {
 type Physical = PhysicalFunction_C_unwind_2<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::SnapshotData) -> *mut crate::__pgrx_c_bindings::TableScanDescData>>;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
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
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
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
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0c__int__0c__struct__ScanKeyData__0p__0r;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::sealed::Sealed for Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0c__int__0c__struct__ScanKeyData__0p__0r {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::NativeFunctionSignature for Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0c__int__0c__struct__ScanKeyData__0p__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::SnapshotData, i32, *mut crate::__pgrx_c_bindings::ScanKeyData) -> *mut crate::__pgrx_c_bindings::TableScanDescData>>;
}
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
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0c__int__0c__struct__ScanKeyData__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0r;
impl c::sealed::Sealed for Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0c__int__0c__struct__ScanKeyData__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0r {}
impl c::expression::NativeFunctionSignature for Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0c__int__0c__struct__ScanKeyData__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0r {
 type Physical = PhysicalFunction_C_unwind_6<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::SnapshotData, i32, *mut crate::__pgrx_c_bindings::ScanKeyData, bool, bool) -> *mut crate::__pgrx_c_bindings::TableScanDescData>>;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
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
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
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
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0c__int__0c__struct__ScanKeyData__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0r;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::sealed::Sealed for Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0c__int__0c__struct__ScanKeyData__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0r {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::NativeFunctionSignature for Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0c__int__0c__struct__ScanKeyData__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0r {
 type Physical = PhysicalFunction_C_unwind_7<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::SnapshotData, i32, *mut crate::__pgrx_c_bindings::ScanKeyData, bool, bool, bool) -> *mut crate::__pgrx_c_bindings::TableScanDescData>>;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
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
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
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
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0c__struct__ItemPointerData__0p__0c__struct__ItemPointerData__0p__0r;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::sealed::Sealed for Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0c__struct__ItemPointerData__0p__0c__struct__ItemPointerData__0p__0r {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
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
