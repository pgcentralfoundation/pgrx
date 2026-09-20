//! The C-shaped call frame the pgrx runtime reads, built around pgrust's
//! native frame for the duration of one call.
//!
//! pgrx code reads `fcinfo->args`, `fcinfo->flinfo->fn_extra`, `fn_mcxt`,
//! `nargs`, `isnull`... as C struct fields. pgrust's frame has the same
//! information in a different shape (typed `FnExtra`, `flinfo` as a separate
//! parameter, no `fn_mcxt`). [`call_v1`] copies the header and the argument
//! datums into a C-layout view, runs the pgrx wrapper body on it, then
//! copies `isnull` back. The `FmgrInfo` view persists across calls inside
//! the native `FmgrInfo`'s `fn_extra`, so `fn_extra` memoization keeps
//! working.

use core::ffi::{c_short, c_uchar, c_void};
use core::ptr::NonNull;
use std::panic::{AssertUnwindSafe, catch_unwind};

use super::error::caught_to_pg_error;
use super::mem::{
    AllocSetContextCreateInternal, MemoryContext, MemoryContextDelete, current_memory_context,
    set_current_memory_context, top_memory_context,
};
use super::types::{__IncompleteArrayField, NullableDatum, PGFunction, fmNodePtr};
use crate::panic::downcast_panic_payload;
use crate::{Datum, Oid};

/// C's `FmgrInfo`, plus a link to the native pgrust `FmgrInfo` it views.
#[repr(C)]
pub struct FmgrInfo {
    pub fn_addr: PGFunction,
    pub fn_oid: Oid,
    pub fn_nargs: c_short,
    pub fn_strict: bool,
    pub fn_retset: bool,
    pub fn_stats: c_uchar,
    pub fn_extra: *mut c_void,
    pub fn_mcxt: MemoryContext,
    pub fn_expr: fmNodePtr,
    /// pgrust: the native FmgrInfo this view belongs to (null for a
    /// temporary view of a call without one). Typed opaque so the view stays
    /// `UnwindSafe` for pgrx's catch_unwind-bounded closures.
    pub native: *mut c_void,
}

impl Default for FmgrInfo {
    fn default() -> Self {
        FmgrInfo {
            fn_addr: None,
            fn_oid: Oid::INVALID,
            fn_nargs: 0,
            fn_strict: false,
            fn_retset: false,
            fn_stats: 0,
            fn_extra: core::ptr::null_mut(),
            fn_mcxt: core::ptr::null_mut(),
            fn_expr: core::ptr::null_mut(),
            native: core::ptr::null_mut(),
        }
    }
}

/// C's `FunctionCallInfoBaseData`, plus a link to the native pgrust frame.
/// `args` is a trailing array, exactly as in C, so `size_of` + `nargs *
/// size_of::<NullableDatum>()` sizes a frame.
#[repr(C)]
pub struct FunctionCallInfoBaseData {
    pub flinfo: *mut FmgrInfo,
    pub context: fmNodePtr,
    pub resultinfo: fmNodePtr,
    pub fncollation: Oid,
    pub isnull: bool,
    pub nargs: c_short,
    /// pgrust: the native frame (null for a frame pgrx built itself for a
    /// direct function call), typed opaque as `FmgrInfo::native`. The native
    /// frame is a DST (its args tail); `native_len` is that tail's length.
    pub native: *mut c_void,
    pub native_len: usize,
    pub args: __IncompleteArrayField<NullableDatum>,
}

impl Default for FunctionCallInfoBaseData {
    fn default() -> Self {
        let mut s = core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: an all-zero frame is a valid empty frame (null pointers,
        // nargs 0, isnull false).
        unsafe {
            core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}

/// The per-`FmgrInfo` state kept in the native `fn_extra`: the C view and the
/// `fn_mcxt` context pgrx allocates memoized state into.
struct FlinfoShim {
    c: FmgrInfo,
}

impl Drop for FlinfoShim {
    fn drop(&mut self) {
        if !self.c.fn_mcxt.is_null() {
            // SAFETY: fn_mcxt was created by AllocSetContextCreateInternal in
            // `shim_for`, so it is one of ours.
            unsafe { MemoryContextDelete(self.c.fn_mcxt) };
        }
    }
}

/// The C view of a native `FmgrInfo`, created on first use.
fn shim_for(flinfo: &mut ::pgr_fmgr::FmgrInfo) -> *mut FmgrInfo {
    if !flinfo.has_fn_extra() {
        let mut c = FmgrInfo {
            fn_oid: Oid::from_u32(flinfo.fn_oid),
            fn_nargs: flinfo.fn_nargs,
            fn_strict: flinfo.fn_strict,
            fn_retset: flinfo.fn_retset,
            fn_stats: flinfo.fn_stats,
            ..FmgrInfo::default()
        };
        // fn_mcxt: C's is the context the FmgrInfo was made in (query
        // lifetime). Ours lives exactly as long as the native FmgrInfo.
        c.fn_mcxt = unsafe {
            AllocSetContextCreateInternal(top_memory_context(), c"pgrx fn_mcxt".as_ptr(), 0, 0, 0)
        };
        c.native = flinfo as *mut ::pgr_fmgr::FmgrInfo as *mut c_void;
        flinfo.set_fn_extra(FlinfoShim { c });
    }
    let shim = flinfo
        .fn_extra_mut::<FlinfoShim>()
        .expect("pgrx: native fn_extra is not the pgrx FmgrInfo view");
    &mut shim.c as *mut FmgrInfo
}

const INLINE_ARGS: usize = 8;

/// Run a pgrx V1 wrapper body against a native pgrust call.
///
/// `body` receives the C-shaped frame pointer and returns the datum the pgrx
/// wrapper produced; the frame's `isnull` is copied back to the native frame.
/// Any panic inside `body` becomes the call's `Err`.
pub fn call_v1(
    flinfo: Option<&mut ::pgr_fmgr::FmgrInfo>,
    fcinfo: &mut ::pgr_fmgr::FunctionCallInfoBaseData,
    body: impl FnOnce(*mut FunctionCallInfoBaseData) -> Datum,
) -> ::pgr_types_error::PgResult<::pgr_datum::Datum> {
    // The FmgrInfo view: persistent when the call carries a native FmgrInfo,
    // a stack temporary otherwise.
    let mut temp_flinfo: Option<FmgrInfo> = None;
    let c_flinfo: *mut FmgrInfo = match flinfo {
        Some(fl) => shim_for(fl),
        None => {
            temp_flinfo = Some(FmgrInfo::default());
            temp_flinfo.as_mut().unwrap() as *mut FmgrInfo
        }
    };

    // The frame: header + nargs trailing NullableDatums, on the stack when small.
    let nargs = fcinfo.nargs();
    let frame_bytes = core::mem::size_of::<FunctionCallInfoBaseData>()
        + nargs * core::mem::size_of::<NullableDatum>();
    const INLINE_BYTES: usize = core::mem::size_of::<FunctionCallInfoBaseData>()
        + INLINE_ARGS * core::mem::size_of::<NullableDatum>();
    let mut inline = core::mem::MaybeUninit::<[u64; INLINE_BYTES / 8 + 1]>::uninit();
    let mut heap: Vec<u64> = Vec::new();
    let frame: *mut FunctionCallInfoBaseData = if frame_bytes <= INLINE_BYTES {
        inline.as_mut_ptr() as *mut FunctionCallInfoBaseData
    } else {
        heap.resize(frame_bytes / 8 + 1, 0);
        heap.as_mut_ptr() as *mut FunctionCallInfoBaseData
    };
    // SAFETY: `frame` has room for the header and `nargs` args.
    unsafe {
        frame.write(FunctionCallInfoBaseData {
            flinfo: c_flinfo,
            context: fcinfo
                .context
                .map_or(core::ptr::null_mut(), |p| p.as_ptr().cast()),
            resultinfo: fcinfo
                .resultinfo
                .map_or(core::ptr::null_mut(), |p| p.as_ptr().cast()),
            fncollation: Oid::from_u32(fcinfo.get_collation()),
            isnull: false,
            nargs: nargs as c_short,
            native: (fcinfo as *mut ::pgr_fmgr::FunctionCallInfoBaseData).cast::<c_void>(),
            native_len: fcinfo.args.len(),
            args: __IncompleteArrayField::new(),
        });
        let args = (*frame).args.as_mut_ptr();
        for i in 0..nargs {
            args.add(i).write(NullableDatum {
                value: Datum::from(fcinfo.arg(i).as_usize()),
                isnull: fcinfo.argisnull(i),
            });
        }
    }

    // Arm the current context with the call's result context.
    let result_ctx: MemoryContext = super::mem::handle_of(fcinfo.result_mcx().context());
    let prev = current_memory_context();
    set_current_memory_context(result_ctx);
    let outcome = catch_unwind(AssertUnwindSafe(|| body(frame)));
    set_current_memory_context(prev);

    match outcome {
        Ok(d) => {
            // SAFETY: frame is live and was written above.
            let isnull = unsafe { (*frame).isnull };
            fcinfo.isnull = isnull;
            Ok(::pgr_datum::Datum::from_usize(d.value()))
        }
        Err(payload) => Err(caught_to_pg_error(downcast_panic_payload(payload))),
    }
}

/// The native frame behind a C view, if the view was built by [`call_v1`].
///
/// # Safety
/// `fcinfo` must be a live frame.
pub unsafe fn native_fcinfo<'a>(
    fcinfo: *mut FunctionCallInfoBaseData,
) -> Option<&'a mut ::pgr_fmgr::FunctionCallInfoBaseData> {
    if (*fcinfo).native.is_null() {
        return None;
    }
    // Rebuild the DST pointer from its data pointer and args length.
    let fat = core::ptr::slice_from_raw_parts_mut(
        (*fcinfo).native as *mut ::pgr_datum::NullableDatum,
        (*fcinfo).native_len,
    ) as *mut ::pgr_fmgr::FunctionCallInfoBaseData;
    Some(&mut *fat)
}

/// The native FmgrInfo behind a C view's `flinfo`, if any.
///
/// # Safety
/// `fcinfo` must be a live frame.
pub unsafe fn native_flinfo<'a>(
    fcinfo: *mut FunctionCallInfoBaseData,
) -> Option<&'a mut ::pgr_fmgr::FmgrInfo> {
    let fl = (*fcinfo).flinfo;
    if fl.is_null() {
        return None;
    }
    NonNull::new((*fl).native as *mut ::pgr_fmgr::FmgrInfo).map(|mut p| p.as_mut())
}

/// The pgrust memory context to allocate call results in: the armed current
/// context.
pub fn result_mcx<'a>() -> ::pgr_mcx::Mcx<'a> {
    // SAFETY: the current context is a live pgrust context for the thread.
    unsafe { super::mem::native_ctx(current_memory_context()).mcx() }
}
