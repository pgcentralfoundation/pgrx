// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.


use crate::*;

pub trait InOutFuncs: serde::de::DeserializeOwned + serde::ser::Serialize {
    fn input(input: &str) -> std::result::Result<Self, String> {
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

/// Decode a Postgres varlena pointer into a Rust type instance
pub unsafe fn from_varlena<T: serde::de::DeserializeOwned>(
    varlena: *const pg_sys::varlena,
) -> serde_cbor::Result<T> {
    let varlena = pg_sys::pg_detoast_datum(varlena as *mut pg_sys::varlena);
    let size = varsize_any(varlena);
    let slice = std::slice::from_raw_parts(varlena as *const u8, size);
    let (_, mut data) = slice.split_at(pg_sys::VARHDRSZ);
    let object = serde_cbor::from_reader(&mut data)?;

    Ok(object)
}

/// Encode a Rust type instance that is `serde::Serialize` into a Postgres varlena pointer
pub fn to_varlena<T: serde::Serialize>(data: &T) -> serde_cbor::Result<*const pg_sys::varlena> {
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
