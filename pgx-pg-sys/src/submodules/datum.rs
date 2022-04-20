// Polyfill while #![feature(strict_provenance)] is unstable
use sptr::Strict;
use std::ptr::NonNull;

/// Postgres defines the "Datum" type as uintptr_t, so bindgen decides it is usize.
/// Normally, this would be fine, except Postgres uses it more like void*:
/// A pointer to anything that could mean anything, check your assumptions before using.
///
///
/// Accordingly, the "Datum" type from bindgen is not entirely correct, as
/// Rust's `usize` may match the size of `uintptr_t` but it is not quite the same.
/// The compiler would rather know which integers are integers and which are pointers.
/// As a result, Datum is now a wrapper around `*mut DatumBlob`.
// This struct uses a Rust idiom invented before `extern type` was designed,
// but should probably be replaced when #![feature(extern_type)] stabilizes
#[repr(C)]
pub struct DatumBlob {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

/// Datum is an abstract value that is effectively a union of all scalar types
/// and all possible pointers in a Postgres context. That is, it is either
/// "pass-by-value" (if the value fits into the platform's `uintptr_t`) or
/// "pass-by-reference" (if it does not).
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Datum(*mut DatumBlob);

impl Datum {
    /// Assume the datum is a value and extract the bits from
    /// the memory address, interpreting them as an integer.
    pub fn value(self) -> usize {
        self.0.addr()
    }

    /// Assume the datum is a pointer and treat it as void*.
    pub fn to_void(self) -> *mut core::ffi::c_void {
        self.0.cast()
    }

    /// True if the datum is equal to the null pointer.
    pub fn is_null(self) -> bool {
        self.0.is_null()
    }

    /// Assume the datum is a pointer and cast it to point to T.
    pub fn ptr_cast<T>(self) -> *mut T {
        self.0.cast()
    }
}

impl From<usize> for Datum {
    fn from(val: usize) -> Datum {
        Datum(NonNull::<DatumBlob>::dangling().as_ptr().with_addr(val))
    }
}

impl From<Datum> for usize {
    fn from(val: Datum) -> usize {
        val.0.addr()
    }
}

impl From<isize> for Datum {
    fn from(val: isize) -> Datum {
        Datum::from(val as usize)
    }
}

impl From<u8> for Datum {
    fn from(val: u8) -> Datum {
        Datum::from(usize::from(val))
    }
}

impl From<u16> for Datum {
    fn from(val: u16) -> Datum {
        Datum::from(usize::from(val))
    }
}

impl From<u32> for Datum {
    fn from(val: u32) -> Datum {
        Datum::from(val as usize)
    }
}

impl From<u64> for Datum {
    fn from(val: u64) -> Datum {
        Datum::from(val as usize)
    }
}

impl From<i8> for Datum {
    fn from(val: i8) -> Datum {
        Datum::from(isize::from(val))
    }
}

impl From<i16> for Datum {
    fn from(val: i16) -> Datum {
        Datum::from(isize::from(val))
    }
}

impl From<i32> for Datum {
    fn from(val: i32) -> Datum {
        Datum::from(val as usize)
    }
}

impl From<i64> for Datum {
    fn from(val: i64) -> Datum {
        Datum::from(val as usize)
    }
}

impl<T> From<*mut T> for Datum {
    fn from(val: *mut T) -> Datum {
        Datum(val.cast())
    }
}

impl<T> From<*const T> for Datum {
    fn from(val: *const T) -> Datum {
        Datum(val as *mut _)
    }
}

impl<T> PartialEq<*mut T> for Datum {
    fn eq(&self, other: &*mut T) -> bool {
        &self.0.cast() == other
    }
}

impl<T> PartialEq<Datum> for *mut T {
    fn eq(&self, other: &Datum) -> bool {
        self == &other.0.cast()
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NullableDatum {
    pub value: Datum,
    pub isnull: bool,
}
