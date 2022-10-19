// Polyfill while #![feature(strict_provenance)] is unstable
#[cfg(not(nightly))]
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
/// This type need not be exported unless the details of the type idiom become important.
// This struct uses a Rust idiom invented before `extern type` was designed,
// but should probably be replaced when #![feature(extern_type)] stabilizes
#[repr(C)]
struct DatumBlob {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

/// Datum is an abstract value that is effectively a union of all scalar types
/// and all possible pointers in a Postgres context. That is, it is either
/// "pass-by-value" (if the value fits into the platform's `uintptr_t`) or
/// "pass-by-reference" (if it does not).
///
/// In Rust, it is best to treat this largely as a pointer while passing it around
/// for code that doesn't care about what the Datum "truly is".
/// If for some reason it is important to manipulate the address/value
/// without "knowing the type" of the Datum, cast to a pointer and use pointer methods.
///
/// Only create Datums from non-pointers when you know you want to pass a value, as
/// it is erroneous for `unsafe` code to dereference the address of "only a value" as a pointer.
/// It is still a "safe" operation to create such pointers: validity is asserted by dereferencing,
/// **or by creating a safe reference such as &T or &mut T**. Also be aware that the validity
/// of Datum's Copy is premised on the same implicit issues with pointers being Copy:
/// while any `&T` is live, other `*mut T` must not be used to write to that `&T`,
/// and `&mut T` implies no other `*mut T` even exists outside an `&mut T`'s borrowing ancestry.
/// It is thus of dubious soundness for Rust code to receive `*mut T`, create another `*mut T`,
/// cast the first to `&mut T`, and then later try to use the second `*mut T` to write.
/// It _is_ sound for Postgres itself to pass a copied pointer as a Datum to Rust code, then later
/// to mutate that data through its original pointer after Rust creates and releases a `&mut T`.
///
/// For all intents and purposes, Postgres counts as `unsafe` code that may be relying
/// on you communicating pointers correctly to it. Do not play games with your database.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Datum(*mut DatumBlob);

impl Datum {
    /// Assume the datum is a value and extract the bits from
    /// the memory address, interpreting them as an integer.
    pub fn value(self) -> usize {
        #[allow(unstable_name_collisions)]
        self.0.addr()
    }

    /// True if the datum is equal to the null pointer.
    pub fn is_null(self) -> bool {
        self.0.is_null()
    }

    /// Assume the datum is a pointer and cast it to point to T.
    /// It is recommended to explicitly use `datum.cast_mut_ptr::<T>()`.
    pub fn cast_mut_ptr<T>(self) -> *mut T {
        #[allow(unstable_name_collisions)]
        self.0.cast()
    }
}

impl From<usize> for Datum {
    fn from(val: usize) -> Datum {
        #[allow(unstable_name_collisions)]
        Datum(NonNull::<DatumBlob>::dangling().as_ptr().with_addr(val))
    }
}

impl From<Datum> for usize {
    fn from(val: Datum) -> usize {
        #[allow(unstable_name_collisions)]
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

impl From<bool> for Datum {
    fn from(val: bool) -> Datum {
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

/// This struct consists of a Datum and a bool, matching Postgres's definition
/// as of Postgres 12. This isn't efficient in terms of storage size, due to padding,
/// but sometimes it's more cache-friendly, so sometimes it is the preferred type.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NullableDatum {
    pub value: Datum,
    pub isnull: bool,
}

impl TryFrom<NullableDatum> for Datum {
    type Error = ();

    fn try_from(nd: NullableDatum) -> Result<Datum, ()> {
        let NullableDatum { value, isnull } = nd;
        if isnull {
            Err(())
        } else {
            Ok(value)
        }
    }
}

impl From<NullableDatum> for Option<Datum> {
    fn from(nd: NullableDatum) -> Option<Datum> {
        Some(Datum::try_from(nd).ok()?)
    }
}
