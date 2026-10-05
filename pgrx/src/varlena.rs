//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
//! Borrowed Rust text and byte views of PostgreSQL varlena values.

use crate::{PgBox, pg_sys};
use core::{slice, str};

/// Borrow a PostgreSQL text value, validating UTF-8.
///
/// This is a zero-copy view of the original varlena allocation; it does not
/// extend that allocation's lifetime.
///
/// # Safety
///
/// `varlena` must point to a complete, initialized, uncompressed PostgreSQL varlena
/// allocation, with its ordinary or short header describing the allocation's bounds.
/// The allocation must stay live and its payload must not be mutated for `'a`.
#[inline]
pub unsafe fn text_to_rust_str<'a>(
    varlena: *const pg_sys::varlena,
) -> Result<&'a str, str::Utf8Error> {
    // SAFETY: the caller keeps this complete initialized varlena and its payload live and immutable.
    unsafe {
        let len = pg_sys::VARSIZE_ANY_EXHDR!(varlena).get() as usize;
        let data = pg_sys::VARDATA_ANY!(varlena).get();
        str::from_utf8(slice::from_raw_parts(data.cast::<u8>(), len))
    }
}

/// Borrow a PostgreSQL text value whose UTF-8 encoding is already established.
///
/// This is a zero-copy view of the original varlena allocation; it does not
/// extend that allocation's lifetime.
///
/// # Safety
///
/// `varlena` must point to a complete, initialized, uncompressed PostgreSQL varlena
/// allocation, with its ordinary or short header describing the allocation's bounds.
/// The allocation must stay live and its payload must not be mutated for `'a`.
/// The payload must also contain valid UTF-8.
#[inline]
pub unsafe fn text_to_rust_str_unchecked<'a>(varlena: *const pg_sys::varlena) -> &'a str {
    // SAFETY: the caller keeps this complete initialized varlena and its payload live and immutable.
    unsafe {
        let len = pg_sys::VARSIZE_ANY_EXHDR!(varlena).get() as usize;
        let data = pg_sys::VARDATA_ANY!(varlena).get();
        str::from_utf8_unchecked(slice::from_raw_parts(data.cast::<u8>(), len))
    }
}

/// Borrow the payload bytes of an uncompressed PostgreSQL varlena value.
///
/// This is a zero-copy view of the original varlena allocation; it does not
/// extend that allocation's lifetime.
///
/// # Safety
///
/// `varlena` must point to a complete, initialized, uncompressed PostgreSQL varlena
/// allocation, with its ordinary or short header describing the allocation's bounds.
/// The allocation must stay live and its payload must not be mutated for `'a`.
#[inline]
pub unsafe fn varlena_to_byte_slice<'a>(varlena: *const pg_sys::varlena) -> &'a [u8] {
    // SAFETY: the caller keeps this complete initialized varlena and its payload live and immutable.
    unsafe {
        let len = pg_sys::VARSIZE_ANY_EXHDR!(varlena).get() as usize;
        let data = pg_sys::VARDATA_ANY!(varlena).get();
        slice::from_raw_parts(data.cast::<u8>(), len)
    }
}

/// Convert a Rust `&str` into a Postgres `text *`.
///
/// This allocates the returned Postgres `text *` in `CurrentMemoryContext`.
#[inline]
pub fn rust_str_to_text_p(s: &str) -> PgBox<pg_sys::varlena> {
    let bytea = rust_byte_slice_to_bytea(s.as_bytes());

    // a pg_sys::bytea is a type alias for pg_sys::varlena so no cast is needed
    // SAFETY: bytea will be a valid pointer
    unsafe { PgBox::from_pg(bytea.as_ptr()) }
}

/// Convert a Rust `&[u8]` into a Postgres `bytea *` (which is really a varchar)
///
/// This allocates the returned Postgres `bytea *` in `CurrentMemoryContext`.
#[inline]
pub fn rust_byte_slice_to_bytea(slice: &[u8]) -> PgBox<pg_sys::bytea> {
    // SAFETY:  `slice` will provide a valid pointer and pg_sys::cstring_to_text_with_len() will too
    unsafe {
        PgBox::from_pg(pg_sys::cstring_to_text_with_len(
            slice.as_ptr() as *const std::os::raw::c_char,
            slice.len() as i32,
        ))
    }
}
