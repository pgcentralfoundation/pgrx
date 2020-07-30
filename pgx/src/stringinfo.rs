// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.


#![allow(dead_code, non_snake_case)]

use crate::pg_sys;
use std::ffi::CStr;
use std::io::Error;

#[derive(Debug)]
pub struct StringInfo {
    sid: pg_sys::StringInfo,
    is_from_pg: bool,
}

impl Into<pg_sys::StringInfo> for StringInfo {
    fn into(self) -> pg_sys::StringInfo {
        self.into_postgres()
    }
}

impl Into<&'static std::ffi::CStr> for StringInfo {
    fn into(self) -> &'static std::ffi::CStr {
        let len = self.len();
        let ptr = self.into_char_ptr();

        unsafe {
            std::ffi::CStr::from_bytes_with_nul_unchecked(std::slice::from_raw_parts(
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

pub trait IntoPostgres {
    fn into_postgres(self) -> pg_sys::StringInfo;
}

impl IntoPostgres for StringInfo {
    fn into_postgres(self) -> pg_sys::StringInfo {
        // TODO: doesn't 'self' (which is a StringInfo) get leaked here?
        let rc = self.sid;
        std::mem::forget(self);
        rc
    }
}

impl IntoPostgres for String {
    fn into_postgres(self) -> pg_sys::StringInfo {
        StringInfo::from(self).into_postgres()
    }
}

impl IntoPostgres for &str {
    fn into_postgres(self) -> pg_sys::StringInfo {
        StringInfo::from(self).into_postgres()
    }
}

impl IntoPostgres for Vec<u8> {
    fn into_postgres(self) -> pg_sys::StringInfo {
        StringInfo::from(self).into_postgres()
    }
}

impl IntoPostgres for &[u8] {
    fn into_postgres(self) -> pg_sys::StringInfo {
        StringInfo::from(self).into_postgres()
    }
}

impl ToString for StringInfo {
    fn to_string(&self) -> String {
        unsafe {
            CStr::from_bytes_with_nul_unchecked(std::slice::from_raw_parts(
                (*self.sid).data as *const u8,
                (*self.sid).len as usize + 1, // + 1 to include the null byte
            ))
            .to_string_lossy()
            .to_string()
        }
    }
}

impl StringInfo {
    pub fn new() -> Self {
        StringInfo {
            sid: unsafe { pg_sys::makeStringInfo() },
            is_from_pg: false,
        }
    }

    pub fn from_pg(sid: pg_sys::StringInfo) -> Option<Self> {
        if sid.is_null() {
            None
        } else {
            Some(StringInfo {
                sid,
                is_from_pg: true,
            })
        }
    }

    pub fn len(&self) -> i32 {
        unsafe { &mut *self.sid }.len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push(&mut self, ch: char) {
        unsafe { pg_sys::appendStringInfoChar(self.sid, ch as std::os::raw::c_char) }
    }

    pub fn push_str(&mut self, s: &str) {
        unsafe {
            pg_sys::appendBinaryStringInfo(
                self.sid,
                s.as_ptr() as *const std::os::raw::c_char,
                s.len() as i32,
            )
        }
    }

    pub fn push_bytes(&mut self, bytes: &[u8]) {
        unsafe {
            pg_sys::appendBinaryStringInfo(
                self.sid,
                bytes.as_ptr() as *const std::os::raw::c_char,
                bytes.len() as i32,
            )
        }
    }

    pub fn reset(&mut self) {
        unsafe { pg_sys::resetStringInfo(self.sid) }
    }

    pub fn enlarge(&mut self, needed: i32) {
        unsafe { pg_sys::enlargeStringInfo(self.sid, needed) }
    }

    pub fn into_char_ptr(self) -> *const std::os::raw::c_char {
        let ptr = unsafe { self.sid.as_ref() }.unwrap().data as *const std::os::raw::c_char;
        unsafe {
            pg_sys::pfree(self.sid as *mut std::os::raw::c_void);
        }
        std::mem::forget(self);
        ptr
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
        if !self.is_from_pg {
            // instruct Rust to free self.sid.data, and self.sid
            // via Postgres' pfree()
            unsafe {
                if !self.sid.is_null() {
                    if !(*self.sid).data.is_null() {
                        pg_sys::pfree((*self.sid).data as *mut std::os::raw::c_void);
                    }
                    pg_sys::pfree(self.sid as *mut std::os::raw::c_void);
                }
            }
        }
    }
}
