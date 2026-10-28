// Shared generated C macro support.

#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__RelationData__0p__0c__struct__RelationData__0p__0c__struct__RelationData__0p__0c__0x5f426f6f6c__0c__unsigned__int__0c__unsigned__int__0p__0c__unsigned__int__0p__0c__double__0p__0c__double__0p__0c__double__0p__0r {
 type Physical = PhysicalFunction_C_unwind_10<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::RelationData, bool, crate::__pgrx_c_bindings::TransactionId, *mut crate::__pgrx_c_bindings::TransactionId, *mut crate::__pgrx_c_bindings::MultiXactId, *mut f64, *mut f64, *mut f64) -> ()>>;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut *mut crate::__pgrx_c_bindings::TupleTableSlot,
                i32,
                u32,
                i32,
                *mut crate::__pgrx_c_bindings::BulkInsertStateData,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut *mut crate::__pgrx_c_bindings::TupleTableSlot,
                i32,
                u32,
                i32,
                *mut crate::__pgrx_c_bindings::BulkInsertStateData,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__RelationData__0p__0c__struct__TupleTableSlot__0p__0p__0c__int__0c__unsigned__int__0c__int__0c__struct__BulkInsertStateData__0p__0r;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::sealed::Sealed for Signature_void__0l__struct__RelationData__0p__0c__struct__TupleTableSlot__0p__0p__0c__int__0c__unsigned__int__0c__int__0c__struct__BulkInsertStateData__0p__0r {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__RelationData__0p__0c__struct__TupleTableSlot__0p__0p__0c__int__0c__unsigned__int__0c__int__0c__struct__BulkInsertStateData__0p__0r {
 type Physical = PhysicalFunction_C_unwind_6<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut *mut crate::__pgrx_c_bindings::TupleTableSlot, i32, u32, i32, *mut crate::__pgrx_c_bindings::BulkInsertStateData) -> ()>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                u32,
                bool,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                u32,
                bool,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__RelationData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__0x5f426f6f6c__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__RelationData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__0x5f426f6f6c__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__RelationData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__0x5f426f6f6c__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::TupleTableSlot, u32, bool) -> ()>>;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                u32,
                i32,
                *mut crate::__pgrx_c_bindings::BulkInsertStateData,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                u32,
                i32,
                *mut crate::__pgrx_c_bindings::BulkInsertStateData,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__RelationData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__int__0c__struct__BulkInsertStateData__0p__0r;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::sealed::Sealed for Signature_void__0l__struct__RelationData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__int__0c__struct__BulkInsertStateData__0p__0r {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__RelationData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__int__0c__struct__BulkInsertStateData__0p__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::TupleTableSlot, u32, i32, *mut crate::__pgrx_c_bindings::BulkInsertStateData) -> ()>>;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                u32,
                i32,
                *mut crate::__pgrx_c_bindings::BulkInsertStateData,
                u32,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                u32,
                i32,
                *mut crate::__pgrx_c_bindings::BulkInsertStateData,
                u32,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__RelationData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__int__0c__struct__BulkInsertStateData__0p__0c__unsigned__int__0r;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::sealed::Sealed for Signature_void__0l__struct__RelationData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__int__0c__struct__BulkInsertStateData__0p__0c__unsigned__int__0r {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__RelationData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__int__0c__struct__BulkInsertStateData__0p__0c__unsigned__int__0r {
 type Physical = PhysicalFunction_C_unwind_6<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::TupleTableSlot, u32, i32, *mut crate::__pgrx_c_bindings::BulkInsertStateData, u32) -> ()>>;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::VacuumParams,
                *mut crate::__pgrx_c_bindings::BufferAccessStrategyData,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::VacuumParams,
                *mut crate::__pgrx_c_bindings::BufferAccessStrategyData,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__RelationData__0p__0c__struct__VacuumParams__0p__0c__struct__BufferAccessStrategyData__0p__0r;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::sealed::Sealed for Signature_void__0l__struct__RelationData__0p__0c__struct__VacuumParams__0p__0c__struct__BufferAccessStrategyData__0p__0r {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__RelationData__0p__0c__struct__VacuumParams__0p__0c__struct__BufferAccessStrategyData__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::VacuumParams, *mut crate::__pgrx_c_bindings::BufferAccessStrategyData) -> ()>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                crate::__pgrx_c_bindings::Oid,
                i32,
                i32,
                i32,
                *mut crate::__pgrx_c_bindings::varlena,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                crate::__pgrx_c_bindings::Oid,
                i32,
                i32,
                i32,
                *mut crate::__pgrx_c_bindings::varlena,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__RelationData__0p__0c__unsigned__int__0c__int__0c__int__0c__int__0c__struct__varlena__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__RelationData__0p__0c__unsigned__int__0c__int__0c__int__0c__int__0c__struct__varlena__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__RelationData__0p__0c__unsigned__int__0c__int__0c__int__0c__int__0c__struct__varlena__0p__0r {
 type Physical = PhysicalFunction_C_unwind_6<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, crate::__pgrx_c_bindings::Oid, i32, i32, i32, *mut crate::__pgrx_c_bindings::varlena) -> ()>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ReorderBuffer,
                *mut crate::__pgrx_c_bindings::ReorderBufferTXN,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ReorderBuffer,
                *mut crate::__pgrx_c_bindings::ReorderBufferTXN,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0r;
impl c::sealed::Sealed
    for Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ReorderBuffer,
                *mut crate::__pgrx_c_bindings::ReorderBufferTXN,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ReorderBuffer,
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
                *mut crate::__pgrx_c_bindings::ReorderBuffer,
                *mut crate::__pgrx_c_bindings::ReorderBufferTXN,
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::ReorderBufferChange,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0c__struct__RelationData__0p__0c__struct__ReorderBufferChange__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0c__struct__RelationData__0p__0c__struct__ReorderBufferChange__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0c__struct__RelationData__0p__0c__struct__ReorderBufferChange__0p__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ReorderBuffer, *mut crate::__pgrx_c_bindings::ReorderBufferTXN, *mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::ReorderBufferChange) -> ()>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ReorderBuffer,
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
                *mut crate::__pgrx_c_bindings::ReorderBuffer,
                *mut crate::__pgrx_c_bindings::ReorderBufferTXN,
                u64,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ReorderBuffer, *mut crate::__pgrx_c_bindings::ReorderBufferTXN, u64) -> ()>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ReorderBuffer,
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
                *mut crate::__pgrx_c_bindings::ReorderBuffer,
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
pub struct Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0c__0x5f426f6f6c__0c__const__char__0p__0c__unsigned__long__0c__const__char__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0c__0x5f426f6f6c__0c__const__char__0p__0c__unsigned__long__0c__const__char__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0c__0x5f426f6f6c__0c__const__char__0p__0c__unsigned__long__0c__const__char__0p__0r {
 type Physical = PhysicalFunction_C_unwind_7<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ReorderBuffer, *mut crate::__pgrx_c_bindings::ReorderBufferTXN, u64, bool, *const ::core::ffi::c_char, usize, *const ::core::ffi::c_char) -> ()>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ReorderBuffer,
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
                *mut crate::__pgrx_c_bindings::ReorderBuffer,
                *mut crate::__pgrx_c_bindings::ReorderBufferTXN,
                u64,
                i64,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0c__long__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0c__long__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0c__long__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ReorderBuffer, *mut crate::__pgrx_c_bindings::ReorderBufferTXN, u64, i64) -> ()>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::SampleScanState) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::SampleScanState) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__SampleScanState__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__SampleScanState__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__SampleScanState__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::SampleScanState) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::SampleScanState, i32) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::SampleScanState, i32) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__SampleScanState__0p__0c__int__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__SampleScanState__0p__0c__int__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__SampleScanState__0p__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::SampleScanState, i32) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::SampleScanState,
                *mut crate::__pgrx_c_bindings::Datum,
                i32,
                u32,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::SampleScanState,
                *mut crate::__pgrx_c_bindings::Datum,
                i32,
                u32,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__SampleScanState__0p__0c__unsigned__long__0p__0c__int__0c__unsigned__int__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__SampleScanState__0p__0c__unsigned__long__0p__0c__int__0c__unsigned__int__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__SampleScanState__0p__0c__unsigned__long__0p__0c__int__0c__unsigned__int__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::SampleScanState, *mut crate::__pgrx_c_bindings::Datum, i32, u32) -> ()>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::StringInfoData,
                *const crate::__pgrx_c_bindings::ExtensibleNode,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::StringInfoData,
                *const crate::__pgrx_c_bindings::ExtensibleNode,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__StringInfoData__0p__0c__const__struct__ExtensibleNode__0p__0r;
impl c::sealed::Sealed
    for Signature_void__0l__struct__StringInfoData__0p__0c__const__struct__ExtensibleNode__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__StringInfoData__0p__0c__const__struct__ExtensibleNode__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::StringInfoData,
                *const crate::__pgrx_c_bindings::ExtensibleNode,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::StringInfoData,
                *mut crate::__pgrx_c_bindings::XLogReaderState,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::StringInfoData,
                *mut crate::__pgrx_c_bindings::XLogReaderState,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__StringInfoData__0p__0c__struct__XLogReaderState__0p__0r;
impl c::sealed::Sealed
    for Signature_void__0l__struct__StringInfoData__0p__0c__struct__XLogReaderState__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__StringInfoData__0p__0c__struct__XLogReaderState__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::StringInfoData,
                *mut crate::__pgrx_c_bindings::XLogReaderState,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::SubscriptingRef,
                *mut crate::__pgrx_c_bindings::List,
                *mut crate::__pgrx_c_bindings::ParseState,
                bool,
                bool,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::SubscriptingRef,
                *mut crate::__pgrx_c_bindings::List,
                *mut crate::__pgrx_c_bindings::ParseState,
                bool,
                bool,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__SubscriptingRef__0p__0c__struct__List__0p__0c__struct__ParseState__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__SubscriptingRef__0p__0c__struct__List__0p__0c__struct__ParseState__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__SubscriptingRef__0p__0c__struct__List__0p__0c__struct__ParseState__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::SubscriptingRef, *mut crate::__pgrx_c_bindings::List, *mut crate::__pgrx_c_bindings::ParseState, bool, bool) -> ()>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TableFuncScanState) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TableFuncScanState) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__TableFuncScanState__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__TableFuncScanState__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__TableFuncScanState__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TableFuncScanState) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableFuncScanState,
                *const ::core::ffi::c_char,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableFuncScanState,
                *const ::core::ffi::c_char,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__TableFuncScanState__0p__0c__const__char__0p__0r;
impl c::sealed::Sealed
    for Signature_void__0l__struct__TableFuncScanState__0p__0c__const__char__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__TableFuncScanState__0p__0c__const__char__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableFuncScanState,
                *const ::core::ffi::c_char,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableFuncScanState,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableFuncScanState,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__TableFuncScanState__0p__0c__const__char__0p__0c__const__char__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__TableFuncScanState__0p__0c__const__char__0p__0c__const__char__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__TableFuncScanState__0p__0c__const__char__0p__0c__const__char__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TableFuncScanState, *const ::core::ffi::c_char, *const ::core::ffi::c_char) -> ()>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableFuncScanState,
                *const ::core::ffi::c_char,
                i32,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableFuncScanState,
                *const ::core::ffi::c_char,
                i32,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__TableFuncScanState__0p__0c__const__char__0p__0c__int__0r;
impl c::sealed::Sealed
    for Signature_void__0l__struct__TableFuncScanState__0p__0c__const__char__0p__0c__int__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__TableFuncScanState__0p__0c__const__char__0p__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableFuncScanState,
                *const ::core::ffi::c_char,
                i32,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableFuncScanState,
                i32,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableFuncScanState,
                i32,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__TableFuncScanState__0p__0c__int__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__TableFuncScanState__0p__0c__int__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__TableFuncScanState__0p__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableFuncScanState,
                i32,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableFuncScanState,
                crate::__pgrx_c_bindings::Datum,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableFuncScanState,
                crate::__pgrx_c_bindings::Datum,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__TableFuncScanState__0p__0c__unsigned__long__0r;
impl c::sealed::Sealed
    for Signature_void__0l__struct__TableFuncScanState__0p__0c__unsigned__long__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__TableFuncScanState__0p__0c__unsigned__long__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableFuncScanState,
                crate::__pgrx_c_bindings::Datum,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TableScanDescData) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TableScanDescData) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__TableScanDescData__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__TableScanDescData__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__TableScanDescData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TableScanDescData) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableScanDescData,
                *mut crate::__pgrx_c_bindings::ItemPointerData,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableScanDescData,
                *mut crate::__pgrx_c_bindings::ItemPointerData,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__TableScanDescData__0p__0c__struct__ItemPointerData__0p__0r;
impl c::sealed::Sealed
    for Signature_void__0l__struct__TableScanDescData__0p__0c__struct__ItemPointerData__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__TableScanDescData__0p__0c__struct__ItemPointerData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableScanDescData,
                *mut crate::__pgrx_c_bindings::ItemPointerData,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
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
const _: () = assert!(
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
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__TableScanDescData__0p__0c__struct__ItemPointerData__0p__0c__struct__ItemPointerData__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__TableScanDescData__0p__0c__struct__ItemPointerData__0p__0c__struct__ItemPointerData__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__TableScanDescData__0p__0c__struct__ItemPointerData__0p__0c__struct__ItemPointerData__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TableScanDescData, *mut crate::__pgrx_c_bindings::ItemPointerData, *mut crate::__pgrx_c_bindings::ItemPointerData) -> ()>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableScanDescData,
                *mut crate::__pgrx_c_bindings::ScanKeyData,
                bool,
                bool,
                bool,
                bool,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableScanDescData,
                *mut crate::__pgrx_c_bindings::ScanKeyData,
                bool,
                bool,
                bool,
                bool,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__TableScanDescData__0p__0c__struct__ScanKeyData__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__TableScanDescData__0p__0c__struct__ScanKeyData__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__TableScanDescData__0p__0c__struct__ScanKeyData__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0r {
 type Physical = PhysicalFunction_C_unwind_6<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TableScanDescData, *mut crate::__pgrx_c_bindings::ScanKeyData, bool, bool, bool, bool) -> ()>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TupleTableSlot) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TupleTableSlot) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__TupleTableSlot__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__TupleTableSlot__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__TupleTableSlot__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TupleTableSlot) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TupleTableSlot, i32) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TupleTableSlot, i32) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__TupleTableSlot__0p__0c__int__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__TupleTableSlot__0p__0c__int__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__TupleTableSlot__0p__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TupleTableSlot, i32) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__TupleTableSlot__0p__0c__struct__TupleTableSlot__0p__0r;
impl c::sealed::Sealed
    for Signature_void__0l__struct__TupleTableSlot__0p__0c__struct__TupleTableSlot__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__TupleTableSlot__0p__0c__struct__TupleTableSlot__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::VacAttrStats,
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::VacAttrStats,
                        i32,
                        *mut bool,
                    )
                        -> crate::__pgrx_c_bindings::Datum,
                >,
                i32,
                f64,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::VacAttrStats,
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::VacAttrStats,
                        i32,
                        *mut bool,
                    )
                        -> crate::__pgrx_c_bindings::Datum,
                >,
                i32,
                f64,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__VacAttrStats__0p__0c__unsigned__long__0l__0p__0r__0l__struct__VacAttrStats__0p__0c__int__0c__0x5f426f6f6c__0p__0r__0c__int__0c__double__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__VacAttrStats__0p__0c__unsigned__long__0l__0p__0r__0l__struct__VacAttrStats__0p__0c__int__0c__0x5f426f6f6c__0p__0r__0c__int__0c__double__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__VacAttrStats__0p__0c__unsigned__long__0l__0p__0r__0l__struct__VacAttrStats__0p__0c__int__0c__0x5f426f6f6c__0p__0r__0c__int__0c__double__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::VacAttrStats, ::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::VacAttrStats, i32, *mut bool) -> crate::__pgrx_c_bindings::Datum>, i32, f64) -> ()>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::WalReceiverConn) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::WalReceiverConn) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__WalReceiverConn__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__WalReceiverConn__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__WalReceiverConn__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::WalReceiverConn) -> (),
        >,
    >;
}
const _: () =
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::WalReceiverConn>() == 8);
const _: () =
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::WalReceiverConn>() == 8);
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<
    A0: c::expression::ImplicitTo<
            c::expression::CPointer<
                c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>,
                c::expression::ReadWrite,
            >,
        >,
> c::expression::Call<(A0,)> for Signature_void__0l__struct__WalReceiverConn__0p__0r
{
    type Output = ();
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
        result
    }
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::WalReceiverConn,
                *mut *mut ::core::ffi::c_char,
                *mut i32,
            ) -> (),
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
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__WalReceiverConn__0p__0c__char__0p__0p__0c__int__0p__0r;
impl c::sealed::Sealed
    for Signature_void__0l__struct__WalReceiverConn__0p__0c__char__0p__0p__0c__int__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__WalReceiverConn__0p__0c__char__0p__0p__0c__int__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::WalReceiverConn,
                *mut *mut ::core::ffi::c_char,
                *mut i32,
            ) -> (),
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
    for Signature_void__0l__struct__WalReceiverConn__0p__0c__char__0p__0p__0c__int__0p__0r
{
    type Output = ();
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
        result
    }
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::WalReceiverConn,
                *const ::core::ffi::c_char,
                i32,
            ) -> (),
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
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__WalReceiverConn__0p__0c__const__char__0p__0c__int__0r;
impl c::sealed::Sealed
    for Signature_void__0l__struct__WalReceiverConn__0p__0c__const__char__0p__0c__int__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__WalReceiverConn__0p__0c__const__char__0p__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::WalReceiverConn,
                *const ::core::ffi::c_char,
                i32,
            ) -> (),
        >,
    >;
}
const _: () =
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::WalReceiverConn>() == 8);
const _: () =
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::WalReceiverConn>() == 8);
const _: () = assert!(::core::mem::size_of::<*const ::core::ffi::c_char>() == 8);
const _: () = assert!(::core::mem::align_of::<*const ::core::ffi::c_char>() == 8);
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
                c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
                c::expression::ReadOnly,
            >,
        >,
    A2: c::expression::ImplicitTo<c::CInt>,
> c::expression::Call<(A0, A1, A2)>
    for Signature_void__0l__struct__WalReceiverConn__0p__0c__const__char__0p__0c__int__0r
{
    type Output = ();
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
        const _: () = assert!(!::core::mem::needs_drop::<*const ::core::ffi::c_char>());
        let native1 = <c::expression::CPointer<
            c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
            c::expression::ReadOnly,
        > as c::expression::CType>::into_storage(c::expression::implicit::<
            c::expression::CPointer<
                c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
                c::expression::ReadOnly,
            >,
            _,
        >(args.1));
        const {
            assert!(!::core::mem::needs_drop::<A2>());
        }
        const _: () = assert!(!::core::mem::needs_drop::<i32>());
        let native2 = <c::CInt as c::expression::CType>::into_storage(c::expression::implicit::<
            c::CInt,
            _,
        >(args.2));
        // SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
        let result = unsafe {
            crate::ffi::pg_guard_ffi_boundary(move || function(native0, native1, native2))
        };
        result
    }
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::WalReceiverConn,
                *mut u32,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::WalReceiverConn,
                *mut u32,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__WalReceiverConn__0p__0c__unsigned__int__0p__0r;
impl c::sealed::Sealed
    for Signature_void__0l__struct__WalReceiverConn__0p__0c__unsigned__int__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__WalReceiverConn__0p__0c__unsigned__int__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::WalReceiverConn,
                *mut u32,
            ) -> (),
        >,
    >;
}
const _: () =
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::WalReceiverConn>() == 8);
const _: () =
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::WalReceiverConn>() == 8);
const _: () = assert!(::core::mem::size_of::<*mut u32>() == 8);
const _: () = assert!(::core::mem::align_of::<*mut u32>() == 8);
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
    for Signature_void__0l__struct__WalReceiverConn__0p__0c__unsigned__int__0p__0r
{
    type Output = ();
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
        result
    }
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::WalReceiverConn,
                u32,
                *mut *mut ::core::ffi::c_char,
                *mut *mut ::core::ffi::c_char,
                *mut i32,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::WalReceiverConn,
                u32,
                *mut *mut ::core::ffi::c_char,
                *mut *mut ::core::ffi::c_char,
                *mut i32,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__WalReceiverConn__0p__0c__unsigned__int__0c__char__0p__0p__0c__char__0p__0p__0c__int__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__WalReceiverConn__0p__0c__unsigned__int__0c__char__0p__0p__0c__char__0p__0p__0c__int__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__WalReceiverConn__0p__0c__unsigned__int__0c__char__0p__0p__0c__char__0p__0p__0c__int__0p__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::WalReceiverConn, u32, *mut *mut ::core::ffi::c_char, *mut *mut ::core::ffi::c_char, *mut i32) -> ()>>;
}
const _: () =
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::WalReceiverConn>() == 8);
const _: () =
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::WalReceiverConn>() == 8);
const _: () = assert!(::core::mem::size_of::<u32>() == 4);
const _: () = assert!(::core::mem::align_of::<u32>() == 4);
const _: () = assert!(::core::mem::size_of::<*mut *mut ::core::ffi::c_char>() == 8);
const _: () = assert!(::core::mem::align_of::<*mut *mut ::core::ffi::c_char>() == 8);
const _: () = assert!(::core::mem::size_of::<*mut *mut ::core::ffi::c_char>() == 8);
const _: () = assert!(::core::mem::align_of::<*mut *mut ::core::ffi::c_char>() == 8);
const _: () = assert!(::core::mem::size_of::<*mut i32>() == 8);
const _: () = assert!(::core::mem::align_of::<*mut i32>() == 8);
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<A0: c::expression::ImplicitTo<c::expression::CPointer<c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>, c::expression::ReadWrite>>, A1: c::expression::ImplicitTo<c::CUnsignedInt>, A2: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadWrite>, c::expression::ReadWrite>>, A3: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadWrite>, c::expression::ReadWrite>>, A4: c::expression::ImplicitTo<c::expression::CPointer<c::CInt, c::expression::ReadWrite>>> c::expression::Call<(A0,A1,A2,A3,A4,)> for Signature_void__0l__struct__WalReceiverConn__0p__0c__unsigned__int__0c__char__0p__0p__0c__char__0p__0p__0c__int__0p__0r {
 type Output = ();
 unsafe fn call(pointer: Self::Pointer, args: (A0,A1,A2,A3,A4,)) -> Self::Output {
let function = pointer.expect("C indirect call requires a non-null function pointer");
const { assert!(!::core::mem::needs_drop::<A0>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::WalReceiverConn>());
let native0 = <c::expression::CPointer<c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>, c::expression::ReadWrite>, _>(args.0));
const { assert!(!::core::mem::needs_drop::<A1>()); }
const _: () = assert!(!::core::mem::needs_drop::<u32>());
let native1 = <c::CUnsignedInt as c::expression::CType>::into_storage(c::expression::implicit::<c::CUnsignedInt, _>(args.1));
const { assert!(!::core::mem::needs_drop::<A2>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut *mut ::core::ffi::c_char>());
let native2 = <c::expression::CPointer<c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadWrite>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadWrite>, c::expression::ReadWrite>, _>(args.2));
const { assert!(!::core::mem::needs_drop::<A3>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut *mut ::core::ffi::c_char>());
let native3 = <c::expression::CPointer<c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadWrite>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadWrite>, c::expression::ReadWrite>, _>(args.3));
const { assert!(!::core::mem::needs_drop::<A4>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut i32>());
let native4 = <c::expression::CPointer<c::CInt, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::CInt, c::expression::ReadWrite>, _>(args.4));
// SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
let result = unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0, native1, native2, native3, native4)) };
result
}
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::XLogReaderState) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::XLogReaderState) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__XLogReaderState__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__XLogReaderState__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__XLogReaderState__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::XLogReaderState) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::XLogReaderState,
                u64,
                *mut u32,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::XLogReaderState,
                u64,
                *mut u32,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__XLogReaderState__0p__0c__unsigned__long__0c__unsigned__int__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__XLogReaderState__0p__0c__unsigned__long__0c__unsigned__int__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__XLogReaderState__0p__0c__unsigned__long__0c__unsigned__int__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::XLogReaderState, u64, *mut u32) -> ()>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::_DestReceiver) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::_DestReceiver) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__0x5f446573745265636569766572__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__0x5f446573745265636569766572__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__0x5f446573745265636569766572__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::_DestReceiver) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::_DestReceiver,
                i32,
                *mut crate::__pgrx_c_bindings::TupleDescData,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::_DestReceiver,
                i32,
                *mut crate::__pgrx_c_bindings::TupleDescData,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__0x5f446573745265636569766572__0p__0c__int__0c__struct__TupleDescData__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__0x5f446573745265636569766572__0p__0c__int__0c__struct__TupleDescData__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__0x5f446573745265636569766572__0p__0c__int__0c__struct__TupleDescData__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::_DestReceiver, i32, *mut crate::__pgrx_c_bindings::TupleDescData) -> ()>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::dsm_segment,
                *mut crate::__pgrx_c_bindings::shm_toc,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::dsm_segment,
                *mut crate::__pgrx_c_bindings::shm_toc,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__dsm_segment__0p__0c__struct__shm_toc__0p__0r;
impl c::sealed::Sealed
    for Signature_void__0l__struct__dsm_segment__0p__0c__struct__shm_toc__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__dsm_segment__0p__0c__struct__shm_toc__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::dsm_segment,
                *mut crate::__pgrx_c_bindings::shm_toc,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::dsm_segment,
                crate::__pgrx_c_bindings::Datum,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::dsm_segment,
                crate::__pgrx_c_bindings::Datum,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__dsm_segment__0p__0c__unsigned__long__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__dsm_segment__0p__0c__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__dsm_segment__0p__0c__unsigned__long__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::dsm_segment,
                crate::__pgrx_c_bindings::Datum,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Oid,
                *mut crate::__pgrx_c_bindings::List,
                *mut crate::__pgrx_c_bindings::List,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Oid,
                *mut crate::__pgrx_c_bindings::List,
                *mut crate::__pgrx_c_bindings::List,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__unsigned__int__0c__unsigned__int__0c__struct__List__0p__0c__struct__List__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__unsigned__int__0c__unsigned__int__0c__struct__List__0p__0c__struct__List__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__unsigned__int__0c__unsigned__int__0c__struct__List__0p__0c__struct__List__0p__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Oid, crate::__pgrx_c_bindings::Oid, *mut crate::__pgrx_c_bindings::List, *mut crate::__pgrx_c_bindings::List) -> ()>>;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::TransactionId,
                u16,
                *mut ::core::ffi::c_void,
                u32,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::TransactionId,
                u16,
                *mut ::core::ffi::c_void,
                u32,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__unsigned__int__0c__unsigned__short__0c__void__0p__0c__unsigned__int__0r;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::sealed::Sealed
    for Signature_void__0l__unsigned__int__0c__unsigned__short__0c__void__0p__0c__unsigned__int__0r
{
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__unsigned__int__0c__unsigned__short__0c__void__0p__0c__unsigned__int__0r
{
    type Physical = PhysicalFunction_C_unwind_4<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::TransactionId,
                u16,
                *mut ::core::ffi::c_void,
                u32,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Datum) -> ()>,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Datum) -> ()>,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__unsigned__long__0r;
impl c::sealed::Sealed for Signature_void__0l__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__unsigned__long__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Datum) -> ()>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Datum, i32, u32) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Datum, i32, u32) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__unsigned__long__0c__int__0c__unsigned__int__0r;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::sealed::Sealed for Signature_void__0l__unsigned__long__0c__int__0c__unsigned__int__0r {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__unsigned__long__0c__int__0c__unsigned__int__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Datum, i32, u32) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
                *mut crate::__pgrx_c_bindings::TSQueryParserStateData,
                *mut ::core::ffi::c_char,
                i32,
                i16,
                bool,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
                *mut crate::__pgrx_c_bindings::TSQueryParserStateData,
                *mut ::core::ffi::c_char,
                i32,
                i16,
                bool,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__unsigned__long__0c__struct__TSQueryParserStateData__0p__0c__char__0p__0c__int__0c__short__0c__0x5f426f6f6c__0r;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::sealed::Sealed for Signature_void__0l__unsigned__long__0c__struct__TSQueryParserStateData__0p__0c__char__0p__0c__int__0c__short__0c__0x5f426f6f6c__0r {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::NativeFunctionSignature for Signature_void__0l__unsigned__long__0c__struct__TSQueryParserStateData__0p__0c__char__0p__0c__int__0c__short__0c__0x5f426f6f6c__0r {
 type Physical = PhysicalFunction_C_unwind_6<::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Datum, *mut crate::__pgrx_c_bindings::TSQueryParserStateData, *mut ::core::ffi::c_char, i32, i16, bool) -> ()>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Oid,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Oid,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__unsigned__long__0c__unsigned__int__0r;
impl c::sealed::Sealed for Signature_void__0l__unsigned__long__0c__unsigned__int__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__unsigned__long__0c__unsigned__int__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
                crate::__pgrx_c_bindings::Oid,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void) -> ()>,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void) -> ()>,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__void__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__void__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__void__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void) -> ()>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
                *mut crate::__pgrx_c_bindings::relopt_value,
                i32,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
                *mut crate::__pgrx_c_bindings::relopt_value,
                i32,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__void__0p__0c__struct__relopt_value__0p__0c__int__0r;
impl c::sealed::Sealed for Signature_void__0l__void__0p__0c__struct__relopt_value__0p__0c__int__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__void__0p__0c__struct__relopt_value__0p__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
                *mut crate::__pgrx_c_bindings::relopt_value,
                i32,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn() -> ()>>() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn() -> ()>>() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__void__0r;
impl c::sealed::Sealed for Signature_void__0l__void__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__void__0r {
    type Physical =
        PhysicalFunction_C_unwind_0<::core::option::Option<unsafe extern "C-unwind" fn() -> ()>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::SpinDelayStatus,
                *const ::core::ffi::c_char,
                i32,
                *const ::core::ffi::c_char,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::SpinDelayStatus,
                *const ::core::ffi::c_char,
                i32,
                *const ::core::ffi::c_char,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__SpinDelayStatus__0p__0c__const__char__0p__0c__int__0c__const__char__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__SpinDelayStatus__0p__0c__const__char__0p__0c__int__0c__const__char__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__SpinDelayStatus__0p__0c__const__char__0p__0c__int__0c__const__char__0p__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::SpinDelayStatus, *const ::core::ffi::c_char, i32, *const ::core::ffi::c_char) -> ()>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_char,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_char,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__char__0p__0c__struct__FullTransactionId__0r;
impl c::sealed::Sealed for Signature_void__0l__char__0p__0c__struct__FullTransactionId__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__char__0p__0c__struct__FullTransactionId__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_char,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_void) -> ()>,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_void) -> ()>,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__const__void__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__const__void__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__const__void__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_void) -> ()>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(i32, i32, i32, i32, i32) -> ()>,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(i32, i32, i32, i32, i32) -> ()>,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__int__0c__int__0c__int__0c__int__0c__int__0r;
impl c::sealed::Sealed for Signature_void__0l__int__0c__int__0c__int__0c__int__0c__int__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__int__0c__int__0c__int__0c__int__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_5<
        ::core::option::Option<unsafe extern "C-unwind" fn(i32, i32, i32, i32, i32) -> ()>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ExpandedRecordHeader,
                i32,
                crate::__pgrx_c_bindings::Datum,
                bool,
                bool,
                bool,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ExpandedRecordHeader,
                i32,
                crate::__pgrx_c_bindings::Datum,
                bool,
                bool,
                bool,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__ExpandedRecordHeader__0p__0c__int__0c__unsigned__long__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__ExpandedRecordHeader__0p__0c__int__0c__unsigned__long__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__ExpandedRecordHeader__0p__0c__int__0c__unsigned__long__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0r {
 type Physical = PhysicalFunction_C_unwind_6<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ExpandedRecordHeader, i32, crate::__pgrx_c_bindings::Datum, bool, bool, bool) -> ()>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::FullTransactionId) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::FullTransactionId) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__FullTransactionId__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__FullTransactionId__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__FullTransactionId__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::FullTransactionId) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::FunctionCallInfoBaseData,
                *mut crate::__pgrx_c_bindings::FuncCallContext,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::FunctionCallInfoBaseData,
                *mut crate::__pgrx_c_bindings::FuncCallContext,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__FunctionCallInfoBaseData__0p__0c__struct__FuncCallContext__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__FunctionCallInfoBaseData__0p__0c__struct__FuncCallContext__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__FunctionCallInfoBaseData__0p__0c__struct__FuncCallContext__0p__0r {
 type Physical = PhysicalFunction_C_unwind_2<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::FunctionCallInfoBaseData, *mut crate::__pgrx_c_bindings::FuncCallContext) -> ()>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::IndexTupleData, u32) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::IndexTupleData, u32) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__IndexTupleData__0p__0c__unsigned__int__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__IndexTupleData__0p__0c__unsigned__int__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__IndexTupleData__0p__0c__unsigned__int__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::IndexTupleData, u32) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::IndexTupleData,
                u16,
                bool,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::IndexTupleData,
                u16,
                bool,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__IndexTupleData__0p__0c__unsigned__short__0c__0x5f426f6f6c__0r;
impl c::sealed::Sealed
    for Signature_void__0l__struct__IndexTupleData__0p__0c__unsigned__short__0c__0x5f426f6f6c__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__IndexTupleData__0p__0c__unsigned__short__0c__0x5f426f6f6c__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::IndexTupleData,
                u16,
                bool,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::IndexTupleData,
                u16,
                i32,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::IndexTupleData,
                u16,
                i32,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__IndexTupleData__0p__0c__unsigned__short__0c__int__0r;
impl c::sealed::Sealed
    for Signature_void__0l__struct__IndexTupleData__0p__0c__unsigned__short__0c__int__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__IndexTupleData__0p__0c__unsigned__short__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::IndexTupleData,
                u16,
                i32,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ItemPointerData, i64) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ItemPointerData, i64) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__ItemPointerData__0p__0c__long__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__ItemPointerData__0p__0c__long__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__ItemPointerData__0p__0c__long__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ItemPointerData, i64) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::MemoryContextData,
                *const ::core::ffi::c_char,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::MemoryContextData,
                *const ::core::ffi::c_char,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__MemoryContextData__0p__0c__const__char__0p__0r;
impl c::sealed::Sealed
    for Signature_void__0l__struct__MemoryContextData__0p__0c__const__char__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__MemoryContextData__0p__0c__const__char__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::MemoryContextData,
                *const ::core::ffi::c_char,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::MemoryContextData,
                *mut crate::__pgrx_c_bindings::MemoryContextData,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::MemoryContextData,
                *mut crate::__pgrx_c_bindings::MemoryContextData,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__MemoryContextData__0p__0c__struct__MemoryContextData__0p__0r;
impl c::sealed::Sealed
    for Signature_void__0l__struct__MemoryContextData__0p__0c__struct__MemoryContextData__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__MemoryContextData__0p__0c__struct__MemoryContextData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::MemoryContextData,
                *mut crate::__pgrx_c_bindings::MemoryContextData,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::QueryCompletion,
                *const crate::__pgrx_c_bindings::QueryCompletion,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::QueryCompletion,
                *const crate::__pgrx_c_bindings::QueryCompletion,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__QueryCompletion__0p__0c__const__struct__QueryCompletion__0p__0r;
impl c::sealed::Sealed
    for Signature_void__0l__struct__QueryCompletion__0p__0c__const__struct__QueryCompletion__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__QueryCompletion__0p__0c__const__struct__QueryCompletion__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::QueryCompletion,
                *const crate::__pgrx_c_bindings::QueryCompletion,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::QueryCompletion,
                u32,
                u64,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::QueryCompletion,
                u32,
                u64,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__QueryCompletion__0p__0c__enum__CommandTag__0c__unsigned__long__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__QueryCompletion__0p__0c__enum__CommandTag__0c__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__QueryCompletion__0p__0c__enum__CommandTag__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::QueryCompletion, u32, u64) -> ()>>;
}
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::SMgrRelationData) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::SMgrRelationData) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg15")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__SMgrRelationData__0p__0r;
#[cfg(feature = "pg15")]
impl c::sealed::Sealed for Signature_void__0l__struct__SMgrRelationData__0p__0r {}
#[cfg(feature = "pg15")]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__SMgrRelationData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::SMgrRelationData) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::SnapshotData,
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut ::core::ffi::c_char,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::SnapshotData,
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut ::core::ffi::c_char,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__SnapshotData__0p__0c__struct__RelationData__0p__0c__char__0p__0r;
#[cfg(any(feature = "pg15", feature = "pg16"))]
impl c::sealed::Sealed
    for Signature_void__0l__struct__SnapshotData__0p__0c__struct__RelationData__0p__0c__char__0p__0r
{
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__SnapshotData__0p__0c__struct__RelationData__0p__0c__char__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::SnapshotData,
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut ::core::ffi::c_char,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::StringInfoData, i8) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::StringInfoData, i8) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__StringInfoData__0p__0c__char__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__StringInfoData__0p__0c__char__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__StringInfoData__0p__0c__char__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::StringInfoData, i8) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::StringInfoData,
                *const ::core::ffi::c_char,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::StringInfoData,
                *const ::core::ffi::c_char,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__StringInfoData__0p__0c__const__char__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__StringInfoData__0p__0c__const__char__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__StringInfoData__0p__0c__const__char__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::StringInfoData,
                *const ::core::ffi::c_char,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::StringInfoData, u8) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::StringInfoData, u8) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__StringInfoData__0p__0c__unsigned__char__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__StringInfoData__0p__0c__unsigned__char__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__StringInfoData__0p__0c__unsigned__char__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::StringInfoData, u8) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::StringInfoData, u32) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::StringInfoData, u32) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__StringInfoData__0p__0c__unsigned__int__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__StringInfoData__0p__0c__unsigned__int__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__StringInfoData__0p__0c__unsigned__int__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::StringInfoData, u32) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::StringInfoData,
                u32,
                i32,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::StringInfoData,
                u32,
                i32,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__StringInfoData__0p__0c__unsigned__int__0c__int__0r;
impl c::sealed::Sealed
    for Signature_void__0l__struct__StringInfoData__0p__0c__unsigned__int__0c__int__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__StringInfoData__0p__0c__unsigned__int__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::StringInfoData,
                u32,
                i32,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::StringInfoData, u64) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::StringInfoData, u64) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__StringInfoData__0p__0c__unsigned__long__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__StringInfoData__0p__0c__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__StringInfoData__0p__0c__unsigned__long__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::StringInfoData, u64) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::StringInfoData, u16) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::StringInfoData, u16) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__StringInfoData__0p__0c__unsigned__short__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__StringInfoData__0p__0c__unsigned__short__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__StringInfoData__0p__0c__unsigned__short__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::StringInfoData, u16) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableScanDescData,
                *mut crate::__pgrx_c_bindings::ScanKeyData,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableScanDescData,
                *mut crate::__pgrx_c_bindings::ScanKeyData,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__TableScanDescData__0p__0c__struct__ScanKeyData__0p__0r;
impl c::sealed::Sealed
    for Signature_void__0l__struct__TableScanDescData__0p__0c__struct__ScanKeyData__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__TableScanDescData__0p__0c__struct__ScanKeyData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableScanDescData,
                *mut crate::__pgrx_c_bindings::ScanKeyData,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableScanDescData,
                *mut crate::__pgrx_c_bindings::ScanKeyData,
                bool,
                bool,
                bool,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableScanDescData,
                *mut crate::__pgrx_c_bindings::ScanKeyData,
                bool,
                bool,
                bool,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__TableScanDescData__0p__0c__struct__ScanKeyData__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__TableScanDescData__0p__0c__struct__ScanKeyData__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__TableScanDescData__0p__0c__struct__ScanKeyData__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TableScanDescData, *mut crate::__pgrx_c_bindings::ScanKeyData, bool, bool, bool) -> ()>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TupleDescData) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TupleDescData) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__TupleDescData__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__TupleDescData__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__TupleDescData__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TupleDescData) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::WalRcvExecResult) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::WalRcvExecResult) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__WalRcvExecResult__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__WalRcvExecResult__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__WalRcvExecResult__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::WalRcvExecResult) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::catclist) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::catclist) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__catclist__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__catclist__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__catclist__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::catclist) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::dlist_head) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::dlist_head) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__dlist_head__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__dlist_head__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__dlist_head__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::dlist_head) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::dlist_head,
                *mut crate::__pgrx_c_bindings::dlist_node,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::dlist_head,
                *mut crate::__pgrx_c_bindings::dlist_node,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__dlist_head__0p__0c__struct__dlist_node__0p__0r;
impl c::sealed::Sealed
    for Signature_void__0l__struct__dlist_head__0p__0c__struct__dlist_node__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__dlist_head__0p__0c__struct__dlist_node__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::dlist_head,
                *mut crate::__pgrx_c_bindings::dlist_node,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::dlist_node) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::dlist_node) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__dlist_node__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__dlist_node__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__dlist_node__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::dlist_node) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::dlist_node,
                *mut crate::__pgrx_c_bindings::dlist_node,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::dlist_node,
                *mut crate::__pgrx_c_bindings::dlist_node,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__dlist_node__0p__0c__struct__dlist_node__0p__0r;
impl c::sealed::Sealed
    for Signature_void__0l__struct__dlist_node__0p__0c__struct__dlist_node__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__dlist_node__0p__0c__struct__dlist_node__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::dlist_node,
                *mut crate::__pgrx_c_bindings::dlist_node,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::proclist_head) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::proclist_head) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__proclist_head__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__proclist_head__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__proclist_head__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::proclist_head) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::proclist_head,
                i32,
                usize,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::proclist_head,
                i32,
                usize,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__proclist_head__0p__0c__int__0c__unsigned__long__0r;
impl c::sealed::Sealed
    for Signature_void__0l__struct__proclist_head__0p__0c__int__0c__unsigned__long__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__proclist_head__0p__0c__int__0c__unsigned__long__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::proclist_head,
                i32,
                usize,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::slist_head) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::slist_head) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__slist_head__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__slist_head__0p__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__slist_head__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::slist_head) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::slist_head,
                *mut crate::__pgrx_c_bindings::slist_node,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::slist_head,
                *mut crate::__pgrx_c_bindings::slist_node,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__slist_head__0p__0c__struct__slist_node__0p__0r;
impl c::sealed::Sealed
    for Signature_void__0l__struct__slist_head__0p__0c__struct__slist_node__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__slist_head__0p__0c__struct__slist_node__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::slist_head,
                *mut crate::__pgrx_c_bindings::slist_node,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::slist_mutable_iter) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::slist_mutable_iter) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__slist_mutable_iter__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__struct__slist_mutable_iter__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__slist_mutable_iter__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::slist_mutable_iter) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::slist_node,
                *mut crate::__pgrx_c_bindings::slist_node,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::slist_node,
                *mut crate::__pgrx_c_bindings::slist_node,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__slist_node__0p__0c__struct__slist_node__0p__0r;
impl c::sealed::Sealed
    for Signature_void__0l__struct__slist_node__0p__0c__struct__slist_node__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__slist_node__0p__0c__struct__slist_node__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::slist_node,
                *mut crate::__pgrx_c_bindings::slist_node,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::tuplehash_hash,
                *mut crate::__pgrx_c_bindings::tuplehash_iterator,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::tuplehash_hash,
                *mut crate::__pgrx_c_bindings::tuplehash_iterator,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__tuplehash_hash__0p__0c__struct__tuplehash_iterator__0p__0r;
impl c::sealed::Sealed
    for Signature_void__0l__struct__tuplehash_hash__0p__0c__struct__tuplehash_iterator__0p__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__tuplehash_hash__0p__0c__struct__tuplehash_iterator__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::tuplehash_hash,
                *mut crate::__pgrx_c_bindings::tuplehash_iterator,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                *const ::core::ffi::c_char,
                i32,
                bool,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                *const ::core::ffi::c_char,
                i32,
                bool,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__unsigned__int__0c__const__char__0p__0c__int__0c__0x5f426f6f6c__0r;
impl c::sealed::Sealed
    for Signature_void__0l__unsigned__int__0c__const__char__0p__0c__int__0c__0x5f426f6f6c__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__unsigned__int__0c__const__char__0p__0c__int__0c__0x5f426f6f6c__0r
{
    type Physical = PhysicalFunction_C_unwind_4<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                *const ::core::ffi::c_char,
                i32,
                bool,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                *const ::core::ffi::c_char,
                i32,
                i32,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                *const ::core::ffi::c_char,
                i32,
                i32,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__unsigned__int__0c__const__char__0p__0c__int__0c__int__0r;
impl c::sealed::Sealed
    for Signature_void__0l__unsigned__int__0c__const__char__0p__0c__int__0c__int__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__unsigned__int__0c__const__char__0p__0c__int__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_4<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                *const ::core::ffi::c_char,
                i32,
                i32,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                *const ::core::ffi::c_char,
                i32,
                crate::__pgrx_c_bindings::Oid,
                bool,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                *const ::core::ffi::c_char,
                i32,
                crate::__pgrx_c_bindings::Oid,
                bool,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__unsigned__int__0c__const__char__0p__0c__int__0c__unsigned__int__0c__0x5f426f6f6c__0r;
impl c::sealed::Sealed for Signature_void__0l__unsigned__int__0c__const__char__0p__0c__int__0c__unsigned__int__0c__0x5f426f6f6c__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__unsigned__int__0c__const__char__0p__0c__int__0c__unsigned__int__0c__0x5f426f6f6c__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Oid, *const ::core::ffi::c_char, i32, crate::__pgrx_c_bindings::Oid, bool) -> ()>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Oid,
                i32,
                bool,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Oid,
                i32,
                bool,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__unsigned__int__0c__unsigned__int__0c__int__0c__0x5f426f6f6c__0r;
impl c::sealed::Sealed
    for Signature_void__0l__unsigned__int__0c__unsigned__int__0c__int__0c__0x5f426f6f6c__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__unsigned__int__0c__unsigned__int__0c__int__0c__0x5f426f6f6c__0r
{
    type Physical = PhysicalFunction_C_unwind_4<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Oid,
                i32,
                bool,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Oid,
                i32,
                i32,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Oid,
                i32,
                i32,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__unsigned__int__0c__unsigned__int__0c__int__0c__int__0r;
impl c::sealed::Sealed
    for Signature_void__0l__unsigned__int__0c__unsigned__int__0c__int__0c__int__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__unsigned__int__0c__unsigned__int__0c__int__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_4<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Oid,
                i32,
                i32,
            ) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Oid,
                i32,
                crate::__pgrx_c_bindings::Oid,
                bool,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Oid,
                crate::__pgrx_c_bindings::Oid,
                i32,
                crate::__pgrx_c_bindings::Oid,
                bool,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__unsigned__int__0c__unsigned__int__0c__int__0c__unsigned__int__0c__0x5f426f6f6c__0r;
impl c::sealed::Sealed for Signature_void__0l__unsigned__int__0c__unsigned__int__0c__int__0c__unsigned__int__0c__0x5f426f6f6c__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__unsigned__int__0c__unsigned__int__0c__int__0c__unsigned__int__0c__0x5f426f6f6c__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Oid, crate::__pgrx_c_bindings::Oid, i32, crate::__pgrx_c_bindings::Oid, bool) -> ()>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
                usize,
                usize,
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> i32,
                >,
            ) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
                usize,
                usize,
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> i32,
                >,
            ) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__void__0p__0c__unsigned__long__0c__unsigned__long__0c__int__0l__0p__0r__0l__const__void__0p__0c__const__void__0p__0r__0r;
impl c::sealed::Sealed for Signature_void__0l__void__0p__0c__unsigned__long__0c__unsigned__long__0c__int__0l__0p__0r__0l__const__void__0p__0c__const__void__0p__0r__0r {}
impl c::expression::NativeFunctionSignature for Signature_void__0l__void__0p__0c__unsigned__long__0c__unsigned__long__0c__int__0l__0p__0r__0l__const__void__0p__0c__const__void__0p__0r__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void, usize, usize, ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_void, *const ::core::ffi::c_void) -> i32>) -> ()>>;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn() -> ()>>() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn() -> ()>>() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__void__0r__0x5f5f6174747269627574655f5f__0l__0l__noreturn__0r__0r;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
impl c::sealed::Sealed
    for Signature_void__0l__void__0r__0x5f5f6174747269627574655f5f__0l__0l__noreturn__0r__0r
{
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__void__0r__0x5f5f6174747269627574655f5f__0l__0l__noreturn__0r__0r
{
    type Physical =
        PhysicalFunction_C_unwind_0<::core::option::Option<unsafe extern "C-unwind" fn() -> ()>>;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::pg_atomic_flag) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::pg_atomic_flag) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__volatile__struct__pg_atomic_flag__0p__0r;
impl c::sealed::Sealed for Signature_void__0l__volatile__struct__pg_atomic_flag__0p__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__volatile__struct__pg_atomic_flag__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::pg_atomic_flag) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::pg_atomic_uint32, u32) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::pg_atomic_uint32, u32) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__volatile__struct__pg_atomic_uint32__0p__0c__unsigned__int__0r;
impl c::sealed::Sealed
    for Signature_void__0l__volatile__struct__pg_atomic_uint32__0p__0c__unsigned__int__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__volatile__struct__pg_atomic_uint32__0p__0c__unsigned__int__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::pg_atomic_uint32, u32) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::pg_atomic_uint64, u64) -> (),
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::pg_atomic_uint64, u64) -> (),
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__volatile__struct__pg_atomic_uint64__0p__0c__unsigned__long__0r;
impl c::sealed::Sealed
    for Signature_void__0l__volatile__struct__pg_atomic_uint64__0p__0c__unsigned__long__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__volatile__struct__pg_atomic_uint64__0p__0c__unsigned__long__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::pg_atomic_uint64, u64) -> (),
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::GinBtreeData,
                i32,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::GinBtreeData,
                i32,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0p__0l__struct__GinBtreeData__0p__0c__int__0r;
impl c::sealed::Sealed for Signature_void__0p__0l__struct__GinBtreeData__0p__0c__int__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_void__0p__0l__struct__GinBtreeData__0p__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::GinBtreeData,
                i32,
            ) -> *mut ::core::ffi::c_void,
        >,
    >;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::MemoryContextData,
                usize,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::MemoryContextData,
                usize,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0p__0l__struct__MemoryContextData__0p__0c__unsigned__long__0r;
#[cfg(any(feature = "pg15", feature = "pg16"))]
impl c::sealed::Sealed
    for Signature_void__0p__0l__struct__MemoryContextData__0p__0c__unsigned__long__0r
{
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0p__0l__struct__MemoryContextData__0p__0c__unsigned__long__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::MemoryContextData,
                usize,
            ) -> *mut ::core::ffi::c_void,
        >,
    >;
}
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::MemoryContextData,
                *mut ::core::ffi::c_void,
                usize,
            ) -> *mut ::core::ffi::c_void,
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
                usize,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[cfg(feature = "pg15")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0p__0l__struct__MemoryContextData__0p__0c__void__0p__0c__unsigned__long__0r;
#[cfg(feature = "pg15")]
impl c::sealed::Sealed
    for Signature_void__0p__0l__struct__MemoryContextData__0p__0c__void__0p__0c__unsigned__long__0r
{
}
#[cfg(feature = "pg15")]
impl c::expression::NativeFunctionSignature
    for Signature_void__0p__0l__struct__MemoryContextData__0p__0c__void__0p__0c__unsigned__long__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::MemoryContextData,
                *mut ::core::ffi::c_void,
                usize,
            ) -> *mut ::core::ffi::c_void,
        >,
    >;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(usize) -> *mut ::core::ffi::c_void>,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(usize) -> *mut ::core::ffi::c_void>,
    >() == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0p__0l__unsigned__long__0r;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::sealed::Sealed for Signature_void__0p__0l__unsigned__long__0r {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::NativeFunctionSignature for Signature_void__0p__0l__unsigned__long__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(usize) -> *mut ::core::ffi::c_void>,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                usize,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                usize,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0p__0l__void__0p__0c__const__void__0p__0c__unsigned__long__0r;
impl c::sealed::Sealed
    for Signature_void__0p__0l__void__0p__0c__const__void__0p__0c__unsigned__long__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0p__0l__void__0p__0c__const__void__0p__0c__unsigned__long__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                usize,
            ) -> *mut ::core::ffi::c_void,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::List,
                i32,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::List,
                i32,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0p__0l__const__struct__List__0p__0c__int__0r;
impl c::sealed::Sealed for Signature_void__0p__0l__const__struct__List__0p__0c__int__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_void__0p__0l__const__struct__List__0p__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::List,
                i32,
            ) -> *mut ::core::ffi::c_void,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::dlist_head,
                usize,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::dlist_head,
                usize,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0p__0l__struct__dlist_head__0p__0c__unsigned__long__0r;
impl c::sealed::Sealed for Signature_void__0p__0l__struct__dlist_head__0p__0c__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_void__0p__0l__struct__dlist_head__0p__0c__unsigned__long__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::dlist_head,
                usize,
            ) -> *mut ::core::ffi::c_void,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::slist_head,
                usize,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::slist_head,
                usize,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0p__0l__struct__slist_head__0p__0c__unsigned__long__0r;
impl c::sealed::Sealed for Signature_void__0p__0l__struct__slist_head__0p__0c__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_void__0p__0l__struct__slist_head__0p__0c__unsigned__long__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::slist_head,
                usize,
            ) -> *mut ::core::ffi::c_void,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(usize, usize) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(usize, usize) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0p__0l__unsigned__long__0c__unsigned__long__0r;
impl c::sealed::Sealed for Signature_void__0p__0l__unsigned__long__0c__unsigned__long__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_void__0p__0l__unsigned__long__0c__unsigned__long__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(usize, usize) -> *mut ::core::ffi::c_void,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(usize, usize, i32) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(usize, usize, i32) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0p__0l__unsigned__long__0c__unsigned__long__0c__int__0r;
impl c::sealed::Sealed for Signature_void__0p__0l__unsigned__long__0c__unsigned__long__0c__int__0r {}
impl c::expression::NativeFunctionSignature
    for Signature_void__0p__0l__unsigned__long__0c__unsigned__long__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(usize, usize, i32) -> *mut ::core::ffi::c_void,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
                usize,
                usize,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
                usize,
                usize,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0p__0l__void__0p__0c__unsigned__long__0c__unsigned__long__0r;
impl c::sealed::Sealed
    for Signature_void__0p__0l__void__0p__0c__unsigned__long__0c__unsigned__long__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0p__0l__void__0p__0c__unsigned__long__0c__unsigned__long__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
                usize,
                usize,
            ) -> *mut ::core::ffi::c_void,
        >,
    >;
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
                usize,
                usize,
                i32,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
                usize,
                usize,
                i32,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0p__0l__void__0p__0c__unsigned__long__0c__unsigned__long__0c__int__0r;
impl c::sealed::Sealed
    for Signature_void__0p__0l__void__0p__0c__unsigned__long__0c__unsigned__long__0c__int__0r
{
}
impl c::expression::NativeFunctionSignature
    for Signature_void__0p__0l__void__0p__0c__unsigned__long__0c__unsigned__long__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_4<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
                usize,
                usize,
                i32,
            ) -> *mut ::core::ffi::c_void,
        >,
    >;
}
/// Explicit C typedef identities for the selected native callback capabilities.
pub mod __pgrx_c_callbacks {
    #[doc = "Explicit C callback typedef `AcquireSampleRowsFunc` with its exact native binding storage."]
    pub type AcquireSampleRowsFunc = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_int__0l__struct__RelationData__0p__0c__int__0c__struct__HeapTupleData__0p__0p__0c__int__0c__double__0p__0c__double__0p__0r>;
    #[doc = "Explicit C callback typedef `AddForeignUpdateTargets_function` with its exact native binding storage."]
    pub type AddForeignUpdateTargets_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__PlannerInfo__0p__0c__unsigned__int__0c__struct__RangeTblEntry__0p__0c__struct__RelationData__0p__0r>;
    #[doc = "Explicit C callback typedef `AnalyzeAttrComputeStatsFunc` with its exact native binding storage."]
    pub type AnalyzeAttrComputeStatsFunc = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__VacAttrStats__0p__0c__unsigned__long__0l__0p__0r__0l__struct__VacAttrStats__0p__0c__int__0c__0x5f426f6f6c__0p__0r__0c__int__0c__double__0r>;
    #[doc = "Explicit C callback typedef `AnalyzeAttrFetchFunc` with its exact native binding storage."]
    pub type AnalyzeAttrFetchFunc = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_unsigned__long__0l__struct__VacAttrStats__0p__0c__int__0c__0x5f426f6f6c__0p__0r>;
    #[doc = "Explicit C callback typedef `AnalyzeForeignTable_function` with its exact native binding storage."]
    pub type AnalyzeForeignTable_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__RelationData__0p__0c__int__0l__0p__0p__0r__0l__struct__RelationData__0p__0c__int__0c__struct__HeapTupleData__0p__0p__0c__int__0c__double__0p__0c__double__0p__0r__0c__unsigned__int__0p__0r>;
    #[cfg(feature = "pg15")]
    #[doc = "Explicit C callback typedef `ArchiveCheckConfiguredCB` with its exact native binding storage."]
    pub type ArchiveCheckConfiguredCB = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__void__0r,
    >;
    #[cfg(feature = "pg15")]
    #[doc = "Explicit C callback typedef `ArchiveFileCB` with its exact native binding storage."]
    pub type ArchiveFileCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__const__char__0p__0c__const__char__0p__0r>;
    #[cfg(feature = "pg15")]
    #[doc = "Explicit C callback typedef `ArchiveModuleInit` with its exact native binding storage."]
    pub type ArchiveModuleInit = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__ArchiveModuleCallbacks__0p__0r,
    >;
    #[cfg(feature = "pg15")]
    #[doc = "Explicit C callback typedef `ArchiveShutdownCB` with its exact native binding storage."]
    pub type ArchiveShutdownCB = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__void__0r,
    >;
    #[doc = "Explicit C callback typedef `BeginDirectModify_function` with its exact native binding storage."]
    pub type BeginDirectModify_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__ForeignScanState__0p__0c__int__0r,
    >;
    #[doc = "Explicit C callback typedef `BeginForeignInsert_function` with its exact native binding storage."]
    pub type BeginForeignInsert_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ModifyTableState__0p__0c__struct__ResultRelInfo__0p__0r>;
    #[doc = "Explicit C callback typedef `BeginForeignModify_function` with its exact native binding storage."]
    pub type BeginForeignModify_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ModifyTableState__0p__0c__struct__ResultRelInfo__0p__0c__struct__List__0p__0c__int__0c__int__0r>;
    #[doc = "Explicit C callback typedef `BeginForeignScan_function` with its exact native binding storage."]
    pub type BeginForeignScan_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__ForeignScanState__0p__0c__int__0r,
    >;
    #[doc = "Explicit C callback typedef `BeginSampleScan_function` with its exact native binding storage."]
    pub type BeginSampleScan_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__SampleScanState__0p__0c__unsigned__long__0p__0c__int__0c__unsigned__int__0r>;
    #[doc = "Explicit C callback typedef `CCFastEqualFN` with its exact native binding storage."]
    pub type CCFastEqualFN = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__unsigned__long__0c__unsigned__long__0r>;
    #[doc = "Explicit C callback typedef `CCHashFN` with its exact native binding storage."]
    pub type CCHashFN = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_unsigned__int__0l__unsigned__long__0r,
    >;
    #[doc = "Explicit C callback typedef `CoerceParamHook` with its exact native binding storage."]
    pub type CoerceParamHook = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__Node__0p__0l__struct__ParseState__0p__0c__struct__Param__0p__0c__unsigned__int__0c__int__0c__int__0r>;
    #[doc = "Explicit C callback typedef `EOM_flatten_into_method` with its exact native binding storage."]
    pub type EOM_flatten_into_method = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ExpandedObjectHeader__0p__0c__void__0p__0c__unsigned__long__0r>;
    #[doc = "Explicit C callback typedef `EOM_get_flat_size_method` with its exact native binding storage."]
    pub type EOM_get_flat_size_method = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_unsigned__long__0l__struct__ExpandedObjectHeader__0p__0r>;
    #[doc = "Explicit C callback typedef `EndDirectModify_function` with its exact native binding storage."]
    pub type EndDirectModify_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__ForeignScanState__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `EndForeignInsert_function` with its exact native binding storage."]
    pub type EndForeignInsert_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__EState__0p__0c__struct__ResultRelInfo__0p__0r>;
    #[doc = "Explicit C callback typedef `EndForeignModify_function` with its exact native binding storage."]
    pub type EndForeignModify_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__EState__0p__0c__struct__ResultRelInfo__0p__0r>;
    #[doc = "Explicit C callback typedef `EndForeignScan_function` with its exact native binding storage."]
    pub type EndForeignScan_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__ForeignScanState__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `EndSampleScan_function` with its exact native binding storage."]
    pub type EndSampleScan_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__SampleScanState__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `EstimateDSMForeignScan_function` with its exact native binding storage."]
    pub type EstimateDSMForeignScan_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_unsigned__long__0l__struct__ForeignScanState__0p__0c__struct__ParallelContext__0p__0r>;
    #[doc = "Explicit C callback typedef `ExecEvalBoolSubroutine` with its exact native binding storage."]
    pub type ExecEvalBoolSubroutine = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__ExprState__0p__0c__struct__ExprEvalStep__0p__0c__struct__ExprContext__0p__0r>;
    #[doc = "Explicit C callback typedef `ExecEvalSubroutine` with its exact native binding storage."]
    pub type ExecEvalSubroutine = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ExprState__0p__0c__struct__ExprEvalStep__0p__0c__struct__ExprContext__0p__0r>;
    #[doc = "Explicit C callback typedef `ExecForeignBatchInsert_function` with its exact native binding storage."]
    pub type ExecForeignBatchInsert_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__TupleTableSlot__0p__0p__0l__struct__EState__0p__0c__struct__ResultRelInfo__0p__0c__struct__TupleTableSlot__0p__0p__0c__struct__TupleTableSlot__0p__0p__0c__int__0p__0r>;
    #[doc = "Explicit C callback typedef `ExecForeignDelete_function` with its exact native binding storage."]
    pub type ExecForeignDelete_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__TupleTableSlot__0p__0l__struct__EState__0p__0c__struct__ResultRelInfo__0p__0c__struct__TupleTableSlot__0p__0c__struct__TupleTableSlot__0p__0r>;
    #[doc = "Explicit C callback typedef `ExecForeignInsert_function` with its exact native binding storage."]
    pub type ExecForeignInsert_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__TupleTableSlot__0p__0l__struct__EState__0p__0c__struct__ResultRelInfo__0p__0c__struct__TupleTableSlot__0p__0c__struct__TupleTableSlot__0p__0r>;
    #[doc = "Explicit C callback typedef `ExecForeignTruncate_function` with its exact native binding storage."]
    pub type ExecForeignTruncate_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__List__0p__0c__enum__DropBehavior__0c__0x5f426f6f6c__0r>;
    #[doc = "Explicit C callback typedef `ExecForeignUpdate_function` with its exact native binding storage."]
    pub type ExecForeignUpdate_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__TupleTableSlot__0p__0l__struct__EState__0p__0c__struct__ResultRelInfo__0p__0c__struct__TupleTableSlot__0p__0c__struct__TupleTableSlot__0p__0r>;
    #[doc = "Explicit C callback typedef `ExecProcNodeMtd` with its exact native binding storage."]
    pub type ExecProcNodeMtd = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__TupleTableSlot__0p__0l__struct__PlanState__0p__0r>;
    #[doc = "Explicit C callback typedef `ExecScanAccessMtd` with its exact native binding storage."]
    pub type ExecScanAccessMtd = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__TupleTableSlot__0p__0l__struct__ScanState__0p__0r>;
    #[doc = "Explicit C callback typedef `ExecScanRecheckMtd` with its exact native binding storage."]
    pub type ExecScanRecheckMtd = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__ScanState__0p__0c__struct__TupleTableSlot__0p__0r>;
    #[cfg(feature = "pg15")]
    #[doc = "Explicit C callback typedef `ExecutorCheckPerms_hook_type` with its exact native binding storage."]
    pub type ExecutorCheckPerms_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__List__0p__0c__0x5f426f6f6c__0r>;
    #[doc = "Explicit C callback typedef `ExecutorEnd_hook_type` with its exact native binding storage."]
    pub type ExecutorEnd_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__QueryDesc__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `ExecutorFinish_hook_type` with its exact native binding storage."]
    pub type ExecutorFinish_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__QueryDesc__0p__0r,
    >;
    #[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
    #[doc = "Explicit C callback typedef `ExecutorRun_hook_type` with its exact native binding storage."]
    pub type ExecutorRun_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__QueryDesc__0p__0c__enum__ScanDirection__0c__unsigned__long__0c__0x5f426f6f6c__0r>;
    #[doc = "Explicit C callback typedef `ExecutorStart_hook_type` with its exact native binding storage."]
    pub type ExecutorStart_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__QueryDesc__0p__0c__int__0r,
    >;
    #[doc = "Explicit C callback typedef `ExplainDirectModify_function` with its exact native binding storage."]
    pub type ExplainDirectModify_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ForeignScanState__0p__0c__struct__ExplainState__0p__0r>;
    #[doc = "Explicit C callback typedef `ExplainForeignModify_function` with its exact native binding storage."]
    pub type ExplainForeignModify_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ModifyTableState__0p__0c__struct__ResultRelInfo__0p__0c__struct__List__0p__0c__int__0c__struct__ExplainState__0p__0r>;
    #[doc = "Explicit C callback typedef `ExplainForeignScan_function` with its exact native binding storage."]
    pub type ExplainForeignScan_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ForeignScanState__0p__0c__struct__ExplainState__0p__0r>;
    #[doc = "Explicit C callback typedef `ExplainOneQuery_hook_type` with its exact native binding storage."]
    pub type ExplainOneQuery_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__Query__0p__0c__int__0c__struct__IntoClause__0p__0c__struct__ExplainState__0p__0c__const__char__0p__0c__struct__ParamListInfoData__0p__0c__struct__QueryEnvironment__0p__0r>;
    #[doc = "Explicit C callback typedef `ExprContextCallbackFunction` with its exact native binding storage."]
    pub type ExprContextCallbackFunction = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__unsigned__long__0r,
    >;
    #[doc = "Explicit C callback typedef `ExprStateEvalFunc` with its exact native binding storage."]
    pub type ExprStateEvalFunc = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_unsigned__long__0l__struct__ExprState__0p__0c__struct__ExprContext__0p__0c__0x5f426f6f6c__0p__0r>;
    #[doc = "Explicit C callback typedef `ForeignAsyncConfigureWait_function` with its exact native binding storage."]
    pub type ForeignAsyncConfigureWait_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__AsyncRequest__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `ForeignAsyncNotify_function` with its exact native binding storage."]
    pub type ForeignAsyncNotify_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__AsyncRequest__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `ForeignAsyncRequest_function` with its exact native binding storage."]
    pub type ForeignAsyncRequest_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__AsyncRequest__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `GetForeignJoinPaths_function` with its exact native binding storage."]
    pub type GetForeignJoinPaths_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__RelOptInfo__0p__0c__enum__JoinType__0c__struct__JoinPathExtraData__0p__0r>;
    #[doc = "Explicit C callback typedef `GetForeignModifyBatchSize_function` with its exact native binding storage."]
    pub type GetForeignModifyBatchSize_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_int__0l__struct__ResultRelInfo__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `GetForeignPaths_function` with its exact native binding storage."]
    pub type GetForeignPaths_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__unsigned__int__0r>;
    #[doc = "Explicit C callback typedef `GetForeignPlan_function` with its exact native binding storage."]
    pub type GetForeignPlan_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__ForeignScan__0p__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__unsigned__int__0c__struct__ForeignPath__0p__0c__struct__List__0p__0c__struct__List__0p__0c__struct__Plan__0p__0r>;
    #[doc = "Explicit C callback typedef `GetForeignRelSize_function` with its exact native binding storage."]
    pub type GetForeignRelSize_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__unsigned__int__0r>;
    #[doc = "Explicit C callback typedef `GetForeignRowMarkType_function` with its exact native binding storage."]
    pub type GetForeignRowMarkType_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_enum__RowMarkType__0l__struct__RangeTblEntry__0p__0c__enum__LockClauseStrength__0r>;
    #[doc = "Explicit C callback typedef `GetForeignUpperPaths_function` with its exact native binding storage."]
    pub type GetForeignUpperPaths_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__PlannerInfo__0p__0c__enum__UpperRelationKind__0c__struct__RelOptInfo__0p__0c__struct__RelOptInfo__0p__0c__void__0p__0r>;
    #[doc = "Explicit C callback typedef `GucBoolAssignHook` with its exact native binding storage."]
    pub type GucBoolAssignHook = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__0x5f426f6f6c__0c__void__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `GucBoolCheckHook` with its exact native binding storage."]
    pub type GucBoolCheckHook = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__0x5f426f6f6c__0p__0c__void__0p__0p__0c__GucSource__0r>;
    #[doc = "Explicit C callback typedef `GucEnumAssignHook` with its exact native binding storage."]
    pub type GucEnumAssignHook = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__int__0c__void__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `GucEnumCheckHook` with its exact native binding storage."]
    pub type GucEnumCheckHook = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__int__0p__0c__void__0p__0p__0c__GucSource__0r>;
    #[doc = "Explicit C callback typedef `GucIntAssignHook` with its exact native binding storage."]
    pub type GucIntAssignHook = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__int__0c__void__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `GucIntCheckHook` with its exact native binding storage."]
    pub type GucIntCheckHook = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__int__0p__0c__void__0p__0p__0c__GucSource__0r>;
    #[doc = "Explicit C callback typedef `GucRealAssignHook` with its exact native binding storage."]
    pub type GucRealAssignHook = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__double__0c__void__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `GucRealCheckHook` with its exact native binding storage."]
    pub type GucRealCheckHook = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__double__0p__0c__void__0p__0p__0c__GucSource__0r>;
    #[doc = "Explicit C callback typedef `GucShowHook` with its exact native binding storage."]
    pub type GucShowHook = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_const__char__0p__0l__void__0r,
    >;
    #[doc = "Explicit C callback typedef `GucStringAssignHook` with its exact native binding storage."]
    pub type GucStringAssignHook = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__const__char__0p__0c__void__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `GucStringCheckHook` with its exact native binding storage."]
    pub type GucStringCheckHook = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__char__0p__0p__0c__void__0p__0p__0c__GucSource__0r>;
    #[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
    #[doc = "Explicit C callback typedef `HashAllocFunc` with its exact native binding storage."]
    pub type HashAllocFunc = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0p__0l__unsigned__long__0r,
    >;
    #[doc = "Explicit C callback typedef `HashCompareFunc` with its exact native binding storage."]
    pub type HashCompareFunc = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_int__0l__const__void__0p__0c__const__void__0p__0c__unsigned__long__0r>;
    #[doc = "Explicit C callback typedef `HashCopyFunc` with its exact native binding storage."]
    pub type HashCopyFunc = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0p__0l__void__0p__0c__const__void__0p__0c__unsigned__long__0r>;
    #[doc = "Explicit C callback typedef `HashValueFunc` with its exact native binding storage."]
    pub type HashValueFunc = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_unsigned__int__0l__const__void__0p__0c__unsigned__long__0r>;
    #[doc = "Explicit C callback typedef `ImportForeignSchema_function` with its exact native binding storage."]
    pub type ImportForeignSchema_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__List__0p__0l__struct__ImportForeignSchemaStmt__0p__0c__unsigned__int__0r>;
    #[doc = "Explicit C callback typedef `IndexBuildCallback` with its exact native binding storage."]
    pub type IndexBuildCallback = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__RelationData__0p__0c__struct__ItemPointerData__0p__0c__unsigned__long__0p__0c__0x5f426f6f6c__0p__0c__0x5f426f6f6c__0c__void__0p__0r>;
    #[doc = "Explicit C callback typedef `IndexBulkDeleteCallback` with its exact native binding storage."]
    pub type IndexBulkDeleteCallback = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__ItemPointerData__0p__0c__void__0p__0r>;
    #[doc = "Explicit C callback typedef `InitSampleScan_function` with its exact native binding storage."]
    pub type InitSampleScan_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__SampleScanState__0p__0c__int__0r,
    >;
    #[doc = "Explicit C callback typedef `InitializeDSMForeignScan_function` with its exact native binding storage."]
    pub type InitializeDSMForeignScan_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ForeignScanState__0p__0c__struct__ParallelContext__0p__0c__void__0p__0r>;
    #[doc = "Explicit C callback typedef `InitializeWorkerForeignScan_function` with its exact native binding storage."]
    pub type InitializeWorkerForeignScan_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ForeignScanState__0p__0c__struct__shm_toc__0p__0c__void__0p__0r>;
    #[doc = "Explicit C callback typedef `IsForeignPathAsyncCapable_function` with its exact native binding storage."]
    pub type IsForeignPathAsyncCapable_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__ForeignPath__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `IsForeignRelUpdatable_function` with its exact native binding storage."]
    pub type IsForeignRelUpdatable_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_int__0l__struct__RelationData__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `IsForeignScanParallelSafe_function` with its exact native binding storage."]
    pub type IsForeignScanParallelSafe_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__RangeTblEntry__0p__0r>;
    #[doc = "Explicit C callback typedef `IterateDirectModify_function` with its exact native binding storage."]
    pub type IterateDirectModify_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__TupleTableSlot__0p__0l__struct__ForeignScanState__0p__0r>;
    #[doc = "Explicit C callback typedef `IterateForeignScan_function` with its exact native binding storage."]
    pub type IterateForeignScan_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__TupleTableSlot__0p__0l__struct__ForeignScanState__0p__0r>;
    #[doc = "Explicit C callback typedef `JitProviderCompileExprCB` with its exact native binding storage."]
    pub type JitProviderCompileExprCB = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__ExprState__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `JitProviderInit` with its exact native binding storage."]
    pub type JitProviderInit = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__JitProviderCallbacks__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `JitProviderReleaseContextCB` with its exact native binding storage."]
    pub type JitProviderReleaseContextCB = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__JitContext__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `JitProviderResetAfterErrorCB` with its exact native binding storage."]
    pub type JitProviderResetAfterErrorCB = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__void__0r,
    >;
    #[doc = "Explicit C callback typedef `LogicalDecodeBeginCB` with its exact native binding storage."]
    pub type LogicalDecodeBeginCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__ReorderBufferTXN__0p__0r>;
    #[doc = "Explicit C callback typedef `LogicalDecodeBeginPrepareCB` with its exact native binding storage."]
    pub type LogicalDecodeBeginPrepareCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__ReorderBufferTXN__0p__0r>;
    #[doc = "Explicit C callback typedef `LogicalDecodeChangeCB` with its exact native binding storage."]
    pub type LogicalDecodeChangeCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__ReorderBufferTXN__0p__0c__struct__RelationData__0p__0c__struct__ReorderBufferChange__0p__0r>;
    #[doc = "Explicit C callback typedef `LogicalDecodeCommitCB` with its exact native binding storage."]
    pub type LogicalDecodeCommitCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0r>;
    #[doc = "Explicit C callback typedef `LogicalDecodeCommitPreparedCB` with its exact native binding storage."]
    pub type LogicalDecodeCommitPreparedCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0r>;
    #[doc = "Explicit C callback typedef `LogicalDecodeFilterByOriginCB` with its exact native binding storage."]
    pub type LogicalDecodeFilterByOriginCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__LogicalDecodingContext__0p__0c__unsigned__short__0r>;
    #[doc = "Explicit C callback typedef `LogicalDecodeFilterPrepareCB` with its exact native binding storage."]
    pub type LogicalDecodeFilterPrepareCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__LogicalDecodingContext__0p__0c__unsigned__int__0c__const__char__0p__0r>;
    #[doc = "Explicit C callback typedef `LogicalDecodeMessageCB` with its exact native binding storage."]
    pub type LogicalDecodeMessageCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0c__0x5f426f6f6c__0c__const__char__0p__0c__unsigned__long__0c__const__char__0p__0r>;
    #[doc = "Explicit C callback typedef `LogicalDecodePrepareCB` with its exact native binding storage."]
    pub type LogicalDecodePrepareCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0r>;
    #[doc = "Explicit C callback typedef `LogicalDecodeRollbackPreparedCB` with its exact native binding storage."]
    pub type LogicalDecodeRollbackPreparedCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0c__long__0r>;
    #[doc = "Explicit C callback typedef `LogicalDecodeShutdownCB` with its exact native binding storage."]
    pub type LogicalDecodeShutdownCB = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__LogicalDecodingContext__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `LogicalDecodeStartupCB` with its exact native binding storage."]
    pub type LogicalDecodeStartupCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__OutputPluginOptions__0p__0c__0x5f426f6f6c__0r>;
    #[doc = "Explicit C callback typedef `LogicalDecodeStreamAbortCB` with its exact native binding storage."]
    pub type LogicalDecodeStreamAbortCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0r>;
    #[doc = "Explicit C callback typedef `LogicalDecodeStreamChangeCB` with its exact native binding storage."]
    pub type LogicalDecodeStreamChangeCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__ReorderBufferTXN__0p__0c__struct__RelationData__0p__0c__struct__ReorderBufferChange__0p__0r>;
    #[doc = "Explicit C callback typedef `LogicalDecodeStreamCommitCB` with its exact native binding storage."]
    pub type LogicalDecodeStreamCommitCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0r>;
    #[doc = "Explicit C callback typedef `LogicalDecodeStreamMessageCB` with its exact native binding storage."]
    pub type LogicalDecodeStreamMessageCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0c__0x5f426f6f6c__0c__const__char__0p__0c__unsigned__long__0c__const__char__0p__0r>;
    #[doc = "Explicit C callback typedef `LogicalDecodeStreamPrepareCB` with its exact native binding storage."]
    pub type LogicalDecodeStreamPrepareCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0r>;
    #[doc = "Explicit C callback typedef `LogicalDecodeStreamStartCB` with its exact native binding storage."]
    pub type LogicalDecodeStreamStartCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__ReorderBufferTXN__0p__0r>;
    #[doc = "Explicit C callback typedef `LogicalDecodeStreamStopCB` with its exact native binding storage."]
    pub type LogicalDecodeStreamStopCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__ReorderBufferTXN__0p__0r>;
    #[doc = "Explicit C callback typedef `LogicalOutputPluginInit` with its exact native binding storage."]
    pub type LogicalOutputPluginInit = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__OutputPluginCallbacks__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `LogicalOutputPluginWriterPrepareWrite` with its exact native binding storage."]
    pub type LogicalOutputPluginWriterPrepareWrite = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__LogicalDecodingContext__0p__0c__unsigned__long__0c__unsigned__int__0c__0x5f426f6f6c__0r>;
    #[doc = "Explicit C callback typedef `LogicalOutputPluginWriterUpdateProgress` with its exact native binding storage."]
    pub type LogicalOutputPluginWriterUpdateProgress = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__LogicalDecodingContext__0p__0c__unsigned__long__0c__unsigned__int__0c__0x5f426f6f6c__0r>;
    #[doc = "Explicit C callback typedef `LogicalOutputPluginWriterWrite` with its exact native binding storage."]
    pub type LogicalOutputPluginWriterWrite = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__LogicalDecodingContext__0p__0c__unsigned__long__0c__unsigned__int__0c__0x5f426f6f6c__0r>;
    #[doc = "Explicit C callback typedef `MemoryContextCallbackFunction` with its exact native binding storage."]
    pub type MemoryContextCallbackFunction = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__void__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `MemoryStatsPrintFunc` with its exact native binding storage."]
    pub type MemoryStatsPrintFunc = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__MemoryContextData__0p__0c__void__0p__0c__const__char__0p__0c__0x5f426f6f6c__0r>;
    #[doc = "Explicit C callback typedef `NextSampleBlock_function` with its exact native binding storage."]
    pub type NextSampleBlock_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_unsigned__int__0l__struct__SampleScanState__0p__0c__unsigned__int__0r>;
    #[doc = "Explicit C callback typedef `NextSampleTuple_function` with its exact native binding storage."]
    pub type NextSampleTuple_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_unsigned__short__0l__struct__SampleScanState__0p__0c__unsigned__int__0c__unsigned__short__0r>;
    #[doc = "Explicit C callback typedef `PGFInfoFunction` with its exact native binding storage."]
    pub type PGFInfoFunction = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_const__Pg_finfo_record__0p__0l__void__0r,
    >;
    #[doc = "Explicit C callback typedef `PGFunction` with its exact native binding storage."]
    pub type PGFunction = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_unsigned__long__0l__struct__FunctionCallInfoBaseData__0p__0r>;
    #[doc = "Explicit C callback typedef `PGModuleMagicFunction` with its exact native binding storage."]
    pub type PGModuleMagicFunction = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_const__Pg_magic_struct__0p__0l__void__0r,
    >;
    #[doc = "Explicit C callback typedef `ParamCompileHook` with its exact native binding storage."]
    pub type ParamCompileHook = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ParamListInfoData__0p__0c__struct__Param__0p__0c__struct__ExprState__0p__0c__unsigned__long__0p__0c__0x5f426f6f6c__0p__0r>;
    #[doc = "Explicit C callback typedef `ParamFetchHook` with its exact native binding storage."]
    pub type ParamFetchHook = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__ParamExternData__0p__0l__struct__ParamListInfoData__0p__0c__int__0c__0x5f426f6f6c__0c__struct__ParamExternData__0p__0r>;
    #[doc = "Explicit C callback typedef `ParseParamRefHook` with its exact native binding storage."]
    pub type ParseParamRefHook = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__Node__0p__0l__struct__ParseState__0p__0c__struct__ParamRef__0p__0r>;
    #[doc = "Explicit C callback typedef `ParserSetupHook` with its exact native binding storage."]
    pub type ParserSetupHook = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__ParseState__0p__0c__void__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `PlanDirectModify_function` with its exact native binding storage."]
    pub type PlanDirectModify_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__PlannerInfo__0p__0c__struct__ModifyTable__0p__0c__unsigned__int__0c__int__0r>;
    #[doc = "Explicit C callback typedef `PlanForeignModify_function` with its exact native binding storage."]
    pub type PlanForeignModify_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__List__0p__0l__struct__PlannerInfo__0p__0c__struct__ModifyTable__0p__0c__unsigned__int__0c__int__0r>;
    #[doc = "Explicit C callback typedef `PostParseColumnRefHook` with its exact native binding storage."]
    pub type PostParseColumnRefHook = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__Node__0p__0l__struct__ParseState__0p__0c__struct__ColumnRef__0p__0c__struct__Node__0p__0r>;
    #[doc = "Explicit C callback typedef `PreParseColumnRefHook` with its exact native binding storage."]
    pub type PreParseColumnRefHook = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__Node__0p__0l__struct__ParseState__0p__0c__struct__ColumnRef__0p__0r>;
    #[doc = "Explicit C callback typedef `ProcessUtility_hook_type` with its exact native binding storage."]
    pub type ProcessUtility_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__PlannedStmt__0p__0c__const__char__0p__0c__0x5f426f6f6c__0c__ProcessUtilityContext__0c__struct__ParamListInfoData__0p__0c__struct__QueryEnvironment__0p__0c__struct__0x5f446573745265636569766572__0p__0c__struct__QueryCompletion__0p__0r>;
    #[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
    #[doc = "Explicit C callback typedef `PushFunction` with its exact native binding storage."]
    pub type PushFunction = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__unsigned__long__0c__struct__TSQueryParserStateData__0p__0c__char__0p__0c__int__0c__short__0c__0x5f426f6f6c__0r>;
    #[doc = "Explicit C callback typedef `RangeVarGetRelidCallback` with its exact native binding storage."]
    pub type RangeVarGetRelidCallback = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__const__struct__RangeVar__0p__0c__unsigned__int__0c__unsigned__int__0c__void__0p__0r>;
    #[doc = "Explicit C callback typedef `ReInitializeDSMForeignScan_function` with its exact native binding storage."]
    pub type ReInitializeDSMForeignScan_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ForeignScanState__0p__0c__struct__ParallelContext__0p__0c__void__0p__0r>;
    #[doc = "Explicit C callback typedef `ReScanForeignScan_function` with its exact native binding storage."]
    pub type ReScanForeignScan_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__ForeignScanState__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `RecheckForeignScan_function` with its exact native binding storage."]
    pub type RecheckForeignScan_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__ForeignScanState__0p__0c__struct__TupleTableSlot__0p__0r>;
    #[doc = "Explicit C callback typedef `RefetchForeignRow_function` with its exact native binding storage."]
    pub type RefetchForeignRow_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__EState__0p__0c__struct__ExecRowMark__0p__0c__unsigned__long__0c__struct__TupleTableSlot__0p__0c__0x5f426f6f6c__0p__0r>;
    #[doc = "Explicit C callback typedef `RelcacheCallbackFunction` with its exact native binding storage."]
    pub type RelcacheCallbackFunction = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__unsigned__long__0c__unsigned__int__0r,
    >;
    #[doc = "Explicit C callback typedef `ReorderBufferApplyChangeCB` with its exact native binding storage."]
    pub type ReorderBufferApplyChangeCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0c__struct__RelationData__0p__0c__struct__ReorderBufferChange__0p__0r>;
    #[doc = "Explicit C callback typedef `ReorderBufferBeginCB` with its exact native binding storage."]
    pub type ReorderBufferBeginCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0r>;
    #[doc = "Explicit C callback typedef `ReorderBufferBeginPrepareCB` with its exact native binding storage."]
    pub type ReorderBufferBeginPrepareCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0r>;
    #[doc = "Explicit C callback typedef `ReorderBufferCommitCB` with its exact native binding storage."]
    pub type ReorderBufferCommitCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0r>;
    #[doc = "Explicit C callback typedef `ReorderBufferCommitPreparedCB` with its exact native binding storage."]
    pub type ReorderBufferCommitPreparedCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0r>;
    #[doc = "Explicit C callback typedef `ReorderBufferMessageCB` with its exact native binding storage."]
    pub type ReorderBufferMessageCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0c__0x5f426f6f6c__0c__const__char__0p__0c__unsigned__long__0c__const__char__0p__0r>;
    #[doc = "Explicit C callback typedef `ReorderBufferPrepareCB` with its exact native binding storage."]
    pub type ReorderBufferPrepareCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0r>;
    #[doc = "Explicit C callback typedef `ReorderBufferRollbackPreparedCB` with its exact native binding storage."]
    pub type ReorderBufferRollbackPreparedCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0c__long__0r>;
    #[doc = "Explicit C callback typedef `ReorderBufferStreamAbortCB` with its exact native binding storage."]
    pub type ReorderBufferStreamAbortCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0r>;
    #[doc = "Explicit C callback typedef `ReorderBufferStreamChangeCB` with its exact native binding storage."]
    pub type ReorderBufferStreamChangeCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0c__struct__RelationData__0p__0c__struct__ReorderBufferChange__0p__0r>;
    #[doc = "Explicit C callback typedef `ReorderBufferStreamCommitCB` with its exact native binding storage."]
    pub type ReorderBufferStreamCommitCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0r>;
    #[doc = "Explicit C callback typedef `ReorderBufferStreamMessageCB` with its exact native binding storage."]
    pub type ReorderBufferStreamMessageCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0c__0x5f426f6f6c__0c__const__char__0p__0c__unsigned__long__0c__const__char__0p__0r>;
    #[doc = "Explicit C callback typedef `ReorderBufferStreamPrepareCB` with its exact native binding storage."]
    pub type ReorderBufferStreamPrepareCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0r>;
    #[doc = "Explicit C callback typedef `ReorderBufferStreamStartCB` with its exact native binding storage."]
    pub type ReorderBufferStreamStartCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0r>;
    #[doc = "Explicit C callback typedef `ReorderBufferStreamStopCB` with its exact native binding storage."]
    pub type ReorderBufferStreamStopCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0r>;
    #[doc = "Explicit C callback typedef `ReparameterizeForeignPathByChild_function` with its exact native binding storage."]
    pub type ReparameterizeForeignPathByChild_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__List__0p__0l__struct__PlannerInfo__0p__0c__struct__List__0p__0c__struct__RelOptInfo__0p__0r>;
    #[doc = "Explicit C callback typedef `ResourceReleaseCallback` with its exact native binding storage."]
    pub type ResourceReleaseCallback = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__ResourceReleasePhase__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__void__0p__0r>;
    #[doc = "Explicit C callback typedef `SampleScanGetSampleSize_function` with its exact native binding storage."]
    pub type SampleScanGetSampleSize_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__List__0p__0c__unsigned__int__0p__0c__double__0p__0r>;
    #[doc = "Explicit C callback typedef `ScanKeywordHashFunc` with its exact native binding storage."]
    pub type ScanKeywordHashFunc = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_int__0l__const__void__0p__0c__unsigned__long__0r,
    >;
    #[doc = "Explicit C callback typedef `ShutdownForeignScan_function` with its exact native binding storage."]
    pub type ShutdownForeignScan_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__ForeignScanState__0p__0r,
    >;
    #[cfg(any(feature = "pg15", feature = "pg16"))]
    #[doc = "Explicit C callback typedef `SlruScanCallback` with its exact native binding storage."]
    pub type SlruScanCallback = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__SlruCtlData__0p__0c__char__0p__0c__int__0c__void__0p__0r>;
    #[doc = "Explicit C callback typedef `SubXactCallback` with its exact native binding storage."]
    pub type SubXactCallback = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__SubXactEvent__0c__unsigned__int__0c__unsigned__int__0c__void__0p__0r>;
    #[doc = "Explicit C callback typedef `SubscriptExecSetup` with its exact native binding storage."]
    pub type SubscriptExecSetup = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__const__struct__SubscriptingRef__0p__0c__struct__SubscriptingRefState__0p__0c__struct__SubscriptExecSteps__0p__0r>;
    #[doc = "Explicit C callback typedef `SubscriptTransform` with its exact native binding storage."]
    pub type SubscriptTransform = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__SubscriptingRef__0p__0c__struct__List__0p__0c__struct__ParseState__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0r>;
    #[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
    #[doc = "Explicit C callback typedef `SyscacheCallbackFunction` with its exact native binding storage."]
    pub type SyscacheCallbackFunction = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__unsigned__long__0c__int__0c__unsigned__int__0r>;
    #[doc = "Explicit C callback typedef `TSExecuteCallback` with its exact native binding storage."]
    pub type TSExecuteCallback = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_TSTernaryValue__0l__void__0p__0c__QueryOperand__0p__0c__struct__ExecPhraseData__0p__0r>;
    #[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
    #[doc = "Explicit C callback typedef `TwoPhaseCallback` with its exact native binding storage."]
    pub type TwoPhaseCallback = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__unsigned__int__0c__unsigned__short__0c__void__0p__0c__unsigned__int__0r>;
    #[doc = "Explicit C callback typedef `WALSegmentCloseCB` with its exact native binding storage."]
    pub type WALSegmentCloseCB = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__XLogReaderState__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `WALSegmentOpenCB` with its exact native binding storage."]
    pub type WALSegmentOpenCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__XLogReaderState__0p__0c__unsigned__long__0c__unsigned__int__0p__0r>;
    #[doc = "Explicit C callback typedef `XLogPageReadCB` with its exact native binding storage."]
    pub type XLogPageReadCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_int__0l__struct__XLogReaderState__0p__0c__unsigned__long__0c__int__0c__unsigned__long__0c__char__0p__0r>;
    #[doc = "Explicit C callback typedef `XactCallback` with its exact native binding storage."]
    pub type XactCallback = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__XactEvent__0c__void__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `amadjustmembers_function` with its exact native binding storage."]
    pub type amadjustmembers_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__unsigned__int__0c__unsigned__int__0c__struct__List__0p__0c__struct__List__0p__0r>;
    #[doc = "Explicit C callback typedef `ambeginscan_function` with its exact native binding storage."]
    pub type ambeginscan_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__IndexScanDescData__0p__0l__struct__RelationData__0p__0c__int__0c__int__0r>;
    #[doc = "Explicit C callback typedef `ambuild_function` with its exact native binding storage."]
    pub type ambuild_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__IndexBuildResult__0p__0l__struct__RelationData__0p__0c__struct__RelationData__0p__0c__struct__IndexInfo__0p__0r>;
    #[doc = "Explicit C callback typedef `ambuildempty_function` with its exact native binding storage."]
    pub type ambuildempty_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__RelationData__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `ambuildphasename_function` with its exact native binding storage."]
    pub type ambuildphasename_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_char__0p__0l__long__0r,
    >;
    #[doc = "Explicit C callback typedef `ambulkdelete_function` with its exact native binding storage."]
    pub type ambulkdelete_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__IndexBulkDeleteResult__0p__0l__struct__IndexVacuumInfo__0p__0c__struct__IndexBulkDeleteResult__0p__0c__0x5f426f6f6c__0l__0p__0r__0l__struct__ItemPointerData__0p__0c__void__0p__0r__0c__void__0p__0r>;
    #[doc = "Explicit C callback typedef `amcanreturn_function` with its exact native binding storage."]
    pub type amcanreturn_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__RelationData__0p__0c__int__0r>;
    #[doc = "Explicit C callback typedef `amcostestimate_function` with its exact native binding storage."]
    pub type amcostestimate_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__PlannerInfo__0p__0c__struct__IndexPath__0p__0c__double__0c__double__0p__0c__double__0p__0c__double__0p__0c__double__0p__0c__double__0p__0r>;
    #[doc = "Explicit C callback typedef `amendscan_function` with its exact native binding storage."]
    pub type amendscan_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__IndexScanDescData__0p__0r,
    >;
    #[cfg(any(feature = "pg15", feature = "pg16"))]
    #[doc = "Explicit C callback typedef `amestimateparallelscan_function` with its exact native binding storage."]
    pub type amestimateparallelscan_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_unsigned__long__0l__void__0r,
    >;
    #[doc = "Explicit C callback typedef `amgetbitmap_function` with its exact native binding storage."]
    pub type amgetbitmap_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_long__0l__struct__IndexScanDescData__0p__0c__struct__TIDBitmap__0p__0r>;
    #[doc = "Explicit C callback typedef `amgettuple_function` with its exact native binding storage."]
    pub type amgettuple_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__IndexScanDescData__0p__0c__enum__ScanDirection__0r>;
    #[doc = "Explicit C callback typedef `aminitparallelscan_function` with its exact native binding storage."]
    pub type aminitparallelscan_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__void__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `aminsert_function` with its exact native binding storage."]
    pub type aminsert_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__RelationData__0p__0c__unsigned__long__0p__0c__0x5f426f6f6c__0p__0c__struct__ItemPointerData__0p__0c__struct__RelationData__0p__0c__enum__IndexUniqueCheck__0c__0x5f426f6f6c__0c__struct__IndexInfo__0p__0r>;
    #[doc = "Explicit C callback typedef `ammarkpos_function` with its exact native binding storage."]
    pub type ammarkpos_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__IndexScanDescData__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `amoptions_function` with its exact native binding storage."]
    pub type amoptions_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__varlena__0p__0l__unsigned__long__0c__0x5f426f6f6c__0r>;
    #[doc = "Explicit C callback typedef `amparallelrescan_function` with its exact native binding storage."]
    pub type amparallelrescan_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__IndexScanDescData__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `amproperty_function` with its exact native binding storage."]
    pub type amproperty_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__unsigned__int__0c__int__0c__enum__IndexAMProperty__0c__const__char__0p__0c__0x5f426f6f6c__0p__0c__0x5f426f6f6c__0p__0r>;
    #[doc = "Explicit C callback typedef `amrescan_function` with its exact native binding storage."]
    pub type amrescan_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__IndexScanDescData__0p__0c__struct__ScanKeyData__0p__0c__int__0c__struct__ScanKeyData__0p__0c__int__0r>;
    #[doc = "Explicit C callback typedef `amrestrpos_function` with its exact native binding storage."]
    pub type amrestrpos_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__IndexScanDescData__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `amvacuumcleanup_function` with its exact native binding storage."]
    pub type amvacuumcleanup_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__IndexBulkDeleteResult__0p__0l__struct__IndexVacuumInfo__0p__0c__struct__IndexBulkDeleteResult__0p__0r>;
    #[doc = "Explicit C callback typedef `amvalidate_function` with its exact native binding storage."]
    pub type amvalidate_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__unsigned__int__0r,
    >;
    #[doc = "Explicit C callback typedef `bgworker_main_type` with its exact native binding storage."]
    pub type bgworker_main_type = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__unsigned__long__0r,
    >;
    #[doc = "Explicit C callback typedef `brin_serialize_callback_type` with its exact native binding storage."]
    pub type brin_serialize_callback_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__BrinDesc__0p__0c__unsigned__long__0c__unsigned__long__0p__0r>;
    #[doc = "Explicit C callback typedef `check_function_callback` with its exact native binding storage."]
    pub type check_function_callback = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__unsigned__int__0c__void__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `check_object_relabel_type` with its exact native binding storage."]
    pub type check_object_relabel_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__const__struct__ObjectAddress__0p__0c__const__char__0p__0r>;
    #[doc = "Explicit C callback typedef `check_password_hook_type` with its exact native binding storage."]
    pub type check_password_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__const__char__0p__0c__const__char__0p__0c__enum__PasswordType__0c__unsigned__long__0c__0x5f426f6f6c__0r>;
    #[doc = "Explicit C callback typedef `copy_data_source_cb` with its exact native binding storage."]
    pub type copy_data_source_cb = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_int__0l__void__0p__0c__int__0c__int__0r,
    >;
    #[doc = "Explicit C callback typedef `create_upper_paths_hook_type` with its exact native binding storage."]
    pub type create_upper_paths_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__PlannerInfo__0p__0c__enum__UpperRelationKind__0c__struct__RelOptInfo__0p__0c__struct__RelOptInfo__0p__0c__void__0p__0r>;
    #[doc = "Explicit C callback typedef `dshash_compare_function` with its exact native binding storage."]
    pub type dshash_compare_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_int__0l__const__void__0p__0c__const__void__0p__0c__unsigned__long__0c__void__0p__0r>;
    #[doc = "Explicit C callback typedef `dshash_hash_function` with its exact native binding storage."]
    pub type dshash_hash_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_unsigned__int__0l__const__void__0p__0c__unsigned__long__0c__void__0p__0r>;
    #[doc = "Explicit C callback typedef `ec_matches_callback_type` with its exact native binding storage."]
    pub type ec_matches_callback_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__EquivalenceClass__0p__0c__struct__EquivalenceMember__0p__0c__void__0p__0r>;
    #[doc = "Explicit C callback typedef `emit_log_hook_type` with its exact native binding storage."]
    pub type emit_log_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__ErrorData__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `explain_get_index_name_hook_type` with its exact native binding storage."]
    pub type explain_get_index_name_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_const__char__0p__0l__unsigned__int__0r,
    >;
    #[doc = "Explicit C callback typedef `fill_string_relopt` with its exact native binding storage."]
    pub type fill_string_relopt = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_unsigned__long__0l__const__char__0p__0c__void__0p__0r,
    >;
    #[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
    #[doc = "Explicit C callback typedef `fmExprContextCallbackFunction` with its exact native binding storage."]
    pub type fmExprContextCallbackFunction = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__unsigned__long__0r,
    >;
    #[doc = "Explicit C callback typedef `fmgr_hook_type` with its exact native binding storage."]
    pub type fmgr_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__enum__FmgrHookEventType__0c__struct__FmgrInfo__0p__0c__unsigned__long__0p__0r>;
    #[doc = "Explicit C callback typedef `get_attavgwidth_hook_type` with its exact native binding storage."]
    pub type get_attavgwidth_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_int__0l__unsigned__int__0c__short__0r,
    >;
    #[doc = "Explicit C callback typedef `get_index_stats_hook_type` with its exact native binding storage."]
    pub type get_index_stats_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__PlannerInfo__0p__0c__unsigned__int__0c__short__0c__struct__VariableStatData__0p__0r>;
    #[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
    #[doc = "Explicit C callback typedef `get_relation_info_hook_type` with its exact native binding storage."]
    pub type get_relation_info_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__PlannerInfo__0p__0c__unsigned__int__0c__0x5f426f6f6c__0c__struct__RelOptInfo__0p__0r>;
    #[doc = "Explicit C callback typedef `get_relation_stats_hook_type` with its exact native binding storage."]
    pub type get_relation_stats_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__PlannerInfo__0p__0c__struct__RangeTblEntry__0p__0c__short__0c__struct__VariableStatData__0p__0r>;
    #[doc = "Explicit C callback typedef `join_search_hook_type` with its exact native binding storage."]
    pub type join_search_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__RelOptInfo__0p__0l__struct__PlannerInfo__0p__0c__int__0c__struct__List__0p__0r>;
    #[doc = "Explicit C callback typedef `list_sort_comparator` with its exact native binding storage."]
    pub type list_sort_comparator = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_int__0l__const__union__ListCell__0p__0c__const__union__ListCell__0p__0r>;
    #[doc = "Explicit C callback typedef `mb2wchar_with_len_converter` with its exact native binding storage."]
    pub type mb2wchar_with_len_converter = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_int__0l__const__unsigned__char__0p__0c__unsigned__int__0p__0c__int__0r>;
    #[doc = "Explicit C callback typedef `mbchar_verifier` with its exact native binding storage."]
    pub type mbchar_verifier = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_int__0l__const__unsigned__char__0p__0c__int__0r,
    >;
    #[doc = "Explicit C callback typedef `mbcharacter_incrementer` with its exact native binding storage."]
    pub type mbcharacter_incrementer = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__unsigned__char__0p__0c__int__0r,
    >;
    #[doc = "Explicit C callback typedef `mbdisplaylen_converter` with its exact native binding storage."]
    pub type mbdisplaylen_converter = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_int__0l__const__unsigned__char__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `mblen_converter` with its exact native binding storage."]
    pub type mblen_converter = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_int__0l__const__unsigned__char__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `mbstr_verifier` with its exact native binding storage."]
    pub type mbstr_verifier = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_int__0l__const__unsigned__char__0p__0c__int__0r,
    >;
    #[doc = "Explicit C callback typedef `needs_fmgr_hook_type` with its exact native binding storage."]
    pub type needs_fmgr_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__unsigned__int__0r,
    >;
    #[doc = "Explicit C callback typedef `object_access_hook_type` with its exact native binding storage."]
    pub type object_access_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__enum__ObjectAccessType__0c__unsigned__int__0c__unsigned__int__0c__int__0c__void__0p__0r>;
    #[doc = "Explicit C callback typedef `object_access_hook_type_str` with its exact native binding storage."]
    pub type object_access_hook_type_str = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__enum__ObjectAccessType__0c__unsigned__int__0c__const__char__0p__0c__int__0c__void__0p__0r>;
    #[doc = "Explicit C callback typedef `on_dsm_detach_callback` with its exact native binding storage."]
    pub type on_dsm_detach_callback = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__dsm_segment__0p__0c__unsigned__long__0r>;
    #[doc = "Explicit C callback typedef `pairingheap_comparator` with its exact native binding storage."]
    pub type pairingheap_comparator = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_int__0l__const__struct__pairingheap_node__0p__0c__const__struct__pairingheap_node__0p__0c__void__0p__0r>;
    #[doc = "Explicit C callback typedef `parallel_worker_main_type` with its exact native binding storage."]
    pub type parallel_worker_main_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__dsm_segment__0p__0c__struct__shm_toc__0p__0r>;
    #[doc = "Explicit C callback typedef `pg_funcptr_t` with its exact native binding storage."]
    pub type pg_funcptr_t = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__void__0r,
    >;
    #[doc = "Explicit C callback typedef `pg_on_exit_callback` with its exact native binding storage."]
    pub type pg_on_exit_callback = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__int__0c__unsigned__long__0r,
    >;
    #[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
    #[doc = "Explicit C callback typedef `planner_hook_type` with its exact native binding storage."]
    pub type planner_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__PlannedStmt__0p__0l__struct__Query__0p__0c__const__char__0p__0c__int__0c__struct__ParamListInfoData__0p__0r>;
    #[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
    #[doc = "Explicit C callback typedef `post_parse_analyze_hook_type` with its exact native binding storage."]
    pub type post_parse_analyze_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ParseState__0p__0c__struct__Query__0p__0c__struct__JumbleState__0p__0r>;
    #[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
    #[doc = "Explicit C callback typedef `pqsigfunc` with its exact native binding storage."]
    pub type pqsigfunc = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__int__0r,
    >;
    #[doc = "Explicit C callback typedef `qsort_arg_comparator` with its exact native binding storage."]
    pub type qsort_arg_comparator = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_int__0l__const__void__0p__0c__const__void__0p__0c__void__0p__0r>;
    #[doc = "Explicit C callback typedef `query_pathkeys_callback` with its exact native binding storage."]
    pub type query_pathkeys_callback = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__PlannerInfo__0p__0c__void__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `rbt_allocfunc` with its exact native binding storage."]
    pub type rbt_allocfunc = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_struct__RBTNode__0p__0l__void__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `rbt_combiner` with its exact native binding storage."]
    pub type rbt_combiner = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__RBTNode__0p__0c__const__struct__RBTNode__0p__0c__void__0p__0r>;
    #[doc = "Explicit C callback typedef `rbt_comparator` with its exact native binding storage."]
    pub type rbt_comparator = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_int__0l__const__struct__RBTNode__0p__0c__const__struct__RBTNode__0p__0c__void__0p__0r>;
    #[doc = "Explicit C callback typedef `rbt_freefunc` with its exact native binding storage."]
    pub type rbt_freefunc = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__RBTNode__0p__0c__void__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `relopts_validator` with its exact native binding storage."]
    pub type relopts_validator = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__void__0p__0c__struct__relopt_value__0p__0c__int__0r>;
    #[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
    #[doc = "Explicit C callback typedef `replace_rte_variables_callback` with its exact native binding storage."]
    pub type replace_rte_variables_callback = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__Node__0p__0l__struct__Var__0p__0c__struct__replace_rte_variables_context__0p__0r>;
    #[doc = "Explicit C callback typedef `row_security_policy_hook_type` with its exact native binding storage."]
    pub type row_security_policy_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__List__0p__0l__enum__CmdType__0c__struct__RelationData__0p__0r>;
    #[doc = "Explicit C callback typedef `set_join_pathlist_hook_type` with its exact native binding storage."]
    pub type set_join_pathlist_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__RelOptInfo__0p__0c__enum__JoinType__0c__struct__JoinPathExtraData__0p__0r>;
    #[doc = "Explicit C callback typedef `set_rel_pathlist_hook_type` with its exact native binding storage."]
    pub type set_rel_pathlist_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__unsigned__int__0c__struct__RangeTblEntry__0p__0r>;
    #[doc = "Explicit C callback typedef `shmem_request_hook_type` with its exact native binding storage."]
    pub type shmem_request_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__void__0r,
    >;
    #[doc = "Explicit C callback typedef `shmem_startup_hook_type` with its exact native binding storage."]
    pub type shmem_startup_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__void__0r,
    >;
    #[doc = "Explicit C callback typedef `utf_local_conversion_func` with its exact native binding storage."]
    pub type utf_local_conversion_func = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_unsigned__int__0l__unsigned__int__0r,
    >;
    #[doc = "Explicit C callback typedef `validate_string_relopt` with its exact native binding storage."]
    pub type validate_string_relopt = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__const__char__0p__0r,
    >;
    #[cfg(feature = "pg15")]
    #[doc = "Explicit C callback typedef `walrcv_check_conninfo_fn` with its exact native binding storage."]
    pub type walrcv_check_conninfo_fn = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__const__char__0p__0r,
    >;
    #[cfg(feature = "pg15")]
    #[doc = "Explicit C callback typedef `walrcv_connect_fn` with its exact native binding storage."]
    pub type walrcv_connect_fn = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__WalReceiverConn__0p__0l__const__char__0p__0c__0x5f426f6f6c__0c__const__char__0p__0c__char__0p__0p__0r>;
    #[cfg(any(feature = "pg15", feature = "pg16"))]
    #[doc = "Explicit C callback typedef `walrcv_create_slot_fn` with its exact native binding storage."]
    pub type walrcv_create_slot_fn = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_char__0p__0l__struct__WalReceiverConn__0p__0c__const__char__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__CRSSnapshotAction__0c__unsigned__long__0p__0r>;
    #[doc = "Explicit C callback typedef `walrcv_disconnect_fn` with its exact native binding storage."]
    pub type walrcv_disconnect_fn = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__WalReceiverConn__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `walrcv_endstreaming_fn` with its exact native binding storage."]
    pub type walrcv_endstreaming_fn = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__WalReceiverConn__0p__0c__unsigned__int__0p__0r>;
    #[doc = "Explicit C callback typedef `walrcv_exec_fn` with its exact native binding storage."]
    pub type walrcv_exec_fn = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__WalRcvExecResult__0p__0l__struct__WalReceiverConn__0p__0c__const__char__0p__0c__int__0c__const__unsigned__int__0p__0r>;
    #[doc = "Explicit C callback typedef `walrcv_get_backend_pid_fn` with its exact native binding storage."]
    pub type walrcv_get_backend_pid_fn = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_int__0l__struct__WalReceiverConn__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `walrcv_get_conninfo_fn` with its exact native binding storage."]
    pub type walrcv_get_conninfo_fn = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_char__0p__0l__struct__WalReceiverConn__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `walrcv_get_senderinfo_fn` with its exact native binding storage."]
    pub type walrcv_get_senderinfo_fn = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__WalReceiverConn__0p__0c__char__0p__0p__0c__int__0p__0r>;
    #[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
    #[doc = "Explicit C callback typedef `walrcv_identify_system_fn` with its exact native binding storage."]
    pub type walrcv_identify_system_fn = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_char__0p__0l__struct__WalReceiverConn__0p__0c__unsigned__int__0p__0r>;
    #[doc = "Explicit C callback typedef `walrcv_readtimelinehistoryfile_fn` with its exact native binding storage."]
    pub type walrcv_readtimelinehistoryfile_fn = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__WalReceiverConn__0p__0c__unsigned__int__0c__char__0p__0p__0c__char__0p__0p__0c__int__0p__0r>;
    #[doc = "Explicit C callback typedef `walrcv_receive_fn` with its exact native binding storage."]
    pub type walrcv_receive_fn = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_int__0l__struct__WalReceiverConn__0p__0c__char__0p__0p__0c__int__0p__0r>;
    #[doc = "Explicit C callback typedef `walrcv_send_fn` with its exact native binding storage."]
    pub type walrcv_send_fn = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__WalReceiverConn__0p__0c__const__char__0p__0c__int__0r>;
    #[doc = "Explicit C callback typedef `walrcv_server_version_fn` with its exact native binding storage."]
    pub type walrcv_server_version_fn = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_int__0l__struct__WalReceiverConn__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `walrcv_startstreaming_fn` with its exact native binding storage."]
    pub type walrcv_startstreaming_fn = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__WalReceiverConn__0p__0c__const__WalRcvStreamOptions__0p__0r>;
    #[doc = "Explicit C callback typedef `wchar2mb_with_len_converter` with its exact native binding storage."]
    pub type wchar2mb_with_len_converter = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_int__0l__const__unsigned__int__0p__0c__unsigned__char__0p__0c__int__0r>;
    #[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `ExecutorCheckPerms_hook_type` with its exact native binding storage."]
    pub type ExecutorCheckPerms_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__List__0p__0c__struct__List__0p__0c__0x5f426f6f6c__0r>;
    #[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `ReorderBufferUpdateProgressTxnCB` with its exact native binding storage."]
    pub type ReorderBufferUpdateProgressTxnCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0r>;
    #[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `SortTupleComparator` with its exact native binding storage."]
    pub type SortTupleComparator = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_int__0l__const__SortTuple__0p__0c__const__SortTuple__0p__0c__struct__Tuplesortstate__0p__0r>;
    #[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `copy_data_dest_cb` with its exact native binding storage."]
    pub type copy_data_dest_cb = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__void__0p__0c__int__0r,
    >;
    #[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `planstate_tree_walker_callback` with its exact native binding storage."]
    pub type planstate_tree_walker_callback = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__PlanState__0p__0c__void__0p__0r>;
    #[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `tree_mutator_callback` with its exact native binding storage."]
    pub type tree_mutator_callback = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__Node__0p__0l__struct__Node__0p__0c__void__0p__0r>;
    #[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `tree_walker_callback` with its exact native binding storage."]
    pub type tree_walker_callback = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__Node__0p__0c__void__0p__0r,
    >;
    #[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `walrcv_check_conninfo_fn` with its exact native binding storage."]
    pub type walrcv_check_conninfo_fn = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__const__char__0p__0c__0x5f426f6f6c__0r,
    >;
    #[cfg(feature = "pg16")]
    #[doc = "Explicit C callback typedef `walrcv_connect_fn` with its exact native binding storage."]
    pub type walrcv_connect_fn = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__WalReceiverConn__0p__0l__const__char__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__const__char__0p__0c__char__0p__0p__0r>;
    #[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `ReadStreamBlockNumberCB` with its exact native binding storage."]
    pub type ReadStreamBlockNumberCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_unsigned__int__0l__struct__ReadStream__0p__0c__void__0p__0c__void__0p__0r>;
    #[cfg(any(feature = "pg17", feature = "pg18"))]
    #[doc = "Explicit C callback typedef `SlruScanCallback` with its exact native binding storage."]
    pub type SlruScanCallback = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__SlruCtlData__0p__0c__char__0p__0c__long__0c__void__0p__0r>;
    #[cfg(feature = "pg17")]
    #[doc = "Explicit C callback typedef `amestimateparallelscan_function` with its exact native binding storage."]
    pub type amestimateparallelscan_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_unsigned__long__0l__int__0c__int__0r,
    >;
    #[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `aminsertcleanup_function` with its exact native binding storage."]
    pub type aminsertcleanup_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__RelationData__0p__0c__struct__IndexInfo__0p__0r>;
    #[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `dshash_copy_function` with its exact native binding storage."]
    pub type dshash_copy_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__void__0p__0c__const__void__0p__0c__unsigned__long__0c__void__0p__0r>;
    #[cfg(feature = "pg17")]
    #[doc = "Explicit C callback typedef `walrcv_alter_slot_fn` with its exact native binding storage."]
    pub type walrcv_alter_slot_fn = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__WalReceiverConn__0p__0c__const__char__0p__0c__0x5f426f6f6c__0r>;
    #[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `walrcv_connect_fn` with its exact native binding storage."]
    pub type walrcv_connect_fn = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__WalReceiverConn__0p__0l__const__char__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__const__char__0p__0c__char__0p__0p__0r>;
    #[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `walrcv_create_slot_fn` with its exact native binding storage."]
    pub type walrcv_create_slot_fn = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_char__0p__0l__struct__WalReceiverConn__0p__0c__const__char__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__CRSSnapshotAction__0c__unsigned__long__0p__0r>;
    #[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `walrcv_get_dbname_from_conninfo_fn` with its exact native binding storage."]
    pub type walrcv_get_dbname_from_conninfo_fn = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_char__0p__0l__const__char__0p__0r,
    >;
    #[cfg(any(feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `CachedFunctionCompileCallback` with its exact native binding storage."]
    pub type CachedFunctionCompileCallback = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__FunctionCallInfoBaseData__0p__0c__struct__HeapTupleData__0p__0c__const__struct__CachedFunctionHashKey__0p__0c__struct__CachedFunction__0p__0c__0x5f426f6f6c__0r>;
    #[cfg(any(feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `CachedFunctionDeleteCallback` with its exact native binding storage."]
    pub type CachedFunctionDeleteCallback = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__CachedFunction__0p__0r,
    >;
    #[cfg(feature = "pg18")]
    #[doc = "Explicit C callback typedef `ChangeVarNodes_callback` with its exact native binding storage."]
    pub type ChangeVarNodes_callback = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__Node__0p__0c__struct__ChangeVarNodes_context__0p__0r>;
    #[cfg(any(feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `ExecutorRun_hook_type` with its exact native binding storage."]
    pub type ExecutorRun_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__QueryDesc__0p__0c__enum__ScanDirection__0c__unsigned__long__0r>;
    #[cfg(any(feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `ExplainOptionHandler` with its exact native binding storage."]
    pub type ExplainOptionHandler = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ExplainState__0p__0c__struct__DefElem__0p__0c__struct__ParseState__0p__0r>;
    #[cfg(any(feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `PgAioHandleCallbackComplete` with its exact native binding storage."]
    pub type PgAioHandleCallbackComplete = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__PgAioResult__0l__struct__PgAioHandle__0p__0c__struct__PgAioResult__0c__unsigned__char__0r>;
    #[cfg(any(feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `PgAioHandleCallbackReport` with its exact native binding storage."]
    pub type PgAioHandleCallbackReport = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__PgAioResult__0c__const__union__PgAioTargetData__0p__0c__int__0r>;
    #[cfg(any(feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `PgAioHandleCallbackStage` with its exact native binding storage."]
    pub type PgAioHandleCallbackStage = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__PgAioHandle__0p__0c__unsigned__char__0r>;
    #[cfg(any(feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `PostRewriteHook` with its exact native binding storage."]
    pub type PostRewriteHook = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__List__0p__0c__void__0p__0r,
    >;
    #[cfg(any(feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `RelSyncCallbackFunction` with its exact native binding storage."]
    pub type RelSyncCallbackFunction = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__unsigned__long__0c__unsigned__int__0r,
    >;
    #[cfg(any(feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `SkipSupportIncDec` with its exact native binding storage."]
    pub type SkipSupportIncDec = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_unsigned__long__0l__struct__RelationData__0p__0c__unsigned__long__0c__0x5f426f6f6c__0p__0r>;
    #[cfg(any(feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `amestimateparallelscan_function` with its exact native binding storage."]
    pub type amestimateparallelscan_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_unsigned__long__0l__struct__RelationData__0p__0c__int__0c__int__0r>;
    #[cfg(any(feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `amgettreeheight_function` with its exact native binding storage."]
    pub type amgettreeheight_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_int__0l__struct__RelationData__0p__0r,
    >;
    #[cfg(any(feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `amtranslate_cmptype_function` with its exact native binding storage."]
    pub type amtranslate_cmptype_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_unsigned__short__0l__enum__CompareType__0c__unsigned__int__0r>;
    #[cfg(any(feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `amtranslate_strategy_function` with its exact native binding storage."]
    pub type amtranslate_strategy_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_enum__CompareType__0l__unsigned__short__0c__unsigned__int__0r>;
    #[cfg(any(feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `explain_per_node_hook_type` with its exact native binding storage."]
    pub type explain_per_node_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__PlanState__0p__0c__struct__List__0p__0c__const__char__0p__0c__const__char__0p__0c__struct__ExplainState__0p__0r>;
    #[cfg(any(feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `explain_per_plan_hook_type` with its exact native binding storage."]
    pub type explain_per_plan_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__PlannedStmt__0p__0c__struct__IntoClause__0p__0c__struct__ExplainState__0p__0c__const__char__0p__0c__struct__ParamListInfoData__0p__0c__struct__QueryEnvironment__0p__0r>;
    #[cfg(any(feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `explain_validate_options_hook_type` with its exact native binding storage."]
    pub type explain_validate_options_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ExplainState__0p__0c__struct__List__0p__0c__struct__ParseState__0p__0r>;
    #[cfg(any(feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `grouping_eqop_callback` with its exact native binding storage."]
    pub type grouping_eqop_callback = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_unsigned__int__0l__struct__Var__0p__0c__void__0p__0r,
    >;
    #[cfg(any(feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `walrcv_alter_slot_fn` with its exact native binding storage."]
    pub type walrcv_alter_slot_fn = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__WalReceiverConn__0p__0c__const__char__0p__0c__const__0x5f426f6f6c__0p__0c__const__0x5f426f6f6c__0p__0r>;
    #[cfg(feature = "pg19")]
    #[doc = "Explicit C callback typedef `ConnectionWarningFilter` with its exact native binding storage."]
    pub type ConnectionWarningFilter = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__void__0r,
    >;
    #[cfg(feature = "pg19")]
    #[doc = "Explicit C callback typedef `ExplainOptionGUCCheckHandler` with its exact native binding storage."]
    pub type ExplainOptionGUCCheckHandler = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__const__char__0p__0c__const__char__0p__0c__enum__NodeTag__0r>;
    #[cfg(feature = "pg19")]
    #[doc = "Explicit C callback typedef `HashAllocFunc` with its exact native binding storage."]
    pub type HashAllocFunc = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0p__0l__unsigned__long__0c__void__0p__0r,
    >;
    #[cfg(feature = "pg19")]
    #[doc = "Explicit C callback typedef `ImportForeignStatistics_function` with its exact native binding storage."]
    pub type ImportForeignStatistics_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__RelationData__0p__0c__struct__List__0p__0c__int__0r>;
    #[cfg(feature = "pg19")]
    #[doc = "Explicit C callback typedef `PushFunction` with its exact native binding storage."]
    pub type PushFunction = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__void__0p__0c__struct__TSQueryParserStateData__0p__0c__char__0p__0c__int__0c__short__0c__0x5f426f6f6c__0r>;
    #[cfg(feature = "pg19")]
    #[doc = "Explicit C callback typedef `ShmemAttachCallback` with its exact native binding storage."]
    pub type ShmemAttachCallback = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__void__0p__0r,
    >;
    #[cfg(feature = "pg19")]
    #[doc = "Explicit C callback typedef `ShmemInitCallback` with its exact native binding storage."]
    pub type ShmemInitCallback = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__void__0p__0r,
    >;
    #[cfg(feature = "pg19")]
    #[doc = "Explicit C callback typedef `ShmemRequestCallback` with its exact native binding storage."]
    pub type ShmemRequestCallback = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__void__0p__0r,
    >;
    #[cfg(feature = "pg19")]
    #[doc = "Explicit C callback typedef `SlruScanCallback` with its exact native binding storage."]
    pub type SlruScanCallback = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__SlruDesc__0p__0c__char__0p__0c__long__0c__void__0p__0r>;
    #[cfg(feature = "pg19")]
    #[doc = "Explicit C callback typedef `SyscacheCallbackFunction` with its exact native binding storage."]
    pub type SyscacheCallbackFunction = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__unsigned__long__0c__enum__SysCacheIdentifier__0c__unsigned__int__0r>;
    #[cfg(feature = "pg19")]
    #[doc = "Explicit C callback typedef `TwoPhaseCallback` with its exact native binding storage."]
    pub type TwoPhaseCallback = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__FullTransactionId__0c__unsigned__short__0c__void__0p__0c__unsigned__int__0r>;
    #[cfg(feature = "pg19")]
    #[doc = "Explicit C callback typedef `build_simple_rel_hook_type` with its exact native binding storage."]
    pub type build_simple_rel_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__RangeTblEntry__0p__0r>;
    #[cfg(feature = "pg19")]
    #[doc = "Explicit C callback typedef `join_path_setup_hook_type` with its exact native binding storage."]
    pub type join_path_setup_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__RelOptInfo__0p__0c__enum__JoinType__0c__struct__JoinPathExtraData__0p__0r>;
    #[cfg(feature = "pg19")]
    #[doc = "Explicit C callback typedef `joinrel_setup_hook_type` with its exact native binding storage."]
    pub type joinrel_setup_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__SpecialJoinInfo__0p__0c__struct__List__0p__0r>;
    #[cfg(feature = "pg19")]
    #[doc = "Explicit C callback typedef `planner_hook_type` with its exact native binding storage."]
    pub type planner_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__PlannedStmt__0p__0l__struct__Query__0p__0c__const__char__0p__0c__int__0c__struct__ParamListInfoData__0p__0c__struct__ExplainState__0p__0r>;
    #[cfg(feature = "pg19")]
    #[doc = "Explicit C callback typedef `planner_setup_hook_type` with its exact native binding storage."]
    pub type planner_setup_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__PlannerGlobal__0p__0c__struct__Query__0p__0c__const__char__0p__0c__int__0c__double__0p__0c__struct__ExplainState__0p__0r>;
    #[cfg(feature = "pg19")]
    #[doc = "Explicit C callback typedef `planner_shutdown_hook_type` with its exact native binding storage."]
    pub type planner_shutdown_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__PlannerGlobal__0p__0c__struct__Query__0p__0c__const__char__0p__0c__struct__PlannedStmt__0p__0r>;
    #[cfg(feature = "pg19")]
    #[doc = "Explicit C callback typedef `post_parse_analyze_hook_type` with its exact native binding storage."]
    pub type post_parse_analyze_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ParseState__0p__0c__struct__Query__0p__0c__const__struct__JumbleState__0p__0r>;
    #[cfg(feature = "pg19")]
    #[doc = "Explicit C callback typedef `pqsigfunc` with its exact native binding storage."]
    pub type pqsigfunc = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__int__0c__const__struct__pg_signal_info__0p__0r>;
    #[cfg(feature = "pg19")]
    #[doc = "Explicit C callback typedef `replace_rte_variables_callback` with its exact native binding storage."]
    pub type replace_rte_variables_callback = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__Node__0p__0l__const__struct__Var__0p__0c__struct__replace_rte_variables_context__0p__0r>;
    #[cfg(feature = "pg19")]
    #[doc = "Explicit C callback typedef `walrcv_identify_system_fn` with its exact native binding storage."]
    pub type walrcv_identify_system_fn = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_char__0p__0l__struct__WalReceiverConn__0p__0c__unsigned__int__0p__0c__unsigned__long__0p__0r>;
}
#[doc(hidden)]
pub struct Field_N;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BlockSamplerData>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BlockSamplerData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BlockSamplerData {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BlockSamplerData, N) == 0);
const _: () = {
    assert!(::core::mem::size_of::<u32>() == 4);
    assert!(::core::mem::align_of::<u32>() == 4);
};
use c::expression::{ReadOnly as FieldReadOnly, ReadWrite as FieldReadWrite};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_N>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BlockSamplerData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
use c::expression::FieldProjection as Projection;
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BlockSamplerData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BlockSamplerData> as c::expression::OrdinaryField<Field_N>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).N) };
#[doc(hidden)]
pub struct Field_n;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BlockSamplerData, n) == 4);
const _: () = {
    assert!(::core::mem::size_of::<i32>() == 4);
    assert!(::core::mem::align_of::<i32>() == 4);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_n>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BlockSamplerData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BlockSamplerData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BlockSamplerData> as c::expression::OrdinaryField<Field_n>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).n) };
#[doc(hidden)]
pub struct Field_t;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BlockSamplerData, t) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BlockSamplerData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BlockSamplerData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BlockSamplerData> as c::expression::OrdinaryField<Field_t>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t) };
#[doc(hidden)]
pub struct Field_m;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BlockSamplerData, m) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_m>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BlockSamplerData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BlockSamplerData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BlockSamplerData> as c::expression::OrdinaryField<Field_m>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).m) };
#[doc(hidden)]
pub struct Field_allocatedMemory;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BuildAccumulator>() == 72);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BuildAccumulator>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BuildAccumulator {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BuildAccumulator, allocatedMemory) == 8
);
const _: () = {
    assert!(::core::mem::size_of::<usize>() == 8);
    assert!(::core::mem::align_of::<usize>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_allocatedMemory>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BuildAccumulator>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BuildAccumulator, <<c::expression::CRecord<crate::__pgrx_c_bindings::BuildAccumulator> as c::expression::OrdinaryField<Field_allocatedMemory>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).allocatedMemory) };
#[doc(hidden)]
pub struct Field_eas_used;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BuildAccumulator, eas_used) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_eas_used>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BuildAccumulator>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BuildAccumulator, <<c::expression::CRecord<crate::__pgrx_c_bindings::BuildAccumulator> as c::expression::OrdinaryField<Field_eas_used>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).eas_used) };
#[doc(hidden)]
pub struct Field_mutex;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ConditionVariable>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ConditionVariable>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ConditionVariable {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ConditionVariable, mutex) == 0);
const _: () = {
    assert!(::core::mem::size_of::<u8>() == 1);
    assert!(::core::mem::align_of::<u8>() == 1);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mutex>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ConditionVariable>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ConditionVariable, <<c::expression::CRecord<crate::__pgrx_c_bindings::ConditionVariable> as c::expression::OrdinaryField<Field_mutex>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mutex) };
#[doc(hidden)]
pub struct Field_in_use;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::DecodedBkpBlock>() == 64);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::DecodedBkpBlock>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::DecodedBkpBlock {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodedBkpBlock, in_use) == 0);
const _: () = {
    assert!(::core::mem::size_of::<bool>() == 1);
    assert!(::core::mem::align_of::<bool>() == 1);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_in_use>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodedBkpBlock, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock> as c::expression::OrdinaryField<Field_in_use>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).in_use) };
#[doc(hidden)]
pub struct Field_forknum;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodedBkpBlock, forknum) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_forknum>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ForkNumber,
        c::CInt,
        i32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodedBkpBlock, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock> as c::expression::OrdinaryField<Field_forknum>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).forknum) };
#[doc(hidden)]
pub struct Field_blkno;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodedBkpBlock, blkno) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_blkno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodedBkpBlock, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock> as c::expression::OrdinaryField<Field_blkno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).blkno) };
#[doc(hidden)]
pub struct Field_prefetch_buffer;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodedBkpBlock, prefetch_buffer) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_prefetch_buffer>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodedBkpBlock, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock> as c::expression::OrdinaryField<Field_prefetch_buffer>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).prefetch_buffer) };
#[doc(hidden)]
pub struct Field_flags;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodedBkpBlock, flags) == 28);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodedBkpBlock, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock> as c::expression::OrdinaryField<Field_flags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).flags) };
#[doc(hidden)]
pub struct Field_has_image;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodedBkpBlock, has_image) == 29);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_has_image>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 29;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodedBkpBlock, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock> as c::expression::OrdinaryField<Field_has_image>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).has_image) };
#[doc(hidden)]
pub struct Field_apply_image;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodedBkpBlock, apply_image) == 30);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_apply_image>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 30;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodedBkpBlock, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock> as c::expression::OrdinaryField<Field_apply_image>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).apply_image) };
#[doc(hidden)]
pub struct Field_hole_offset;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodedBkpBlock, hole_offset) == 40);
const _: () = {
    assert!(::core::mem::size_of::<u16>() == 2);
    assert!(::core::mem::align_of::<u16>() == 2);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hole_offset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodedBkpBlock, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock> as c::expression::OrdinaryField<Field_hole_offset>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hole_offset) };
#[doc(hidden)]
pub struct Field_hole_length;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodedBkpBlock, hole_length) == 42);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hole_length>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 42;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodedBkpBlock, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock> as c::expression::OrdinaryField<Field_hole_length>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hole_length) };
#[doc(hidden)]
pub struct Field_bimg_len;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodedBkpBlock, bimg_len) == 44);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bimg_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodedBkpBlock, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock> as c::expression::OrdinaryField<Field_bimg_len>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).bimg_len) };
#[doc(hidden)]
pub struct Field_bimg_info;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodedBkpBlock, bimg_info) == 46);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bimg_info>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 46;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodedBkpBlock, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock> as c::expression::OrdinaryField<Field_bimg_info>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).bimg_info) };
#[doc(hidden)]
pub struct Field_has_data;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodedBkpBlock, has_data) == 47);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_has_data>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 47;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodedBkpBlock, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock> as c::expression::OrdinaryField<Field_has_data>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).has_data) };
#[doc(hidden)]
pub struct Field_data;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodedBkpBlock, data) == 48);
const _: () = {
    assert!(::core::mem::size_of::<*mut ::core::ffi::c_char>() == 8);
    assert!(::core::mem::align_of::<*mut ::core::ffi::c_char>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_data>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock>
{
    type Member = c::expression::CPointer<
        c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodedBkpBlock, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock> as c::expression::OrdinaryField<Field_data>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).data) };
#[doc(hidden)]
pub struct Field_data_len;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodedBkpBlock, data_len) == 56);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_data_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodedBkpBlock, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock> as c::expression::OrdinaryField<Field_data_len>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).data_len) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
pub struct Field_data_bufsz;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodedBkpBlock, data_bufsz) == 58);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_data_bufsz>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 58;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodedBkpBlock, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock> as c::expression::OrdinaryField<Field_data_bufsz>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).data_bufsz) };
#[doc(hidden)]
pub struct Field_isend;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::DictSubState>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::DictSubState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::DictSubState {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DictSubState, isend) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isend>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DictSubState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DictSubState, <<c::expression::CRecord<crate::__pgrx_c_bindings::DictSubState> as c::expression::OrdinaryField<Field_isend>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isend) };
#[doc(hidden)]
pub struct Field_getnext;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DictSubState, getnext) == 1);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_getnext>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DictSubState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DictSubState, <<c::expression::CRecord<crate::__pgrx_c_bindings::DictSubState> as c::expression::OrdinaryField<Field_getnext>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).getnext) };
#[doc(hidden)]
pub struct Field_lastRec;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo>() == 80);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::EndOfWalRecoveryInfo {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EndOfWalRecoveryInfo, lastRec) == 0);
const _: () = {
    assert!(::core::mem::size_of::<u64>() == 8);
    assert!(::core::mem::align_of::<u64>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lastRec>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo> as c::expression::OrdinaryField<Field_lastRec>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lastRec) };
#[doc(hidden)]
pub struct Field_lastRecTLI;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EndOfWalRecoveryInfo, lastRecTLI) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lastRecTLI>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo> as c::expression::OrdinaryField<Field_lastRecTLI>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lastRecTLI) };
#[doc(hidden)]
pub struct Field_endOfLog;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EndOfWalRecoveryInfo, endOfLog) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_endOfLog>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo> as c::expression::OrdinaryField<Field_endOfLog>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).endOfLog) };
#[doc(hidden)]
pub struct Field_endOfLogTLI;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EndOfWalRecoveryInfo, endOfLogTLI) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_endOfLogTLI>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo> as c::expression::OrdinaryField<Field_endOfLogTLI>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).endOfLogTLI) };
#[doc(hidden)]
pub struct Field_lastPageBeginPtr;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EndOfWalRecoveryInfo, lastPageBeginPtr) == 32
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lastPageBeginPtr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo> as c::expression::OrdinaryField<Field_lastPageBeginPtr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lastPageBeginPtr) };
#[doc(hidden)]
pub struct Field_abortedRecPtr;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EndOfWalRecoveryInfo, abortedRecPtr) == 48
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_abortedRecPtr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo> as c::expression::OrdinaryField<Field_abortedRecPtr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).abortedRecPtr) };
#[doc(hidden)]
pub struct Field_missingContrecPtr;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EndOfWalRecoveryInfo, missingContrecPtr)
        == 56
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_missingContrecPtr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo> as c::expression::OrdinaryField<Field_missingContrecPtr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).missingContrecPtr) };
#[doc(hidden)]
pub struct Field_standby_signal_file_found;
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::EndOfWalRecoveryInfo,
        standby_signal_file_found
    ) == 72
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_standby_signal_file_found>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo> as c::expression::OrdinaryField<Field_standby_signal_file_found>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).standby_signal_file_found) };
#[doc(hidden)]
pub struct Field_recovery_signal_file_found;
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::EndOfWalRecoveryInfo,
        recovery_signal_file_found
    ) == 73
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_recovery_signal_file_found>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 73;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo> as c::expression::OrdinaryField<Field_recovery_signal_file_found>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).recovery_signal_file_found) };
