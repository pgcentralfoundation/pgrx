// Shared generated C macro support.

#[cfg(feature = "pg16")]
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<A0: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadOnly>>, A1: c::expression::ImplicitTo<c::CBool>, A2: c::expression::ImplicitTo<c::CBool>, A3: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadOnly>>, A4: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadWrite>, c::expression::ReadWrite>>> c::expression::Call<(A0,A1,A2,A3,A4,)> for Signature_struct__WalReceiverConn__0p__0l__const__char__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__const__char__0p__0c__char__0p__0p__0r {
 type Output = <c::expression::CPointer<c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>, c::expression::ReadWrite> as c::expression::CType>::Value;
 unsafe fn call(pointer: Self::Pointer, args: (A0,A1,A2,A3,A4,)) -> Self::Output {
let function = pointer.expect("C indirect call requires a non-null function pointer");
const { assert!(!::core::mem::needs_drop::<A0>()); }
const _: () = assert!(!::core::mem::needs_drop::<*const ::core::ffi::c_char>());
let native0 = <c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadOnly> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadOnly>, _>(args.0));
const { assert!(!::core::mem::needs_drop::<A1>()); }
const _: () = assert!(!::core::mem::needs_drop::<bool>());
let native1 = <c::CBool as c::expression::CType>::into_storage(c::expression::implicit::<c::CBool, _>(args.1));
const { assert!(!::core::mem::needs_drop::<A2>()); }
const _: () = assert!(!::core::mem::needs_drop::<bool>());
let native2 = <c::CBool as c::expression::CType>::into_storage(c::expression::implicit::<c::CBool, _>(args.2));
const { assert!(!::core::mem::needs_drop::<A3>()); }
const _: () = assert!(!::core::mem::needs_drop::<*const ::core::ffi::c_char>());
let native3 = <c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadOnly> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadOnly>, _>(args.3));
const { assert!(!::core::mem::needs_drop::<A4>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut *mut ::core::ffi::c_char>());
let native4 = <c::expression::CPointer<c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadWrite>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadWrite>, c::expression::ReadWrite>, _>(args.4));
// SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
let result = unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0, native1, native2, native3, native4)) };
<c::expression::CPointer<c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>, c::expression::ReadWrite> as c::expression::CType>::from_storage(result)
}
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::dclist_head,
            ) -> *mut crate::__pgrx_c_bindings::dlist_node,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::dclist_head,
            ) -> *mut crate::__pgrx_c_bindings::dlist_node,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__dlist_node__0p__0l__struct__dclist_head__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_struct__dlist_node__0p__0l__struct__dclist_head__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_struct__dlist_node__0p__0l__struct__dclist_head__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::dclist_head,
            ) -> *mut crate::__pgrx_c_bindings::dlist_node,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::dclist_head,
                *mut crate::__pgrx_c_bindings::dlist_node,
            ) -> *mut crate::__pgrx_c_bindings::dlist_node,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::dclist_head,
                *mut crate::__pgrx_c_bindings::dlist_node,
            ) -> *mut crate::__pgrx_c_bindings::dlist_node,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__dlist_node__0p__0l__struct__dclist_head__0p__0c__struct__dlist_node__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_struct__dlist_node__0p__0l__struct__dclist_head__0p__0c__struct__dlist_node__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_struct__dlist_node__0p__0l__struct__dclist_head__0p__0c__struct__dlist_node__0p__0r {
 type Physical = PhysicalFunction_C_unwind_2<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::dclist_head, *mut crate::__pgrx_c_bindings::dlist_node) -> *mut crate::__pgrx_c_bindings::dlist_node>>;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn() -> ::core::mem::MaybeUninit<
                crate::__pgrx_c_bindings::instr_time,
            >,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn() -> ::core::mem::MaybeUninit<
                crate::__pgrx_c_bindings::instr_time,
            >,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__instr_time__0l__void__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_struct__instr_time__0l__void__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_struct__instr_time__0l__void__0r {
    type Physical = PhysicalFunction_C_unwind_0<
        ::core::option::Option<
            unsafe extern "C-unwind" fn() -> ::core::mem::MaybeUninit<
                crate::__pgrx_c_bindings::instr_time,
            >,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::nameData,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::nameData,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__nameData__0p__0l__unsigned__long__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_struct__nameData__0p__0l__unsigned__long__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_struct__nameData__0p__0l__unsigned__long__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::nameData,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::BlockIdData) -> u32,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::BlockIdData) -> u32,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__int__0l__const__struct__BlockIdData__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_unsigned__int__0l__const__struct__BlockIdData__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__int__0l__const__struct__BlockIdData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::BlockIdData) -> u32,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::ItemPointerData) -> u32,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::ItemPointerData) -> u32,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__int__0l__const__struct__ItemPointerData__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_unsigned__int__0l__const__struct__ItemPointerData__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__int__0l__const__struct__ItemPointerData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::ItemPointerData) -> u32,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::buftag,
            ) -> crate::__pgrx_c_bindings::Oid,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::buftag,
            ) -> crate::__pgrx_c_bindings::Oid,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__int__0l__const__struct__buftag__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_unsigned__int__0l__const__struct__buftag__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__int__0l__const__struct__buftag__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::buftag,
            ) -> crate::__pgrx_c_bindings::Oid,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::dclist_head) -> u32,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::dclist_head) -> u32,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__int__0l__const__struct__dclist_head__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_unsigned__int__0l__const__struct__dclist_head__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__int__0l__const__struct__dclist_head__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::dclist_head) -> u32,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void) -> usize>,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void) -> usize>,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__void__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_unsigned__long__0l__void__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__void__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void) -> usize>,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::PageXLogRecPtr>,
            ) -> u64,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::PageXLogRecPtr>,
            ) -> u64,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__PageXLogRecPtr__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::sealed::Sealed for Signature_unsigned__long__0l__PageXLogRecPtr__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__PageXLogRecPtr__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::PageXLogRecPtr>,
            ) -> u64,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(bool) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(bool) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__0x5f426f6f6c__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_unsigned__long__0l__0x5f426f6f6c__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__0x5f426f6f6c__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(bool) -> crate::__pgrx_c_bindings::Datum,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(i8) -> crate::__pgrx_c_bindings::Datum>,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(i8) -> crate::__pgrx_c_bindings::Datum>,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__char__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_unsigned__long__0l__char__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__char__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(i8) -> crate::__pgrx_c_bindings::Datum>,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::BOX,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::BOX,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__const__BOX__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_unsigned__long__0l__const__BOX__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__const__BOX__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::BOX,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::CIRCLE,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::CIRCLE,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__const__CIRCLE__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_unsigned__long__0l__const__CIRCLE__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__long__0l__const__CIRCLE__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::CIRCLE,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::Interval,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::Interval,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__const__Interval__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_unsigned__long__0l__const__Interval__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__long__0l__const__Interval__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::Interval,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::LINE,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::LINE,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__const__LINE__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_unsigned__long__0l__const__LINE__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__const__LINE__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::LINE,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::LSEG,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::LSEG,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__const__LSEG__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_unsigned__long__0l__const__LSEG__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__const__LSEG__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::LSEG,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::PATH,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::PATH,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__const__PATH__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_unsigned__long__0l__const__PATH__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__const__PATH__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::PATH,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::POLYGON,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::POLYGON,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__const__POLYGON__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_unsigned__long__0l__const__POLYGON__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__long__0l__const__POLYGON__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::POLYGON,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::Point,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::Point,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__const__Point__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_unsigned__long__0l__const__Point__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__const__Point__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::Point,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::RangeType,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::RangeType,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__const__RangeType__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_unsigned__long__0l__const__RangeType__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__long__0l__const__RangeType__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::RangeType,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::TSQueryData,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::TSQueryData,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__const__TSQueryData__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_unsigned__long__0l__const__TSQueryData__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__long__0l__const__TSQueryData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::TSQueryData,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::TSVectorData,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::TSVectorData,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__const__TSVectorData__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_unsigned__long__0l__const__TSVectorData__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__long__0l__const__TSVectorData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::TSVectorData,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::TimeTzADT,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::TimeTzADT,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__const__TimeTzADT__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_unsigned__long__0l__const__TimeTzADT__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__long__0l__const__TimeTzADT__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::TimeTzADT,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::ExpandedObjectHeader,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::ExpandedObjectHeader,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__const__struct__ExpandedObjectHeader__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed
    for Signature_unsigned__long__0l__const__struct__ExpandedObjectHeader__0p__0r
{
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__long__0l__const__struct__ExpandedObjectHeader__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::ExpandedObjectHeader,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::ExpandedRecordHeader,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::ExpandedRecordHeader,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__const__struct__ExpandedRecordHeader__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed
    for Signature_unsigned__long__0l__const__struct__ExpandedRecordHeader__0p__0r
{
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__long__0l__const__struct__ExpandedRecordHeader__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::ExpandedRecordHeader,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::HeapTupleData,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::HeapTupleData,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__const__struct__HeapTupleData__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_unsigned__long__0l__const__struct__HeapTupleData__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__long__0l__const__struct__HeapTupleData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::HeapTupleData,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::ItemPointerData,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::ItemPointerData,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__const__struct__ItemPointerData__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_unsigned__long__0l__const__struct__ItemPointerData__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__long__0l__const__struct__ItemPointerData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::ItemPointerData,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::nameData,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::nameData,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__const__struct__nameData__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_unsigned__long__0l__const__struct__nameData__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__long__0l__const__struct__nameData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::nameData,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_void,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_void,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__const__void__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_unsigned__long__0l__const__void__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__const__void__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_void,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_void,
                bool,
                i32,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_void,
                bool,
                i32,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__const__void__0p__0c__0x5f426f6f6c__0c__int__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed
    for Signature_unsigned__long__0l__const__void__0p__0c__0x5f426f6f6c__0c__int__0r
{
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__long__0l__const__void__0p__0c__0x5f426f6f6c__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_void,
                bool,
                i32,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(i64) -> crate::__pgrx_c_bindings::Datum>,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(i64) -> crate::__pgrx_c_bindings::Datum>,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__long__0r;
#[cfg(any(feature = "pg16", feature = "pg17"))]
impl c::sealed::Sealed for Signature_unsigned__long__0l__long__0r {}
#[cfg(any(feature = "pg16", feature = "pg17"))]
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__long__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(i64) -> crate::__pgrx_c_bindings::Datum>,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(i16) -> crate::__pgrx_c_bindings::Datum>,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(i16) -> crate::__pgrx_c_bindings::Datum>,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__short__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_unsigned__long__0l__short__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__short__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(i16) -> crate::__pgrx_c_bindings::Datum>,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::IndexTupleData,
                i32,
                *mut crate::__pgrx_c_bindings::TupleDescData,
                *mut bool,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::IndexTupleData,
                i32,
                *mut crate::__pgrx_c_bindings::TupleDescData,
                *mut bool,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__struct__IndexTupleData__0p__0c__int__0c__struct__TupleDescData__0p__0c__0x5f426f6f6c__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_unsigned__long__0l__struct__IndexTupleData__0p__0c__int__0c__struct__TupleDescData__0p__0c__0x5f426f6f6c__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__struct__IndexTupleData__0p__0c__int__0c__struct__TupleDescData__0p__0c__0x5f426f6f6c__0p__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::IndexTupleData, i32, *mut crate::__pgrx_c_bindings::TupleDescData, *mut bool) -> crate::__pgrx_c_bindings::Datum>>;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::NumericData,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::NumericData,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__struct__NumericData__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_unsigned__long__0l__struct__NumericData__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__long__0l__struct__NumericData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::NumericData,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::ItemPointerData) -> u16,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::ItemPointerData) -> u16,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__short__0l__const__struct__ItemPointerData__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_unsigned__short__0l__const__struct__ItemPointerData__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__short__0l__const__struct__ItemPointerData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::ItemPointerData) -> u16,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Datum) -> u16>,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Datum) -> u16>,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__short__0l__unsigned__long__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_unsigned__short__0l__unsigned__long__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_unsigned__short__0l__unsigned__long__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Datum) -> u16>,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_char, bool) -> ()>,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_char, bool) -> ()>,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__const__char__0p__0c__0x5f426f6f6c__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_void__0l__const__char__0p__0c__0x5f426f6f6c__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__const__char__0p__0c__0x5f426f6f6c__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_char, bool) -> ()>,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<
    A0: c::expression::ImplicitTo<
            c::expression::CPointer<
                c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
                c::expression::ReadOnly,
            >,
        >,
    A1: c::expression::ImplicitTo<c::CBool>,
> c::expression::Call<(A0, A1)> for Signature_void__0l__const__char__0p__0c__0x5f426f6f6c__0r
{
    type Output = ();
    unsafe fn call(pointer: Self::Pointer, args: (A0, A1)) -> Self::Output {
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
        const {
            assert!(!::core::mem::needs_drop::<A1>());
        }
        const _: () = assert!(!::core::mem::needs_drop::<bool>());
        let native1 = <c::CBool as c::expression::CType>::into_storage(c::expression::implicit::<
            c::CBool,
            _,
        >(args.1));
        // SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
        let result =
            unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0, native1)) };
        result
    }
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *const crate::__pgrx_c_bindings::RelFileLocator,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *const crate::__pgrx_c_bindings::RelFileLocator,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__RelationData__0p__0c__const__struct__RelFileLocator__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed
    for Signature_void__0l__struct__RelationData__0p__0c__const__struct__RelFileLocator__0p__0r
{
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__RelationData__0p__0c__const__struct__RelFileLocator__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *const crate::__pgrx_c_bindings::RelFileLocator,
            ) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::size_of::<*const crate::__pgrx_c_bindings::RelFileLocator>() == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::align_of::<*const crate::__pgrx_c_bindings::RelFileLocator>() == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
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
                c::expression::CRecord<crate::__pgrx_c_bindings::RelFileLocator>,
                c::expression::ReadOnly,
            >,
        >,
> c::expression::Call<(A0, A1)>
    for Signature_void__0l__struct__RelationData__0p__0c__const__struct__RelFileLocator__0p__0r
{
    type Output = ();
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
            assert!(!::core::mem::needs_drop::<*const crate::__pgrx_c_bindings::RelFileLocator>());
        let native1 = <c::expression::CPointer<
            c::expression::CRecord<crate::__pgrx_c_bindings::RelFileLocator>,
            c::expression::ReadOnly,
        > as c::expression::CType>::into_storage(c::expression::implicit::<
            c::expression::CPointer<
                c::expression::CRecord<crate::__pgrx_c_bindings::RelFileLocator>,
                c::expression::ReadOnly,
            >,
            _,
        >(args.1));
        // SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
        let result =
            unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0, native1)) };
        result
    }
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *const crate::__pgrx_c_bindings::RelFileLocator,
                ::core::ffi::c_char,
                *mut crate::__pgrx_c_bindings::TransactionId,
                *mut crate::__pgrx_c_bindings::MultiXactId,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *const crate::__pgrx_c_bindings::RelFileLocator,
                ::core::ffi::c_char,
                *mut crate::__pgrx_c_bindings::TransactionId,
                *mut crate::__pgrx_c_bindings::MultiXactId,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__RelationData__0p__0c__const__struct__RelFileLocator__0p__0c__char__0c__unsigned__int__0p__0c__unsigned__int__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_void__0l__struct__RelationData__0p__0c__const__struct__RelFileLocator__0p__0c__char__0c__unsigned__int__0p__0c__unsigned__int__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__RelationData__0p__0c__const__struct__RelFileLocator__0p__0c__char__0c__unsigned__int__0p__0c__unsigned__int__0p__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *const crate::__pgrx_c_bindings::RelFileLocator, ::core::ffi::c_char, *mut crate::__pgrx_c_bindings::TransactionId, *mut crate::__pgrx_c_bindings::MultiXactId) -> ()>>;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::size_of::<*const crate::__pgrx_c_bindings::RelFileLocator>() == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::align_of::<*const crate::__pgrx_c_bindings::RelFileLocator>() == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<A0: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>, c::expression::ReadWrite>>, A1: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::RelFileLocator>, c::expression::ReadOnly>>, A2: c::expression::ImplicitTo<c::CChar>, A3: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>, c::expression::ReadWrite>>, A4: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::MultiXactId>, c::expression::ReadWrite>>> c::expression::Call<(A0,A1,A2,A3,A4,)> for Signature_void__0l__struct__RelationData__0p__0c__const__struct__RelFileLocator__0p__0c__char__0c__unsigned__int__0p__0c__unsigned__int__0p__0r {
 type Output = ();
 unsafe fn call(pointer: Self::Pointer, args: (A0,A1,A2,A3,A4,)) -> Self::Output {
let function = pointer.expect("C indirect call requires a non-null function pointer");
// SAFETY: The caller establishes that this address has the original C prototype, including plain-char signedness. The raw signature uses that compiler-proven char sign and enum-compatible integer ABI; MaybeUninit<R> preserves record layout/ABI. The binding signature is pointer storage only and is not invoked. Calling convention and pointer representation are unchanged; no invalid enum or uninitialized record value is materialized.
let function: unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *const crate::__pgrx_c_bindings::RelFileLocator, i8, *mut crate::__pgrx_c_bindings::TransactionId, *mut crate::__pgrx_c_bindings::MultiXactId) -> () = unsafe { ::core::mem::transmute::<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *const crate::__pgrx_c_bindings::RelFileLocator, ::core::ffi::c_char, *mut crate::__pgrx_c_bindings::TransactionId, *mut crate::__pgrx_c_bindings::MultiXactId) -> (), unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *const crate::__pgrx_c_bindings::RelFileLocator, i8, *mut crate::__pgrx_c_bindings::TransactionId, *mut crate::__pgrx_c_bindings::MultiXactId) -> ()>(function) };
const { assert!(!::core::mem::needs_drop::<A0>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::RelationData>());
let native0 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>, c::expression::ReadWrite>, _>(args.0));
const { assert!(!::core::mem::needs_drop::<A1>()); }
const _: () = assert!(!::core::mem::needs_drop::<*const crate::__pgrx_c_bindings::RelFileLocator>());
let native1 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::RelFileLocator>, c::expression::ReadOnly> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::RelFileLocator>, c::expression::ReadOnly>, _>(args.1));
const { assert!(!::core::mem::needs_drop::<A2>()); }
const _: () = assert!(!::core::mem::needs_drop::<i8>());
let native2 = <c::CChar as c::expression::CType>::into_storage(c::expression::implicit::<c::CChar, _>(args.2));
const { assert!(!::core::mem::needs_drop::<A3>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::TransactionId>());
let native3 = <c::expression::CPointer<c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>, c::expression::ReadWrite>, _>(args.3));
const { assert!(!::core::mem::needs_drop::<A4>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::MultiXactId>());
let native4 = <c::expression::CPointer<c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::MultiXactId>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::MultiXactId>, c::expression::ReadWrite>, _>(args.4));
// SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
let result = unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0, native1, native2, native3, native4)) };
result
}
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::size_of::<*mut crate::__pgrx_c_bindings::ParallelTableScanDescData>() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::align_of::<*mut crate::__pgrx_c_bindings::ParallelTableScanDescData>() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::SnapshotData>() == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::SnapshotData>() == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::StringInfoData,
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::StringInfoData,
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__StringInfoData__0p__0c__void__0p__0c__void__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed
    for Signature_void__0l__struct__StringInfoData__0p__0c__void__0p__0c__void__0p__0r
{
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__StringInfoData__0p__0c__void__0p__0c__void__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::StringInfoData,
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::ItemPointerData>() == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::ItemPointerData>() == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::ItemPointerData>() == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::ItemPointerData>() == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<A0: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TableScanDescData>, c::expression::ReadWrite>>, A1: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::ItemPointerData>, c::expression::ReadWrite>>, A2: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::ItemPointerData>, c::expression::ReadWrite>>> c::expression::Call<(A0,A1,A2,)> for Signature_void__0l__struct__TableScanDescData__0p__0c__struct__ItemPointerData__0p__0c__struct__ItemPointerData__0p__0r {
 type Output = ();
 unsafe fn call(pointer: Self::Pointer, args: (A0,A1,A2,)) -> Self::Output {
let function = pointer.expect("C indirect call requires a non-null function pointer");
const { assert!(!::core::mem::needs_drop::<A0>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::TableScanDescData>());
let native0 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TableScanDescData>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TableScanDescData>, c::expression::ReadWrite>, _>(args.0));
const { assert!(!::core::mem::needs_drop::<A1>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::ItemPointerData>());
let native1 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::ItemPointerData>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::ItemPointerData>, c::expression::ReadWrite>, _>(args.1));
const { assert!(!::core::mem::needs_drop::<A2>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::ItemPointerData>());
let native2 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::ItemPointerData>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::ItemPointerData>, c::expression::ReadWrite>, _>(args.2));
// SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
let result = unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0, native1, native2)) };
result
}
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::ScanKeyData>() == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::ScanKeyData>() == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::Tuplesortstate) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::Tuplesortstate) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__Tuplesortstate__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_void__0l__struct__Tuplesortstate__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__Tuplesortstate__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::Tuplesortstate) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::Tuplesortstate,
                *mut crate::__pgrx_c_bindings::SortTuple,
                i32,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::Tuplesortstate,
                *mut crate::__pgrx_c_bindings::SortTuple,
                i32,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__Tuplesortstate__0p__0c__SortTuple__0p__0c__int__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed
    for Signature_void__0l__struct__Tuplesortstate__0p__0c__SortTuple__0p__0c__int__0r
{
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__Tuplesortstate__0p__0c__SortTuple__0p__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::Tuplesortstate,
                *mut crate::__pgrx_c_bindings::SortTuple,
                i32,
            ) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::Tuplesortstate,
                *mut crate::__pgrx_c_bindings::SortTuple,
                *mut crate::__pgrx_c_bindings::LogicalTape,
                u32,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::Tuplesortstate,
                *mut crate::__pgrx_c_bindings::SortTuple,
                *mut crate::__pgrx_c_bindings::LogicalTape,
                u32,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__Tuplesortstate__0p__0c__SortTuple__0p__0c__struct__LogicalTape__0p__0c__unsigned__int__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_void__0l__struct__Tuplesortstate__0p__0c__SortTuple__0p__0c__struct__LogicalTape__0p__0c__unsigned__int__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__Tuplesortstate__0p__0c__SortTuple__0p__0c__struct__LogicalTape__0p__0c__unsigned__int__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::Tuplesortstate, *mut crate::__pgrx_c_bindings::SortTuple, *mut crate::__pgrx_c_bindings::LogicalTape, u32) -> ()>>;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::Tuplesortstate,
                *mut crate::__pgrx_c_bindings::LogicalTape,
                *mut crate::__pgrx_c_bindings::SortTuple,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::Tuplesortstate,
                *mut crate::__pgrx_c_bindings::LogicalTape,
                *mut crate::__pgrx_c_bindings::SortTuple,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__Tuplesortstate__0p__0c__struct__LogicalTape__0p__0c__SortTuple__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_void__0l__struct__Tuplesortstate__0p__0c__struct__LogicalTape__0p__0c__SortTuple__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__Tuplesortstate__0p__0c__struct__LogicalTape__0p__0c__SortTuple__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::Tuplesortstate, *mut crate::__pgrx_c_bindings::LogicalTape, *mut crate::__pgrx_c_bindings::SortTuple) -> ()>>;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::size_of::<*mut u32>() == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::align_of::<*mut u32>() == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void, i32) -> ()>,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void, i32) -> ()>,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__void__0p__0c__int__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_void__0l__void__0p__0c__int__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_void__0l__void__0p__0c__int__0r {
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void, i32) -> ()>,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn(bool, bool) -> ()>>()
        == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn(bool, bool) -> ()>>(
    ) == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__0x5f426f6f6c__0c__0x5f426f6f6c__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_void__0l__0x5f426f6f6c__0c__0x5f426f6f6c__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__0x5f426f6f6c__0c__0x5f426f6f6c__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<unsafe extern "C-unwind" fn(bool, bool) -> ()>,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_char) -> ()>,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_char) -> ()>,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__char__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17"))]
impl c::sealed::Sealed for Signature_void__0l__char__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17"))]
impl c::expression::NativeFunctionSignature for Signature_void__0l__char__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_char) -> ()>,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__char__0p__0c__const__char__0p__0c__const__char__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed
    for Signature_void__0l__char__0p__0c__const__char__0p__0c__const__char__0p__0r
{
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__char__0p__0c__const__char__0p__0c__const__char__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
            ) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut ::core::ffi::c_char, u32, u32, u32) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut ::core::ffi::c_char, u32, u32, u32) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__char__0p__0c__unsigned__int__0c__unsigned__int__0c__unsigned__int__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed
    for Signature_void__0l__char__0p__0c__unsigned__int__0c__unsigned__int__0c__unsigned__int__0r
{
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__char__0p__0c__unsigned__int__0c__unsigned__int__0c__unsigned__int__0r
{
    type Physical = PhysicalFunction_C_unwind_4<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut ::core::ffi::c_char, u32, u32, u32) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut ::core::ffi::c_char, u32, u64, i32) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut ::core::ffi::c_char, u32, u64, i32) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__char__0p__0c__unsigned__int__0c__unsigned__long__0c__int__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed
    for Signature_void__0l__char__0p__0c__unsigned__int__0c__unsigned__long__0c__int__0r
{
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__char__0p__0c__unsigned__int__0c__unsigned__long__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_4<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut ::core::ffi::c_char, u32, u64, i32) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut ::core::ffi::c_char, u32, u64, u64, i32) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut ::core::ffi::c_char, u32, u64, u64, i32) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__char__0p__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__int__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_void__0l__char__0p__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__int__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_void__0l__char__0p__0c__unsigned__int__0c__unsigned__long__0c__unsigned__long__0c__int__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_char, u32, u64, u64, i32) -> ()>>;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_char, u64) -> ()>,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_char, u64) -> ()>,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__char__0p__0c__unsigned__long__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::sealed::Sealed for Signature_void__0l__char__0p__0c__unsigned__long__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__char__0p__0c__unsigned__long__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_char, u64) -> ()>,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut ::core::ffi::c_char, usize, u8) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut ::core::ffi::c_char, usize, u8) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__char__0p__0c__unsigned__long__0c__unsigned__char__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed
    for Signature_void__0l__char__0p__0c__unsigned__long__0c__unsigned__char__0r
{
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__char__0p__0c__unsigned__long__0c__unsigned__char__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut ::core::ffi::c_char, usize, u8) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const ::core::ffi::c_char, *mut u32, *mut u64, i32) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const ::core::ffi::c_char, *mut u32, *mut u64, i32) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__const__char__0p__0c__unsigned__int__0p__0c__unsigned__long__0p__0c__int__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_void__0l__const__char__0p__0c__unsigned__int__0p__0c__unsigned__long__0p__0c__int__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_void__0l__const__char__0p__0c__unsigned__int__0p__0c__unsigned__long__0p__0c__int__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_char, *mut u32, *mut u64, i32) -> ()>>;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const ::core::ffi::c_char, usize) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const ::core::ffi::c_char, usize) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__const__char__0p__0c__unsigned__long__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_void__0l__const__char__0p__0c__unsigned__long__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__const__char__0p__0c__unsigned__long__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const ::core::ffi::c_char, usize) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::ItemPointerData,
                *mut crate::__pgrx_c_bindings::ItemPointerData,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::ItemPointerData,
                *mut crate::__pgrx_c_bindings::ItemPointerData,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__const__struct__ItemPointerData__0p__0c__struct__ItemPointerData__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed
    for Signature_void__0l__const__struct__ItemPointerData__0p__0c__struct__ItemPointerData__0p__0r
{
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__const__struct__ItemPointerData__0p__0c__struct__ItemPointerData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::ItemPointerData,
                *mut crate::__pgrx_c_bindings::ItemPointerData,
            ) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::BlockIdData, u32) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::BlockIdData, u32) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__BlockIdData__0p__0c__unsigned__int__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_void__0l__struct__BlockIdData__0p__0c__unsigned__int__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__BlockIdData__0p__0c__unsigned__int__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::BlockIdData, u32) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::BufferDesc, u32) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::BufferDesc, u32) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__BufferDesc__0p__0c__unsigned__int__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::sealed::Sealed for Signature_void__0l__struct__BufferDesc__0p__0c__unsigned__int__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__BufferDesc__0p__0c__unsigned__int__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::BufferDesc, u32) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ItemPointerData) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ItemPointerData) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__ItemPointerData__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_void__0l__struct__ItemPointerData__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__ItemPointerData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ItemPointerData) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ItemPointerData, u32) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ItemPointerData, u32) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__ItemPointerData__0p__0c__unsigned__int__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_void__0l__struct__ItemPointerData__0p__0c__unsigned__int__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__ItemPointerData__0p__0c__unsigned__int__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ItemPointerData, u32) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ItemPointerData,
                u32,
                u16,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ItemPointerData,
                u32,
                u16,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__ItemPointerData__0p__0c__unsigned__int__0c__unsigned__short__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed
    for Signature_void__0l__struct__ItemPointerData__0p__0c__unsigned__int__0c__unsigned__short__0r
{
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__ItemPointerData__0p__0c__unsigned__int__0c__unsigned__short__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ItemPointerData,
                u32,
                u16,
            ) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ItemPointerData, u16) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ItemPointerData, u16) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__ItemPointerData__0p__0c__unsigned__short__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed
    for Signature_void__0l__struct__ItemPointerData__0p__0c__unsigned__short__0r
{
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__ItemPointerData__0p__0c__unsigned__short__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ItemPointerData, u16) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PlanState) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PlanState) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__PlanState__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::sealed::Sealed for Signature_void__0l__struct__PlanState__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__PlanState__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PlanState) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::buftag,
                crate::__pgrx_c_bindings::Oid,
                i32,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::buftag,
                crate::__pgrx_c_bindings::Oid,
                i32,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__buftag__0p__0c__unsigned__int__0c__enum__ForkNumber__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed
    for Signature_void__0l__struct__buftag__0p__0c__unsigned__int__0c__enum__ForkNumber__0r
{
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__buftag__0p__0c__unsigned__int__0c__enum__ForkNumber__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::buftag,
                crate::__pgrx_c_bindings::Oid,
                i32,
            ) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::dclist_head,
                *mut crate::__pgrx_c_bindings::dlist_node,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::dclist_head,
                *mut crate::__pgrx_c_bindings::dlist_node,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__dclist_head__0p__0c__struct__dlist_node__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed
    for Signature_void__0l__struct__dclist_head__0p__0c__struct__dlist_node__0p__0r
{
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__dclist_head__0p__0c__struct__dlist_node__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::dclist_head,
                *mut crate::__pgrx_c_bindings::dlist_node,
            ) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::dclist_head,
                *mut crate::__pgrx_c_bindings::dlist_node,
                *mut crate::__pgrx_c_bindings::dlist_node,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::dclist_head,
                *mut crate::__pgrx_c_bindings::dlist_node,
                *mut crate::__pgrx_c_bindings::dlist_node,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__dclist_head__0p__0c__struct__dlist_node__0p__0c__struct__dlist_node__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_void__0l__struct__dclist_head__0p__0c__struct__dlist_node__0p__0c__struct__dlist_node__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__dclist_head__0p__0c__struct__dlist_node__0p__0c__struct__dlist_node__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::dclist_head, *mut crate::__pgrx_c_bindings::dlist_node, *mut crate::__pgrx_c_bindings::dlist_node) -> ()>>;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::dlist_head) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::dlist_head) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__dlist_head__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_void__0l__struct__dlist_head__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__dlist_head__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::dlist_head) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::dlist_node) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::dlist_node) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__dlist_node__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_void__0l__struct__dlist_node__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__dlist_node__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::dlist_node) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
                crate::__pgrx_c_bindings::Datum,
                i32,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
                crate::__pgrx_c_bindings::Datum,
                i32,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__void__0p__0c__unsigned__long__0c__int__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_void__0l__void__0p__0c__unsigned__long__0c__int__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__void__0p__0c__unsigned__long__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
                crate::__pgrx_c_bindings::Datum,
                i32,
            ) -> (),
        >,
    >;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
                usize,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
                usize,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[cfg(feature = "pg16")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0p__0l__void__0p__0c__unsigned__long__0r;
#[cfg(feature = "pg16")]
impl c::sealed::Sealed for Signature_void__0p__0l__void__0p__0c__unsigned__long__0r {}
#[cfg(feature = "pg16")]
impl c::expression::NativeFunctionSignature
    for Signature_void__0p__0l__void__0p__0c__unsigned__long__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
                usize,
            ) -> *mut ::core::ffi::c_void,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(i32) -> *mut ::core::ffi::c_void>,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(i32) -> *mut ::core::ffi::c_void>,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0p__0l__int__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_void__0p__0l__int__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_void__0p__0l__int__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(i32) -> *mut ::core::ffi::c_void>,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::dclist_head,
                usize,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::dclist_head,
                usize,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0p__0l__struct__dclist_head__0p__0c__unsigned__long__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_void__0p__0l__struct__dclist_head__0p__0c__unsigned__long__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0p__0l__struct__dclist_head__0p__0c__unsigned__long__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::dclist_head,
                usize,
            ) -> *mut ::core::ffi::c_void,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_family;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AddrInfo>() == 144);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AddrInfo>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AddrInfo {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AddrInfo, family) == 0);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_family>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AddrInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::AddrInfo,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::AddrInfo> as c::expression::OrdinaryField<
        Field_family,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).family) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::InternalGrant>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::InternalGrant>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::InternalGrant, privileges) == 24);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_privileges>
    for c::expression::CRecord<crate::__pgrx_c_bindings::InternalGrant>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::InternalGrant, col_privs) == 32);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_col_privs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::InternalGrant>
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
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::InternalGrant, grantees) == 40);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_grantees>
    for c::expression::CRecord<crate::__pgrx_c_bindings::InternalGrant>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::InternalGrant, grant_option) == 48);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_grant_option>
    for c::expression::CRecord<crate::__pgrx_c_bindings::InternalGrant>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::InternalGrant, behavior) == 52);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_behavior>
    for c::expression::CRecord<crate::__pgrx_c_bindings::InternalGrant>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__DropBehavior,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_datum1;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SortTuple>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SortTuple>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SortTuple {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SortTuple, datum1) == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_datum1>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SortTuple>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, crate::__pgrx_c_bindings::Datum>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SortTuple,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SortTuple> as c::expression::OrdinaryField<
        Field_datum1,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).datum1) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_isnull1;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SortTuple, isnull1) == 16);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isnull1>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SortTuple>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SortTuple,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SortTuple> as c::expression::OrdinaryField<
        Field_isnull1,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isnull1) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_srctape;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SortTuple, srctape) == 20);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_srctape>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SortTuple>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SortTuple,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SortTuple> as c::expression::OrdinaryField<
        Field_srctape,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).srctape) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_haveDatum1;
#[cfg(feature = "pg16")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TuplesortPublic>() == 104);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TuplesortPublic>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TuplesortPublic {}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TuplesortPublic, haveDatum1) == 64);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_haveDatum1>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TuplesortPublic>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TuplesortPublic, <<c::expression::CRecord<crate::__pgrx_c_bindings::TuplesortPublic> as c::expression::OrdinaryField<Field_haveDatum1>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).haveDatum1) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_nKeys;
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TuplesortPublic, nKeys) == 68);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nKeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TuplesortPublic>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 68;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TuplesortPublic, <<c::expression::CRecord<crate::__pgrx_c_bindings::TuplesortPublic> as c::expression::OrdinaryField<Field_nKeys>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nKeys) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_sortopt;
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TuplesortPublic, sortopt) == 88);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sortopt>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TuplesortPublic>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TuplesortPublic, <<c::expression::CRecord<crate::__pgrx_c_bindings::TuplesortPublic> as c::expression::OrdinaryField<Field_sortopt>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sortopt) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_tuples;
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TuplesortPublic, tuples) == 92);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tuples>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TuplesortPublic>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 92;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TuplesortPublic, <<c::expression::CRecord<crate::__pgrx_c_bindings::TuplesortPublic> as c::expression::OrdinaryField<Field_tuples>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tuples) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::WalRcvStreamOptions>() == 72);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::WalRcvStreamOptions>() == 8);
};
#[cfg(feature = "pg16")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::WalSndCtlData>() == 104);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::WalSndCtlData>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AclItem>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AclItem>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ai_privs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AclItem>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Bitmapset, type_) == 0);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Bitmapset>
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
const _: Projection<
    crate::__pgrx_c_bindings::Bitmapset,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Bitmapset> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Bitmapset, nwords) == 4);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nwords>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Bitmapset>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(feature = "pg16")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BufferUsage>() == 112);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BufferUsage>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_er_tupdesc;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ExpandedRecordHeader, er_tupdesc) == 72
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_er_tupdesc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedRecordHeader>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::TupleDescData>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExpandedRecordHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExpandedRecordHeader> as c::expression::OrdinaryField<Field_er_tupdesc>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).er_tupdesc) };
#[cfg(feature = "pg16")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::FormData_pg_attribute>() == 108);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::FormData_pg_attribute>() == 4);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, attlen) == 72);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attlen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, attnum) == 74);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attnum>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 74;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, attcacheoff) == 76
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attcacheoff>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 76;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, atttypmod) == 80
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_atttypmod>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, attndims) == 84
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attndims>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 84;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, attbyval) == 86
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attbyval>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 86;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, attalign) == 87
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attalign>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 87;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, attstorage) == 88
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attstorage>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, attcompression) == 89
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attcompression>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 89;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, attnotnull) == 90
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attnotnull>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 90;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, atthasdef) == 91
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_atthasdef>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 91;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, atthasmissing) == 92
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_atthasmissing>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 92;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, attidentity) == 93
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attidentity>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 93;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, attgenerated) == 94
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attgenerated>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 94;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, attisdropped) == 95
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attisdropped>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 95;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, attislocal) == 96
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attislocal>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, attinhcount) == 98
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attinhcount>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 98;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, attstattarget) == 100
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attstattarget>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 100;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, attcollation) == 104
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attcollation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry>() == 72);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry, spill_txns) == 0
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_spill_txns>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry, spill_count) == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_spill_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry, spill_bytes) == 16
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_spill_bytes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry, stream_txns) == 24
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stream_txns>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry, stream_count) == 32
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stream_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry, stream_bytes) == 40
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stream_bytes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry, total_txns) == 48
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_total_txns>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry, total_bytes) == 56
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_total_bytes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry,
        stat_reset_timestamp
    ) == 64
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stat_reset_timestamp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_spcOid;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RelFileLocator>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RelFileLocator>() == 4);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RelFileLocator {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelFileLocator, spcOid) == 0);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_spcOid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelFileLocator>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelFileLocator, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelFileLocator> as c::expression::OrdinaryField<Field_spcOid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).spcOid) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelFileLocator, dbOid) == 4);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dbOid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelFileLocator>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelFileLocator, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelFileLocator> as c::expression::OrdinaryField<Field_dbOid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dbOid) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_relNumber;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelFileLocator, relNumber) == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relNumber>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelFileLocator>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelFileLocator, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelFileLocator> as c::expression::OrdinaryField<Field_relNumber>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relNumber) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
pub struct Field_scan_begin;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TableAmRoutine, scan_begin) == 16);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(
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
    assert!(
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
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_scan_begin>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0c__int__0c__struct__ScanKeyData__0p__0c__struct__ParallelTableScanDescData__0p__0c__unsigned__int__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine> as c::expression::OrdinaryField<Field_scan_begin>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).scan_begin) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
pub struct Field_scan_set_tidrange;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TableAmRoutine, scan_set_tidrange) == 48
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::TableScanDescData,
                    *mut crate::__pgrx_c_bindings::ItemPointerData,
                    *mut crate::__pgrx_c_bindings::ItemPointerData,
                ) -> (),
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::TableScanDescData,
                    *mut crate::__pgrx_c_bindings::ItemPointerData,
                    *mut crate::__pgrx_c_bindings::ItemPointerData,
                ) -> (),
            >,
        >() == 8
    );
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_scan_set_tidrange>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_void__0l__struct__TableScanDescData__0p__0c__struct__ItemPointerData__0p__0c__struct__ItemPointerData__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine> as c::expression::OrdinaryField<Field_scan_set_tidrange>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).scan_set_tidrange) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(
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
                    *mut u32,
                ) -> u32,
            >,
        >() == 8
    );
    assert!(
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
                    *mut u32,
                ) -> u32,
            >,
        >() == 8
    );
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tuple_update>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_enum__TM_Result__0l__struct__RelationData__0p__0c__struct__ItemPointerData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__struct__SnapshotData__0p__0c__struct__SnapshotData__0p__0c__0x5f426f6f6c__0c__struct__TM_FailureData__0p__0c__enum__LockTupleMode__0p__0c__enum__TU_UpdateIndexes__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 200;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_relation_set_new_filelocator;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TableAmRoutine, relation_set_new_filelocator)
        == 224
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *const crate::__pgrx_c_bindings::RelFileLocator,
                    ::core::ffi::c_char,
                    *mut crate::__pgrx_c_bindings::TransactionId,
                    *mut crate::__pgrx_c_bindings::MultiXactId,
                ) -> (),
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *const crate::__pgrx_c_bindings::RelFileLocator,
                    ::core::ffi::c_char,
                    *mut crate::__pgrx_c_bindings::TransactionId,
                    *mut crate::__pgrx_c_bindings::MultiXactId,
                ) -> (),
            >,
        >() == 8
    );
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relation_set_new_filelocator>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_void__0l__struct__RelationData__0p__0c__const__struct__RelFileLocator__0p__0c__char__0c__unsigned__int__0p__0c__unsigned__int__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 224;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine> as c::expression::OrdinaryField<Field_relation_set_new_filelocator>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relation_set_new_filelocator) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *const crate::__pgrx_c_bindings::RelFileLocator,
                ) -> (),
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *const crate::__pgrx_c_bindings::RelFileLocator,
                ) -> (),
            >,
        >() == 8
    );
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relation_copy_data>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_void__0l__struct__RelationData__0p__0c__const__struct__RelFileLocator__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::VacuumCutoffs>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::VacuumCutoffs>() == 4);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::VacuumCutoffs {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacuumCutoffs, relfrozenxid) == 0);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relfrozenxid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacuumCutoffs>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VacuumCutoffs, <<c::expression::CRecord<crate::__pgrx_c_bindings::VacuumCutoffs> as c::expression::OrdinaryField<Field_relfrozenxid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relfrozenxid) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacuumCutoffs, relminmxid) == 4);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relminmxid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacuumCutoffs>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::MultiXactId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VacuumCutoffs, <<c::expression::CRecord<crate::__pgrx_c_bindings::VacuumCutoffs> as c::expression::OrdinaryField<Field_relminmxid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relminmxid) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_OldestXmin;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacuumCutoffs, OldestXmin) == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_OldestXmin>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacuumCutoffs>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VacuumCutoffs, <<c::expression::CRecord<crate::__pgrx_c_bindings::VacuumCutoffs> as c::expression::OrdinaryField<Field_OldestXmin>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).OldestXmin) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_OldestMxact;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacuumCutoffs, OldestMxact) == 12);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_OldestMxact>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacuumCutoffs>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::MultiXactId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VacuumCutoffs, <<c::expression::CRecord<crate::__pgrx_c_bindings::VacuumCutoffs> as c::expression::OrdinaryField<Field_OldestMxact>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).OldestMxact) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_FreezeLimit;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacuumCutoffs, FreezeLimit) == 16);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_FreezeLimit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacuumCutoffs>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VacuumCutoffs, <<c::expression::CRecord<crate::__pgrx_c_bindings::VacuumCutoffs> as c::expression::OrdinaryField<Field_FreezeLimit>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).FreezeLimit) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_MultiXactCutoff;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::VacuumCutoffs, MultiXactCutoff) == 20
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_MultiXactCutoff>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacuumCutoffs>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::MultiXactId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VacuumCutoffs, <<c::expression::CRecord<crate::__pgrx_c_bindings::VacuumCutoffs> as c::expression::OrdinaryField<Field_MultiXactCutoff>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).MultiXactCutoff) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::buftag, spcOid) == 0);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_spcOid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::buftag>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::buftag,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::buftag> as c::expression::OrdinaryField<
        Field_spcOid,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).spcOid) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::buftag, dbOid) == 4);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dbOid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::buftag>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::buftag,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::buftag> as c::expression::OrdinaryField<
        Field_dbOid,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dbOid) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::buftag, relNumber) == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relNumber>
    for c::expression::CRecord<crate::__pgrx_c_bindings::buftag>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::buftag,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::buftag> as c::expression::OrdinaryField<
        Field_relNumber,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relNumber) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_dlist;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::dclist_head>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::dclist_head>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::dclist_head {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::dclist_head, dlist) == 0);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dlist>
    for c::expression::CRecord<crate::__pgrx_c_bindings::dclist_head>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::dlist_head>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::dclist_head, <<c::expression::CRecord<crate::__pgrx_c_bindings::dclist_head> as c::expression::OrdinaryField<Field_dlist>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dlist) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::dclist_head, count) == 16);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::dclist_head>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::dclist_head, <<c::expression::CRecord<crate::__pgrx_c_bindings::dclist_head> as c::expression::OrdinaryField<Field_count>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).count) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_ticks;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::instr_time>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::instr_time>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::instr_time {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::instr_time, ticks) == 0);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ticks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::instr_time>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::instr_time, <<c::expression::CRecord<crate::__pgrx_c_bindings::instr_time> as c::expression::OrdinaryField<Field_ticks>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ticks) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_xid_value;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ListCell, xid_value) == 0);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xid_value>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ListCell>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ListCell,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ListCell> as c::expression::OrdinaryField<
        Field_xid_value,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xid_value) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AggInfo>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AggInfo>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggInfo, type_) == 0);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggInfo>
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
const _: Projection<
    crate::__pgrx_c_bindings::AggInfo,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::AggInfo> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_aggrefs;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggInfo, aggrefs) == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aggrefs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggInfo>
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
const _: Projection<
    crate::__pgrx_c_bindings::AggInfo,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::AggInfo> as c::expression::OrdinaryField<
        Field_aggrefs,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).aggrefs) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggInfo, transno) == 16);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_transno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggInfo, shareable) == 20);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_shareable>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggInfo, finalfn_oid) == 24);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_finalfn_oid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AggStatePerTransData>() == 376);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AggStatePerTransData>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_aggsortrequired;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AggStatePerTransData, aggsortrequired) == 9
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aggsortrequired>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerTransData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 9;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggStatePerTransData, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerTransData> as c::expression::OrdinaryField<Field_aggsortrequired>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).aggsortrequired) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_lastdatum;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AggStatePerTransData, lastdatum) == 328
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lastdatum>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerTransData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, crate::__pgrx_c_bindings::Datum>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 328;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggStatePerTransData, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerTransData> as c::expression::OrdinaryField<Field_lastdatum>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lastdatum) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_lastisnull;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AggStatePerTransData, lastisnull) == 336
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lastisnull>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerTransData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 336;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggStatePerTransData, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerTransData> as c::expression::OrdinaryField<Field_lastisnull>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lastisnull) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_haslast;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AggStatePerTransData, haslast) == 337
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_haslast>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerTransData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 337;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggStatePerTransData, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerTransData> as c::expression::OrdinaryField<Field_haslast>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).haslast) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AggTransInfo>() == 80);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AggTransInfo>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggTransInfo, type_) == 0);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggTransInfo>
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
const _: Projection<crate::__pgrx_c_bindings::AggTransInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggTransInfo> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggTransInfo, args) == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_args>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggTransInfo>
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
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggTransInfo, transfn_oid) == 24);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_transfn_oid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggTransInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggTransInfo, serialfn_oid) == 28);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_serialfn_oid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggTransInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggTransInfo, deserialfn_oid) == 32);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_deserialfn_oid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggTransInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggTransInfo, combinefn_oid) == 36);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_combinefn_oid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggTransInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggTransInfo, aggtranstype) == 40);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aggtranstype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggTransInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggTransInfo, aggtranstypmod) == 44);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aggtranstypmod>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggTransInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggTransInfo, transtypeLen) == 48);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_transtypeLen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggTransInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggTransInfo, transtypeByVal) == 52);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_transtypeByVal>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggTransInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggTransInfo, aggtransspace) == 56);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aggtransspace>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggTransInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggTransInfo, initValue) == 64);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_initValue>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggTransInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, crate::__pgrx_c_bindings::Datum>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggTransInfo, initValueIsNull) == 72);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_initValueIsNull>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggTransInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_aggpresorted;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Aggref, aggpresorted) == 75);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aggpresorted>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Aggref>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 75;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Aggref,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Aggref> as c::expression::OrdinaryField<
        Field_aggpresorted,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).aggpresorted) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AppendState>() == 352);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AppendState>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_as_valid_subplans_identified;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AppendState, as_valid_subplans_identified)
        == 320
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_as_valid_subplans_identified>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AppendState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 320;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AppendState, <<c::expression::CRecord<crate::__pgrx_c_bindings::AppendState> as c::expression::OrdinaryField<Field_as_valid_subplans_identified>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).as_valid_subplans_identified) };
#[cfg(feature = "pg16")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BackupState>() == 1096);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BackupState>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BackupState {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BackupState, startpoint) == 1032);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_startpoint>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BackupState>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1032;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BackupState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BackupState> as c::expression::OrdinaryField<Field_startpoint>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).startpoint) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_starttli;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BackupState, starttli) == 1040);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_starttli>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BackupState>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1040;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BackupState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BackupState> as c::expression::OrdinaryField<Field_starttli>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).starttli) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_checkpointloc;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BackupState, checkpointloc) == 1048);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_checkpointloc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BackupState>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1048;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BackupState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BackupState> as c::expression::OrdinaryField<Field_checkpointloc>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).checkpointloc) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_starttime;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BackupState, starttime) == 1056);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_starttime>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BackupState>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1056;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BackupState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BackupState> as c::expression::OrdinaryField<Field_starttime>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).starttime) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_started_in_recovery;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BackupState, started_in_recovery) == 1064
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_started_in_recovery>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BackupState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1064;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BackupState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BackupState> as c::expression::OrdinaryField<Field_started_in_recovery>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).started_in_recovery) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_stoppoint;
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BackupState, stoppoint) == 1072);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stoppoint>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BackupState>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1072;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BackupState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BackupState> as c::expression::OrdinaryField<Field_stoppoint>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stoppoint) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_stoptli;
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BackupState, stoptli) == 1080);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stoptli>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BackupState>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1080;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BackupState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BackupState> as c::expression::OrdinaryField<Field_stoptli>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stoptli) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_stoptime;
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BackupState, stoptime) == 1088);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stoptime>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BackupState>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1088;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BackupState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BackupState> as c::expression::OrdinaryField<Field_stoptime>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stoptime) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BufferManagerRelation>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BufferManagerRelation>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BufferManagerRelation {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BufferManagerRelation, relpersistence) == 16
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relpersistence>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BufferManagerRelation>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BufferManagerRelation, <<c::expression::CRecord<crate::__pgrx_c_bindings::BufferManagerRelation> as c::expression::OrdinaryField<Field_relpersistence>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relpersistence) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BulkInsertStateData>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BulkInsertStateData>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_next_free;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BulkInsertStateData, next_free) == 12
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_next_free>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BulkInsertStateData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
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
