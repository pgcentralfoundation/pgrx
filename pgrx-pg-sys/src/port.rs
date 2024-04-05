use crate as pg_sys;
use core::mem::offset_of;
use core::str::FromStr;

/// this comes from `postgres_ext.h`
pub const InvalidOid: crate::Oid = crate::Oid::INVALID;
pub const InvalidOffsetNumber: super::OffsetNumber = 0;
pub const FirstOffsetNumber: super::OffsetNumber = 1;
pub const MaxOffsetNumber: super::OffsetNumber =
    (super::BLCKSZ as usize / std::mem::size_of::<super::ItemIdData>()) as super::OffsetNumber;
pub const InvalidBlockNumber: u32 = 0xFFFF_FFFF as crate::BlockNumber;
pub const VARHDRSZ: usize = std::mem::size_of::<super::int32>();
pub const InvalidTransactionId: super::TransactionId = 0 as super::TransactionId;
pub const InvalidCommandId: super::CommandId = (!(0 as super::CommandId)) as super::CommandId;
pub const FirstCommandId: super::CommandId = 0 as super::CommandId;
pub const BootstrapTransactionId: super::TransactionId = 1 as super::TransactionId;
pub const FrozenTransactionId: super::TransactionId = 2 as super::TransactionId;
pub const FirstNormalTransactionId: super::TransactionId = 3 as super::TransactionId;
pub const MaxTransactionId: super::TransactionId = 0xFFFF_FFFF as super::TransactionId;

/// Given a valid HeapTuple pointer, return address of the user data
///
/// # Safety
///
/// This function cannot determine if the `tuple` argument is really a non-null pointer to a [`pg_sys::HeapTuple`].
#[inline(always)]
pub unsafe fn GETSTRUCT(tuple: crate::HeapTuple) -> *mut std::os::raw::c_char {
    // #define GETSTRUCT(TUP) ((char *) ((TUP)->t_data) + (TUP)->t_data->t_hoff)

    // SAFETY:  The caller has asserted `tuple` is a valid HeapTuple and is properly aligned
    // Additionally, t_data.t_hoff is an a u8, so it'll fit inside a usize
    (*tuple).t_data.cast::<std::os::raw::c_char>().add((*(*tuple).t_data).t_hoff as _)
}

//
// TODO: [`TYPEALIGN`] and [`MAXALIGN`] are also part of PR #948 and when that's all merged,
//       their uses should be switched to these
//

#[allow(non_snake_case)]
#[inline(always)]
pub const unsafe fn TYPEALIGN(alignval: usize, len: usize) -> usize {
    // #define TYPEALIGN(ALIGNVAL,LEN)  \
    // (((uintptr_t) (LEN) + ((ALIGNVAL) - 1)) & ~((uintptr_t) ((ALIGNVAL) - 1)))
    ((len) + ((alignval) - 1)) & !((alignval) - 1)
}

#[allow(non_snake_case)]
#[inline(always)]
pub const unsafe fn MAXALIGN(len: usize) -> usize {
    // #define MAXALIGN(LEN) TYPEALIGN(MAXIMUM_ALIGNOF, (LEN))
    TYPEALIGN(pg_sys::MAXIMUM_ALIGNOF as _, len)
}

///  Given a currently-allocated chunk of Postgres allocated memory, determine the context
///  it belongs to.
///
/// All chunks allocated by any memory context manager are required to be
/// preceded by the corresponding MemoryContext stored, without padding, in the
/// preceding sizeof(void*) bytes.  A currently-allocated chunk must contain a
/// backpointer to its owning context.  The backpointer is used by pfree() and
/// repalloc() to find the context to call.
///
/// # Safety
///
/// The specified `pointer` **must** be one allocated by Postgres (via [`palloc`] and friends).
///
///
/// # Panics
///
/// This function will panic if `pointer` is null, if it's not properly aligned, or if the memory
/// it points to doesn't have a prefix that looks like a memory context pointer
///
/// [`palloc`]: crate::palloc
#[allow(non_snake_case)]
#[cfg(any(feature = "pg12", feature = "pg13", feature = "pg14", feature = "pg15"))]
pub unsafe fn GetMemoryChunkContext(pointer: *mut std::os::raw::c_void) -> pg_sys::MemoryContext {
    // Postgres versions <16 don't export the "GetMemoryChunkContext" function.  It's a "static inline"
    // function in `memutils.h`, so we port it to Rust right here
    /*
     * Try to detect bogus pointers handed to us, poorly though we can.
     * Presumably, a pointer that isn't MAXALIGNED isn't pointing at an
     * allocated chunk.
     */
    assert!(!pointer.is_null());
    assert_eq!(pointer, MAXALIGN(pointer as usize) as *mut ::std::os::raw::c_void);

    /*
     * OK, it's probably safe to look at the context.
     */
    // 	context = *(MemoryContext *) (((char *) pointer) - sizeof(void *));
    let context = unsafe {
        // SAFETY: the caller has assured us that `pointer` points to palloc'd memory, which
        // means it'll have this header before it
        *(pointer
            .cast::<::std::os::raw::c_char>()
            .sub(std::mem::size_of::<*mut ::std::os::raw::c_void>())
            .cast())
    };

    assert!(MemoryContextIsValid(context));

    context
}

/// Returns true if memory context is tagged correctly according to Postgres.
///
/// # Safety
///
/// The caller must only attempt this on a pointer to a Node.
/// This may clarify if the pointee is correctly-initialized [`pg_sys::MemoryContextData`].
///
/// # Implementation Note
///
/// If Postgres adds more memory context types in the future, we need to do that here too.
#[allow(non_snake_case)]
#[inline(always)]
pub unsafe fn MemoryContextIsValid(context: crate::MemoryContext) -> bool {
    // #define MemoryContextIsValid(context) \
    // 	((context) != NULL && \
    // 	 (IsA((context), AllocSetContext) || \
    // 	  IsA((context), SlabContext) || \
    // 	  IsA((context), GenerationContext)))

    !context.is_null()
        && unsafe {
            // SAFETY:  we just determined the pointer isn't null, and
            // the caller asserts that it is being used on a Node.
            let tag = (*context.cast::<crate::Node>()).type_;
            use crate::NodeTag::*;
            matches!(tag, T_AllocSetContext | T_SlabContext | T_GenerationContext)
        }
}

pub const VARHDRSZ_EXTERNAL: usize = offset_of!(super::varattrib_1b_e, va_data);
pub const VARHDRSZ_SHORT: usize = offset_of!(super::varattrib_1b, va_data);

#[inline]
pub fn get_pg_major_version_string() -> &'static str {
    let mver = core::ffi::CStr::from_bytes_with_nul(super::PG_MAJORVERSION).unwrap();
    mver.to_str().unwrap()
}

#[inline]
pub fn get_pg_major_version_num() -> u16 {
    u16::from_str(super::get_pg_major_version_string()).unwrap()
}

#[inline]
pub fn get_pg_version_string() -> &'static str {
    let ver = core::ffi::CStr::from_bytes_with_nul(super::PG_VERSION_STR).unwrap();
    ver.to_str().unwrap()
}

#[inline]
pub fn get_pg_major_minor_version_string() -> &'static str {
    let mver = core::ffi::CStr::from_bytes_with_nul(super::PG_VERSION).unwrap();
    mver.to_str().unwrap()
}

#[inline]
pub fn TransactionIdIsNormal(xid: super::TransactionId) -> bool {
    xid >= FirstNormalTransactionId
}

/// ```c
///     #define type_is_array(typid)  (get_element_type(typid) != InvalidOid)
/// ```
#[inline]
pub unsafe fn type_is_array(typoid: super::Oid) -> bool {
    super::get_element_type(typoid) != InvalidOid
}

/// #define BufferGetPage(buffer) ((Page)BufferGetBlock(buffer))
#[inline]
pub unsafe fn BufferGetPage(buffer: crate::Buffer) -> crate::Page {
    BufferGetBlock(buffer) as crate::Page
}

/// #define BufferGetBlock(buffer) \
/// ( \
///      AssertMacro(BufferIsValid(buffer)), \
///      BufferIsLocal(buffer) ? \
///            LocalBufferBlockPointers[-(buffer) - 1] \
///      : \
///            (Block) (BufferBlocks + ((Size) ((buffer) - 1)) * BLCKSZ) \
/// )
#[inline]
pub unsafe fn BufferGetBlock(buffer: crate::Buffer) -> crate::Block {
    if BufferIsLocal(buffer) {
        *crate::LocalBufferBlockPointers.offset(((-buffer) - 1) as isize)
    } else {
        crate::BufferBlocks.add(((buffer as crate::Size) - 1) * crate::BLCKSZ as usize)
            as crate::Block
    }
}

/// #define BufferIsLocal(buffer)      ((buffer) < 0)
#[inline]
pub unsafe fn BufferIsLocal(buffer: crate::Buffer) -> bool {
    buffer < 0
}

/// Retrieve the "user data" of the specified [`pg_sys::HeapTuple`] as a specific type. Typically this
/// will be a struct that represents a Postgres system catalog, such as [`FormData_pg_class`].
///
/// # Returns
///
/// A pointer to the [`pg_sys::HeapTuple`]'s "user data", cast as a mutable pointer to `T`.  If the
/// specified `htup` pointer is null, the null pointer is returned.
///
/// # Safety
///
/// This function cannot verify that the specified `htup` points to a valid [`pg_sys::HeapTuple`] nor
/// that if it does, that its bytes are bitwise compatible with `T`.
///
/// [`FormData_pg_class`]: crate::FormData_pg_class
#[inline]
pub unsafe fn heap_tuple_get_struct<T>(htup: super::HeapTuple) -> *mut T {
    if htup.is_null() {
        std::ptr::null_mut()
    } else {
        unsafe {
            // SAFETY:  The caller has told us `htop` is a valid HeapTuple
            GETSTRUCT(htup).cast()
        }
    }
}

// All of this weird code is in response to Postgres having a relatively cavalier attitude about types:
// - https://github.com/postgres/postgres/commit/1c27d16e6e5c1f463bbe1e9ece88dda811235165
//
// As a result, we redeclare their functions with the arguments they should have on earlier Postgres
// and we route people to the old symbols they were using before on later ones.
#[cfg(any(feature = "pg12", feature = "pg13", feature = "pg14", feature = "pg15"))]
#[::pgrx_macros::pg_guard]
extern "C" {
    pub fn planstate_tree_walker(
        planstate: *mut super::PlanState,
        walker: ::core::option::Option<
            unsafe extern "C" fn(*mut super::PlanState, *mut ::core::ffi::c_void) -> bool,
        >,
        context: *mut ::core::ffi::c_void,
    ) -> bool;

    pub fn query_tree_walker(
        query: *mut super::Query,
        walker: ::core::option::Option<
            unsafe extern "C" fn(*mut super::Node, *mut ::core::ffi::c_void) -> bool,
        >,
        context: *mut ::core::ffi::c_void,
        flags: ::core::ffi::c_int,
    ) -> bool;

    pub fn query_or_expression_tree_walker(
        node: *mut super::Node,
        walker: ::core::option::Option<
            unsafe extern "C" fn(*mut super::Node, *mut ::core::ffi::c_void) -> bool,
        >,
        context: *mut ::core::ffi::c_void,
        flags: ::core::ffi::c_int,
    ) -> bool;

    pub fn range_table_entry_walker(
        rte: *mut super::RangeTblEntry,
        walker: ::core::option::Option<
            unsafe extern "C" fn(*mut super::Node, *mut ::core::ffi::c_void) -> bool,
        >,
        context: *mut ::core::ffi::c_void,
        flags: ::core::ffi::c_int,
    ) -> bool;

    pub fn range_table_walker(
        rtable: *mut super::List,
        walker: ::core::option::Option<
            unsafe extern "C" fn(*mut super::Node, *mut ::core::ffi::c_void) -> bool,
        >,
        context: *mut ::core::ffi::c_void,
        flags: ::core::ffi::c_int,
    ) -> bool;

    pub fn expression_tree_walker(
        node: *mut super::Node,
        walker: ::core::option::Option<
            unsafe extern "C" fn(*mut super::Node, *mut ::core::ffi::c_void) -> bool,
        >,
        context: *mut ::core::ffi::c_void,
    ) -> bool;

    pub fn raw_expression_tree_walker(
        node: *mut super::Node,
        walker: ::core::option::Option<
            unsafe extern "C" fn(*mut super::Node, *mut ::core::ffi::c_void) -> bool,
        >,
        context: *mut ::core::ffi::c_void,
    ) -> bool;
}

#[cfg(feature = "pg16")]
pub unsafe fn planstate_tree_walker(
    planstate: *mut super::PlanState,
    walker: ::core::option::Option<
        unsafe extern "C" fn(*mut super::PlanState, *mut ::core::ffi::c_void) -> bool,
    >,
    context: *mut ::core::ffi::c_void,
) -> bool {
    crate::planstate_tree_walker_impl(planstate, walker, context)
}

#[cfg(feature = "pg16")]
pub unsafe fn query_tree_walker(
    query: *mut super::Query,
    walker: ::core::option::Option<
        unsafe extern "C" fn(*mut super::Node, *mut ::core::ffi::c_void) -> bool,
    >,
    context: *mut ::core::ffi::c_void,
    flags: ::core::ffi::c_int,
) -> bool {
    crate::query_tree_walker_impl(query, walker, context, flags)
}

#[cfg(feature = "pg16")]
pub unsafe fn query_or_expression_tree_walker(
    node: *mut super::Node,
    walker: ::core::option::Option<
        unsafe extern "C" fn(*mut super::Node, *mut ::core::ffi::c_void) -> bool,
    >,
    context: *mut ::core::ffi::c_void,
    flags: ::core::ffi::c_int,
) -> bool {
    crate::query_or_expression_tree_walker_impl(node, walker, context, flags)
}

#[cfg(feature = "pg16")]
pub unsafe fn expression_tree_walker(
    node: *mut crate::Node,
    walker: Option<unsafe extern "C" fn(*mut crate::Node, *mut ::core::ffi::c_void) -> bool>,
    context: *mut ::core::ffi::c_void,
) -> bool {
    crate::expression_tree_walker_impl(node, walker, context)
}

#[cfg(feature = "pg16")]
pub unsafe fn range_table_entry_walker(
    rte: *mut super::RangeTblEntry,
    walker: ::core::option::Option<
        unsafe extern "C" fn(*mut super::Node, *mut ::core::ffi::c_void) -> bool,
    >,
    context: *mut ::core::ffi::c_void,
    flags: ::core::ffi::c_int,
) -> bool {
    crate::range_table_entry_walker_impl(rte, walker, context, flags)
}

#[cfg(feature = "pg16")]
pub unsafe fn range_table_walker(
    rtable: *mut super::List,
    walker: ::core::option::Option<
        unsafe extern "C" fn(*mut super::Node, *mut ::core::ffi::c_void) -> bool,
    >,
    context: *mut ::core::ffi::c_void,
    flags: ::core::ffi::c_int,
) -> bool {
    crate::range_table_walker_impl(rtable, walker, context, flags)
}

#[cfg(feature = "pg16")]
pub unsafe fn raw_expression_tree_walker(
    node: *mut crate::Node,
    walker: Option<unsafe extern "C" fn(*mut crate::Node, *mut ::core::ffi::c_void) -> bool>,
    context: *mut ::core::ffi::c_void,
) -> bool {
    crate::raw_expression_tree_walker_impl(node, walker, context)
}

#[inline(always)]
pub unsafe fn MemoryContextSwitchTo(context: crate::MemoryContext) -> crate::MemoryContext {
    let old = crate::CurrentMemoryContext;

    crate::CurrentMemoryContext = context;
    old
}
