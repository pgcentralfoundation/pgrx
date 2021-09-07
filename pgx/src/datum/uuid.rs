use crate::{pg_sys, FromDatum, IntoDatum, PgMemoryContexts};
use core::fmt::Write;
use std::ops::{Deref, DerefMut};

const UUID_BYTES_LEN: usize = 16;
pub type UuidBytes = [u8; UUID_BYTES_LEN];

/// A Universally Unique Identifier (UUID).
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd, Debug)]
#[repr(transparent)]
pub struct Uuid(UuidBytes);

impl IntoDatum for Uuid {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        let ptr = PgMemoryContexts::CurrentMemoryContext.palloc_slice::<u8>(UUID_BYTES_LEN);
        ptr.clone_from_slice(&self.0);

        Some(ptr.as_ptr() as pg_sys::Datum)
    }

    #[inline]
    fn type_oid() -> u32 {
        pg_sys::UUIDOID
    }
}

impl FromDatum for Uuid {
    #[inline]
    unsafe fn from_datum(datum: usize, is_null: bool, _typoid: pg_sys::Oid) -> Option<Uuid> {
        if is_null {
            None
        } else if datum == 0 {
            panic!("a uuid Datum as flagged as non-null but the datum is zero");
        } else {
            let bytes = std::slice::from_raw_parts(datum as *const u8, UUID_BYTES_LEN);
            if let Ok(uuid) = Uuid::from_slice(bytes) {
                Some(uuid)
            } else {
                None
            }
        }
    }
}

enum UuidFormatCase {
    Lowercase,
    Uppercase,
}

impl Uuid {
    pub fn from_bytes(b: UuidBytes) -> Self {
        Uuid(b)
    }

    pub const fn as_bytes(&self) -> &UuidBytes {
        &self.0
    }

    pub fn from_slice(b: &[u8]) -> Result<Uuid, String> {
        let len = b.len();

        if len != UUID_BYTES_LEN {
            Err(format!(
                "Expected UUID to be {} bytes, got {}",
                UUID_BYTES_LEN, len
            ))?;
        }

        let mut bytes = [0; UUID_BYTES_LEN];
        bytes.copy_from_slice(b);
        Ok(Uuid::from_bytes(bytes))
    }

    fn format(&self, f: &mut std::fmt::Formatter<'_>, case: UuidFormatCase) -> std::fmt::Result {
        let hyphenated = f.sign_minus();
        for (i, b) in self.0.iter().enumerate() {
            if hyphenated && (i == 4 || i == 6 || i == 8 || i == 10) {
                f.write_char('-')?;
            }
            match case {
                UuidFormatCase::Lowercase => write!(f, "{:02x}", b)?,
                UuidFormatCase::Uppercase => write!(f, "{:02X}", b)?,
            };
        }
        Ok(())
    }
}

impl Deref for Uuid {
    type Target = UuidBytes;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Uuid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for Uuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:-x}", self)
    }
}

impl<'a> std::fmt::LowerHex for Uuid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        self.format(f, UuidFormatCase::Lowercase)
    }
}

impl<'a> std::fmt::UpperHex for Uuid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        self.format(f, UuidFormatCase::Uppercase)
    }
}
