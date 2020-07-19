// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

//! for converting a pg_sys::Datum and a corresponding "is_null" bool into a typed Option

use crate::{
    pg_sys, text_to_rust_str_unchecked, vardata_any, varsize_any_exhdr, void_mut_ptr, PgBox,
    PgMemoryContexts,
};
use std::ffi::CStr;

/// Convert a `(pg_sys::Datum, is_null:bool, type_oid:pg_sys::Oid)` tuple into a Rust type
///
/// Default implementations are provided for the common Rust types.
///
/// If implementing this, also implement `IntoDatum` for the reverse
/// conversion.
pub trait FromDatum {
    /// ## Safety
    ///
    /// This method is inherently unsafe as the `datum` argument can represent an arbitrary
    /// memory address in the case of pass-by-reference Datums.  Referencing that memory address
    /// can cause Postgres to crash if it's invalid.
    ///
    /// If the `(datum, is_null)` tuple comes from Postgres, it's generally okay to consider this
    /// a safe call (ie, wrap it in `unsafe {}`) and move on with life.
    ///
    /// If, however, you're providing an arbitrary datum value, it needs to be considered unsafe
    /// and that unsafeness should be propagated through your API.
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool, typoid: pg_sys::Oid) -> Option<Self>
    where
        Self: Sized;

    /// Default implementation switched to the specified memory context and then simply calls
    /// `From::from_datum(...)` from within that context.
    ///
    /// For certain Datums (such as `&str`), this is likely not enough and this function
    /// should be overridden in the type's trait implementation.
    ///
    /// The intent here is that the returned Rust type, which might be backed by a pass-by-reference
    /// Datum, be copied into the specified memory context, and then the Rust type constructed from
    /// that pointer instead.
    ///
    /// ## Safety
    ///
    /// Same caveats as `From::from_datum(...)`
    unsafe fn from_datum_in_memory_context(
        mut memory_context: PgMemoryContexts,
        datum: pg_sys::Datum,
        is_null: bool,
        typoid: pg_sys::Oid,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        memory_context.switch_to(|_| FromDatum::from_datum(datum, is_null, typoid))
    }
}

/// for pg_sys::Datum
impl FromDatum for pg_sys::Datum {
    #[inline]
    unsafe fn from_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        _: pg_sys::Oid,
    ) -> Option<pg_sys::Datum> {
        if is_null {
            None
        } else {
            Some(datum)
        }
    }
}

/// for bool
impl FromDatum for bool {
    #[inline]
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool, _: pg_sys::Oid) -> Option<bool> {
        if is_null {
            None
        } else {
            Some(datum != 0)
        }
    }
}

/// for char
impl FromDatum for i8 {
    #[inline]
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool, _: pg_sys::Oid) -> Option<i8> {
        if is_null {
            None
        } else {
            Some(datum as i8)
        }
    }
}

/// for smallint
impl FromDatum for i16 {
    #[inline]
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool, _: pg_sys::Oid) -> Option<i16> {
        if is_null {
            None
        } else {
            Some(datum as i16)
        }
    }
}

/// for integer
impl FromDatum for i32 {
    #[inline]
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool, _: pg_sys::Oid) -> Option<i32> {
        if is_null {
            None
        } else {
            Some(datum as i32)
        }
    }
}

/// for oid
impl FromDatum for u32 {
    #[inline]
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool, _: pg_sys::Oid) -> Option<u32> {
        if is_null {
            None
        } else {
            Some(datum as u32)
        }
    }
}

/// for bigint
impl FromDatum for i64 {
    #[inline]
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool, _: pg_sys::Oid) -> Option<i64> {
        if is_null {
            None
        } else {
            Some(datum as i64)
        }
    }
}

/// for real
impl FromDatum for f32 {
    #[inline]
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool, _: pg_sys::Oid) -> Option<f32> {
        if is_null {
            None
        } else {
            Some(f32::from_bits(datum as u32))
        }
    }
}

/// for double precision
impl FromDatum for f64 {
    #[inline]
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool, _: pg_sys::Oid) -> Option<f64> {
        if is_null {
            None
        } else {
            Some(f64::from_bits(datum as u64))
        }
    }
}

/// for text, varchar
impl<'a> FromDatum for &'a str {
    #[inline]
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool, _: pg_sys::Oid) -> Option<&'a str> {
        if is_null {
            None
        } else if datum == 0 {
            panic!("a varlena Datum was flagged as non-null but the datum is zero");
        } else {
            let varlena = pg_sys::pg_detoast_datum(datum as *mut pg_sys::varlena);
            Some(text_to_rust_str_unchecked(varlena))
        }
    }

    unsafe fn from_datum_in_memory_context(
        mut memory_context: PgMemoryContexts,
        datum: usize,
        is_null: bool,
        _typoid: u32,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        if is_null {
            None
        } else if datum == 0 {
            panic!("a varlena Datum was flagged as non-null but the datum is zero");
        } else {
            let varlena = pg_sys::pg_detoast_datum(datum as *mut pg_sys::varlena);
            memory_context.switch_to(|_| {
                // this gets the varlena Datum copied into this memory context
                let cstr = pg_sys::text_to_cstring(varlena as *mut pg_sys::text);

                // and now we return it as a &str
                let cstr = std::ffi::CStr::from_ptr(cstr);
                Some(
                    cstr.to_str()
                        .expect("failed to convert varlena datum into &str"),
                )
            })
        }
    }
}

/// for text, varchar, or any `pg_sys::varlena`-based type
///
/// This returns a **copy**, allocated and managed by Rust, of the underlying `varlena` Datum
impl FromDatum for String {
    #[inline]
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool, _: pg_sys::Oid) -> Option<String> {
        if is_null {
            None
        } else if datum == 0 {
            panic!("a varlena Datum was flagged as non-null but the datum is zero");
        } else {
            let varlena = datum as *mut pg_sys::varlena;
            let detoasted = pg_sys::pg_detoast_datum(varlena);
            let len = varsize_any_exhdr(detoasted);
            let data = vardata_any(detoasted);

            let result =
                std::str::from_utf8_unchecked(std::slice::from_raw_parts(data as *mut u8, len))
                    .to_owned();

            if detoasted != varlena {
                pg_sys::pfree(detoasted as void_mut_ptr);
            }

            Some(result)
        }
    }
}

/// for cstring
impl<'a> FromDatum for &'a std::ffi::CStr {
    #[inline]
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool, _: pg_sys::Oid) -> Option<&'a CStr> {
        if is_null {
            None
        } else if datum == 0 {
            panic!("a cstring Datum was flagged as non-null but the datum is zero");
        } else {
            Some(std::ffi::CStr::from_ptr(
                datum as *const std::os::raw::c_char,
            ))
        }
    }
}

/// for NULL -- always converts to a `None`, even if the is_null argument is false
impl FromDatum for () {
    #[inline]
    unsafe fn from_datum(_datum: pg_sys::Datum, _is_null: bool, _: pg_sys::Oid) -> Option<()> {
        None
    }
}

/// for user types
impl<T> FromDatum for PgBox<T> {
    #[inline]
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool, _: pg_sys::Oid) -> Option<PgBox<T>> {
        if is_null {
            None
        } else if datum == 0 {
            panic!(
                "user type {} Datum was flagged as non-null but the datum is zero",
                std::any::type_name::<T>()
            );
        } else {
            Some(PgBox::<T>::from_pg(datum as *mut T))
        }
    }

    unsafe fn from_datum_in_memory_context(
        mut memory_context: PgMemoryContexts,
        datum: usize,
        is_null: bool,
        _typoid: u32,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        memory_context.switch_to(|context| {
            if is_null {
                None
            } else if datum == 0 {
                panic!(
                    "user type {} Datum was flagged as non-null but the datum is zero",
                    std::any::type_name::<T>()
                );
            } else {
                let copied = context.copy_ptr_into(datum as *mut T, std::mem::size_of::<T>());
                Some(PgBox::<T>::from_pg(copied))
            }
        })
    }
}
