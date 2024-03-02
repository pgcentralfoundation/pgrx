//! Traits and implementations of functionality relating to Postgres null
//! and nullable types, particularly in relation to arrays and other container
//! types which implement nullable (empty slot) behavior in a 
//! "structure-of-arrays" manner

use std::marker::PhantomData;

use bitvec::slice::BitSlice;

use crate::{memcx::MemCx, Array, Datum, UnboxDatum};

pub trait NullableIter {

}

pub struct NullableContainerRef<'src> {

    _marker: PhantomData<&'src MemCx<'src>>,
}

pub enum Nullable<T>{
    Valid(T), 
    Null,
}

impl<T> Nullable<T> { 
    pub fn into_option(self) -> Option<T> { 
        match self {
            Nullable::Valid(val) => Some(val),
            Nullable::Null => None,
        }
    }
    pub fn as_option<'a>(&'a self) -> Option<&'a T> {
        match self {
            Nullable::Valid(val) => Some(val),
            Nullable::Null => None,
        }
    }
    pub fn as_option_mut<'a>(&'a mut self) -> Option<&'a T> {
        match self {
            Nullable::Valid(val) => Some(val),
            Nullable::Null => None,
        }
    }
    pub fn is_valid(&self) -> bool {
        match self { 
            Nullable::Valid(_) => true, 
            Nullable::Null => false,
        }
    }
    pub fn is_null(&self) -> bool { 
        !self.is_valid()
    }
    pub fn unwrap(self) -> T { 
        self.into_option().unwrap()
    }
    pub fn expect(self, msg: &'static str) -> T { 
        self.into_option().expect(msg)
    }
    pub fn ok_or<E>(self, err: E) -> Result<T, E> { 
        self.into_option().ok_or(err)
    }
}

impl<T> Into<Option<T>> for Nullable<T> {
    fn into(self) -> Option<T> {
        match self {
            Nullable::Valid(val) => Some(val),
            Nullable::Null => None,
        }
    }
}
impl<T> From<Option<T>> for Nullable<T> {
    fn from(opt: Option<T>) -> Self {
        match opt {
            Some(val) => Self::Valid(val),
            None => Self::Null,
        }
    }
}


/// This type isn't directly related to [`pg_sys::NullableDatum`], but is
/// intended to reference the same underlying data as a pg_sys::NullableDatum
/// after undergoing several layers of PGRX's Rust memory safety.
pub type NullableDatum<'src> = Nullable<Datum<'src>>;


pub struct IterNullableValuesWithBitmap<'src, T> {
    null_slice: &'src mut BitSlice<u8>,
    values: &'src [T],
}

/* A null-bitmap array of (from postgres' perspective) length 8 with 3 nulls
    in it has an actual data array size of `size_of<Elem>() * 5` (give or 
    take padding), whereas the bool array nulls implementation will still 
    have an underlying data array of `size_of<Elem>() * 8` (give or take 
    padding) no matter how many nulls there are. */
// For the bitmap array, 1 is "valid", 0 is "null",
// for the bool array,  1 is "null", 0 is "valid".

/// Represents Postgres internal types which describe the layout of where
/// filled slots (equivalent to Some(T)) are and where nulls (equivalent
/// to None) are in the array.
pub trait NullLayout {
    fn len(&self) -> usize;
    /// For the given (linear) container index, returns the next index of the
    /// underlying data buffer that should contain a valid value,
    /// or `None` if we have reached the end of the container.
    /// 
    /// Used to implement the skipping behavior expected with null-bitmap
    /// -style arrays.
    fn next_valid_idx(&self, idx: usize) -> Option<usize>;

    /// Returns true if this container skips nulls
    /// i.e. if a null takes up 0 bits in the underlying data buffer.
    /* The reason this isn't const is to support implementations like
       AnyNullLayout */
    fn can_skip(&self) -> bool;

    /// Returns true if this container has any nulls in it presently.
    /// # Performance
    /// Different implementors of this type might have wildly varying
    /// performance costs for checking this - for a null bitslice -style
    /// container, it should complete in O(log(n)) at worst, but for a
    /// null boolean array -style implementation, it could be O(n),
    /// where n is the length of the container.
    fn has_nulls(&self) -> bool;

    /// Returns Some(true) if the element at `idx`` is valid (non-null),
    /// or `None` if `idx` is out-of-bounds
    fn is_valid(&self, idx: usize) -> Option<bool>;
    /// Returns true if the element at idx is null.
    fn is_null(&self, idx: usize) -> Option<bool> { 
        self.is_valid(idx).map(|v| !v)
    }
}

/*
pub trait NullableContainer<T> { 
    type LAYOUT : NullLayout;

    fn get_layout(&self) -> &Self::LAYOUT;

    /// Returns Some(true) if the element at `idx`` is valid (non-null),
    /// or `None` if `idx` is out-of-bounds
    fn is_valid(&self, idx: usize) -> Option<bool> { 
        self.get_layout().is_valid(idx)
    }
    /// Returns true if the element at idx is null.
    fn is_null(&self, idx: usize) -> Option<bool> { 
        self.get_layout().is_null(idx)
    }

    /// Retrieve the element at idx.
    /// Returns `Some(Valid(&T))` if the element is valid, `Some(Null)` if the
    /// element is null, or `None` if `idx` is out of bounds.
    fn get(&self, idx: usize) -> Option<Nullable<&T>> {
        match self.get_layout().is_null(idx) {
            Some(true) => {
                todo!("Implement IterNullable and then use it here.")
            },
            Some(false) => {
                Some(Nullable::Valid(self.get_raw(idx)))
            },
            // Out-of-bounds idx, return early.
            None => None,
        }
    }
    
    /// Retrieve a mutable reference to the element at idx.
    ///
    /// Returns `Some(Valid(&mut T))` if the element is valid, `Some(Null)` if
    /// the element is null, or `None` if `idx` is out of bounds.
    /// 
    /// Note that this cannot be used to set the element to Null - the
    /// mutability here is for mutating the inner value if it is valid.
    fn get_mut(&mut self, idx: usize) -> Option<Nullable<&mut T>>  {
        match self.get_layout().is_null(idx) {
            Some(true) => {
                todo!("Implement IterNullableMut and then use it here.")
            },
            Some(false) => {
                Some(Nullable::Valid(self.get_mut_raw(idx)))
            },
            // Out-of-bounds idx, return early.
            None => None,
        }
    }

    /// For internal use - implement this over underlying data types
    /// Used to implement NullableIter.
    ///
    /// Get the Valid value from the underlying data index of `idx`,
    /// presumably after figuring out things like 
    fn get_raw(&self, idx: usize) -> &T;
    fn get_mut_raw(&mut self, idx: usize) -> &mut T;
}*/

pub trait NullableContainer<'mcx, T> {
    /// Returns Some(true) if the element at `idx`` is valid (non-null),
    /// or `None` if `idx` is out-of-bounds
    fn is_valid(&'mcx self, idx: usize) -> Option<bool>;

    /// Returns true if the element at idx is null.
    fn is_null(&'mcx self, idx: usize) -> Option<bool>;

    /// Retrieve the element at idx.
    /// Returns `Some(Valid(&T))` if the element is valid, `Some(Null)` if the
    /// element is null, or `None` if `idx` is out of bounds.
    fn get(&'mcx self, idx: usize) -> Option<Nullable<T>>;
}

/*
pub enum AnyNullLayout<'a> {
    /// Bitmap of where null slots are in this container.
    /// For example, 00001001 would represent: 
    /// \[value, value, value, value, null, value value, null\]
    /// However, the underlying data buffer would be: 
    /// \[value, value, value, value, value, value\]
    /// because of the skip behavior
    Bitmap(&'a BitSlice<u8>),
    // TODO: Find implementation details on this
    BoolSlice(&'a [bool]),
    /// Bool map, simply an array of booleans telling you
    /// No nulls
    Strict(usize),
}

impl<'a> NullLayout for AnyNullLayout<'a> {
    fn len(&self) -> usize { 
        match self {
            AnyNullLayout::Bitmap(bits) => bits.len(),
            AnyNullLayout::BoolSlice(slice) => slice.len(),
            AnyNullLayout::Strict(len) => *len,
        }
    }
    // For iterating over only the valid elements
    // Can return None early if idx is the last Valid elem
    fn next_valid_idx(&self, idx: usize) -> Option<usize> {
        match self {
            AnyNullLayout::Bitmap(bits) => {
                // Next elem (one after this) would be past the end 
                // of the container
                if (idx+1) >= bits.len() { 
                    return None;
                }
                let mut resulting_idx = 0;
                for bit in &(*bits)[(idx+1)..] { 
                    // Postgres nullbitmaps are 1 for "valid" and 0 for "null"
                    resulting_idx += (*bit) as usize;
                }
                Some(resulting_idx)
            },
            AnyNullLayout::BoolSlice(slice) => { 
                // Next elem (one after this) would be past the end 
                // of the container
                if (idx+1) >= slice.len() { 
                    return None;
                }
                for i in (idx+1)..slice.len() { 
                    // SAFETY: This loop is structured such that it should'nt
                    // be possible to go beyond self.len().
                    unsafe { 
                        // for the bool array,  1 is "null", 0 is "valid".
                        if !slice.get_unchecked(i) {
                            return Some(i);
                        }
                    }
                }
                // There may be more nulls, but there are no more Valid(t)-s
                return None;
            },
            AnyNullLayout::Strict(len) => {
                let next_idx = idx+1;
                (next_idx < *len).then_some(next_idx)
            },
        }
    }

    fn can_skip(&self) -> bool {
        match self {
            AnyNullLayout::Bitmap(b) => true,
            AnyNullLayout::BoolSlice(_) => false,
            AnyNullLayout::Strict(_) => false,
        }
    }

    fn has_nulls(&self) -> bool {
        match self {
            AnyNullLayout::Bitmap(bits) => { 
                bits.any()
            },
            AnyNullLayout::BoolSlice(slice) => {
                for elem in *slice {
                    if *elem { 
                        return true;
                    }
                }
                return false;
            },
            AnyNullLayout::Strict(_) => false,
        }
    }

    fn is_valid(&self, idx: usize) -> Option<bool> {
        match *self {
            AnyNullLayout::Bitmap(bits) => bits.get(idx).map(|b| *b),
            // for the bool array,  1 is "null", 0 is "valid".
            AnyNullLayout::BoolSlice(slice) => slice.get(idx).map(|b| !b),
            AnyNullLayout::Strict(len) => (idx < len).then_some(true),
        }
    }
}*/ 

impl<'a> NullLayout for crate::NullKind<'a> {
    fn len(&self) -> usize {
        match self {
            crate::NullKind::Bits(bit_slice) => bit_slice.len(),
            crate::NullKind::Strict(len) => *len,
        }
    }

    fn next_valid_idx(&self, idx: usize) -> Option<usize> {
        match self { 
            crate::NullKind::Bits(bits) => {
                // Next elem (one after this) would be past the end 
                // of the container
                if (idx+1) >= bits.len() { 
                    return None;
                }
                let mut resulting_idx = 0;
                for bit in &(*bits)[(idx+1)..] { 
                    // Postgres nullbitmaps are 1 for "valid" and 0 for "null"
                    resulting_idx += (*bit) as usize;
                }
                Some(resulting_idx)
            },
            crate::NullKind::Strict(len) => {
                let next_idx = idx+1;
                (next_idx < *len).then_some(next_idx)
            },
        }
    }

    fn can_skip(&self) -> bool {
        match self {
            crate::NullKind::Bits(_) => true,
            crate::NullKind::Strict(_) => false,
        }
    }

    fn has_nulls(&self) -> bool {
        self.any()
    }

    fn is_valid(&self, idx: usize) -> Option<bool> {
        match *self {
            crate::NullKind::Bits(bits) => bits.get(idx).map(|b| *b),
            crate::NullKind::Strict(len) => (idx < len).then_some(true),
        }
    }
    fn is_null(&self, idx: usize) -> Option<bool> {
        match *self {
            crate::NullKind::Bits(bits) => bits.get(idx).map(|b| !b),
            crate::NullKind::Strict(len) => (idx < len).then_some(true),
        }
    }
}

#[deny(unsafe_op_in_unsafe_fn)]
impl<'mcx, T: UnboxDatum> NullableContainer<'mcx, T::As<'mcx>> for Array<'mcx, T> {
    fn is_valid(&'mcx self, idx: usize) -> Option<bool> {
        self.null_slice.is_valid(idx).map(|b| !b)
    }

    fn is_null(&'mcx self, idx: usize) -> Option<bool> {
        self.null_slice.is_null(idx).map(|b| !b)
    }

    fn get(&'mcx self, idx: usize) -> Option<Nullable<T::As<'mcx>>> {
        self.get(idx).map(|elem| elem.into())
    }
}