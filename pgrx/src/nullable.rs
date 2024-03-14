//! Traits and implementations of functionality relating to Postgres null
//! and nullable types, particularly in relation to arrays and other container
//! types which implement nullable (empty slot) behavior in a
//! "structure-of-arrays" manner

use std::{fmt::Debug, iter::Enumerate, marker::PhantomData};
use bitvec::slice::BitSlice;

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
    fn get_raw(&'mcx self, idx: Idx) -> T;

    /// Total array length - doesn't represent the length of the backing array
    /// (as in, the thing accessed by get_raw) but instead represents the
    /// length of the array including skipped nulls.
    fn len(&'mcx self) -> usize;

    /// Represents the length of the underlying array - as in, the container
    /// from which [`NullableContainer::get_raw()`] pulls.
    fn raw_len(&'mcx self) -> usize;
}

pub struct BitSliceNulls<'a>(pub &'a BitSlice<u8>);

impl<'a> NullLayout<usize> for BitSliceNulls<'a> {
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

pub struct MaybeStrictNulls<Inner: NullLayout<usize>> { 
    pub inner: Option<Inner>,
    // Needed for bounds-checking
    pub len: usize,
}

impl<'mcx, Inner> NullLayout<usize> for MaybeStrictNulls<Inner>
        where Inner: NullLayout<usize> {
    fn has_nulls(&self) -> bool {
        match self.inner.as_ref() {
            Some(inner) => inner.has_nulls(),
            None => false,
        }
    }

    fn count_nulls(&self) -> usize {
        match self.inner.as_ref() { 
            Some(inner) => inner.count_nulls(),
            None => 0,
        }
    }

    fn is_valid(&self, idx: usize) -> Option<bool> {
        match self.inner.as_ref() { 
            Some(inner) => inner.is_valid(idx),
            None => (idx < self.len).then_some(true),
        }
    }

    fn is_null(&self, idx: usize) -> Option<bool> {
        match self.inner.as_ref() { 
            Some(inner) => inner.is_null(idx),
            None => (idx < self.len).then_some(false),
        }
    }
}

impl<Inner> SkippingNullLayout<usize> for MaybeStrictNulls<Inner> 
        where Inner: SkippingNullLayout<usize> { 
    fn next_valid_idx(&self, idx: usize) -> Option<usize> {
        match self.inner.as_ref() { 
            Some(inner) => inner.next_valid_idx(idx),
            None => ((idx+1) < self.len).then_some(idx+1)
        }
    }
}

pub struct ContiguousNullableIter<'mcx, T, Container>
        where Container: NullableContainer<'mcx, usize, T>,
        Container::Layout : ContiguousNullLayout<usize> {
    container: &'mcx Container,
    current_idx: usize,
    _marker: PhantomData<T>,
}

impl<'mcx, T: 'mcx, Container> Iterator for ContiguousNullableIter<'mcx, T, Container>
        where Container: NullableContainer<'mcx, usize, T>,
        Container::Layout : ContiguousNullLayout<usize> {
    type Item=Nullable<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let prev_idx = self.current_idx;

        if prev_idx >= self.container.len() { 
            return None;
        }

        self.current_idx = self.current_idx + 1;

        match self.container.get_layout().is_valid(prev_idx) {
            Some(true) => {
                let value = self.container.get_raw(prev_idx);
                Some(Nullable::Valid(value))
            },
            Some(false) => Some(Nullable::Null),
            // End of array (valid array will have a null layout length equal
            // to the array length)
            None => None,
        }
    }
}

pub struct SkippingNullableIter<'mcx, T, Container>
where Container: NullableContainer<'mcx, usize, T>,
Container::Layout : SkippingNullLayout<usize> {
    container: &'mcx Container,
    current_idx: usize,
    /// Current index in the raw / underlying container
    current_valid: Option<usize>,
    _marker: PhantomData<T>,
}

impl<'mcx, T: 'mcx, Container> Iterator for SkippingNullableIter<'mcx, T, Container>
        where Container: NullableContainer<'mcx, usize, T>,
        Container::Layout : SkippingNullLayout<usize> {
    type Item=Nullable<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_idx >= self.container.len() { 
            return None;
        }
        match self.container.get_layout().is_valid(self.current_idx) {
            // Non-null
            Some(true) => {
                let valid_idx = self.current_valid
                    .expect("Null-layout lists a valid value, however there\
                        are no more valid elements left to iterate over in the\
                        underlying array.");
                let value = self.container.get_raw(valid_idx);
                // Make sure monotonically-increasing self.current_valid
                // matches the assumptions of container.layout.next_valid_idx()
                debug_assert_eq!(
                    (self.current_idx < self.container.len())
                        .then_some(self.current_valid + 1),
                    self.container
                        .get_layout()
                        .next_valid_idx(self.current_idx));
                self.current_valid = self.current_valid + 1; 
                self.current_idx = self.current_idx + 1;
                Some(Nullable::Valid(value))
            },
            Some(false) => {
                self.current_idx = self.current_idx + 1;
                Some(Nullable::Null)
            },
            // End of array (valid array will have a null layout length equal
            // to the array length)
            None => None,
        }
    }
}

pub trait IntoNullableIterator<'mcx, T, Container, Iter>
        where Container: NullableContainer<'mcx, usize, T>,
        Container::Layout : NullLayout<usize>,
        Iter: Iterator<Item=Nullable<T>> {
    fn into_nullable_iter(&'mcx self) -> Iter;
}

impl<'mcx, T: 'mcx, Container> IntoNullableIterator<'mcx, T, Container, ContiguousNullableIter<'mcx, T, Container>>
        for &'mcx Container where Container: NullableContainer<'mcx, usize, T>,
        Container::Layout : ContiguousNullLayout<usize> {
    fn into_nullable_iter(&'mcx self) -> ContiguousNullableIter<'mcx, T, Container> {
        ContiguousNullableIter {
            container: self,
            current_idx: 0,
            _marker: PhantomData,
        }
    }
}

impl<'mcx, T: 'mcx, Container> IntoNullableIterator<'mcx, T, Container, SkippingNullableIter<'mcx, T, Container>>
        for &'mcx Container where Container: NullableContainer<'mcx, usize, T>,
        Container::Layout : SkippingNullLayout<usize> {
    fn into_nullable_iter(&'mcx self) -> SkippingNullableIter<'mcx, T, Container> {
        SkippingNullableIter {
            container: self,
            current_idx: 0,
            current_valid: 0,
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
mod nullable_tests {}
