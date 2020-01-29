use crate::{pg_sys, vardata_any, varsize_any_exhdr, void_mut_ptr, FromDatum, IntoDatum};
use std::ops::Deref;

pub struct OwnedVarlenA<'a> {
    ptr: *mut pg_sys::varlena,
    detoasted: *mut pg_sys::varlena,
    unpacked: *mut pg_sys::varlena,
    str: &'a str,
}

impl<'a> FromDatum<OwnedVarlenA<'a>> for OwnedVarlenA<'a> {
    #[inline]
    unsafe fn from_datum(datum: usize, is_null: bool, _typoid: u32) -> Option<OwnedVarlenA<'a>> {
        if is_null {
            None
        } else if datum == 0 {
            panic!("textp datum flagged as not-null but its datum is zero");
        } else {
            let ptr = datum as *mut pg_sys::varlena;
            let detoasted = pg_sys::pg_detoast_datum(ptr);
            let unpacked = pg_sys::pg_detoast_datum_packed(detoasted);

            let len = varsize_any_exhdr(unpacked);
            let data = vardata_any(unpacked);
            let str =
                std::str::from_utf8_unchecked(std::slice::from_raw_parts(data as *mut u8, len));

            Some(OwnedVarlenA {
                ptr,
                detoasted,
                unpacked,
                str,
            })
        }
    }
}

impl<'a> IntoDatum<OwnedVarlenA<'a>> for OwnedVarlenA<'a> {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(self.ptr as pg_sys::Datum)
    }
}

impl<'a> Deref for OwnedVarlenA<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.str
    }
}

impl<'a> Drop for OwnedVarlenA<'a> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            if self.unpacked != self.detoasted {
                pg_sys::pfree(self.unpacked as void_mut_ptr);
            }
            if self.detoasted != self.ptr {
                pg_sys::pfree(self.detoasted as void_mut_ptr);
            }
            pg_sys::pfree(self.ptr as void_mut_ptr);
        }
    }
}
