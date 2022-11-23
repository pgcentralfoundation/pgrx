/*!
Code for interfacing with the layout in memory (and on disk?) of various data types within Postgres.
This is not a mature module yet so prefer to avoid exposing the contents to public callers of pgx,
as this is error-prone stuff to tamper with. It is easy to corrupt the database if an error is made.
Yes, even though its main block of code duplicates htup::DatumWithTypeInfo.

Technically, they can always use CFFI to corrupt the database if they know how to use the C API,
but that's why we should keep the conversation between the programmer and their own `unsafe`.
We don't want programmers trusting our implementation farther than we have solidly evaluated it.
Some may be better off extending the pgx bindings themselves, doing their own layout checks, etc.
When PGX is a bit more mature in its soundness, and we better understand what our callers expect,
then we may want to offer more help.
*/
use crate::pg_sys::{self, TYPALIGN_CHAR, TYPALIGN_DOUBLE, TYPALIGN_INT, TYPALIGN_SHORT};
use core::mem;

/// Postgres type information, corresponds to part of a row in pg_type
/// This layout describes T, not &T, even if passbyval: false, which would mean the datum array is effectively &[&T]
#[derive(Clone, Copy, Debug)]
pub(crate) struct Layout {
    // We could add more fields to this if we are curious enough, as the function we call pulls an entire row
    pub align: Align, // typalign
    pub size: Size,   // typlen
    pub pass: PassBy, // typbyval
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum PassBy {
    Ref,
    Value,
}

impl Layout {
    /**
    Give an Oid and get its Layout.

    # Panics

    May elog if the tuple for the type in question cannot be acquired.
    This should almost never happen in practice (alloc error?).
    */
    pub(crate) fn lookup_oid(oid: pg_sys::Oid) -> Layout {
        let (mut typalign, mut typlen, mut passbyval) = Default::default();
        // Postgres doesn't document any safety conditions. It probably is a safe function in the Rust sense.
        unsafe { pg_sys::get_typlenbyvalalign(oid, &mut typlen, &mut passbyval, &mut typalign) };
        Layout {
            align: Align::try_from(typalign).unwrap(),
            size: Size::try_from(typlen).unwrap(),
            pass: if passbyval { PassBy::Value } else { PassBy::Ref },
        }
    }

    // Attempt to discern if a given Postgres and Rust layout are "matching" in some sense
    // Some(usize) if they seem to agree on one, None if they do not
    pub(crate) fn size_matches<T>(&self) -> Option<usize> {
        const DATUM_SIZE: usize = mem::size_of::<pg_sys::Datum>();
        match (self.pass, mem::size_of::<T>(), self.size.try_as_usize()) {
            (PassBy::Value, rs @ (1 | 2 | 4 | 8), Some(pg @ (1 | 2 | 4 | 8))) if rs == pg => {
                Some(rs)
            }
            (PassBy::Value, _, _) => None,
            (PassBy::Ref, DATUM_SIZE, _) => Some(DATUM_SIZE),
            (PassBy::Ref, _, _) => None,
        }
    }
}

#[repr(usize)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Align {
    Byte = mem::align_of::<u8>(),
    Short = mem::align_of::<libc::c_short>(),
    Int = mem::align_of::<libc::c_int>(),
    Double = mem::align_of::<f64>(),
}

impl TryFrom<libc::c_char> for Align {
    type Error = ();

    fn try_from(cchar: libc::c_char) -> Result<Align, ()> {
        match cchar as u8 {
            TYPALIGN_CHAR => Ok(Align::Byte),
            TYPALIGN_SHORT => Ok(Align::Short),
            TYPALIGN_INT => Ok(Align::Int),
            TYPALIGN_DOUBLE => Ok(Align::Double),
            _ => Err(()),
        }
    }
}

impl Align {
    pub(crate) fn as_typalign(&self) -> libc::c_char {
        (match self {
            Align::Byte => TYPALIGN_CHAR,
            Align::Short => TYPALIGN_SHORT,
            Align::Int => TYPALIGN_INT,
            Align::Double => TYPALIGN_DOUBLE,
        }) as _
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Size {
    CStr,
    Varlena,
    Fixed(u16),
}

impl TryFrom<i16> for Size {
    type Error = ();
    fn try_from(int2: i16) -> Result<Size, ()> {
        match int2 {
            -2 => Ok(Size::CStr),
            -1 => Ok(Size::Varlena),
            v @ 0.. => Ok(Size::Fixed(v as u16)),
            _ => Err(()),
        }
    }
}

impl Size {
    pub(crate) fn as_typlen(&self) -> i16 {
        match self {
            Self::CStr => -2,
            Self::Varlena => -1,
            Self::Fixed(v) => *v as _,
        }
    }

    pub(crate) fn try_as_usize(&self) -> Option<usize> {
        match self {
            Self::CStr => None,
            Self::Varlena => None,
            Self::Fixed(v) => Some(*v as _),
        }
    }
}
