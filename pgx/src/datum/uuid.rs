use crate::{pg_sys, FromDatum, IntoDatum, PgMemoryContexts};
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

impl Uuid {
    pub fn from_bytes(b: UuidBytes) -> Self {
        Uuid(b)
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
