// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use crate::{pg_sys, vardata_any, varsize_any_exhdr, void_mut_ptr, FromDatum, IntoDatum};
use serde::{Serialize, Serializer};
use std::ops::Deref;

pub struct DetoastedVarlenA<'a> {
    ptr: *mut pg_sys::varlena,
    detoasted: *mut pg_sys::varlena,
    str: &'a str,
    drop_ptr: bool,
}

pub struct OwnedVarlenA<'a> {
    detoasted: DetoastedVarlenA<'a>,
}

impl<'a> DetoastedVarlenA<'a> {
    pub fn as_str(&self) -> &'a str {
        self.str
    }
}

impl<'a> OwnedVarlenA<'a> {
    pub fn as_str(&self) -> &'a str {
        self.detoasted.str
    }
}

impl<'a> FromDatum for DetoastedVarlenA<'a> {
    #[inline]
    unsafe fn from_datum(datum: usize, is_null: bool, _: u32) -> Option<DetoastedVarlenA<'a>> {
        if is_null {
            None
        } else if datum == 0 {
            panic!("textp datum flagged as not-null but its datum is zero");
        } else {
            let ptr = datum as *mut pg_sys::varlena;
            let detoasted = pg_sys::pg_detoast_datum_packed(ptr);

            let len = varsize_any_exhdr(detoasted);
            let data = vardata_any(detoasted);
            let str =
                std::str::from_utf8_unchecked(std::slice::from_raw_parts(data as *mut u8, len));

            Some(DetoastedVarlenA {
                ptr,
                detoasted,
                str,
                drop_ptr: false,
            })
        }
    }
}

impl<'a> FromDatum for OwnedVarlenA<'a> {
    #[inline]
    unsafe fn from_datum(datum: usize, is_null: bool, typoid: u32) -> Option<OwnedVarlenA<'a>> {
        if is_null {
            None
        } else if datum == 0 {
            panic!("textp datum flagged as not-null but its datum is zero");
        } else {
            match DetoastedVarlenA::from_datum(datum, is_null, typoid) {
                Some(mut detoasted) => {
                    detoasted.drop_ptr = true;
                    Some(OwnedVarlenA { detoasted })
                }
                None => None,
            }
        }
    }
}

impl<'a> IntoDatum for OwnedVarlenA<'a> {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(self.detoasted.ptr as pg_sys::Datum)
    }

    fn type_oid() -> u32 {
        pg_sys::TEXTOID
    }
}

impl<'a> IntoDatum for DetoastedVarlenA<'a> {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(self.ptr as pg_sys::Datum)
    }

    fn type_oid() -> u32 {
        pg_sys::TEXTOID
    }
}

impl<'a> Deref for OwnedVarlenA<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.detoasted.str
    }
}

impl<'a> Deref for DetoastedVarlenA<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.str
    }
}

impl<'a> Serialize for OwnedVarlenA<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.detoasted.str)
    }
}

impl<'a> Serialize for DetoastedVarlenA<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.str)
    }
}

impl<'a> Drop for DetoastedVarlenA<'a> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            if self.detoasted != self.ptr {
                pg_sys::pfree(self.detoasted as void_mut_ptr);
            }

            if self.drop_ptr {
                pg_sys::pfree(self.ptr as void_mut_ptr);
            }
        }
    }
}
