//! Traits and implementations of functionality relating to Postgres null
//! and nullable types, particularly in relation to arrays and other container
//! types which implement nullable (empty slot) behavior in a
//! "structure-of-arrays" manner

use std::{fmt::Debug, iter::Enumerate, marker::PhantomData};

use bitvec::slice::BitSlice;

use crate::Datum;

pub enum Nullable<T> {
    Valid(T),
    Null,
}

impl<T> Debug for Nullable<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Valid(value) => f.debug_tuple("Valid").field(value).finish(),
            Self::Null => write!(f, "Null"),
        }
    }
}

impl<T> Clone for Nullable<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Self::Valid(arg0) => Self::Valid(arg0.clone()),
            Self::Null => Self::Null,
        }
    }
}

impl<T> PartialEq for Nullable<T>
where
    T: Eq,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Valid(l_value), Self::Valid(r_value)) => l_value == r_value,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl<T> Eq for Nullable<T> where T: PartialEq + Eq {}

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
/// Note that a NullLayout must also capture the length of the container it
/// refers to.
pub trait NullLayout<Idx>
where
    Idx: PartialEq + PartialOrd,
{
    fn len(&self) -> Idx;

    /// Returns true if this container has any nulls in it presently.
    /// # Performance
    /// Different implementors of this type might have wildly varying
    /// performance costs for checking this - for a null bitslice -style
    /// container, it should complete in O(log(n)) at worst, but for a
    /// null boolean array -style implementation, it could be O(n),
    /// where n is the length of the container.
    fn has_nulls(&self) -> bool;

    fn count_nulls(&self) -> usize;

    /// Returns Some(true) if the element at `idx`` is valid (non-null),
    /// or `None` if `idx` is out-of-bounds
    /// Implementors should handle bounds-checking.
    fn is_valid(&self, idx: Idx) -> Option<bool>;
    /// Returns true if the element at idx is null,
    /// or `None` if `idx` is out-of-bounds
    /// Implementors should handle bounds-checking.
    fn is_null(&self, idx: Idx) -> Option<bool> {
        self.is_valid(idx).map(|v| !v)
    }
}

pub trait SkippingNullLayout<Idx>: NullLayout<Idx>
where
    Idx: PartialEq + PartialOrd,
{
    /// For the given (linear) container index, returns the next index of the
    /// underlying data buffer that should contain a valid value,
    /// or `None` if we have reached the end of the container.
    ///
    /// Used to implement the skipping behavior expected with null-bitmap
    /// -style arrays.
    fn next_valid_idx(&self, idx: Idx) -> Option<Idx>;
}

/// All non-skipping null layouts. Marker trait for nullable containers that
/// are nonetheless safe to iterate over linearly.
pub trait ContiguousNullLayout<Idx>: NullLayout<Idx>
where
    Idx: PartialEq + PartialOrd,
{
}

pub trait NullableContainer<'mcx, Idx, T>
where
    Idx: PartialEq + PartialOrd,
{
    type Layout: NullLayout<Idx>;

    fn get_layout(&'mcx self) -> &'mcx Self::Layout;

    /// For internal use - implement this over underlying data types
    /// Used to implement NullableIter.
    ///
    /// Get the Valid value from the underlying data index of `idx`,
    /// presumably after figuring out things like
    fn get_raw(&'mcx self, idx: usize) -> T;
}

pub struct BitSliceNulls<'a>(pub &'a BitSlice<u8>);

impl<'a> NullLayout<usize> for BitSliceNulls<'a> {
    fn len(&self) -> usize {
        BitSlice::<u8>::len(&self.0)
    }

    fn has_nulls(&self) -> bool {
        self.0.not_all()
    }

    fn is_valid(&self, idx: usize) -> Option<bool> {
        self.0.get(idx).map(|b| *b)
    }
    fn is_null(&self, idx: usize) -> Option<bool> {
        self.0.get(idx).map(|b| !b)
    }

    fn count_nulls(&self) -> usize {
        self.0.count_zeros()
    }
}

impl<'a> SkippingNullLayout<usize> for BitSliceNulls<'a> {
    fn next_valid_idx(&self, idx: usize) -> Option<usize> {
        // Next elem (one after this) would be past the end
        // of the container
        if (idx + 1) >= self.0.len() {
            return None;
        }
        let mut resulting_idx = 0;
        for bit in &(*self.0)[(idx + 1)..] {
            // Postgres nullbitmaps are 1 for "valid" and 0 for "null"
            resulting_idx += (*bit) as usize;
        }
        Some(resulting_idx)
    }
}

pub struct BoolSliceNulls<'a>(pub &'a [bool]);

impl<'a> NullLayout<usize> for BoolSliceNulls<'a> {
    fn len(&self) -> usize {
        <[bool]>::len(&self.0)
    }

    fn has_nulls(&self) -> bool {
        let mut has_null = false;
        for value in self.0 {
            if *value {
                has_null = true;
            }
        }
        has_null
    }
    // In a Postgres bool-slice implementation of a null layout,
    // 1 is "null", 0 is "valid"
    fn is_valid(&self, idx: usize) -> Option<bool> {
        if idx < self.0.len() {
            // Invert from 0 being valid to 1 being valid.
            Some(!self.0[idx])
        } else {
            None
        }
    }

    fn count_nulls(&self) -> usize {
        let mut count = 0;
        for elem in self.0 {
            count += *elem as usize;
        }
        count
    }
}

// Postgres arrays using a bool array to map which values are null and which
// values are valid will always be contiguous - that is, the underlying data
// buffer will actually be big enough to contain layout.len() slots of type T
// (give or take padding)
impl<'a> ContiguousNullLayout<usize> for BoolSliceNulls<'a> {}

/// Strict i.e. no nulls.
/// Useful for using nullable primitives on non-null structures,
/// especially when this needs to be determined at runtime.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
pub struct StrictNulls(pub usize);

impl NullLayout<usize> for StrictNulls {
    fn len(&self) -> usize {
        self.0
    }

    fn has_nulls(&self) -> bool {
        false
    }

    fn is_valid(&self, idx: usize) -> Option<bool> {
        (idx < self.0).then_some(true)
    }

    fn is_null(&self, idx: usize) -> Option<bool> {
        (idx < self.0).then_some(false)
    }

    fn count_nulls(&self) -> usize {
        0
    }
}
// No skipping when there are no nulls.
impl ContiguousNullLayout<usize> for StrictNulls {}

// Intended to represent any one of the three nullable container layouts used
// by Postgres.
pub enum AnyNullLayout<'a> {
    /// Bitmap of where null slots are in this container.
    /// For example, 00001001 would represent:
    /// \[value, value, value, value, null, value value, null\]
    /// However, the underlying data buffer would be:
    /// \[value, value, value, value, value, value\]
    /// because of the skip behavior
    Bitmap(BitSliceNulls<'a>),
    // TODO: Find implementation details on this
    BoolSlice(BoolSliceNulls<'a>),
    /// Bool map, simply an array of booleans telling you
    /// No nulls
    Strict(StrictNulls),
}

impl<'a> NullLayout<usize> for AnyNullLayout<'a> {
    fn len(&self) -> usize {
        match self {
            AnyNullLayout::Bitmap(bits) => bits.len(),
            AnyNullLayout::BoolSlice(bools) => bools.len(),
            AnyNullLayout::Strict(strict) => strict.len(),
        }
    }

    fn has_nulls(&self) -> bool {
        match self {
            AnyNullLayout::Bitmap(bits) => bits.has_nulls(),
            AnyNullLayout::BoolSlice(bools) => bools.has_nulls(),
            AnyNullLayout::Strict(_strict) => false,
        }
    }

    fn is_valid(&self, idx: usize) -> Option<bool> {
        match self {
            AnyNullLayout::Bitmap(bits) => bits.is_valid(idx),
            AnyNullLayout::BoolSlice(bools) => bools.is_valid(idx),
            AnyNullLayout::Strict(strict) => strict.is_valid(idx),
        }
    }

    fn is_null(&self, idx: usize) -> Option<bool> {
        match self {
            AnyNullLayout::Bitmap(bits) => bits.is_null(idx),
            AnyNullLayout::BoolSlice(bools) => bools.is_null(idx),
            AnyNullLayout::Strict(strict) => strict.is_null(idx),
        }
    }

    fn count_nulls(&self) -> usize {
        match self {
            AnyNullLayout::Bitmap(bits) => bits.count_nulls(),
            AnyNullLayout::BoolSlice(bools) => bools.count_nulls(),
            AnyNullLayout::Strict(strict) => strict.count_nulls(),
        }
    }
}

/// Iterates over a layout of null values, returning true for values that are null.
pub struct NullsIter<'a, Idx, Layout: NullLayout<Idx>>
where
    Idx: PartialEq + PartialOrd,
{
    layout: &'a Layout,
    current: Idx,
}

impl<'mcx, Layout: NullLayout<usize>> Iterator for NullsIter<'mcx, usize, Layout> {
    type Item = Nullable<()>;

    fn next(&mut self) -> Option<Self::Item> {
        // is_null() should handle bounds checking internally
        let result_value = self.layout.is_null(self.current).map(|b| match b {
            false => Nullable::Valid(()),
            true => Nullable::Null,
        });
        self.current += 1;
        result_value
    }
}

/// Iterator for Strict containers,
/// which is to say, containers which cannot contain nulls
pub struct NoNullsIter {
    /// Will be set to container.len() and so a 0 value means we've reached the end.
    remaining: usize,
}

impl Iterator for NoNullsIter {
    type Item = Nullable<()>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining > 0 {
            self.remaining -= 1;
            Some(Nullable::Valid(()))
        } else {
            None
        }
    }
}

pub struct SkippingNullableIterator<'mcx, T, A, I>
where
    A: NullableContainer<'mcx, usize, T>,
    I: Iterator<Item = Nullable<()>>,
{
    nulls: I,
    container_ref: &'mcx A,
    valid_idx: usize,
    // Required to keep a reference to NullableContainer<T>
    __marker: PhantomData<T>,
}

// SKIPPING (i.e. non-contiguous) implementation
impl<'mcx, T, A, I> Iterator for SkippingNullableIterator<'mcx, T, A, I>
where
    A: NullableContainer<'mcx, usize, T>,
    A::Layout: SkippingNullLayout<usize>,
    I: Iterator<Item = Nullable<()>>,
{
    type Item = Nullable<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.nulls.next() {
            Some(is_null) => match is_null {
                Nullable::Valid(_) => {
                    let value = self.container_ref.get_raw(self.valid_idx);
                    self.valid_idx += 1;
                    Some(Nullable::Valid(value))
                }
                Nullable::Null => Some(Nullable::Null),
            },
            // End of array (valid array will have a null layout length equal
            // to the array length)
            None => None,
        }
    }
}

pub struct ContiguousNullableIterator<'mcx, T, A, I>
where
    A: NullableContainer<'mcx, usize, T>,
    I: Iterator<Item = Nullable<()>>,
{
    nulls: Enumerate<I>,
    container_ref: &'mcx A,
    // Required to keep a reference to NullableContainer<T>
    __marker: PhantomData<T>,
}

// Contiguous (length of null layout = length of underlying) implementation
impl<'mcx, T, A, I> Iterator for ContiguousNullableIterator<'mcx, T, A, I>
where
    A: NullableContainer<'mcx, usize, T>,
    A::Layout: ContiguousNullLayout<usize>,
    I: Iterator<Item = Nullable<()>>,
{
    type Item = Nullable<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.nulls.next() {
            Some((idx, is_null)) => match is_null {
                Nullable::Valid(_) => {
                    let value = self.container_ref.get_raw(idx);
                    Some(Nullable::Valid(value))
                }
                Nullable::Null => Some(Nullable::Null),
            },
            // End of array (valid array will have a null layout length equal
            // to the array length)
            None => None,
        }
    }
}

impl<'a, 'b> IntoIterator for &'b BitSliceNulls<'a> {
    type Item = Nullable<()>;

    type IntoIter = NullsIter<'b, usize, BitSliceNulls<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        NullsIter { layout: self, current: 0 }
    }
}

impl<'a, 'b> IntoIterator for &'b BoolSliceNulls<'a> {
    type Item = Nullable<()>;

    type IntoIter = NullsIter<'b, usize, BoolSliceNulls<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        NullsIter { layout: self, current: 0 }
    }
}

impl<'a> IntoIterator for &'a StrictNulls {
    type Item = Nullable<()>;

    type IntoIter = NoNullsIter;

    fn into_iter(self) -> Self::IntoIter {
        NoNullsIter { remaining: self.0 }
    }
}

impl<'a, 'b> IntoIterator for &'b AnyNullLayout<'a> {
    type Item = Nullable<()>;

    type IntoIter = NullsIter<'b, usize, AnyNullLayout<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        NullsIter { layout: self, current: 0 }
    }
}

#[cfg(test)]
mod nullable_tests {}
