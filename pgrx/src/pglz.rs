//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Safe wrapper around PostgreSQL's PGLZ compression API (`common/pg_lzcompress.h`). The underlying C functions are pure (no palloc, no `elog(ERROR)`), so no `PgTryBuilder` is required.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Strategy {
    Default,
    Always,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PglzError {
    /// `pglz_decompress` returned -1: corrupt or truncated input.
    Decompress,
    /// Source slice length or `rawsize` does not fit in `i32`.
    InputTooLarge,
    /// Failed to allocate the output buffer (e.g. `rawsize` exceeds available memory).
    Allocation,
    /// Caller-provided destination buffer is smaller than required.
    BufferTooSmall,
}

pub struct Pglz;

impl Pglz {
    /// Upper bound on compressed output size for a given input length. Mirrors the C macro `PGLZ_MAX_OUTPUT(_dlen)` which is `(_dlen) + 4`. Saturates at `usize::MAX` instead of overflowing.
    pub const fn max_output(input_len: usize) -> usize {
        input_len.saturating_add(4)
    }

    /// See `pglz_maximum_compressed_size` in `common/pg_lzcompress.h`. Returns `Err(InputTooLarge)` if either argument exceeds `i32::MAX`.
    pub fn max_compressed_size(
        rawsize: usize,
        total_compressed: usize,
    ) -> Result<usize, PglzError> {
        if rawsize > i32::MAX as usize || total_compressed > i32::MAX as usize {
            return Err(PglzError::InputTooLarge);
        }
        // SAFETY: pure function, no PG state touched. Inputs are bounded to fit in i32.
        let r = unsafe {
            crate::pg_sys::pglz_maximum_compressed_size(rawsize as i32, total_compressed as i32)
        };
        Ok(r.max(0) as usize)
    }

    /// Raw FFI helper. Does NOT validate; caller must ensure:
    /// - `src.len() <= i32::MAX`
    /// - `dest` is valid for writes of at least `max_output(src.len())` bytes.
    ///
    /// Never constructs a `&mut [u8]` over uninitialized memory — works in pointer-space only.
    #[inline]
    unsafe fn compress_raw(
        src: &[u8],
        dest: *mut u8,
        strategy: Strategy,
    ) -> Result<Option<usize>, PglzError> {
        let strat = match strategy {
            // SAFETY: PGLZ_strategy_{default,always} are extern `*const PGLZ_Strategy` pointing to global PGLZ_Strategy structs valid for the backend's lifetime.
            Strategy::Default => crate::pg_sys::PGLZ_strategy_default,
            Strategy::Always => crate::pg_sys::PGLZ_strategy_always,
        };
        let cap = Self::max_output(src.len());
        // SAFETY: src is a valid slice; dest is valid for `cap` writes (caller contract); strat is a valid PGLZ_Strategy pointer. PGLZ is pure (no palloc, no elog).
        let ret = crate::pg_sys::pglz_compress(
            src.as_ptr() as *const ::core::ffi::c_char,
            src.len() as i32,
            dest as *mut ::core::ffi::c_char,
            strat,
        );
        if ret < 0 {
            Ok(None)
        } else {
            // Defense in depth: a buggy PGLZ returning ret > cap would let callers read uninitialized memory downstream.
            assert!(ret as usize <= cap, "pglz_compress returned {ret} > cap {cap}");
            Ok(Some(ret as usize))
        }
    }

    /// Raw FFI helper. Does NOT validate; caller must ensure:
    /// - `src.len() <= i32::MAX`
    /// - `rawsize <= i32::MAX`
    /// - `dest` is valid for writes of at least `rawsize` bytes.
    #[inline]
    unsafe fn decompress_raw(
        src: &[u8],
        dest: *mut u8,
        rawsize: usize,
        check_complete: bool,
    ) -> Result<usize, PglzError> {
        // SAFETY: src is a valid slice; dest is valid for `rawsize` writes (caller contract). PGLZ is pure.
        let ret = crate::pg_sys::pglz_decompress(
            src.as_ptr() as *const ::core::ffi::c_char,
            src.len() as i32,
            dest as *mut ::core::ffi::c_char,
            rawsize as i32,
            check_complete,
        );
        if ret < 0 {
            Err(PglzError::Decompress)
        } else {
            assert!(ret as usize <= rawsize, "pglz_decompress returned {ret} > rawsize {rawsize}");
            Ok(ret as usize)
        }
    }

    /// Compress `src` into a caller-provided `dest` buffer. Returns the number of bytes written, or `Ok(None)` if PGLZ rejected the input (incompressible per heuristics). `dest.len()` must be at least `max_output(src.len())`; otherwise returns `Err(BufferTooSmall)`.
    ///
    /// Prefer this over [`Self::compress`] in hot loops to reuse a single buffer across many calls and avoid per-call allocation.
    pub fn compress_into(
        src: &[u8],
        dest: &mut [u8],
        strategy: Strategy,
    ) -> Result<Option<usize>, PglzError> {
        if src.len() > i32::MAX as usize {
            return Err(PglzError::InputTooLarge);
        }
        let cap = Self::max_output(src.len());
        if dest.len() < cap {
            return Err(PglzError::BufferTooSmall);
        }
        // SAFETY: src.len() <= i32::MAX; dest.len() >= cap so dest.as_mut_ptr() is valid for cap writes.
        unsafe { Self::compress_raw(src, dest.as_mut_ptr(), strategy) }
    }

    /// Decompress `src` into the first `rawsize` bytes of caller-provided `dest`. Returns the number of bytes actually written (always `<= rawsize`). If `check_complete` is `true`, all of `src` must be consumed.
    ///
    /// `rawsize` is the expected uncompressed size and is independent of `dest.len()` — pass a buffer larger than `rawsize` to reuse a scratch allocation across multiple decode calls of varying sizes. Returns `Err(BufferTooSmall)` if `dest.len() < rawsize`.
    pub fn decompress_into(
        src: &[u8],
        dest: &mut [u8],
        rawsize: usize,
        check_complete: bool,
    ) -> Result<usize, PglzError> {
        if src.len() > i32::MAX as usize || rawsize > i32::MAX as usize {
            return Err(PglzError::InputTooLarge);
        }
        if dest.len() < rawsize {
            return Err(PglzError::BufferTooSmall);
        }
        // SAFETY: lengths bounded to i32::MAX; dest.len() >= rawsize so dest.as_mut_ptr() is valid for rawsize writes.
        unsafe { Self::decompress_raw(src, dest.as_mut_ptr(), rawsize, check_complete) }
    }

    /// Convenience wrapper around [`Self::compress_into`] that allocates a fresh `Vec`. Returns `Ok(None)` when PGLZ refuses the input.
    pub fn compress(src: &[u8], strategy: Strategy) -> Result<Option<Vec<u8>>, PglzError> {
        if src.len() > i32::MAX as usize {
            return Err(PglzError::InputTooLarge);
        }
        let cap = Self::max_output(src.len());
        let mut dest: Vec<u8> = Vec::new();
        dest.try_reserve_exact(cap).map_err(|_| PglzError::Allocation)?;
        // SAFETY: dest has capacity >= cap; pointer is valid for cap writes. We never construct a slice/reference over the uninitialized capacity — only the raw pointer crosses the FFI boundary, and PGLZ is write-only into that buffer.
        let result = unsafe { Self::compress_raw(src, dest.as_mut_ptr(), strategy)? };
        match result {
            Some(n) => {
                // SAFETY: compress_raw asserted n <= cap; PGLZ wrote exactly n bytes, so first n bytes are initialized.
                unsafe { dest.set_len(n) };
                Ok(Some(dest))
            }
            None => Ok(None),
        }
    }

    /// Convenience wrapper around [`Self::decompress_into`] that allocates a fresh `Vec` of `rawsize` bytes.
    ///
    /// # Security
    ///
    /// `rawsize` controls a buffer allocation up to ~2 GiB and **must come from a trusted source** (e.g. a TOAST varlena header), never from untrusted SQL input. A hostile caller passing `rawsize = i32::MAX` causes a multi-gigabyte allocation per call — a trivial DoS vector. Allocation failures are reported as `PglzError::Allocation` rather than aborting, but callers that accept `rawsize` from users must still bound it themselves.
    pub fn decompress(
        src: &[u8],
        rawsize: usize,
        check_complete: bool,
    ) -> Result<Vec<u8>, PglzError> {
        if src.len() > i32::MAX as usize || rawsize > i32::MAX as usize {
            return Err(PglzError::InputTooLarge);
        }
        if rawsize == 0 {
            return Ok(Vec::new());
        }
        let mut dest: Vec<u8> = Vec::new();
        dest.try_reserve_exact(rawsize).map_err(|_| PglzError::Allocation)?;
        // SAFETY: dest has capacity >= rawsize; pointer valid for rawsize writes. No reference is created over uninit memory.
        let n = unsafe { Self::decompress_raw(src, dest.as_mut_ptr(), rawsize, check_complete)? };
        // SAFETY: decompress_raw asserted n <= rawsize; PGLZ wrote exactly n bytes.
        unsafe { dest.set_len(n) };
        Ok(dest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_output_adds_four() {
        assert_eq!(Pglz::max_output(0), 4);
        assert_eq!(Pglz::max_output(1024), 1028);
        assert_eq!(Pglz::max_output(usize::MAX - 4), usize::MAX);
    }

    #[test]
    fn compress_into_rejects_undersized_buffer() {
        // max_output(1024) = 1028; a 1027-byte buffer must be rejected before any FFI call.
        let src = vec![0u8; 1024];
        let mut buf = vec![0u8; 1027];
        assert_eq!(
            Pglz::compress_into(&src, &mut buf, Strategy::Default).unwrap_err(),
            PglzError::BufferTooSmall
        );
    }

    #[test]
    fn decompress_into_rejects_undersized_buffer() {
        // rawsize > dest.len() must be rejected before any FFI call.
        let mut tiny = [0u8; 4];
        assert_eq!(
            Pglz::decompress_into(&[], &mut tiny, 16, true).unwrap_err(),
            PglzError::BufferTooSmall
        );
    }

    #[test]
    fn input_too_large_is_rejected() {
        // Pure validation path — no FFI.
        let dummy = [0u8; 4];
        let mut out = [0u8; 4];
        assert_eq!(
            Pglz::decompress_into(&dummy, &mut out, usize::MAX, true).unwrap_err(),
            PglzError::InputTooLarge
        );
    }
}
