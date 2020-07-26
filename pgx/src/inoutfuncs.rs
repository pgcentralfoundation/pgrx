// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

//! Helper trait for the `#[derive(PostgresType)]` proc macro for overriding custom Postgres type
//! input/output functions.
//!
//! The default implementations use `serde_json` to serialize a custom type to human-readable strings,
//! and `serde_cbor` to serialize internally as a `varlena *` for storage on disk.

use crate::*;

pub trait InOutFuncs<'de>: serde::de::Deserialize<'de> + serde::ser::Serialize {
    fn input(input: &'de str) -> std::result::Result<Self, String> {
        match serde_json::from_str(input) {
            Ok(obj) => Ok(obj),
            Err(e) => Err(format!("{}", e)),
        }
    }

    fn output(&self, buffer: &mut StringInfo)
    where
        Self: serde::ser::Serialize,
    {
        serde_json::to_writer(buffer, self).expect("failed to serialize a {} to json")
    }
}

/// Decode am owned Postgres varlena pointer from CBOR into a Rust type instance
pub unsafe fn from_varlena_owned<T: serde::de::DeserializeOwned>(
    varlena: *const pg_sys::varlena,
) -> serde_cbor::Result<T> {
    let varlena = pg_sys::pg_detoast_datum_packed(varlena as *mut pg_sys::varlena);
    let len = varsize_any(varlena);
    let slice = std::slice::from_raw_parts(varlena as *const u8, len);
    let (_, mut data) = slice.split_at(pg_sys::VARHDRSZ);
    serde_cbor::from_reader(&mut data)
}

/// Decode a borrowed Postgres varlena pointer from JSON into a Rust type instance
pub unsafe fn from_varlena_borrowed<'de, T: serde::de::Deserialize<'de>>(
    varlena: *const pg_sys::varlena,
) -> serde_json::Result<T> {
    let varlena = pg_sys::pg_detoast_datum(varlena as *mut pg_sys::varlena);
    let size = varsize_any(varlena);
    let slice = std::slice::from_raw_parts(varlena as *const u8, size);
    let (_, data) = slice.split_at(pg_sys::VARHDRSZ);
    serde_json::from_slice(data)
}

/// Encode a Rust type containing only owned values that is `serde::Serialize` into a Postgres
/// varlena pointer as CBOR
pub fn to_varlena_owned<T: serde::Serialize>(
    data: &T,
) -> serde_cbor::Result<*const pg_sys::varlena> {
    let mut serialized = StringInfo::new();

    serialized.push_bytes(&[0u8; pg_sys::VARHDRSZ]); // reserve space fo the header
    serde_cbor::to_writer(&mut serialized, data)?;

    let size = serialized.len() as usize;
    let varlena = serialized.into_char_ptr();
    unsafe {
        set_varsize(varlena as *mut pg_sys::varlena, size as i32);
    }

    Ok(varlena as *const pg_sys::varlena)
}

/// Encode a Rust type containing at least one borrowed value that is `serde::Serialize` into a Postgres
/// varlena pointer as JSON
pub fn to_varlena_borrowed<T: serde::Serialize>(
    data: &T,
) -> serde_json::Result<*const pg_sys::varlena> {
    let mut serialized = StringInfo::new();

    serialized.push_bytes(&[0u8; pg_sys::VARHDRSZ]); // reserve space fo the header
    serde_json::to_writer(&mut serialized, data)?;

    let size = serialized.len() as usize;
    let varlena = serialized.into_char_ptr();
    unsafe {
        set_varsize(varlena as *mut pg_sys::varlena, size as i32);
    }

    Ok(varlena as *const pg_sys::varlena)
}
