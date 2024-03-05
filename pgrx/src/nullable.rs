//! Traits and implementations of functionality relating to Postgres null
//! and nullable types, particularly in relation to arrays and other container
//! types which implement nullable (empty slot) behavior in a 
//! "structure-of-arrays" manner

use std::{marker::PhantomData, ops::Index};

use bitvec::slice::BitSlice;

use crate::{memcx::MemCx, Array, Datum, NullKind, UnboxDatum};

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
        where Idx: PartialEq + PartialOrd {
    fn len(&self) -> Idx;

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
        where Idx: PartialEq + PartialOrd { 
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
    where Idx: PartialEq + PartialOrd {}

pub trait NullableContainer<'mcx, Idx, T> { 
    type Layout : NullLayout<Idx>;

    fn get_layout(&'mcx self) -> &'mcx Self::Layout;

    /// For internal use - implement this over underlying data types
    /// Used to implement NullableIter.
    ///
    /// Get the Valid value from the underlying data index of `idx`,
    /// presumably after figuring out things like 
    fn get_raw(&'mcx self, idx: usize) -> &'mcx T;
}

impl NullLayout<usize> for BitSlice<u8> {
    fn len(&self) -> usize {
        BitSlice::<u8>::len(&self)
    }

    fn has_nulls(&self) -> bool {
        self.not_all()
    }

    fn is_valid(&self, idx: usize) -> Option<bool> {
        self.get(idx).map(|b| *b)
    }
    fn is_null(&self, idx: usize) -> Option<bool> {
        self.get(idx).map(|b| !b)
    }
}

impl SkippingNullLayout<usize> for BitSlice<u8> {
    fn next_valid_idx(&self, idx: usize) -> Option<usize> {
        // Next elem (one after this) would be past the end 
        // of the container
        if (idx+1) >= self.len() {
            return None;
        }
        let mut resulting_idx = 0;
        for bit in &(*self)[(idx+1)..] { 
            // Postgres nullbitmaps are 1 for "valid" and 0 for "null"
            resulting_idx += (*bit) as usize;
        }
        Some(resulting_idx)
    }
}

/// Strict i.e. no nulls.
/// Useful for using nullable primitives on non-null structures,
/// especially when this needs to be determined at runtime. 
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
pub struct StrictNullLayout(usize);

impl NullLayout<usize> for StrictNullLayout {
    fn len(&self) -> usize { self.0 }

    fn has_nulls(&self) -> bool { false }

    fn is_valid(&self, idx: usize) -> Option<bool> {
        (idx < self.0).then_some(true)
    }

    fn is_null(&self, idx: usize) -> Option<bool> {
        (idx < self.0).then_some(false)
    }
}
// No skipping when there are no nulls.
impl ContiguousNullLayout<usize> for StrictNullLayout {}

/// Iterates over a layout of null values, returning true for values that are null. 
pub struct NullsIter<'a, Idx, Layout: NullLayout<Idx>>
        where Idx: PartialEq + PartialOrd { 
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

pub struct NullableIterator<'mcx, T, Idx, A> where A: NullableContainer<'mcx, Idx, T> {
    nulls: NullsIter<'mcx, Idx, A::Layout>,
    container_ref: &'mcx A,
}

impl<'mcx, T, Idx, A> IntoIterator for A
        where A: NullableContainer<'mcx, Idx, T> {
    type Item = Nullable<T>;

    type IntoIter = NullableIterator<'mcx, T, Idx, A>;

    fn into_iter(self) -> Self::IntoIter {
        NullableIterator {
            nulls: self.get_layout(),
            container_ref: &self,
        }
    }
}

/*
pub struct IterNullableValuesWithBitmap<'src, Values> {
    null_slice: &'src BitSlice<u8>,
    values: Values,
}

pub struct IterNullable<Nulls, Values, Idx> 
        where Nulls: NullLayout<Idx>, Values: Index<Idx>, 
            Idx: PartialEq + PartialOrd {
    nulls: Nulls,
    values: Values,
    __marker: PhantomData<Idx>,
}

impl<'mcx, Nulls, Values, Idx> IterNullable<Nulls, Values, Idx>
    where Nulls: NullLayout<Idx>, Values: Index<Idx>, 
        Idx: PartialEq + PartialOrd {
    pub fn new(nulls: Nulls, values: Values) -> Self { 
        Self { 
            nulls, 
            values,
            __marker: PhantomData{}
        }
    }
}*/