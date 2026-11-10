// Shared generated C macro support.

#[cfg(feature = "pg19")]
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<A0: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>, c::expression::ReadWrite>>, A1: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot>, c::expression::ReadWrite>>, A2: c::expression::ImplicitTo<c::CUnsignedInt>, A3: c::expression::ImplicitTo<c::CUnsignedInt>, A4: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::BulkInsertStateData>, c::expression::ReadWrite>>> c::expression::Call<(A0,A1,A2,A3,A4,)> for Signature_void__0l__struct__RelationData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__unsigned__int__0c__struct__BulkInsertStateData__0p__0r {
 type Output = ();
 unsafe fn call(pointer: Self::Pointer, args: (A0,A1,A2,A3,A4,)) -> Self::Output {
let function = pointer.expect("C indirect call requires a non-null function pointer");
const { assert!(!::core::mem::needs_drop::<A0>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::RelationData>());
let native0 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>, c::expression::ReadWrite>, _>(args.0));
const { assert!(!::core::mem::needs_drop::<A1>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::TupleTableSlot>());
let native1 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot>, c::expression::ReadWrite>, _>(args.1));
const { assert!(!::core::mem::needs_drop::<A2>()); }
const _: () = assert!(!::core::mem::needs_drop::<u32>());
let native2 = <c::CUnsignedInt as c::expression::CType>::into_storage(c::expression::implicit::<c::CUnsignedInt, _>(args.2));
const { assert!(!::core::mem::needs_drop::<A3>()); }
const _: () = assert!(!::core::mem::needs_drop::<u32>());
let native3 = <c::CUnsignedInt as c::expression::CType>::into_storage(c::expression::implicit::<c::CUnsignedInt, _>(args.3));
const { assert!(!::core::mem::needs_drop::<A4>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::BulkInsertStateData>());
let native4 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::BulkInsertStateData>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::BulkInsertStateData>, c::expression::ReadWrite>, _>(args.4));
// SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
let result = unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0, native1, native2, native3, native4)) };
result
}
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                u32,
                u32,
                *mut crate::__pgrx_c_bindings::BulkInsertStateData,
                u32,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                u32,
                u32,
                *mut crate::__pgrx_c_bindings::BulkInsertStateData,
                u32,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__RelationData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__unsigned__int__0c__struct__BulkInsertStateData__0p__0c__unsigned__int__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0l__struct__RelationData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__unsigned__int__0c__struct__BulkInsertStateData__0p__0c__unsigned__int__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__RelationData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__unsigned__int__0c__struct__BulkInsertStateData__0p__0c__unsigned__int__0r {
 type Physical = PhysicalFunction_C_unwind_6<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::TupleTableSlot, u32, u32, *mut crate::__pgrx_c_bindings::BulkInsertStateData, u32) -> ()>>;
}
#[cfg(feature = "pg19")]
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<A0: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>, c::expression::ReadWrite>>, A1: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot>, c::expression::ReadWrite>>, A2: c::expression::ImplicitTo<c::CUnsignedInt>, A3: c::expression::ImplicitTo<c::CUnsignedInt>, A4: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::BulkInsertStateData>, c::expression::ReadWrite>>, A5: c::expression::ImplicitTo<c::CUnsignedInt>> c::expression::Call<(A0,A1,A2,A3,A4,A5,)> for Signature_void__0l__struct__RelationData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__unsigned__int__0c__struct__BulkInsertStateData__0p__0c__unsigned__int__0r {
 type Output = ();
 unsafe fn call(pointer: Self::Pointer, args: (A0,A1,A2,A3,A4,A5,)) -> Self::Output {
let function = pointer.expect("C indirect call requires a non-null function pointer");
const { assert!(!::core::mem::needs_drop::<A0>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::RelationData>());
let native0 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>, c::expression::ReadWrite>, _>(args.0));
const { assert!(!::core::mem::needs_drop::<A1>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::TupleTableSlot>());
let native1 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot>, c::expression::ReadWrite>, _>(args.1));
const { assert!(!::core::mem::needs_drop::<A2>()); }
const _: () = assert!(!::core::mem::needs_drop::<u32>());
let native2 = <c::CUnsignedInt as c::expression::CType>::into_storage(c::expression::implicit::<c::CUnsignedInt, _>(args.2));
const { assert!(!::core::mem::needs_drop::<A3>()); }
const _: () = assert!(!::core::mem::needs_drop::<u32>());
let native3 = <c::CUnsignedInt as c::expression::CType>::into_storage(c::expression::implicit::<c::CUnsignedInt, _>(args.3));
const { assert!(!::core::mem::needs_drop::<A4>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::BulkInsertStateData>());
let native4 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::BulkInsertStateData>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::BulkInsertStateData>, c::expression::ReadWrite>, _>(args.4));
const { assert!(!::core::mem::needs_drop::<A5>()); }
const _: () = assert!(!::core::mem::needs_drop::<u32>());
let native5 = <c::CUnsignedInt as c::expression::CType>::into_storage(c::expression::implicit::<c::CUnsignedInt, _>(args.5));
// SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
let result = unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0, native1, native2, native3, native4, native5)) };
result
}
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, u32) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, u32) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__RelationData__0p__0c__unsigned__int__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0l__struct__RelationData__0p__0c__unsigned__int__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__RelationData__0p__0c__unsigned__int__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, u32) -> (),
        >,
    >;
}
#[cfg(feature = "pg19")]
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<
    A0: c::expression::ImplicitTo<
            c::expression::CPointer<
                c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>,
                c::expression::ReadWrite,
            >,
        >,
    A1: c::expression::ImplicitTo<c::CUnsignedInt>,
> c::expression::Call<(A0, A1)>
    for Signature_void__0l__struct__RelationData__0p__0c__unsigned__int__0r
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
        const _: () = assert!(!::core::mem::needs_drop::<u32>());
        let native1 =
            <c::CUnsignedInt as c::expression::CType>::into_storage(c::expression::implicit::<
                c::CUnsignedInt,
                _,
            >(args.1));
        // SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
        let result =
            unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0, native1)) };
        result
    }
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::ItemPointerData>() == 8);
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::ItemPointerData>() == 8);
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::TableScanDescData>() == 8);
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::TableScanDescData>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::TupleTableSlot>() == 8);
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::TupleTableSlot>() == 8);
#[cfg(feature = "pg19")]
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<
    A0: c::expression::ImplicitTo<
            c::expression::CPointer<
                c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot>,
                c::expression::ReadWrite,
            >,
        >,
    A1: c::expression::ImplicitTo<c::CInt>,
> c::expression::Call<(A0, A1)> for Signature_void__0l__struct__TupleTableSlot__0p__0c__int__0r
{
    type Output = ();
    unsafe fn call(pointer: Self::Pointer, args: (A0, A1)) -> Self::Output {
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
        const {
            assert!(!::core::mem::needs_drop::<A1>());
        }
        const _: () = assert!(!::core::mem::needs_drop::<i32>());
        let native1 = <c::CInt as c::expression::CType>::into_storage(c::expression::implicit::<
            c::CInt,
            _,
        >(args.1));
        // SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
        let result =
            unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0, native1)) };
        result
    }
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::TupleTableSlot>() == 8);
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::TupleTableSlot>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::TupleTableSlot>() == 8);
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::TupleTableSlot>() == 8);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__unsigned__long__0c__enum__SysCacheIdentifier__0c__unsigned__int__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed
    for Signature_void__0l__unsigned__long__0c__enum__SysCacheIdentifier__0c__unsigned__int__0r
{
}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__unsigned__long__0c__enum__SysCacheIdentifier__0c__unsigned__int__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Datum, i32, u32) -> (),
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
                *mut crate::__pgrx_c_bindings::TSQueryParserStateData,
                *mut ::core::ffi::c_char,
                i32,
                i16,
                bool,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
                *mut crate::__pgrx_c_bindings::TSQueryParserStateData,
                *mut ::core::ffi::c_char,
                i32,
                i16,
                bool,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__void__0p__0c__struct__TSQueryParserStateData__0p__0c__char__0p__0c__int__0c__short__0c__0x5f426f6f6c__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0l__void__0p__0c__struct__TSQueryParserStateData__0p__0c__char__0p__0c__int__0c__short__0c__0x5f426f6f6c__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_void__0l__void__0p__0c__struct__TSQueryParserStateData__0p__0c__char__0p__0c__int__0c__short__0c__0x5f426f6f6c__0r {
 type Physical = PhysicalFunction_C_unwind_6<::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void, *mut crate::__pgrx_c_bindings::TSQueryParserStateData, *mut ::core::ffi::c_char, i32, i16, bool) -> ()>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__void__0p__0c__void__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0l__void__0p__0c__void__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_void__0l__void__0p__0c__void__0p__0r {
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> (),
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const u8, i32, *mut bool) -> ()>,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const u8, i32, *mut bool) -> ()>,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__const__unsigned__char__0p__0c__int__0c__0x5f426f6f6c__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed
    for Signature_void__0l__const__unsigned__char__0p__0c__int__0c__0x5f426f6f6c__0p__0r
{
}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__const__unsigned__char__0p__0c__int__0c__0x5f426f6f6c__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const u8, i32, *mut bool) -> ()>,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn(i32, u32) -> ()>>()
        == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn(i32, u32) -> ()>>()
        == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__int__0c__enum__BufferLockMode__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0l__int__0c__enum__BufferLockMode__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__int__0c__enum__BufferLockMode__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<unsafe extern "C-unwind" fn(i32, u32) -> ()>,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::BufferDesc) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::BufferDesc) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__BufferDesc__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0l__struct__BufferDesc__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__BufferDesc__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::BufferDesc) -> (),
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::SlruDesc, i32) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::SlruDesc, i32) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__SlruDesc__0p__0c__int__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0l__struct__SlruDesc__0p__0c__int__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__SlruDesc__0p__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::SlruDesc, i32) -> (),
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void, usize) -> ()>,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void, usize) -> ()>,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__void__0p__0c__unsigned__long__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0l__void__0p__0c__unsigned__long__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__void__0p__0c__unsigned__long__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void, usize) -> ()>,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PageXLogRecPtr, u64) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PageXLogRecPtr, u64) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__volatile__PageXLogRecPtr__0p__0c__unsigned__long__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed
    for Signature_void__0l__volatile__PageXLogRecPtr__0p__0c__unsigned__long__0r
{
}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__volatile__PageXLogRecPtr__0p__0c__unsigned__long__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PageXLogRecPtr, u64) -> (),
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn(*mut u8) -> ()>>()
        == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn(*mut u8) -> ()>>()
        == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__volatile__unsigned__char__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0l__volatile__unsigned__char__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__volatile__unsigned__char__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(*mut u8) -> ()>,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                usize,
                *mut ::core::ffi::c_void,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                usize,
                *mut ::core::ffi::c_void,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0p__0l__unsigned__long__0c__void__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0p__0l__unsigned__long__0c__void__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_void__0p__0l__unsigned__long__0c__void__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                usize,
                *mut ::core::ffi::c_void,
            ) -> *mut ::core::ffi::c_void,
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0p__0l__const__void__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0p__0l__const__void__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_void__0p__0l__const__void__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerInfo,
                i32,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerInfo,
                i32,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0p__0l__struct__PlannerInfo__0p__0c__int__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0p__0l__struct__PlannerInfo__0p__0c__int__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_void__0p__0l__struct__PlannerInfo__0p__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerInfo,
                i32,
            ) -> *mut ::core::ffi::c_void,
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::dshash_table,
                *const ::core::ffi::c_void,
                *mut bool,
                i32,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::dshash_table,
                *const ::core::ffi::c_void,
                *mut bool,
                i32,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0p__0l__struct__dshash_table__0p__0c__const__void__0p__0c__0x5f426f6f6c__0p__0c__int__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0p__0l__struct__dshash_table__0p__0c__const__void__0p__0c__0x5f426f6f6c__0p__0c__int__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_void__0p__0l__struct__dshash_table__0p__0c__const__void__0p__0c__0x5f426f6f6c__0p__0c__int__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::dshash_table, *const ::core::ffi::c_void, *mut bool, i32) -> *mut ::core::ffi::c_void>>;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_numNonLeafPages;
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::GenericCosts>() == 72);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::GenericCosts>() == 8);
};
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GenericCosts, numNonLeafPages) == 64);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numNonLeafPages>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GenericCosts>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::GenericCosts, <<c::expression::CRecord<crate::__pgrx_c_bindings::GenericCosts> as c::expression::OrdinaryField<Field_numNonLeafPages>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numNonLeafPages) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::InternalGrant>() == 72);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::InternalGrant>() == 8);
};
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::InternalGrant, behavior) == 64);
#[cfg(feature = "pg19")]
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
    const OFFSET: usize = 64;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PGChecksummablePage>() == 8192);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PGChecksummablePage>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PageXLogRecPtr>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PageXLogRecPtr>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PageXLogRecPtr, lsn) == 0);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PageXLogRecPtr>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PageXLogRecPtr, <<c::expression::CRecord<crate::__pgrx_c_bindings::PageXLogRecPtr> as c::expression::OrdinaryField<Field_lsn>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lsn) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_curbyte;
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SortTuple, curbyte) == 17);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_curbyte>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SortTuple>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 17;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SortTuple,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SortTuple> as c::expression::OrdinaryField<
        Field_curbyte,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).curbyte) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_apply_reply_requested;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WalRcvData, apply_reply_requested) == 2240
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_apply_reply_requested>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 2240;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WalRcvData, <<c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvData> as c::expression::OrdinaryField<Field_apply_reply_requested>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).apply_reply_requested) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BufferDesc>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BufferDesc>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BufferDesc, wait_backend_pgprocno) == 32
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wait_backend_pgprocno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BufferDesc>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::HASHCTL>() == 80);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::HASHCTL>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HASHCTL, keysize) == 8);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_keysize>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HASHCTL>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HASHCTL, entrysize) == 16);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_entrysize>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HASHCTL>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::IoMethodOps>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::IoMethodOps>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry>() == 96);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_mem_exceeded_count;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry, mem_exceeded_count)
        == 48
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mem_exceeded_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry> as c::expression::OrdinaryField<Field_mem_exceeded_count>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mem_exceeded_count) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry, total_txns) == 56
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_total_txns>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry, total_bytes) == 64
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_total_bytes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_slotsync_skip_count;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry,
        slotsync_skip_count
    ) == 72
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_slotsync_skip_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry> as c::expression::OrdinaryField<Field_slotsync_skip_count>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).slotsync_skip_count) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_slotsync_last_skip;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry, slotsync_last_skip)
        == 80
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_slotsync_last_skip>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry> as c::expression::OrdinaryField<Field_slotsync_last_skip>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).slotsync_last_skip) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry,
        stat_reset_timestamp
    ) == 88
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stat_reset_timestamp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ShmemCallbacks>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ShmemCallbacks>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ShmemCallbacks {}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ShmemCallbacks, flags) == 0);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ShmemCallbacks>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ShmemCallbacks, <<c::expression::CRecord<crate::__pgrx_c_bindings::ShmemCallbacks> as c::expression::OrdinaryField<Field_flags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).flags) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ShmemHashOpts>() == 144);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ShmemHashOpts>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ShmemHashOpts {}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ShmemHashOpts, nelems) == 40);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nelems>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ShmemHashOpts>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ShmemHashOpts, <<c::expression::CRecord<crate::__pgrx_c_bindings::ShmemHashOpts> as c::expression::OrdinaryField<Field_nelems>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nelems) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_hash_flags;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ShmemHashOpts, hash_flags) == 128);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ShmemHashOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ShmemHashOpts, <<c::expression::CRecord<crate::__pgrx_c_bindings::ShmemHashOpts> as c::expression::OrdinaryField<Field_hash_flags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hash_flags) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ShmemStructOpts>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ShmemStructOpts>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ShmemStructOpts {}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ShmemStructOpts, size) == 8);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ShmemStructOpts>
{
    type Member = c::expression::CIntegerStorage<c::CLong, isize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ShmemStructOpts, <<c::expression::CRecord<crate::__pgrx_c_bindings::ShmemStructOpts> as c::expression::OrdinaryField<Field_size>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).size) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_alignment;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ShmemStructOpts, alignment) == 16);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_alignment>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ShmemStructOpts>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ShmemStructOpts, <<c::expression::CRecord<crate::__pgrx_c_bindings::ShmemStructOpts> as c::expression::OrdinaryField<Field_alignment>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).alignment) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_nslots;
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SlruOpts>() == 104);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SlruOpts>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SlruOpts {}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SlruOpts, nslots) == 48);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nslots>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SlruOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SlruOpts,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SlruOpts> as c::expression::OrdinaryField<
        Field_nslots,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nslots) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_nlsns;
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SlruOpts, nlsns) == 52);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nlsns>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SlruOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SlruOpts,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SlruOpts> as c::expression::OrdinaryField<
        Field_nlsns,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nlsns) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SlruOpts, sync_handler) == 56);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sync_handler>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SlruOpts>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__SyncRequestHandler,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SlruOpts,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SlruOpts> as c::expression::OrdinaryField<
        Field_sync_handler,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sync_handler) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SlruOpts, long_segment_names) == 72);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_long_segment_names>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SlruOpts>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SlruOpts,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SlruOpts> as c::expression::OrdinaryField<
        Field_long_segment_names,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).long_segment_names) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_buffer_tranche_id;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SlruOpts, buffer_tranche_id) == 96);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_buffer_tranche_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SlruOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SlruOpts,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SlruOpts> as c::expression::OrdinaryField<
        Field_buffer_tranche_id,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).buffer_tranche_id) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_bank_tranche_id;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SlruOpts, bank_tranche_id) == 100);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bank_tranche_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SlruOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 100;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SlruOpts,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SlruOpts> as c::expression::OrdinaryField<
        Field_bank_tranche_id,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).bank_tranche_id) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_scan_getnextslot;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TableAmRoutine, scan_getnextslot) == 40
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_scan_getnextslot>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__enum__ScanDirection__0c__struct__TupleTableSlot__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine> as c::expression::OrdinaryField<Field_scan_getnextslot>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).scan_getnextslot) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_index_fetch_tuple;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TableAmRoutine, index_fetch_tuple) == 112
);
#[cfg(feature = "pg19")]
const _: () = {
    assert!(
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
    assert!(
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
};
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_index_fetch_tuple>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__IndexFetchTableData__0p__0c__struct__ItemPointerData__0p__0c__struct__SnapshotData__0p__0c__struct__TupleTableSlot__0p__0c__0x5f426f6f6c__0p__0c__0x5f426f6f6c__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine> as c::expression::OrdinaryField<Field_index_fetch_tuple>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).index_fetch_tuple) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut crate::__pgrx_c_bindings::TupleTableSlot,
                    u32,
                    u32,
                    *mut crate::__pgrx_c_bindings::BulkInsertStateData,
                ) -> (),
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut crate::__pgrx_c_bindings::TupleTableSlot,
                    u32,
                    u32,
                    *mut crate::__pgrx_c_bindings::BulkInsertStateData,
                ) -> (),
            >,
        >() == 8
    );
};
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tuple_insert>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_void__0l__struct__RelationData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__unsigned__int__0c__struct__BulkInsertStateData__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut crate::__pgrx_c_bindings::TupleTableSlot,
                    u32,
                    u32,
                    *mut crate::__pgrx_c_bindings::BulkInsertStateData,
                    u32,
                ) -> (),
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut crate::__pgrx_c_bindings::TupleTableSlot,
                    u32,
                    u32,
                    *mut crate::__pgrx_c_bindings::BulkInsertStateData,
                    u32,
                ) -> (),
            >,
        >() == 8
    );
};
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tuple_insert_speculative>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_void__0l__struct__RelationData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__unsigned__int__0c__struct__BulkInsertStateData__0p__0c__unsigned__int__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 168;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut *mut crate::__pgrx_c_bindings::TupleTableSlot,
                    i32,
                    u32,
                    u32,
                    *mut crate::__pgrx_c_bindings::BulkInsertStateData,
                ) -> (),
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut *mut crate::__pgrx_c_bindings::TupleTableSlot,
                    i32,
                    u32,
                    u32,
                    *mut crate::__pgrx_c_bindings::BulkInsertStateData,
                ) -> (),
            >,
        >() == 8
    );
};
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_multi_insert>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_void__0l__struct__RelationData__0p__0c__struct__TupleTableSlot__0p__0p__0c__int__0c__unsigned__int__0c__unsigned__int__0c__struct__BulkInsertStateData__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 184;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut crate::__pgrx_c_bindings::ItemPointerData,
                    u32,
                    u32,
                    *mut crate::__pgrx_c_bindings::SnapshotData,
                    *mut crate::__pgrx_c_bindings::SnapshotData,
                    bool,
                    *mut crate::__pgrx_c_bindings::TM_FailureData,
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
                    u32,
                    u32,
                    *mut crate::__pgrx_c_bindings::SnapshotData,
                    *mut crate::__pgrx_c_bindings::SnapshotData,
                    bool,
                    *mut crate::__pgrx_c_bindings::TM_FailureData,
                ) -> u32,
            >,
        >() == 8
    );
};
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tuple_delete>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_enum__TM_Result__0l__struct__RelationData__0p__0c__struct__ItemPointerData__0p__0c__unsigned__int__0c__unsigned__int__0c__struct__SnapshotData__0p__0c__struct__SnapshotData__0p__0c__0x5f426f6f6c__0c__struct__TM_FailureData__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 192;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut crate::__pgrx_c_bindings::ItemPointerData,
                    *mut crate::__pgrx_c_bindings::TupleTableSlot,
                    u32,
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
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tuple_update>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_enum__TM_Result__0l__struct__RelationData__0p__0c__struct__ItemPointerData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__unsigned__int__0c__struct__SnapshotData__0p__0c__struct__SnapshotData__0p__0c__0x5f426f6f6c__0c__struct__TM_FailureData__0p__0c__enum__LockTupleMode__0p__0c__enum__TU_UpdateIndexes__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 200;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, u32) -> (),
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, u32) -> (),
            >,
        >() == 8
    );
};
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_finish_bulk_insert>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_void__0l__struct__RelationData__0p__0c__unsigned__int__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 216;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut crate::__pgrx_c_bindings::RelationData,
                    bool,
                    crate::__pgrx_c_bindings::TransactionId,
                    *mut crate::__pgrx_c_bindings::SnapshotData,
                    *mut crate::__pgrx_c_bindings::TransactionId,
                    *mut crate::__pgrx_c_bindings::MultiXactId,
                    *mut f64,
                    *mut f64,
                    *mut f64,
                ) -> (),
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut crate::__pgrx_c_bindings::RelationData,
                    bool,
                    crate::__pgrx_c_bindings::TransactionId,
                    *mut crate::__pgrx_c_bindings::SnapshotData,
                    *mut crate::__pgrx_c_bindings::TransactionId,
                    *mut crate::__pgrx_c_bindings::MultiXactId,
                    *mut f64,
                    *mut f64,
                    *mut f64,
                ) -> (),
            >,
        >() == 8
    );
};
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relation_copy_for_cluster>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_void__0l__struct__RelationData__0p__0c__struct__RelationData__0p__0c__struct__RelationData__0p__0c__0x5f426f6f6c__0c__unsigned__int__0c__struct__SnapshotData__0p__0c__unsigned__int__0p__0c__unsigned__int__0p__0c__double__0p__0c__double__0p__0c__double__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 248;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *const crate::__pgrx_c_bindings::VacuumParams,
                    *mut crate::__pgrx_c_bindings::BufferAccessStrategyData,
                ) -> (),
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *const crate::__pgrx_c_bindings::VacuumParams,
                    *mut crate::__pgrx_c_bindings::BufferAccessStrategyData,
                ) -> (),
            >,
        >() == 8
    );
};
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relation_vacuum>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_void__0l__struct__RelationData__0p__0c__const__struct__VacuumParams__0p__0c__struct__BufferAccessStrategyData__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 256;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::TableScanDescData,
                    *mut f64,
                    *mut f64,
                    *mut crate::__pgrx_c_bindings::TupleTableSlot,
                ) -> bool,
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::TableScanDescData,
                    *mut f64,
                    *mut f64,
                    *mut crate::__pgrx_c_bindings::TupleTableSlot,
                ) -> bool,
            >,
        >() == 8
    );
};
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_scan_analyze_next_tuple>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__double__0p__0c__double__0p__0c__struct__TupleTableSlot__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 272;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_scan_bitmap_next_tuple;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TableAmRoutine, scan_bitmap_next_tuple)
        == 336
);
#[cfg(feature = "pg19")]
const _: () = {
    assert!(
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
    assert!(
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
};
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_scan_bitmap_next_tuple>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__struct__TupleTableSlot__0p__0c__0x5f426f6f6c__0p__0c__unsigned__long__0p__0c__unsigned__long__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 336;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine> as c::expression::OrdinaryField<Field_scan_bitmap_next_tuple>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).scan_bitmap_next_tuple) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_scan_sample_next_block;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TableAmRoutine, scan_sample_next_block)
        == 344
);
#[cfg(feature = "pg19")]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::TableScanDescData,
                    *mut crate::__pgrx_c_bindings::SampleScanState,
                ) -> bool,
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::TableScanDescData,
                    *mut crate::__pgrx_c_bindings::SampleScanState,
                ) -> bool,
            >,
        >() == 8
    );
};
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_scan_sample_next_block>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__struct__SampleScanState__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 344;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine> as c::expression::OrdinaryField<Field_scan_sample_next_block>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).scan_sample_next_block) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_scan_sample_next_tuple;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TableAmRoutine, scan_sample_next_tuple)
        == 352
);
#[cfg(feature = "pg19")]
const _: () = {
    assert!(
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
    assert!(
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
};
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_scan_sample_next_tuple>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__struct__SampleScanState__0p__0c__struct__TupleTableSlot__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 352;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine> as c::expression::OrdinaryField<Field_scan_sample_next_tuple>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).scan_sample_next_tuple) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_frequency_khz;
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TscClockSourceInfo>() == 136);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TscClockSourceInfo>() == 4);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TscClockSourceInfo {}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TscClockSourceInfo, frequency_khz) == 0
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_frequency_khz>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TscClockSourceInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TscClockSourceInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::TscClockSourceInfo> as c::expression::OrdinaryField<Field_frequency_khz>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).frequency_khz) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_calibrated_frequency_khz;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TscClockSourceInfo, calibrated_frequency_khz)
        == 4
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_calibrated_frequency_khz>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TscClockSourceInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TscClockSourceInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::TscClockSourceInfo> as c::expression::OrdinaryField<Field_calibrated_frequency_khz>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).calibrated_frequency_khz) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_getsomeattrs;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleTableSlotOps, getsomeattrs) == 32
);
#[cfg(feature = "pg19")]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::TupleTableSlot,
                    i32,
                ) -> (),
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::TupleTableSlot,
                    i32,
                ) -> (),
            >,
        >() == 8
    );
};
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_getsomeattrs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlotOps>
{
    type Member = c::expression::CFunction<
        crate::__pgrx_c_generated::Signature_void__0l__struct__TupleTableSlot__0p__0c__int__0r,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TupleTableSlotOps, <<c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlotOps> as c::expression::OrdinaryField<Field_getsomeattrs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).getsomeattrs) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::VacuumParams>() == 64);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::VacuumParams>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_log_vacuum_min_duration;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::VacuumParams, log_vacuum_min_duration) == 24
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_log_vacuum_min_duration>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacuumParams>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VacuumParams, <<c::expression::CRecord<crate::__pgrx_c_bindings::VacuumParams> as c::expression::OrdinaryField<Field_log_vacuum_min_duration>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).log_vacuum_min_duration) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_log_analyze_min_duration;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::VacuumParams, log_analyze_min_duration) == 28
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_log_analyze_min_duration>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacuumParams>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VacuumParams, <<c::expression::CRecord<crate::__pgrx_c_bindings::VacuumParams> as c::expression::OrdinaryField<Field_log_analyze_min_duration>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).log_analyze_min_duration) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacuumParams, index_cleanup) == 32);
#[cfg(feature = "pg19")]
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
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacuumParams, truncate) == 36);
#[cfg(feature = "pg19")]
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
    const OFFSET: usize = 36;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacuumParams, toast_parent) == 40);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_toast_parent>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacuumParams>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacuumParams, nworkers) == 56);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nworkers>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacuumParams>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::WalUsage>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::WalUsage>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_wal_fpi_bytes;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WalUsage, wal_fpi_bytes) == 24);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wal_fpi_bytes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalUsage>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::WalUsage,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::WalUsage> as c::expression::OrdinaryField<
        Field_wal_fpi_bytes,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wal_fpi_bytes) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WalUsage, wal_buffers_full) == 32);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wal_buffers_full>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalUsage>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::collate_methods>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::collate_methods>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::collate_methods, strxfrm_is_safe) == 48
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_strxfrm_is_safe>
    for c::expression::CRecord<crate::__pgrx_c_bindings::collate_methods>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::config_enum>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::config_enum>() == 8);
};
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_enum, boot_val) == 8);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_boot_val>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_enum>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_enum, reset_val) == 48);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_reset_val>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_enum>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::config_generic>() == 216);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::config_generic>() == 8);
};
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_generic, sourceline) == 144);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sourceline>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_generic>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::pg_signal_info>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::pg_signal_info>() == 4);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::pg_signal_info {}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_signal_info, pid) == 0);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_signal_info>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pg_signal_info, <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_signal_info> as c::expression::OrdinaryField<Field_pid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pid) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_uid;
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_signal_info, uid) == 4);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_uid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_signal_info>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pg_signal_info, <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_signal_info> as c::expression::OrdinaryField<Field_uid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).uid) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AggClauseInfo>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AggClauseInfo>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AggClauseInfo {}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggClauseInfo, type_) == 0);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggClauseInfo>
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
const _: Projection<crate::__pgrx_c_bindings::AggClauseInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggClauseInfo> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_subtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterDomainStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__AlterDomainType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterPublicationStmt, action) == 32);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_action>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterPublicationStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__AlterPublicationAction,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterPublicationStmt, for_all_tables) == 36
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_for_all_tables>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterPublicationStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_for_all_sequences;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterPublicationStmt, for_all_sequences)
        == 37
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_for_all_sequences>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterPublicationStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 37;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterPublicationStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterPublicationStmt> as c::expression::OrdinaryField<Field_for_all_sequences>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).for_all_sequences) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AlterSubscriptionStmt>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AlterSubscriptionStmt>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterSubscriptionStmt, publication) == 32
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_publication>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterSubscriptionStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterSubscriptionStmt, options) == 40
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterSubscriptionStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_child_append_relid_sets;
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Append>() == 144);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Append>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::Append, child_append_relid_sets) == 112
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_child_append_relid_sets>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Append>
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
const _: Projection<
    crate::__pgrx_c_bindings::Append,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Append> as c::expression::OrdinaryField<
        Field_child_append_relid_sets,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).child_append_relid_sets) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Append, appendplans) == 120);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_appendplans>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Append>
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
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Append, nasyncplans) == 128);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nasyncplans>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Append>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Append, first_partial_plan) == 132);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_first_partial_plan>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Append>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 132;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Append, part_prune_index) == 136);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_part_prune_index>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Append>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AppendPath>() == 112);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AppendPath>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AppendPath, child_append_relid_sets) == 104
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_child_append_relid_sets>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AppendPath>
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
const _: Projection<crate::__pgrx_c_bindings::AppendPath, <<c::expression::CRecord<crate::__pgrx_c_bindings::AppendPath> as c::expression::OrdinaryField<Field_child_append_relid_sets>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).child_append_relid_sets) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AutoVacOpts>() == 96);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AutoVacOpts>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_autovacuum_parallel_workers;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, autovacuum_parallel_workers)
        == 4
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_autovacuum_parallel_workers>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AutoVacOpts, <<c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts> as c::expression::OrdinaryField<Field_autovacuum_parallel_workers>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).autovacuum_parallel_workers) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, vacuum_threshold) == 8);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vacuum_threshold>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, vacuum_max_threshold) == 12
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vacuum_max_threshold>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, vacuum_ins_threshold) == 16
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vacuum_ins_threshold>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, analyze_threshold) == 20
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_analyze_threshold>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, vacuum_cost_limit) == 24
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vacuum_cost_limit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, freeze_min_age) == 28);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_freeze_min_age>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, freeze_max_age) == 32);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_freeze_max_age>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, freeze_table_age) == 36);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_freeze_table_age>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, multixact_freeze_min_age) == 40
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_multixact_freeze_min_age>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, multixact_freeze_max_age) == 44
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_multixact_freeze_max_age>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, multixact_freeze_table_age)
        == 48
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_multixact_freeze_table_age>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, log_vacuum_min_duration) == 52
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_log_vacuum_min_duration>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AutoVacOpts, <<c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts> as c::expression::OrdinaryField<Field_log_vacuum_min_duration>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).log_vacuum_min_duration) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, log_analyze_min_duration) == 56
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_log_analyze_min_duration>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AutoVacOpts, <<c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts> as c::expression::OrdinaryField<Field_log_analyze_min_duration>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).log_analyze_min_duration) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::BitmapHeapScanInstrumentation>() == 72
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::BitmapHeapScanInstrumentation>() == 8
    );
};
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BitmapHeapScanState>() == 344);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BitmapHeapScanState>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BitmapHeapScanState, initialized) == 312
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_initialized>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BitmapHeapScanState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 312;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BitmapHeapScanState, recheck) == 336);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_recheck>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BitmapHeapScanState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 336;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BufferHeapTupleTableSlot>() == 120);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BufferHeapTupleTableSlot>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BufferHeapTupleTableSlot, buffer) == 112
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_buffer>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BufferHeapTupleTableSlot>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BufferManagerRelation, rel) == 0);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rel>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BufferManagerRelation>
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
const _: Projection<crate::__pgrx_c_bindings::BufferManagerRelation, <<c::expression::CRecord<crate::__pgrx_c_bindings::BufferManagerRelation> as c::expression::OrdinaryField<Field_rel>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rel) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_smgr;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BufferManagerRelation, smgr) == 8);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_smgr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BufferManagerRelation>
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
const _: Projection<crate::__pgrx_c_bindings::BufferManagerRelation, <<c::expression::CRecord<crate::__pgrx_c_bindings::BufferManagerRelation> as c::expression::OrdinaryField<Field_smgr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).smgr) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CheckPoint>() == 96);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CheckPoint>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_logicalDecodingEnabled;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, logicalDecodingEnabled) == 24
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_logicalDecodingEnabled>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckPoint, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint> as c::expression::OrdinaryField<Field_logicalDecodingEnabled>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).logicalDecodingEnabled) };
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, nextOid) == 40);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nextOid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, nextMulti) == 44);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nextMulti>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::MultiXactId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, nextMultiOffset) == 48);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nextMultiOffset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, oldestXid) == 56);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldestXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, oldestXidDB) == 60);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldestXidDB>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 60;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, oldestMulti) == 64);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldestMulti>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::MultiXactId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, oldestMultiDB) == 68);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldestMultiDB>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 68;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, time) == 72);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, oldestCommitTsXid) == 80);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldestCommitTsXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, newestCommitTsXid) == 84);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_newestCommitTsXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 84;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, oldestActiveXid) == 88);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldestActiveXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CheckPointStmt>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CheckPointStmt>() == 8);
};
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPointStmt, options) == 8);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPointStmt>
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
const _: Projection<crate::__pgrx_c_bindings::CheckPointStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckPointStmt> as c::expression::OrdinaryField<Field_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CompactAttribute>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CompactAttribute>() == 2);
};
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attcacheoff>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CompactAttribute>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CompactAttribute, attlen) == 2);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attlen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CompactAttribute>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 2;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CompactAttribute, attbyval) == 4);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attbyval>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CompactAttribute>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CompactAttribute, attalignby) == 5);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attalignby>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CompactAttribute>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 5;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::CompactAttribute, _bitfield_1) * 8 + 0
            == 48
    );
};
#[cfg(feature = "pg19")]
unsafe extern "C" {
    fn __pgrx_bitfield__get__CompactAttribute__attispackable(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set__CompactAttribute__attispackable(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
    fn __pgrx_bitfield__get_volatile__CompactAttribute__attispackable(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set_volatile__CompactAttribute__attispackable(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
    fn __pgrx_bitfield__get_unaligned__CompactAttribute__attispackable(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set_unaligned__CompactAttribute__attispackable(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
    fn __pgrx_bitfield__get_unaligned_volatile__CompactAttribute__attispackable(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set_unaligned_volatile__CompactAttribute__attispackable(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Bitfield_CompactAttribute__attispackable<Q: c::expression::Qualifier> {
    address: Q::Raw<crate::__pgrx_c_bindings::CompactAttribute>,
    access: c::expression::Access,
}
#[cfg(feature = "pg19")]
impl<Q: c::expression::Qualifier> Copy for Bitfield_CompactAttribute__attispackable<Q> {}
#[cfg(feature = "pg19")]
impl<Q: c::expression::Qualifier> Clone for Bitfield_CompactAttribute__attispackable<Q> {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "pg19")]
// SAFETY: Clang and bindgen layout witnesses agree. This projects the record address without a reference or record read.
unsafe impl<Q: c::expression::Qualifier> c::expression::Field<Field_attispackable, Q>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CompactAttribute>
{
    type Output = Bitfield_CompactAttribute__attispackable<Q>;
    unsafe fn project(base: c::expression::Place<Self, Q>) -> Self::Output {
        Bitfield_CompactAttribute__attispackable::<Q> {
            address: <Q as c::expression::Qualifier>::from_mut(base.pointer().as_mut_address()),
            access: c::expression::Access {
                volatile: base.access().volatile || false,
                unaligned: base.access().unaligned,
            },
        }
    }
}
#[cfg(feature = "pg19")]
impl<Q: c::expression::Qualifier> c::expression::VolatilePlace
    for Bitfield_CompactAttribute__attispackable<Q>
{
    type Qualified = Self;
    fn qualify_volatile(mut self) -> Self::Qualified {
        self.access.volatile = true;
        self
    }
}
#[cfg(feature = "pg19")]
// SAFETY: The same-profile C compiler owns bitfield storage reads, including partially initialized neighbor/padding bits. The marker retains declared size and compiler-proved arithmetic promotion.
unsafe impl<Q: c::expression::Qualifier> c::expression::ReadPlace
    for Bitfield_CompactAttribute__attispackable<Q>
{
    type Object = c::CBool;
    type Type = c::expression::CBitfield<c::CBool, c::CInt>;
    unsafe fn load(self) -> <Self::Type as c::expression::CType>::Value {
        let pointer = Q::into_mut(self.address).cast_const(); // SAFETY: The caller establishes the original C field load contract; helpers perform only that field access.
        let storage = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__get_unaligned_volatile__CompactAttribute__attispackable(
                        pointer,
                    )
                } else {
                    __pgrx_bitfield__get_unaligned__CompactAttribute__attispackable(pointer)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__get_volatile__CompactAttribute__attispackable(pointer)
            } else {
                __pgrx_bitfield__get__CompactAttribute__attispackable(pointer)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(storage)
    }
}
#[cfg(feature = "pg19")]
// SAFETY: Write capability retains ReadWrite qualification. Original C performs declared-base conversion and width narrowing without Rust byte reads.
unsafe impl c::expression::WritePlace
    for Bitfield_CompactAttribute__attispackable<c::expression::ReadWrite>
{
    type Assignment = c::CBool;
    unsafe fn store(
        self,
        value: <Self::Assignment as c::expression::CType>::Value,
    ) -> <Self::Type as c::expression::CType>::Value {
        let value = <Self::Assignment as c::expression::CType>::into_storage(value); // SAFETY: The caller establishes writable field storage; this helper evaluates one original-C assignment and returns its narrowed result.
        let stored = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__set_unaligned_volatile__CompactAttribute__attispackable(
                        self.address,
                        value,
                    )
                } else {
                    __pgrx_bitfield__set_unaligned__CompactAttribute__attispackable(
                        self.address,
                        value,
                    )
                }
            } else if self.access.volatile {
                __pgrx_bitfield__set_volatile__CompactAttribute__attispackable(self.address, value)
            } else {
                __pgrx_bitfield__set__CompactAttribute__attispackable(self.address, value)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(stored)
    }
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::CompactAttribute, _bitfield_1) * 8 + 1
            == 49
    );
};
#[cfg(feature = "pg19")]
unsafe extern "C" {
    fn __pgrx_bitfield__get__CompactAttribute__atthasmissing(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set__CompactAttribute__atthasmissing(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
    fn __pgrx_bitfield__get_volatile__CompactAttribute__atthasmissing(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set_volatile__CompactAttribute__atthasmissing(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
    fn __pgrx_bitfield__get_unaligned__CompactAttribute__atthasmissing(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set_unaligned__CompactAttribute__atthasmissing(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
    fn __pgrx_bitfield__get_unaligned_volatile__CompactAttribute__atthasmissing(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set_unaligned_volatile__CompactAttribute__atthasmissing(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Bitfield_CompactAttribute__atthasmissing<Q: c::expression::Qualifier> {
    address: Q::Raw<crate::__pgrx_c_bindings::CompactAttribute>,
    access: c::expression::Access,
}
#[cfg(feature = "pg19")]
impl<Q: c::expression::Qualifier> Copy for Bitfield_CompactAttribute__atthasmissing<Q> {}
#[cfg(feature = "pg19")]
impl<Q: c::expression::Qualifier> Clone for Bitfield_CompactAttribute__atthasmissing<Q> {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "pg19")]
// SAFETY: Clang and bindgen layout witnesses agree. This projects the record address without a reference or record read.
unsafe impl<Q: c::expression::Qualifier> c::expression::Field<Field_atthasmissing, Q>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CompactAttribute>
{
    type Output = Bitfield_CompactAttribute__atthasmissing<Q>;
    unsafe fn project(base: c::expression::Place<Self, Q>) -> Self::Output {
        Bitfield_CompactAttribute__atthasmissing::<Q> {
            address: <Q as c::expression::Qualifier>::from_mut(base.pointer().as_mut_address()),
            access: c::expression::Access {
                volatile: base.access().volatile || false,
                unaligned: base.access().unaligned,
            },
        }
    }
}
#[cfg(feature = "pg19")]
impl<Q: c::expression::Qualifier> c::expression::VolatilePlace
    for Bitfield_CompactAttribute__atthasmissing<Q>
{
    type Qualified = Self;
    fn qualify_volatile(mut self) -> Self::Qualified {
        self.access.volatile = true;
        self
    }
}
#[cfg(feature = "pg19")]
// SAFETY: The same-profile C compiler owns bitfield storage reads, including partially initialized neighbor/padding bits. The marker retains declared size and compiler-proved arithmetic promotion.
unsafe impl<Q: c::expression::Qualifier> c::expression::ReadPlace
    for Bitfield_CompactAttribute__atthasmissing<Q>
{
    type Object = c::CBool;
    type Type = c::expression::CBitfield<c::CBool, c::CInt>;
    unsafe fn load(self) -> <Self::Type as c::expression::CType>::Value {
        let pointer = Q::into_mut(self.address).cast_const(); // SAFETY: The caller establishes the original C field load contract; helpers perform only that field access.
        let storage = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__get_unaligned_volatile__CompactAttribute__atthasmissing(
                        pointer,
                    )
                } else {
                    __pgrx_bitfield__get_unaligned__CompactAttribute__atthasmissing(pointer)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__get_volatile__CompactAttribute__atthasmissing(pointer)
            } else {
                __pgrx_bitfield__get__CompactAttribute__atthasmissing(pointer)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(storage)
    }
}
#[cfg(feature = "pg19")]
// SAFETY: Write capability retains ReadWrite qualification. Original C performs declared-base conversion and width narrowing without Rust byte reads.
unsafe impl c::expression::WritePlace
    for Bitfield_CompactAttribute__atthasmissing<c::expression::ReadWrite>
{
    type Assignment = c::CBool;
    unsafe fn store(
        self,
        value: <Self::Assignment as c::expression::CType>::Value,
    ) -> <Self::Type as c::expression::CType>::Value {
        let value = <Self::Assignment as c::expression::CType>::into_storage(value); // SAFETY: The caller establishes writable field storage; this helper evaluates one original-C assignment and returns its narrowed result.
        let stored = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__set_unaligned_volatile__CompactAttribute__atthasmissing(
                        self.address,
                        value,
                    )
                } else {
                    __pgrx_bitfield__set_unaligned__CompactAttribute__atthasmissing(
                        self.address,
                        value,
                    )
                }
            } else if self.access.volatile {
                __pgrx_bitfield__set_volatile__CompactAttribute__atthasmissing(self.address, value)
            } else {
                __pgrx_bitfield__set__CompactAttribute__atthasmissing(self.address, value)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(stored)
    }
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::CompactAttribute, _bitfield_1) * 8 + 2
            == 50
    );
};
#[cfg(feature = "pg19")]
unsafe extern "C" {
    fn __pgrx_bitfield__get__CompactAttribute__attisdropped(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set__CompactAttribute__attisdropped(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
    fn __pgrx_bitfield__get_volatile__CompactAttribute__attisdropped(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set_volatile__CompactAttribute__attisdropped(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
    fn __pgrx_bitfield__get_unaligned__CompactAttribute__attisdropped(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set_unaligned__CompactAttribute__attisdropped(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
    fn __pgrx_bitfield__get_unaligned_volatile__CompactAttribute__attisdropped(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set_unaligned_volatile__CompactAttribute__attisdropped(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Bitfield_CompactAttribute__attisdropped<Q: c::expression::Qualifier> {
    address: Q::Raw<crate::__pgrx_c_bindings::CompactAttribute>,
    access: c::expression::Access,
}
#[cfg(feature = "pg19")]
impl<Q: c::expression::Qualifier> Copy for Bitfield_CompactAttribute__attisdropped<Q> {}
#[cfg(feature = "pg19")]
impl<Q: c::expression::Qualifier> Clone for Bitfield_CompactAttribute__attisdropped<Q> {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "pg19")]
// SAFETY: Clang and bindgen layout witnesses agree. This projects the record address without a reference or record read.
unsafe impl<Q: c::expression::Qualifier> c::expression::Field<Field_attisdropped, Q>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CompactAttribute>
{
    type Output = Bitfield_CompactAttribute__attisdropped<Q>;
    unsafe fn project(base: c::expression::Place<Self, Q>) -> Self::Output {
        Bitfield_CompactAttribute__attisdropped::<Q> {
            address: <Q as c::expression::Qualifier>::from_mut(base.pointer().as_mut_address()),
            access: c::expression::Access {
                volatile: base.access().volatile || false,
                unaligned: base.access().unaligned,
            },
        }
    }
}
#[cfg(feature = "pg19")]
impl<Q: c::expression::Qualifier> c::expression::VolatilePlace
    for Bitfield_CompactAttribute__attisdropped<Q>
{
    type Qualified = Self;
    fn qualify_volatile(mut self) -> Self::Qualified {
        self.access.volatile = true;
        self
    }
}
#[cfg(feature = "pg19")]
// SAFETY: The same-profile C compiler owns bitfield storage reads, including partially initialized neighbor/padding bits. The marker retains declared size and compiler-proved arithmetic promotion.
unsafe impl<Q: c::expression::Qualifier> c::expression::ReadPlace
    for Bitfield_CompactAttribute__attisdropped<Q>
{
    type Object = c::CBool;
    type Type = c::expression::CBitfield<c::CBool, c::CInt>;
    unsafe fn load(self) -> <Self::Type as c::expression::CType>::Value {
        let pointer = Q::into_mut(self.address).cast_const(); // SAFETY: The caller establishes the original C field load contract; helpers perform only that field access.
        let storage = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__get_unaligned_volatile__CompactAttribute__attisdropped(pointer)
                } else {
                    __pgrx_bitfield__get_unaligned__CompactAttribute__attisdropped(pointer)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__get_volatile__CompactAttribute__attisdropped(pointer)
            } else {
                __pgrx_bitfield__get__CompactAttribute__attisdropped(pointer)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(storage)
    }
}
#[cfg(feature = "pg19")]
// SAFETY: Write capability retains ReadWrite qualification. Original C performs declared-base conversion and width narrowing without Rust byte reads.
unsafe impl c::expression::WritePlace
    for Bitfield_CompactAttribute__attisdropped<c::expression::ReadWrite>
{
    type Assignment = c::CBool;
    unsafe fn store(
        self,
        value: <Self::Assignment as c::expression::CType>::Value,
    ) -> <Self::Type as c::expression::CType>::Value {
        let value = <Self::Assignment as c::expression::CType>::into_storage(value); // SAFETY: The caller establishes writable field storage; this helper evaluates one original-C assignment and returns its narrowed result.
        let stored = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__set_unaligned_volatile__CompactAttribute__attisdropped(
                        self.address,
                        value,
                    )
                } else {
                    __pgrx_bitfield__set_unaligned__CompactAttribute__attisdropped(
                        self.address,
                        value,
                    )
                }
            } else if self.access.volatile {
                __pgrx_bitfield__set_volatile__CompactAttribute__attisdropped(self.address, value)
            } else {
                __pgrx_bitfield__set__CompactAttribute__attisdropped(self.address, value)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(stored)
    }
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::CompactAttribute, _bitfield_1) * 8 + 3
            == 51
    );
};
#[cfg(feature = "pg19")]
unsafe extern "C" {
    fn __pgrx_bitfield__get__CompactAttribute__attgenerated(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set__CompactAttribute__attgenerated(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
    fn __pgrx_bitfield__get_volatile__CompactAttribute__attgenerated(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set_volatile__CompactAttribute__attgenerated(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
    fn __pgrx_bitfield__get_unaligned__CompactAttribute__attgenerated(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set_unaligned__CompactAttribute__attgenerated(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
    fn __pgrx_bitfield__get_unaligned_volatile__CompactAttribute__attgenerated(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set_unaligned_volatile__CompactAttribute__attgenerated(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Bitfield_CompactAttribute__attgenerated<Q: c::expression::Qualifier> {
    address: Q::Raw<crate::__pgrx_c_bindings::CompactAttribute>,
    access: c::expression::Access,
}
#[cfg(feature = "pg19")]
impl<Q: c::expression::Qualifier> Copy for Bitfield_CompactAttribute__attgenerated<Q> {}
#[cfg(feature = "pg19")]
impl<Q: c::expression::Qualifier> Clone for Bitfield_CompactAttribute__attgenerated<Q> {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "pg19")]
// SAFETY: Clang and bindgen layout witnesses agree. This projects the record address without a reference or record read.
unsafe impl<Q: c::expression::Qualifier> c::expression::Field<Field_attgenerated, Q>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CompactAttribute>
{
    type Output = Bitfield_CompactAttribute__attgenerated<Q>;
    unsafe fn project(base: c::expression::Place<Self, Q>) -> Self::Output {
        Bitfield_CompactAttribute__attgenerated::<Q> {
            address: <Q as c::expression::Qualifier>::from_mut(base.pointer().as_mut_address()),
            access: c::expression::Access {
                volatile: base.access().volatile || false,
                unaligned: base.access().unaligned,
            },
        }
    }
}
#[cfg(feature = "pg19")]
impl<Q: c::expression::Qualifier> c::expression::VolatilePlace
    for Bitfield_CompactAttribute__attgenerated<Q>
{
    type Qualified = Self;
    fn qualify_volatile(mut self) -> Self::Qualified {
        self.access.volatile = true;
        self
    }
}
#[cfg(feature = "pg19")]
// SAFETY: The same-profile C compiler owns bitfield storage reads, including partially initialized neighbor/padding bits. The marker retains declared size and compiler-proved arithmetic promotion.
unsafe impl<Q: c::expression::Qualifier> c::expression::ReadPlace
    for Bitfield_CompactAttribute__attgenerated<Q>
{
    type Object = c::CBool;
    type Type = c::expression::CBitfield<c::CBool, c::CInt>;
    unsafe fn load(self) -> <Self::Type as c::expression::CType>::Value {
        let pointer = Q::into_mut(self.address).cast_const(); // SAFETY: The caller establishes the original C field load contract; helpers perform only that field access.
        let storage = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__get_unaligned_volatile__CompactAttribute__attgenerated(pointer)
                } else {
                    __pgrx_bitfield__get_unaligned__CompactAttribute__attgenerated(pointer)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__get_volatile__CompactAttribute__attgenerated(pointer)
            } else {
                __pgrx_bitfield__get__CompactAttribute__attgenerated(pointer)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(storage)
    }
}
#[cfg(feature = "pg19")]
// SAFETY: Write capability retains ReadWrite qualification. Original C performs declared-base conversion and width narrowing without Rust byte reads.
unsafe impl c::expression::WritePlace
    for Bitfield_CompactAttribute__attgenerated<c::expression::ReadWrite>
{
    type Assignment = c::CBool;
    unsafe fn store(
        self,
        value: <Self::Assignment as c::expression::CType>::Value,
    ) -> <Self::Type as c::expression::CType>::Value {
        let value = <Self::Assignment as c::expression::CType>::into_storage(value); // SAFETY: The caller establishes writable field storage; this helper evaluates one original-C assignment and returns its narrowed result.
        let stored = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__set_unaligned_volatile__CompactAttribute__attgenerated(
                        self.address,
                        value,
                    )
                } else {
                    __pgrx_bitfield__set_unaligned__CompactAttribute__attgenerated(
                        self.address,
                        value,
                    )
                }
            } else if self.access.volatile {
                __pgrx_bitfield__set_volatile__CompactAttribute__attgenerated(self.address, value)
            } else {
                __pgrx_bitfield__set__CompactAttribute__attgenerated(self.address, value)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(stored)
    }
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CompactAttribute, attnullability) == 7
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attnullability>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CompactAttribute>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 7;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ControlFileData>() == 312);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ControlFileData>() == 8);
};
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, unloggedLSN) == 136);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_unloggedLSN>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, minRecoveryPoint) == 144
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_minRecoveryPoint>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, minRecoveryPointTLI) == 152
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_minRecoveryPointTLI>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, backupStartPoint) == 160
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_backupStartPoint>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, backupEndPoint) == 168
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_backupEndPoint>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 168;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, backupEndRequired) == 176
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_backupEndRequired>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 176;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, wal_level) == 180);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wal_level>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 180;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, wal_log_hints) == 184
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wal_log_hints>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 184;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, MaxConnections) == 188
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_MaxConnections>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 188;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, max_worker_processes) == 192
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_max_worker_processes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 192;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, max_wal_senders) == 196
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_max_wal_senders>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 196;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, max_prepared_xacts) == 200
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_max_prepared_xacts>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 200;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, max_locks_per_xact) == 204
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_max_locks_per_xact>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 204;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, track_commit_timestamp)
        == 208
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_track_commit_timestamp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 208;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, maxAlign) == 212);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_maxAlign>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 212;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, blcksz) == 224);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_blcksz>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 224;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, relseg_size) == 228);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relseg_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 228;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_slru_pages_per_segment;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, slru_pages_per_segment)
        == 232
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_slru_pages_per_segment>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 232;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ControlFileData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData> as c::expression::OrdinaryField<Field_slru_pages_per_segment>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).slru_pages_per_segment) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, xlog_blcksz) == 236);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xlog_blcksz>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 236;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, xlog_seg_size) == 240
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xlog_seg_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, nameDataLen) == 244);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nameDataLen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 244;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, indexMaxKeys) == 248
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indexMaxKeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 248;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, toast_max_chunk_size) == 252
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_toast_max_chunk_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 252;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, loblksize) == 256);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_loblksize>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 256;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, float8ByVal) == 260);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_float8ByVal>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 260;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_data_checksum_version_init;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, data_checksum_version_init)
        == 264
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_data_checksum_version_init>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 264;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ControlFileData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData> as c::expression::OrdinaryField<Field_data_checksum_version_init>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).data_checksum_version_init) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, data_checksum_version)
        == 268
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_data_checksum_version>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 268;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, default_char_signedness)
        == 272
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_default_char_signedness>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 272;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, crc) == 308);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_crc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 308;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CopyFormatOptions>() == 192);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CopyFormatOptions>() == 8);
};
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, format) == 4);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_format>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__CopyFormat,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFormatOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions> as c::expression::OrdinaryField<Field_format>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).format) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, freeze) == 8);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_freeze>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, header_line) == 12
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_header_line>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_force_array;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, force_array) == 128
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_force_array>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFormatOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions> as c::expression::OrdinaryField<Field_force_array>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).force_array) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, force_null) == 136
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_force_null>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
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
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, force_null_all) == 144
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_force_null_all>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, convert_selectively)
        == 160
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_convert_selectively>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, on_error) == 164);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_on_error>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__CopyOnErrorChoice,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 164;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, log_verbosity) == 168
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_log_verbosity>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__CopyLogVerbosityChoice,
        c::CInt,
        i32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 168;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, reject_limit) == 176
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_reject_limit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 176;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, convert_select) == 184
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_convert_select>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 184;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CopyFromStateData>() == 592);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CopyFromStateData>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, cur_lineno) == 304
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cur_lineno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 304;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, relname_only) == 328
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relname_only>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 328;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, num_defaults) == 344
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_defaults>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 344;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, num_errors) == 376
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_errors>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 376;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_simd_enabled;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, simd_enabled) == 408
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_simd_enabled>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 408;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFromStateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData> as c::expression::OrdinaryField<Field_simd_enabled>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).simd_enabled) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, volatile_defexprs) == 424
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_volatile_defexprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 424;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, range_table) == 432
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_range_table>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 432;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, rteperminfos) == 440
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rteperminfos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 440;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, max_fields) == 488
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_max_fields>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 488;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, line_buf_valid) == 528
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_line_buf_valid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 528;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, input_buf_index) == 544
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_input_buf_index>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 544;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, input_buf_len) == 548
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_input_buf_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 548;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, input_reached_eof) == 552
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_input_reached_eof>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 552;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, input_reached_error)
        == 553
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_input_reached_error>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 553;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, raw_buf_index) == 568
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_raw_buf_index>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 568;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, raw_buf_len) == 572
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_raw_buf_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 572;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, raw_reached_eof) == 576
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_raw_reached_eof>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 576;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, bytes_processed) == 584
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bytes_processed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 584;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CreatePublicationStmt, for_all_sequences)
        == 33
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_for_all_sequences>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreatePublicationStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 33;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CreatePublicationStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreatePublicationStmt> as c::expression::OrdinaryField<Field_for_all_sequences>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).for_all_sequences) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CreateSubscriptionStmt>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CreateSubscriptionStmt>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateSubscriptionStmt, publication) == 32
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_publication>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateSubscriptionStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateSubscriptionStmt, options) == 40
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateSubscriptionStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::DecodingWorkerShared>() == 120);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::DecodingWorkerShared>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::DecodingWorkerShared {}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodingWorkerShared, initialized) == 0
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_initialized>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodingWorkerShared, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared> as c::expression::OrdinaryField<Field_initialized>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).initialized) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_lsn_upto;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodingWorkerShared, lsn_upto) == 8);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lsn_upto>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodingWorkerShared, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared> as c::expression::OrdinaryField<Field_lsn_upto>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lsn_upto) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodingWorkerShared, done) == 16);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_done>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodingWorkerShared, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared> as c::expression::OrdinaryField<Field_done>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).done) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_last_exported;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodingWorkerShared, last_exported) == 72
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_last_exported>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodingWorkerShared, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared> as c::expression::OrdinaryField<Field_last_exported>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).last_exported) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodingWorkerShared, mutex) == 76);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mutex>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 76;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodingWorkerShared, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared> as c::expression::OrdinaryField<Field_mutex>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mutex) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodingWorkerShared, dbid) == 80);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dbid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodingWorkerShared, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared> as c::expression::OrdinaryField<Field_dbid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dbid) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodingWorkerShared, roleid) == 84);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_roleid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 84;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodingWorkerShared, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared> as c::expression::OrdinaryField<Field_roleid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).roleid) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_lock_timeout;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodingWorkerShared, lock_timeout) == 88
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lock_timeout>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodingWorkerShared, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared> as c::expression::OrdinaryField<Field_lock_timeout>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lock_timeout) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_transaction_timeout;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodingWorkerShared, transaction_timeout)
        == 92
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_transaction_timeout>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 92;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodingWorkerShared, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared> as c::expression::OrdinaryField<Field_transaction_timeout>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).transaction_timeout) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodingWorkerShared, relid) == 96);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodingWorkerShared, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared> as c::expression::OrdinaryField<Field_relid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relid) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_backend_pid;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodingWorkerShared, backend_pid) == 112
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_backend_pid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodingWorkerShared, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared> as c::expression::OrdinaryField<Field_backend_pid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).backend_pid) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_backend_proc_number;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodingWorkerShared, backend_proc_number)
        == 116
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_backend_proc_number>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 116;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodingWorkerShared, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared> as c::expression::OrdinaryField<Field_backend_proc_number>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).backend_proc_number) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::DistinctColInfo>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::DistinctColInfo>() == 4);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::DistinctColInfo {}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DistinctColInfo, colno) == 0);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_colno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DistinctColInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DistinctColInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::DistinctColInfo> as c::expression::OrdinaryField<Field_colno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).colno) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_opid;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DistinctColInfo, opid) == 4);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_opid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DistinctColInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DistinctColInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::DistinctColInfo> as c::expression::OrdinaryField<Field_opid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).opid) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_collid;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DistinctColInfo, collid) == 8);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_collid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DistinctColInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DistinctColInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::DistinctColInfo> as c::expression::OrdinaryField<Field_collid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).collid) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ElidedNode>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ElidedNode>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ElidedNode {}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ElidedNode, type_) == 0);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ElidedNode>
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
const _: Projection<crate::__pgrx_c_bindings::ElidedNode, <<c::expression::CRecord<crate::__pgrx_c_bindings::ElidedNode> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ElidedNode, plan_node_id) == 4);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_plan_node_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ElidedNode>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ElidedNode, <<c::expression::CRecord<crate::__pgrx_c_bindings::ElidedNode> as c::expression::OrdinaryField<Field_plan_node_id>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).plan_node_id) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_elided_type;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ElidedNode, elided_type) == 8);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_elided_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ElidedNode>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ElidedNode, <<c::expression::CRecord<crate::__pgrx_c_bindings::ElidedNode> as c::expression::OrdinaryField<Field_elided_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).elided_type) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_relids;
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ElidedNode, relids) == 16);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relids>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ElidedNode>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::Bitmapset>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ElidedNode, <<c::expression::CRecord<crate::__pgrx_c_bindings::ElidedNode> as c::expression::OrdinaryField<Field_relids>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relids) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ElidedNode, child_append_relid_sets) == 24
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_child_append_relid_sets>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ElidedNode>
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
const _: Projection<crate::__pgrx_c_bindings::ElidedNode, <<c::expression::CRecord<crate::__pgrx_c_bindings::ElidedNode> as c::expression::OrdinaryField<Field_child_append_relid_sets>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).child_append_relid_sets) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_io;
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExplainState, io) == 17);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_io>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 17;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExplainState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState> as c::expression::OrdinaryField<Field_io>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).io) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExplainState, generic) == 18);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_generic>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 18;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::FdwRoutine>() == 376);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::FdwRoutine>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::FetchStmt>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::FetchStmt>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_direction_keyword;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::FetchStmt, direction_keyword) == 28);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_direction_keyword>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FetchStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__FetchDirectionKeywords,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::FetchStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::FetchStmt> as c::expression::OrdinaryField<
        Field_direction_keyword,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).direction_keyword) };
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::FetchStmt, location) == 32);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FetchStmt>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::FetchStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::FetchStmt> as c::expression::OrdinaryField<
        Field_location,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Fitness>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Fitness>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::Fitness {}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Fitness, disabled_nodes) == 0);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_disabled_nodes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Fitness>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Fitness,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Fitness> as c::expression::OrdinaryField<
        Field_disabled_nodes,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).disabled_nodes) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ForeignDataWrapper>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ForeignDataWrapper>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_fdwconnection;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ForeignDataWrapper, fdwconnection) == 24
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fdwconnection>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ForeignDataWrapper>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ForeignDataWrapper, <<c::expression::CRecord<crate::__pgrx_c_bindings::ForeignDataWrapper> as c::expression::OrdinaryField<Field_fdwconnection>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fdwconnection) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ForeignDataWrapper, options) == 32);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ForeignDataWrapper>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::FormData_pg_foreign_data_wrapper>() == 84
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::FormData_pg_foreign_data_wrapper>() == 4
    );
};
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::FormData_pg_foreign_data_wrapper,
        fdwconnection
    ) == 80
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fdwconnection>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_foreign_data_wrapper>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_foreign_data_wrapper, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_foreign_data_wrapper> as c::expression::OrdinaryField<Field_fdwconnection>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fdwconnection) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_puballsequences;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_publication, puballsequences)
        == 73
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_puballsequences>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_publication>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 73;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_publication, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_publication> as c::expression::OrdinaryField<Field_puballsequences>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).puballsequences) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_publication, pubinsert) == 74
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pubinsert>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_publication>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 74;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_publication, pubupdate) == 75
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pubupdate>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_publication>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 75;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_publication, pubdelete) == 76
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pubdelete>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_publication>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 76;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_publication, pubtruncate) == 77
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pubtruncate>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_publication>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 77;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_publication, pubviaroot) == 78
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pubviaroot>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_publication>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 78;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_publication, pubgencols) == 79
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pubgencols>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_publication>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 79;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::FormData_pg_publication_rel>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::FormData_pg_publication_rel>() == 4);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_prexcept;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_publication_rel, prexcept) == 12
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_prexcept>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_publication_rel>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_publication_rel, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_publication_rel> as c::expression::OrdinaryField<Field_prexcept>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).prexcept) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::FormData_pg_range>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::FormData_pg_range>() == 4);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_rngconstruct2;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_range, rngconstruct2) == 20
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rngconstruct2>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_range>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_range, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_range> as c::expression::OrdinaryField<Field_rngconstruct2>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rngconstruct2) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_rngconstruct3;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_range, rngconstruct3) == 24
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rngconstruct3>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_range>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_range, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_range> as c::expression::OrdinaryField<Field_rngconstruct3>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rngconstruct3) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_rngmltconstruct0;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_range, rngmltconstruct0) == 28
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rngmltconstruct0>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_range>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_range, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_range> as c::expression::OrdinaryField<Field_rngmltconstruct0>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rngmltconstruct0) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_rngmltconstruct1;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_range, rngmltconstruct1) == 32
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rngmltconstruct1>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_range>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_range, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_range> as c::expression::OrdinaryField<Field_rngmltconstruct1>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rngmltconstruct1) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_rngmltconstruct2;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_range, rngmltconstruct2) == 36
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rngmltconstruct2>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_range>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_range, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_range> as c::expression::OrdinaryField<Field_rngmltconstruct2>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rngmltconstruct2) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_range, rngcanonical) == 40
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rngcanonical>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_range>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_range, rngsubdiff) == 44);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rngsubdiff>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_range>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::FormData_pg_subscription>() == 112);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::FormData_pg_subscription>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_subretaindeadtuples;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::FormData_pg_subscription,
        subretaindeadtuples
    ) == 92
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_subretaindeadtuples>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_subscription>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 92;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_subscription, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_subscription> as c::expression::OrdinaryField<Field_subretaindeadtuples>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).subretaindeadtuples) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_submaxretention;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_subscription, submaxretention)
        == 96
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_submaxretention>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_subscription>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_subscription, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_subscription> as c::expression::OrdinaryField<Field_submaxretention>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).submaxretention) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_subretentionactive;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_subscription, subretentionactive)
        == 100
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_subretentionactive>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_subscription>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 100;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_subscription, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_subscription> as c::expression::OrdinaryField<Field_subretentionactive>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).subretentionactive) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_subserver;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_subscription, subserver) == 104
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_subserver>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_subscription>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_subscription, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_subscription> as c::expression::OrdinaryField<Field_subserver>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).subserver) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_ignore_nulls;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::FuncCall, ignore_nulls) == 48);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ignore_nulls>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FuncCall>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::FuncCall,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::FuncCall> as c::expression::OrdinaryField<
        Field_ignore_nulls,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ignore_nulls) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::FuncCall, agg_within_group) == 52);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_agg_within_group>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FuncCall>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::FuncCall, agg_star) == 53);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_agg_star>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FuncCall>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 53;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::FuncCall, agg_distinct) == 54);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_agg_distinct>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FuncCall>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 54;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::FuncCall, func_variadic) == 55);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_func_variadic>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FuncCall>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 55;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::FuncCall, funcformat) == 56);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_funcformat>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FuncCall>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__CoercionForm,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::FuncCall, location) == 60);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FuncCall>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 60;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::GISTPageOpaqueData>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::GISTPageOpaqueData>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_compareFn;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GinState, compareFn) == 280);
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<[crate::__pgrx_c_bindings::FmgrInfo; 32]>() == 1536);
    assert!(::core::mem::align_of::<[crate::__pgrx_c_bindings::FmgrInfo; 32]>() == 8);
};
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_compareFn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GinState>
{
    type Member =
        c::expression::CArray<c::expression::CRecord<crate::__pgrx_c_bindings::FmgrInfo>, 32>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 280;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::GinState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::GinState> as c::expression::OrdinaryField<
        Field_compareFn,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).compareFn) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_supportCollation;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GinState, supportCollation) == 9528);
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<[crate::__pgrx_c_bindings::Oid; 32]>() == 128);
    assert!(::core::mem::align_of::<[crate::__pgrx_c_bindings::Oid; 32]>() == 4);
};
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_supportCollation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GinState>
{
    type Member = c::expression::CArray<
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>,
        32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 9528;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::GinState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::GinState> as c::expression::OrdinaryField<
        Field_supportCollation,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).supportCollation) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::GinTuple>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::GinTuple>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GinTuple, keylen) == 8);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_keylen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GinTuple>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GinTuple, typlen) == 16);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typlen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GinTuple>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GinTuple, typbyval) == 18);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typbyval>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GinTuple>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 18;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GinTuple, category) == 19);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_category>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GinTuple>
{
    type Member = c::CSignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 19;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GinTuple, nitems) == 20);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nitems>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GinTuple>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GinTuple, data) == 24);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_data>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GinTuple>
{
    type Member = c::expression::CFlexibleArray<
        c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
        crate::__pgrx_c_bindings::__IncompleteArrayField<::core::ffi::c_char>,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::GroupingExprInfo>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::GroupingExprInfo>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::GroupingExprInfo {}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GroupingExprInfo, type_) == 0);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GroupingExprInfo>
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
const _: Projection<crate::__pgrx_c_bindings::GroupingExprInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::GroupingExprInfo> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_sortgroupref;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::GroupingExprInfo, sortgroupref) == 16
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sortgroupref>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GroupingExprInfo>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::GroupingExprInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::GroupingExprInfo> as c::expression::OrdinaryField<Field_sortgroupref>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sortgroupref) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::HashJoinState>() == 320);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::HashJoinState>() == 8);
};
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HashJoinState, hj_JoinState) == 312);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hj_JoinState>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 312;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_hj_KeepNullTuples;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashJoinState, hj_KeepNullTuples) == 316
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hj_KeepNullTuples>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 316;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashJoinState, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinState> as c::expression::OrdinaryField<Field_hj_KeepNullTuples>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hj_KeepNullTuples) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashJoinState, hj_MatchedOuter) == 317
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hj_MatchedOuter>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 317;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashJoinState, hj_OuterNotEmpty) == 318
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hj_OuterNotEmpty>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 318;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::HashState>() == 272);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::HashState>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_keep_null_tuples;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HashState, keep_null_tuples) == 240);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_keep_null_tuples>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::HashState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::HashState> as c::expression::OrdinaryField<
        Field_keep_null_tuples,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).keep_null_tuples) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HashState, parallel_state) == 264);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parallel_state>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinState>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 264;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::HeapPageFreeze>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::HeapPageFreeze>() == 4);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_FreezePageConflictXid;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapPageFreeze, FreezePageConflictXid) == 12
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_FreezePageConflictXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapPageFreeze>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HeapPageFreeze, <<c::expression::CRecord<crate::__pgrx_c_bindings::HeapPageFreeze> as c::expression::OrdinaryField<Field_FreezePageConflictXid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).FreezePageConflictXid) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapPageFreeze, NoFreezePageRelfrozenXid)
        == 16
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_NoFreezePageRelfrozenXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapPageFreeze>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapPageFreeze, NoFreezePageRelminMxid) == 20
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_NoFreezePageRelminMxid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapPageFreeze>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::MultiXactId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::HeapScanDescData>() == 752);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::HeapScanDescData>() == 8);
};
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapScanDescData, rs_nblocks) == 72);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rs_nblocks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapScanDescData, rs_startblock) == 76
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rs_startblock>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 76;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapScanDescData, rs_numblocks) == 80
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rs_numblocks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapScanDescData, rs_inited) == 84);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rs_inited>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 84;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapScanDescData, rs_coffset) == 86);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rs_coffset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 86;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapScanDescData, rs_cblock) == 88);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rs_cblock>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapScanDescData, rs_cbuf) == 92);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rs_cbuf>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 92;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapScanDescData, rs_dir) == 136);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rs_dir>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ScanDirection,
        c::CInt,
        i32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapScanDescData, rs_prefetch_block) == 140
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rs_prefetch_block>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 140;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapScanDescData, rs_vmbuffer) == 152
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rs_vmbuffer>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapScanDescData, rs_cindex) == 156);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rs_cindex>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 156;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapScanDescData, rs_ntuples) == 160);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rs_ntuples>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::HeapTupleTableSlot>() == 112);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::HeapTupleTableSlot>() == 8);
};
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapTupleTableSlot, off) == 80);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_off>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapTupleTableSlot>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_prefetch_count;
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::IOStats>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::IOStats>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::IOStats {}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IOStats, prefetch_count) == 0);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_prefetch_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IOStats>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IOStats,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IOStats> as c::expression::OrdinaryField<
        Field_prefetch_count,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).prefetch_count) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_distance_sum;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IOStats, distance_sum) == 8);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_distance_sum>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IOStats>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IOStats,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IOStats> as c::expression::OrdinaryField<
        Field_distance_sum,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).distance_sum) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_distance_max;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IOStats, distance_max) == 16);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_distance_max>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IOStats>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IOStats,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IOStats> as c::expression::OrdinaryField<
        Field_distance_max,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).distance_max) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_distance_capacity;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IOStats, distance_capacity) == 18);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_distance_capacity>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IOStats>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 18;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IOStats,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IOStats> as c::expression::OrdinaryField<
        Field_distance_capacity,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).distance_capacity) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_wait_count;
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IOStats, wait_count) == 24);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wait_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IOStats>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IOStats,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IOStats> as c::expression::OrdinaryField<
        Field_wait_count,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wait_count) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_io_count;
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IOStats, io_count) == 32);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_io_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IOStats>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IOStats,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IOStats> as c::expression::OrdinaryField<
        Field_io_count,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).io_count) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_io_nblocks;
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IOStats, io_nblocks) == 40);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_io_nblocks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IOStats>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IOStats,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IOStats> as c::expression::OrdinaryField<
        Field_io_nblocks,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).io_nblocks) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_io_in_progress;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IOStats, io_in_progress) == 48);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_io_in_progress>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IOStats>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IOStats,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IOStats> as c::expression::OrdinaryField<
        Field_io_in_progress,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).io_in_progress) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::IndexElem>() == 72);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::IndexElem>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexElem, location) == 64);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexElem>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
