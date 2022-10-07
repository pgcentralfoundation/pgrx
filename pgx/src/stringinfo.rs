/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

//! A safe wrapper around Postgres `StringInfo` structure
#![allow(dead_code, non_snake_case)]

use crate::{pg_sys, void_mut_ptr};
use std::io::Error;

/// StringInfoData holds information about an extensible string that is allocated by Postgres'
/// memory system, but generally follows Rust's drop semantics
pub struct StringInfo {
    sid: pg_sys::StringInfo,
    needs_pfree: bool,
}

impl From<StringInfo> for pg_sys::StringInfo {
    fn from(val: StringInfo) -> Self {
        val.sid
    }
}

impl From<StringInfo> for &'static std::ffi::CStr {
    fn from(val: StringInfo) -> Self {
        let len = val.len();
        let ptr = val.into_char_ptr();

        unsafe {
            std::ffi::CStr::from_bytes_with_nul_unchecked(std::slice::from_raw_parts(
                ptr as *const u8,
                (len + 1) as usize, // +1 to get the trailing null byte
            ))
        }
    }
}

impl From<StringInfo> for &'static crate::cstr_core::CStr {
    fn from(val: StringInfo) -> Self {
        let len = val.len();
        let ptr = val.into_char_ptr();

        unsafe {
            crate::cstr_core::CStr::from_bytes_with_nul_unchecked(std::slice::from_raw_parts(
                ptr as *const u8,
                (len + 1) as usize, // +1 to get the trailing null byte
            ))
        }
    }
}

impl std::io::Write for StringInfo {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        self.push_bytes(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

impl ToString for StringInfo {
    fn to_string(&self) -> String {
        unsafe { std::str::from_utf8_unchecked(self.as_bytes()).to_owned() }
    }
}

impl StringInfo {
    /// Construct a new `StringInfo` of its default size, allocated by Postgres in `CurrentMemoryContext`
    ///
    /// Unless `.into_pg()` or `.into_char_ptr()` are called, memory management of
    /// this `StringInfo` follow Rust's drop semantics.
    pub fn new() -> Self {
        StringInfo { sid: unsafe { pg_sys::makeStringInfo() }, needs_pfree: true }
    }

    /// Construct a new `StringInfo`, allocated by Postgres in `CurrentMemoryContext`, ensuring it
    /// has a capacity of the specified `len`.
    ///
    /// Note that Postgres can only represent up to 1 gigabyte of data in a `StringInfo`
    ///
    /// Unless `.into_pg()` or `.into_char_ptr()` are called, memory management of
    /// this `StringInfo` follow Rust's drop semantics.
    pub fn with_capacity(len: usize) -> Self {
        let mut si = StringInfo::default();
        si.enlarge(len);
        si
    }

    /// Construct a `StringInfo` from a Postgres-allocated `pg_sys::StringInfo`.
    ///
    /// The backing `pg_sys::StringInfo` structure will be freed whenever the memory context in which
    /// it was originally allocated is reset.
    pub fn from_pg(sid: pg_sys::StringInfo) -> Option<Self> {
        if sid.is_null() {
            None
        } else {
            Some(StringInfo { sid, needs_pfree: false })
        }
    }

    /// What is the length, excluding the trailing null byte
    #[inline]
    pub fn len(&self) -> usize {
        // safe:  self.sid will never be null
        unsafe { &mut *self.sid }.len as usize
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

    /// Push arbitrary bytes onto the end.  Any byte sequence is allowed, include those with
    /// embedded NULLs
    #[inline]
    pub fn push_bytes(&mut self, bytes: &[u8]) {
        // safe:  self.sid will never be null
        unsafe {
            pg_sys::appendBinaryStringInfo(
                self.sid,
                bytes.as_ptr() as *const std::os::raw::c_char,
                bytes.len() as i32,
            )
        }
    }

    /// Push the bytes behind a raw pointer of a given length onto the end
    #[inline]
    pub fn push_raw(&mut self, ptr: void_mut_ptr, len: usize) {
        // safe:  self.sid will never be null
        unsafe {
            pg_sys::appendBinaryStringInfo(self.sid, ptr as *const std::os::raw::c_char, len as i32)
        }
    }

    /// Reset the size of the `StringInfo` back to zero-length.  This does/// *not** free any
    /// previously-allocated memory
    #[inline]
    pub fn reset(&mut self) {
        // safe:  self.sid will never be null
        unsafe { pg_sys::resetStringInfo(self.sid) }
    }

    /// Ensure that this `StringInfo` is at least `needed` bytes long
    #[inline]
    pub fn enlarge(&mut self, needed: usize) {
        // safe:  self.sid will never be null
        unsafe { pg_sys::enlargeStringInfo(self.sid, needed as i32) }
    }

    /// A pointer representation
    #[inline]
    pub fn as_ptr(&self) -> *mut std::os::raw::c_char {
        // safe:  self.sid will never be null
        unsafe { (*self.sid).data }
    }

    /// A `&[u8]` byte slice representation
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        // safe:  self.sid will never be null
        unsafe {
            if (*self.sid).data.is_null() {
                return &[];
            }

            std::slice::from_raw_parts((*self.sid).data as *const u8, self.len())
        }
    }

    /// A mutable `&[u8]` byte slice representation
    #[inline]
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        // safe:  self.sid will never be null
        unsafe {
            if (*self.sid).data.is_null() {
                return &mut [];
            }

            std::slice::from_raw_parts_mut((*self.sid).data as *mut u8, self.len())
        }
    }

    /// Convert this `StringInfo` into one that is wholly owned and now managed by Postgres
    #[inline]
    pub fn into_pg(mut self) -> *mut pg_sys::StringInfoData {
        self.needs_pfree = false;
        self.sid
    }

    /// Convert this `StringInfo` into a `"char *"` that is wholly owned and now managed by Postgres
    #[inline]
    pub fn into_char_ptr(mut self) -> *const std::os::raw::c_char {
        self.needs_pfree = false;
        // safe:  self.sid will never be null
        unsafe {
            let ptr = (*self.sid).data;
            (&mut *self.sid).data = std::ptr::null_mut();
            ptr as *const std::os::raw::c_char
        }
    }
}

impl Default for StringInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl From<String> for StringInfo {
    fn from(s: String) -> Self {
        StringInfo::from(s.as_str())
    }
}

impl From<&str> for StringInfo {
    fn from(s: &str) -> Self {
        let mut rc = StringInfo::new();
        rc.push_str(s);
        rc
    }
}

impl From<Vec<u8>> for StringInfo {
    fn from(v: Vec<u8>) -> Self {
        let mut rc = StringInfo::new();
        rc.push_bytes(v.as_slice());
        rc
    }
}

impl From<&[u8]> for StringInfo {
    fn from(v: &[u8]) -> Self {
        let mut rc = StringInfo::new();
        rc.push_bytes(v);
        rc
    }
}

impl Drop for StringInfo {
    fn drop(&mut self) {
        // we only pfree our internal pointers if we weren't constructed from a/// mut pg_sys::StringInfo
        // given to us from Postgres
        if self.needs_pfree {
            // safe:  self.sid will never be null
            unsafe {
                if !(*self.sid).data.is_null() {
                    pg_sys::pfree((*self.sid).data as void_mut_ptr);
                }
                pg_sys::pfree(self.sid as void_mut_ptr);
            }
        }
    }
}
