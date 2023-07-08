/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

//! A safe wrapper around Postgres `StringInfo` structure
#![allow(dead_code, non_snake_case)]

use crate::{pg_sys, AllocatedByPostgres, AllocatedByRust, PgBox, WhoAllocated};
use core::fmt::{Display, Formatter};
use core::str::Utf8Error;
use std::io::Error;

/// StringInfoData holds information about an extensible string that is allocated by Postgres'
/// memory system, but generally follows Rust's drop semantics
pub struct StringInfo<AllocatedBy: WhoAllocated = AllocatedByRust> {
    inner: PgBox<pg_sys::StringInfoData, AllocatedBy>,
}

impl<AllocatedBy: WhoAllocated> From<StringInfo<AllocatedBy>> for &'static core::ffi::CStr {
    fn from(val: StringInfo<AllocatedBy>) -> Self {
        let len = val.len();
        let ptr = val.into_char_ptr();

        unsafe {
            core::ffi::CStr::from_bytes_with_nul_unchecked(std::slice::from_raw_parts(
                ptr as *const u8,
                (len + 1) as usize, // +1 to get the trailing null byte
            ))
        }
    }
}

impl<AllocatedBy: WhoAllocated> std::io::Write for StringInfo<AllocatedBy> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        self.push_bytes(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

impl<AllocatedBy: WhoAllocated> std::fmt::Write for StringInfo<AllocatedBy> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.push_str(s);
        Ok(())
    }
}

impl<AllocatedBy: WhoAllocated> Display for StringInfo<AllocatedBy> {
    /// Convert this [`StringInfo`] into a Rust string.  This uses [`String::from_utf8_lossy`] as
    /// it's fine for a Postgres [`StringInfo`] to contain null bytes and also not even be proper
    /// UTF8.
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(&String::from_utf8_lossy(self.as_bytes()))
    }
}

impl StringInfo<AllocatedByRust> {
    /// Construct a new `StringInfo` of its default size, allocated in `CurrentMemoryContext`
    ///
    /// Note that Postgres can only represent up to 1 gigabyte of data in a `StringInfo`
    pub fn new() -> Self {
        StringInfo {
            inner: unsafe {
                // SAFETY: makeStringInfo() always returns a valid StringInfoData pointer.  It'll
                // ereport if it can't
                PgBox::<_, AllocatedByRust>::from_rust(pg_sys::makeStringInfo())
            },
        }
    }

    /// Construct a new `StringInfo`, allocated by Postgres in `CurrentMemoryContext`, ensuring it
    /// has a capacity of the specified `len`.
    ///
    /// Note that Postgres can only represent up to 1 gigabyte of data in a `StringInfo`
    pub fn with_capacity(len: i32) -> Self {
        let mut si = StringInfo::default();
        si.enlarge(len);
        si
    }
}

impl StringInfo<AllocatedByPostgres> {
    /// Construct a `StringInfo` from a Postgres-allocated `pg_sys::StringInfo`.
    ///
    /// The backing `pg_sys::StringInfo` structure will be freed whenever the memory context in which
    /// it was originally allocated is reset.
    ///
    /// # Safety
    ///
    /// This function is unsafe as it cannot confirm the provided [`pg_sys::StringInfo`] pointer is
    /// valid
    pub unsafe fn from_pg(sid: pg_sys::StringInfo) -> Option<Self> {
        if sid.is_null() {
            None
        } else {
            Some(StringInfo { inner: PgBox::from_pg(sid) })
        }
    }
}

impl<AllocatedBy: WhoAllocated> StringInfo<AllocatedBy> {
    /// What is the length, excluding the trailing null byte
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len as _
    }

    /// Do we have any characters?
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Push a Rust character onto the end.  A Rust `char` could be 4 bytes in total, so it
    /// is converted into a String first to ensure unicode correctness
    #[inline]
    pub fn push(&mut self, ch: char) {
        self.push_str(&ch.to_string());
    }

    /// Push a String reference onto the end
    #[inline]
    pub fn push_str(&mut self, s: &str) {
        self.push_bytes(s.as_bytes())
    }

    /// Push arbitrary bytes onto the end.  Any byte sequence is allowed, including those with
    /// embedded NULLs
    ///
    /// # Panics
    ///
    /// This function will panic if the length of bytes is larger than an `i32`
    #[inline]
    pub fn push_bytes(&mut self, bytes: &[u8]) {
        unsafe {
            // SAFETY:  self.inner will always be valid
            pg_sys::appendBinaryStringInfo(
                self.inner.as_ptr(),
                bytes.as_ptr() as _,
                bytes.len().try_into().expect("len of bytes doesn't fit in an i32"),
            )
        }
    }

    /// Push the bytes behind a raw pointer of a given length onto the end
    ///
    /// # Safety
    ///
    /// This function is unsafe as we cannot ensure the specified `ptr` and `len` arguments
    /// are what you say that are and otherwise in agreement
    #[inline]
    pub unsafe fn push_raw(&mut self, ptr: *const std::os::raw::c_char, len: i32) {
        unsafe {
            // SAFETY:  self.inner will always be a valid StringInfoData pointer
            // and the caller gets to decide if `ptr` and `len` line up
            pg_sys::appendBinaryStringInfo(self.inner.as_ptr(), ptr.cast(), len)
        }
    }

    /// Reset the size of the `StringInfo` back to zero-length.  This does/// *not** free any
    /// previously-allocated memory
    #[inline]
    pub fn reset(&mut self) {
        unsafe {
            // SAFETY:  self.inner will always be a valid StringInfoData pointer
            pg_sys::resetStringInfo(self.inner.as_ptr())
        }
    }

    /// Ensure that this `StringInfo` is at least `needed` bytes long
    #[inline]
    pub fn enlarge(&mut self, needed: i32) {
        unsafe {
            // SAFETY:  self.inner will always be a valid StringInfoData pointer
            pg_sys::enlargeStringInfo(self.inner.as_ptr(), needed)
        }
    }

    /// A `&str` representation.
    ///
    /// # Errors
    ///
    /// If the contained bytes aren't valid UTF8, a [UTF8Error] is returned.  Postgres StringInfo
    /// is allowed to contain non-UTF8 byte sequences, so this is a real possibility.
    #[inline]
    pub fn as_str(&self) -> Result<&str, Utf8Error> {
        std::str::from_utf8(self.as_bytes())
    }

    /// A pointer to the backing bytes
    #[inline]
    pub fn as_ptr(&self) -> *const std::os::raw::c_char {
        self.inner.data
    }

    /// A mutable pointer to the backing bytes
    #[inline]
    pub fn as_mut_ptr(&self) -> *mut std::os::raw::c_char {
        self.inner.data
    }

    /// A `&[u8]` byte slice representation
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            // SAFETY:  self.inner will always be a valid StringInfoData pointer, and Postgres will
            // never have self.inner.data be invalid
            std::slice::from_raw_parts(self.inner.data as *const u8, self.len())
        }
    }

    /// A mutable `&[u8]` byte slice representation
    #[inline]
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        unsafe {
            // SAFETY:  self.inner will always be a valid StringInfoData
            // pointer, and Postgres will never have self.inner.data be invalid
            std::slice::from_raw_parts_mut(self.inner.data as *mut u8, self.len())
        }
    }

    /// Convert this `StringInfo` into one that is wholly owned and now managed by Postgres
    #[inline]
    pub fn into_pg(mut self) -> *mut pg_sys::StringInfoData {
        // NB:  We are replacing self.inner with a PgBox containing the null pointer.  However,
        // `self` will be dropped as soon as this function ends and we account for this case in our
        // drop implementation
        let inner = std::mem::replace(&mut self.inner, PgBox::<_, AllocatedBy>::null());
        inner.into_pg()
    }

    /// Convert this `StringInfo` into a `"char *"` that is wholly owned and now managed by Postgres
    #[inline]
    pub fn into_char_ptr(self) -> *const std::os::raw::c_char {
        // in case we're AllocatedByRust, we don't want drop to free `self.inner.data` now that we've
        // consumed `self` and are returning a raw pointer to some memory we allocated, so we "round-trip"
        // to ensure Rust thinks we're now `AllocatedByPostgres`, which has an empty drop impl
        let sid_ptr = self.into_pg();
        unsafe {
            // SAFETY: we just made the StringInfoData pointer so we know it's valid and properly
            // initialized throughout
            sid_ptr.as_ref().unwrap_unchecked().data
        }
    }
}

impl Default for StringInfo<AllocatedByRust> {
    fn default() -> Self {
        Self::new()
    }
}

impl From<String> for StringInfo<AllocatedByRust> {
    fn from(s: String) -> Self {
        StringInfo::from(s.as_str())
    }
}

impl From<&str> for StringInfo<AllocatedByRust> {
    fn from(s: &str) -> Self {
        let mut rc = StringInfo::new();
        rc.push_str(s);
        rc
    }
}

impl From<Vec<u8>> for StringInfo<AllocatedByRust> {
    fn from(v: Vec<u8>) -> Self {
        let mut rc = StringInfo::new();
        rc.push_bytes(v.as_slice());
        rc
    }
}

impl From<&[u8]> for StringInfo<AllocatedByRust> {
    fn from(v: &[u8]) -> Self {
        let mut rc = StringInfo::new();
        rc.push_bytes(v);
        rc
    }
}

impl<AllocatedBy: WhoAllocated> Drop for StringInfo<AllocatedBy> {
    fn drop(&mut self) {
        unsafe {
            // SAFETY:  self.inner could represent the null pointer, but if it doesn't, then it's
            // one we can use as self.inner.data will always be allocated if self.inner is.
            //
            // It's also prescribed by Postgres that to fully deallocate a StringInfo pointer, the
            // owner is responsible for freeing its .data member... and that's us
            if !self.inner.is_null() {
                AllocatedBy::maybe_pfree(self.inner.data.cast())
            }
        }
    }
}
