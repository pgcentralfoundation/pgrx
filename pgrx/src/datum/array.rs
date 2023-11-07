//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use crate::array::RawArray;
use crate::layout::*;
use crate::toast::Toast;
use crate::{pg_sys, FromDatum, IntoDatum, PgMemoryContexts};
use bitvec::slice::BitSlice;
use core::fmt::{Debug, Formatter};
use core::iter::FusedIterator;
use core::ops::DerefMut;
use core::ptr::NonNull;
use pgrx_sql_entity_graph::metadata::{
    ArgumentError, Returns, ReturnsError, SqlMapping, SqlTranslatable,
};
use serde::{Serialize, Serializer};

/** An array of some type (eg. `TEXT[]`, `int[]`)

While conceptually similar to a [`Vec<T>`][std::vec::Vec], arrays are lazy.

Using a [`Vec<T>`][std::vec::Vec] here means each element of the passed array will be eagerly fetched and converted into a Rust type:

```rust,no_run
use pgrx::prelude::*;

#[pg_extern]
fn with_vec(elems: Vec<String>) {
    // Elements all already converted.
    for elem in elems {
        todo!()
    }
}
```

Using an array, elements are only fetched and converted into a Rust type on demand:

```rust,no_run
use pgrx::prelude::*;

#[pg_extern]
fn with_vec(elems: Array<String>) {
    // Elements converted one by one
    for maybe_elem in elems {
        let elem = maybe_elem.unwrap();
        todo!()
    }
}
```
*/
pub struct Array<'a, T> {
    null_slice: NullKind<'a>,
    slide_impl: ChaChaSlideImpl<T>,
    // Rust drops in FIFO order, drop this last
    raw: Toast<RawArray>,
}

impl<'a, T: FromDatum + Debug> Debug for Array<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

type ChaChaSlideImpl<T> = Box<dyn casper::ChaChaSlide<T>>;

enum NullKind<'a> {
    Bits(&'a BitSlice<u8>),
    Strict(usize),
}

impl NullKind<'_> {
    fn get(&self, index: usize) -> Option<bool> {
        match self {
            // Note this flips the bit:
            // Postgres nullbitmaps are 1 for "valid" and 0 for "null"
            Self::Bits(b1) => b1.get(index).map(|b| !b),
            Self::Strict(len) => index.lt(len).then_some(false),
        }
    }

    fn any(&self) -> bool {
        match self {
            // Note the reversed polarity:
            // Postgres nullbitmaps are 1 for "valid" and 0 for "null"
            Self::Bits(b1) => !b1.all(),
            Self::Strict(_) => false,
        }
    }
}

impl<T: FromDatum + Serialize> Serialize for Array<'_, T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(self.iter())
    }
}

#[deny(unsafe_op_in_unsafe_fn)]
impl<'a, T: FromDatum> Array<'a, T> {
    /// # Safety
    ///
    /// This function requires that the RawArray was obtained in a properly-constructed form
    /// (probably from Postgres).
    unsafe fn deconstruct_from(mut raw: Toast<RawArray>) -> Array<'a, T> {
        let oid = raw.oid();
        let elem_layout = Layout::lookup_oid(oid);
        let nelems = raw.len();
        let null_slice = raw
            .nulls_bitslice()
            .map(|nonnull| NullKind::Bits(unsafe { &*nonnull.as_ptr() }))
            .unwrap_or(NullKind::Strict(nelems));

        // do a little two-step before jumping into the Cha-Cha Slide and figure out
        // which implementation is correct for the type of element in this Array.
        let slide_impl: ChaChaSlideImpl<T> = match elem_layout.pass {
            PassBy::Value => match elem_layout.size {
                // The layout is one that we know how to handle efficiently.
                Size::Fixed(1) => Box::new(casper::FixedSizeByVal::<1>),
                Size::Fixed(2) => Box::new(casper::FixedSizeByVal::<2>),
                Size::Fixed(4) => Box::new(casper::FixedSizeByVal::<4>),
                #[cfg(target_pointer_width = "64")]
                Size::Fixed(8) => Box::new(casper::FixedSizeByVal::<8>),

                _ => {
                    panic!("unrecognized pass-by-value array element layout: {:?}", elem_layout)
                }
            },

            PassBy::Ref => match elem_layout.size {
                // Array elements are varlenas, which are pass-by-reference and have a known alignment size
                Size::Varlena => Box::new(casper::PassByVarlena { align: elem_layout.align }),

                // Array elements are C strings, which are pass-by-reference and alignments are
                // determined at runtime based on the length of the string
                Size::CStr => Box::new(casper::PassByCStr),

                // Array elements are fixed sizes yet the data is "pass-by-reference"
                // Most commonly, this is because of elements larger than a Datum.
                Size::Fixed(size) => Box::new(casper::PassByFixed {
                    padded_size: elem_layout.align.pad(size.into()),
                }),
            },
        };

        Array { raw, slide_impl, null_slice }
    }

    /// Return an iterator of `Option<T>`.
    #[inline]
    pub fn iter(&self) -> ArrayIterator<'_, T> {
        let ptr = self.raw.data_ptr();
        ArrayIterator { array: self, curr: 0, ptr }
    }

    /// Return an iterator over the Array's elements.
    ///
    /// # Panics
    /// This function will panic when called if the array contains any SQL NULL values.
    #[inline]
    pub fn iter_deny_null(&self) -> ArrayTypedIterator<'_, T> {
        if self.null_slice.any() {
            panic!("array contains NULL");
        }

        let ptr = self.raw.data_ptr();
        ArrayTypedIterator { array: self, curr: 0, ptr }
    }

    #[allow(clippy::option_option)]
    #[inline]
    pub fn get(&self, index: usize) -> Option<Option<T>> {
        let Some(is_null) = self.null_slice.get(index) else { return None };
        if is_null {
            return Some(None);
        }

        // This pointer is what's walked over the entire array's data buffer.
        // If the array has varlena or cstr elements, we can't index into the array.
        // If the elements are fixed size, we could, but we do not exploit that optimization yet
        // as it would significantly complicate the code and impact debugging it.
        // Such improvements should wait until a later version (today's: 0.7.4, preparing 0.8.0).
        let mut at_byte = self.raw.data_ptr();
        for i in 0..index {
            match self.null_slice.get(i) {
                None => unreachable!("array was exceeded while walking to known non-null index???"),
                // Skip nulls: the data buffer has no placeholders for them!
                Some(true) => continue,
                Some(false) => {
                    // SAFETY: Note this entire function has to be correct,
                    // not just this one call, for this to be correct!
                    at_byte = unsafe { self.one_hop_this_time(at_byte) };
                }
            }
        }

        // If this has gotten this far, it is known to be non-null,
        // all the null values in the array up to this index were skipped,
        // and the only offsets were via our hopping function.
        Some(unsafe { self.bring_it_back_now(at_byte, false) })
    }

    /// Extracts an element from a Postgres Array's data buffer
    ///
    /// # Safety
    /// This assumes the pointer is to a valid element of that type.
    #[inline]
    unsafe fn bring_it_back_now(&self, ptr: *const u8, is_null: bool) -> Option<T> {
        match is_null {
            true => None,
            false => unsafe { self.slide_impl.bring_it_back_now(self, ptr) },
        }
    }

    /// Walk the data of a Postgres Array, "hopping" according to element layout.
    ///
    /// # Safety
    /// For the varlena/cstring layout, data in the buffer is read.
    /// In either case, pointer arithmetic is done, with the usual implications,
    /// e.g. the pointer must be <= a "one past the end" pointer
    /// This means this function must be invoked with the correct layout, and
    /// either the array's `data_ptr` or a correctly offset pointer into it.
    ///
    /// Null elements will NOT be present in a Postgres Array's data buffer!
    /// Do not cumulatively invoke this more than `len - null_count`!
    /// Doing so will result in reading uninitialized data, which is UB!
    #[inline]
    unsafe fn one_hop_this_time(&self, ptr: *const u8) -> *const u8 {
        unsafe {
            let offset = self.slide_impl.hop_size(ptr);
            // SAFETY: ptr stops at 1-past-end of the array's varlena
            debug_assert!(ptr.wrapping_add(offset) <= self.raw.end_ptr());
            ptr.add(offset)
        }
    }
}

#[deny(unsafe_op_in_unsafe_fn)]
impl<T> Array<'_, T> {
    /// Rips out the underlying `pg_sys::ArrayType` pointer.
    /// Note that Array may have caused Postgres to allocate to unbox the datum,
    /// and this can hypothetically cause a memory leak if so.
    #[inline]
    pub fn into_array_type(self) -> *const pg_sys::ArrayType {
        // may be worth replacing this function when Toast<T> matures enough
        // to be used as a public type with a fn(self) -> Toast<RawArray>

        let Array { raw, .. } = self;
        // Wrap the Toast<RawArray> to prevent it from deallocating itself
        let mut raw = core::mem::ManuallyDrop::new(raw);
        let ptr = raw.deref_mut().deref_mut() as *mut RawArray;
        // SAFETY: Leaks are safe if they aren't use-after-frees!
        unsafe { ptr.read() }.into_ptr().as_ptr() as _
    }

    /// Returns `true` if this [`Array`] contains one or more SQL "NULL" values
    #[inline]
    pub fn contains_nulls(&self) -> bool {
        self.null_slice.any()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.raw.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.raw.len() == 0
    }
}

#[derive(thiserror::Error, Debug, Copy, Clone, Eq, PartialEq)]
pub enum ArraySliceError {
    #[error("Cannot create a slice of an Array that contains nulls")]
    ContainsNulls,
}

#[cfg(target_pointer_width = "64")]
impl Array<'_, f64> {
    /// Returns a slice of `f64`s which comprise this [`Array`].
    ///
    /// # Errors
    ///
    /// Returns a [`ArraySliceError::ContainsNulls`] error if this [`Array`] contains one or more
    /// SQL "NULL" values.  In this case, you'd likely want to fallback to using [`Array::iter()`].
    #[inline]
    pub fn as_slice(&self) -> Result<&[f64], ArraySliceError> {
        as_slice(self)
    }
}

impl Array<'_, f32> {
    /// Returns a slice of `f32`s which comprise this [`Array`].
    ///
    /// # Errors
    ///
    /// Returns a [`ArraySliceError::ContainsNulls`] error if this [`Array`] contains one or more
    /// SQL "NULL" values.  In this case, you'd likely want to fallback to using [`Array::iter()`].
    #[inline]
    pub fn as_slice(&self) -> Result<&[f32], ArraySliceError> {
        as_slice(self)
    }
}

#[cfg(target_pointer_width = "64")]
impl Array<'_, i64> {
    /// Returns a slice of `i64`s which comprise this [`Array`].
    ///
    /// # Errors
    ///
    /// Returns a [`ArraySliceError::ContainsNulls`] error if this [`Array`] contains one or more
    /// SQL "NULL" values.  In this case, you'd likely want to fallback to using [`Array::iter()`].
    #[inline]
    pub fn as_slice(&self) -> Result<&[i64], ArraySliceError> {
        as_slice(self)
    }
}

impl Array<'_, i32> {
    /// Returns a slice of `i32`s which comprise this [`Array`].
    ///
    /// # Errors
    ///
    /// Returns a [`ArraySliceError::ContainsNulls`] error if this [`Array`] contains one or more
    /// SQL "NULL" values.  In this case, you'd likely want to fallback to using [`Array::iter()`].
    #[inline]
    pub fn as_slice(&self) -> Result<&[i32], ArraySliceError> {
        as_slice(self)
    }
}

impl Array<'_, i16> {
    /// Returns a slice of `i16`s which comprise this [`Array`].
    ///
    /// # Errors
    ///
    /// Returns a [`ArraySliceError::ContainsNulls`] error if this [`Array`] contains one or more
    /// SQL "NULL" values.  In this case, you'd likely want to fallback to using [`Array::iter()`].
    #[inline]
    pub fn as_slice(&self) -> Result<&[i16], ArraySliceError> {
        as_slice(self)
    }
}

impl Array<'_, i8> {
    /// Returns a slice of `i8`s which comprise this [`Array`].
    ///
    /// # Errors
    ///
    /// Returns a [`ArraySliceError::ContainsNulls`] error if this [`Array`] contains one or more
    /// SQL "NULL" values.  In this case, you'd likely want to fallback to using [`Array::iter()`].
    #[inline]
    pub fn as_slice(&self) -> Result<&[i8], ArraySliceError> {
        as_slice(self)
    }
}

#[inline(always)]
fn as_slice<'a, T: Sized>(array: &'a Array<'_, T>) -> Result<&'a [T], ArraySliceError> {
    if array.contains_nulls() {
        return Err(ArraySliceError::ContainsNulls);
    }

    let slice =
        unsafe { std::slice::from_raw_parts(array.raw.data_ptr() as *const _, array.len()) };
    Ok(slice)
}

mod casper {
    use crate::layout::Align;
    use crate::{pg_sys, varlena, Array, FromDatum};

    // it's a pop-culture reference (https://en.wikipedia.org/wiki/Cha_Cha_Slide) not some fancy crypto thing you nerd
    /// Describes how to instantiate a value `T` from an [`Array`] and its backing byte array pointer.
    /// It also knows how to determine the size of an [`Array`] element value.
    pub(super) trait ChaChaSlide<T: FromDatum> {
        /// Instantiate a `T` from the head of `ptr`
        ///
        /// # Safety
        ///
        /// This function is unsafe as it cannot guarantee that `ptr` points to the proper bytes
        /// that represent a `T`, or even that it belongs to `array`.  Both of which must be true
        unsafe fn bring_it_back_now(&self, array: &Array<T>, ptr: *const u8) -> Option<T>;

        /// Determine how many bytes are used to represent `T`.  This could be fixed size or
        /// even determined at runtime by whatever `ptr` is known to be pointing at.
        ///
        /// # Safety
        ///
        /// This function is unsafe as it cannot guarantee that `ptr` points to the bytes of a `T`,
        /// which it must for implementations that rely on that.
        unsafe fn hop_size(&self, ptr: *const u8) -> usize;
    }

    #[inline(always)]
    fn is_aligned<T>(p: *const T) -> bool {
        (p as usize) & (core::mem::align_of::<T>() - 1) == 0
    }

    /// Safety: Equivalent to a (potentially) aligned read of `ptr`, which
    /// should be `Copy` (ideally...).
    #[track_caller]
    #[inline(always)]
    pub(super) unsafe fn byval_read<T: Copy>(ptr: *const u8) -> T {
        let ptr = ptr.cast::<T>();
        debug_assert!(is_aligned(ptr), "not aligned to {}: {ptr:p}", std::mem::align_of::<T>());
        ptr.read()
    }

    /// Fixed-size byval array elements. N should be 1, 2, 4, or 8. Note that
    /// `T` (the rust type) may have a different size than `N`.
    pub(super) struct FixedSizeByVal<const N: usize>;
    impl<T: FromDatum, const N: usize> ChaChaSlide<T> for FixedSizeByVal<N> {
        #[inline(always)]
        unsafe fn bring_it_back_now(&self, array: &Array<T>, ptr: *const u8) -> Option<T> {
            // This branch is optimized away (because `N` is constant).
            let datum = match N {
                // for match with `Datum`, read through that directly to
                // preserve provenance (may not be relevant but doesn't hurt).
                1 => pg_sys::Datum::from(byval_read::<u8>(ptr)),
                2 => pg_sys::Datum::from(byval_read::<u16>(ptr)),
                4 => pg_sys::Datum::from(byval_read::<u32>(ptr)),
                8 => pg_sys::Datum::from(byval_read::<u64>(ptr)),
                _ => unreachable!("`N` must be 1, 2, 4, or 8 (got {N})"),
            };
            T::from_polymorphic_datum(datum, false, array.raw.oid())
        }

        #[inline(always)]
        unsafe fn hop_size(&self, _ptr: *const u8) -> usize {
            N
        }
    }

    /// Array elements are [`pg_sys::varlena`] types, which are pass-by-reference
    pub(super) struct PassByVarlena {
        pub(super) align: Align,
    }
    impl<T: FromDatum> ChaChaSlide<T> for PassByVarlena {
        #[inline]
        unsafe fn bring_it_back_now(&self, array: &Array<T>, ptr: *const u8) -> Option<T> {
            let datum = pg_sys::Datum::from(ptr);
            unsafe { T::from_polymorphic_datum(datum, false, array.raw.oid()) }
        }

        #[inline]
        unsafe fn hop_size(&self, ptr: *const u8) -> usize {
            // SAFETY: This uses the varsize_any function to be safe,
            // and the caller was informed of pointer requirements.
            let varsize = varlena::varsize_any(ptr.cast());

            // Now make sure this is aligned-up
            self.align.pad(varsize)
        }
    }

    /// Array elements are standard C strings (`char *`), which are pass-by-reference
    pub(super) struct PassByCStr;
    impl<T: FromDatum> ChaChaSlide<T> for PassByCStr {
        #[inline]
        unsafe fn bring_it_back_now(&self, array: &Array<T>, ptr: *const u8) -> Option<T> {
            let datum = pg_sys::Datum::from(ptr);
            unsafe { T::from_polymorphic_datum(datum, false, array.raw.oid()) }
        }

        #[inline]
        unsafe fn hop_size(&self, ptr: *const u8) -> usize {
            // SAFETY: The caller was informed of pointer requirements.
            let strlen = core::ffi::CStr::from_ptr(ptr.cast()).to_bytes().len();

            // Skip over the null which points us to the head of the next cstr
            strlen + 1
        }
    }

    pub(super) struct PassByFixed {
        pub(super) padded_size: usize,
    }

    impl<T: FromDatum> ChaChaSlide<T> for PassByFixed {
        #[inline]
        unsafe fn bring_it_back_now(&self, array: &Array<T>, ptr: *const u8) -> Option<T> {
            let datum = pg_sys::Datum::from(ptr);
            unsafe { T::from_polymorphic_datum(datum, false, array.raw.oid()) }
        }

        #[inline]
        unsafe fn hop_size(&self, _ptr: *const u8) -> usize {
            self.padded_size
        }
    }
}

pub struct VariadicArray<'a, T>(Array<'a, T>);

impl<T: FromDatum + Serialize> Serialize for VariadicArray<'_, T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(self.0.iter())
    }
}

impl<T: FromDatum> VariadicArray<'_, T> {
    /// Return an Iterator of `Option<T>` over the contained Datums.
    #[inline]
    pub fn iter(&self) -> ArrayIterator<'_, T> {
        self.0.iter()
    }

    /// Return an iterator over the Array's elements.
    ///
    /// # Panics
    /// This function will panic when called if the array contains any SQL NULL values.
    #[inline]
    pub fn iter_deny_null(&self) -> ArrayTypedIterator<'_, T> {
        self.0.iter_deny_null()
    }

    #[allow(clippy::option_option)]
    #[inline]
    pub fn get(&self, i: usize) -> Option<Option<T>> {
        self.0.get(i)
    }
}

impl<T> VariadicArray<'_, T> {
    #[inline]
    pub fn into_array_type(self) -> *const pg_sys::ArrayType {
        self.0.into_array_type()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

pub struct ArrayTypedIterator<'a, T> {
    array: &'a Array<'a, T>,
    curr: usize,
    ptr: *const u8,
}

impl<'a, T: FromDatum> Iterator for ArrayTypedIterator<'a, T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let Self { array, curr, ptr } = self;
        if *curr >= array.raw.len() {
            None
        } else {
            // SAFETY: The constructor for this type instantly panics if any nulls are present!
            // Thus as an invariant, this will never have to reckon with the nullbitmap.
            let element = unsafe { array.bring_it_back_now(*ptr, false) };
            *curr += 1;
            *ptr = unsafe { array.one_hop_this_time(*ptr) };
            element
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.array.raw.len().saturating_sub(self.curr);
        (len, Some(len))
    }
}

impl<'a, T: FromDatum> ExactSizeIterator for ArrayTypedIterator<'a, T> {}
impl<'a, T: FromDatum> FusedIterator for ArrayTypedIterator<'a, T> {}

impl<'a, T: FromDatum + Serialize> Serialize for ArrayTypedIterator<'a, T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(self.array.iter())
    }
}

pub struct ArrayIterator<'a, T> {
    array: &'a Array<'a, T>,
    curr: usize,
    ptr: *const u8,
}

impl<'a, T: FromDatum> Iterator for ArrayIterator<'a, T> {
    type Item = Option<T>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let Self { array, curr, ptr } = self;
        let Some(is_null) = array.null_slice.get(*curr) else { return None };
        *curr += 1;

        let element = unsafe { array.bring_it_back_now(*ptr, is_null) };
        if !is_null {
            // SAFETY: This has to not move for nulls, as they occupy 0 data bytes,
            // and it has to move only after unpacking a non-null varlena element,
            // as the iterator starts by pointing to the first non-null element!
            *ptr = unsafe { array.one_hop_this_time(*ptr) };
        }
        Some(element)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.array.raw.len().saturating_sub(self.curr);
        (len, Some(len))
    }
}

impl<'a, T: FromDatum> ExactSizeIterator for ArrayIterator<'a, T> {}
impl<'a, T: FromDatum> FusedIterator for ArrayIterator<'a, T> {}

pub struct ArrayIntoIterator<'a, T> {
    array: Array<'a, T>,
    curr: usize,
    ptr: *const u8,
}

impl<'a, T: FromDatum> IntoIterator for Array<'a, T> {
    type Item = Option<T>;
    type IntoIter = ArrayIntoIterator<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        let ptr = self.raw.data_ptr();
        ArrayIntoIterator { array: self, curr: 0, ptr }
    }
}

impl<'a, T: FromDatum> IntoIterator for VariadicArray<'a, T> {
    type Item = Option<T>;
    type IntoIter = ArrayIntoIterator<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        let ptr = self.0.raw.data_ptr();
        ArrayIntoIterator { array: self.0, curr: 0, ptr }
    }
}

impl<'a, T: FromDatum> Iterator for ArrayIntoIterator<'a, T> {
    type Item = Option<T>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let Self { array, curr, ptr } = self;
        let Some(is_null) = array.null_slice.get(*curr) else { return None };
        *curr += 1;

        let element = unsafe { array.bring_it_back_now(*ptr, is_null) };
        if !is_null {
            // SAFETY: This has to not move for nulls, as they occupy 0 data bytes,
            // and it has to move only after unpacking a non-null varlena element,
            // as the iterator starts by pointing to the first non-null element!
            *ptr = unsafe { array.one_hop_this_time(*ptr) };
        }
        Some(element)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.array.raw.len().saturating_sub(self.curr);
        (len, Some(len))
    }
}

impl<'a, T: FromDatum> ExactSizeIterator for ArrayIntoIterator<'a, T> {}
impl<'a, T: FromDatum> FusedIterator for ArrayIntoIterator<'a, T> {}

impl<'a, T: FromDatum> FromDatum for VariadicArray<'a, T> {
    #[inline]
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        oid: pg_sys::Oid,
    ) -> Option<VariadicArray<'a, T>> {
        Array::from_polymorphic_datum(datum, is_null, oid).map(Self)
    }
}

impl<'a, T: FromDatum> FromDatum for Array<'a, T> {
    #[inline]
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        _typoid: pg_sys::Oid,
    ) -> Option<Array<'a, T>> {
        if is_null {
            None
        } else {
            let Some(ptr) = NonNull::new(datum.cast_mut_ptr()) else { return None };
            let raw = RawArray::detoast_from_varlena(ptr);
            Some(Array::deconstruct_from(raw))
        }
    }

    unsafe fn from_datum_in_memory_context(
        mut memory_context: PgMemoryContexts,
        datum: pg_sys::Datum,
        is_null: bool,
        typoid: pg_sys::Oid,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        if is_null {
            None
        } else {
            memory_context.switch_to(|_| {
                // copy the Datum into this MemoryContext, and then instantiate the Array wrapper
                let copy = pg_sys::pg_detoast_datum_copy(datum.cast_mut_ptr());
                Array::<T>::from_polymorphic_datum(pg_sys::Datum::from(copy), false, typoid)
            })
        }
    }
}

impl<T: IntoDatum> IntoDatum for Array<'_, T> {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        let array_type = self.into_array_type();
        let datum = pg_sys::Datum::from(array_type);
        Some(datum)
    }

    #[inline]
    fn type_oid() -> pg_sys::Oid {
        unsafe { pg_sys::get_array_type(T::type_oid()) }
    }

    fn composite_type_oid(&self) -> Option<pg_sys::Oid> {
        Some(unsafe { pg_sys::get_array_type(self.raw.oid()) })
    }
}

impl<T: FromDatum> FromDatum for Vec<T> {
    #[inline]
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        typoid: pg_sys::Oid,
    ) -> Option<Vec<T>> {
        if is_null {
            None
        } else {
            Array::<T>::from_polymorphic_datum(datum, is_null, typoid)
                .map(|array| array.iter_deny_null().collect::<Vec<_>>())
        }
    }

    unsafe fn from_datum_in_memory_context(
        memory_context: PgMemoryContexts,
        datum: pg_sys::Datum,
        is_null: bool,
        typoid: pg_sys::Oid,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        Array::<T>::from_datum_in_memory_context(memory_context, datum, is_null, typoid)
            .map(|array| array.iter_deny_null().collect::<Vec<_>>())
    }
}

impl<T: FromDatum> FromDatum for Vec<Option<T>> {
    #[inline]
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        typoid: pg_sys::Oid,
    ) -> Option<Vec<Option<T>>> {
        Array::<T>::from_polymorphic_datum(datum, is_null, typoid)
            .map(|array| array.iter().collect::<Vec<_>>())
    }

    unsafe fn from_datum_in_memory_context(
        memory_context: PgMemoryContexts,
        datum: pg_sys::Datum,
        is_null: bool,
        typoid: pg_sys::Oid,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        Array::<T>::from_datum_in_memory_context(memory_context, datum, is_null, typoid)
            .map(|array| array.iter().collect::<Vec<_>>())
    }
}

impl<T> IntoDatum for Vec<T>
where
    T: IntoDatum,
{
    fn into_datum(self) -> Option<pg_sys::Datum> {
        let mut state = unsafe {
            pg_sys::initArrayResult(
                T::type_oid(),
                PgMemoryContexts::CurrentMemoryContext.value(),
                false,
            )
        };
        for s in self {
            let datum = s.into_datum();
            let isnull = datum.is_none();

            unsafe {
                state = pg_sys::accumArrayResult(
                    state,
                    datum.unwrap_or(0.into()),
                    isnull,
                    T::type_oid(),
                    PgMemoryContexts::CurrentMemoryContext.value(),
                );
            }
        }

        if state.is_null() {
            // shouldn't happen
            None
        } else {
            Some(unsafe {
                pg_sys::makeArrayResult(state, PgMemoryContexts::CurrentMemoryContext.value())
            })
        }
    }

    fn type_oid() -> pg_sys::Oid {
        unsafe { pg_sys::get_array_type(T::type_oid()) }
    }

    fn composite_type_oid(&self) -> Option<pg_sys::Oid> {
        // the composite type oid for a vec of composite types is the array type of the base composite type
        self.get(0)
            .and_then(|v| v.composite_type_oid().map(|oid| unsafe { pg_sys::get_array_type(oid) }))
    }

    #[inline]
    fn is_compatible_with(other: pg_sys::Oid) -> bool {
        Self::type_oid() == other || other == unsafe { pg_sys::get_array_type(T::type_oid()) }
    }
}

impl<'a, T> IntoDatum for &'a [T]
where
    T: IntoDatum + Copy + 'a,
{
    fn into_datum(self) -> Option<pg_sys::Datum> {
        let mut state = unsafe {
            pg_sys::initArrayResult(
                T::type_oid(),
                PgMemoryContexts::CurrentMemoryContext.value(),
                false,
            )
        };
        for s in self {
            let datum = s.into_datum();
            let isnull = datum.is_none();

            unsafe {
                state = pg_sys::accumArrayResult(
                    state,
                    datum.unwrap_or(0.into()),
                    isnull,
                    T::type_oid(),
                    PgMemoryContexts::CurrentMemoryContext.value(),
                );
            }
        }

        if state.is_null() {
            // shouldn't happen
            None
        } else {
            Some(unsafe {
                pg_sys::makeArrayResult(state, PgMemoryContexts::CurrentMemoryContext.value())
            })
        }
    }

    fn type_oid() -> pg_sys::Oid {
        unsafe { pg_sys::get_array_type(T::type_oid()) }
    }

    #[inline]
    fn is_compatible_with(other: pg_sys::Oid) -> bool {
        Self::type_oid() == other || other == unsafe { pg_sys::get_array_type(T::type_oid()) }
    }
}

unsafe impl<T> SqlTranslatable for Array<'_, T>
where
    T: SqlTranslatable,
{
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        match T::argument_sql()? {
            SqlMapping::As(sql) => Ok(SqlMapping::As(format!("{sql}[]"))),
            SqlMapping::Skip => Err(ArgumentError::SkipInArray),
            SqlMapping::Composite { .. } => Ok(SqlMapping::Composite { array_brackets: true }),
            SqlMapping::Source { .. } => Ok(SqlMapping::Source { array_brackets: true }),
        }
    }

    fn return_sql() -> Result<Returns, ReturnsError> {
        match T::return_sql()? {
            Returns::One(SqlMapping::As(sql)) => {
                Ok(Returns::One(SqlMapping::As(format!("{sql}[]"))))
            }
            Returns::One(SqlMapping::Composite { array_brackets: _ }) => {
                Ok(Returns::One(SqlMapping::Composite { array_brackets: true }))
            }
            Returns::One(SqlMapping::Source { array_brackets: _ }) => {
                Ok(Returns::One(SqlMapping::Source { array_brackets: true }))
            }
            Returns::One(SqlMapping::Skip) => Err(ReturnsError::SkipInArray),
            Returns::SetOf(_) => Err(ReturnsError::SetOfInArray),
            Returns::Table(_) => Err(ReturnsError::TableInArray),
        }
    }
}

unsafe impl<T> SqlTranslatable for VariadicArray<'_, T>
where
    T: SqlTranslatable,
{
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        match T::argument_sql()? {
            SqlMapping::As(sql) => Ok(SqlMapping::As(format!("{sql}[]"))),
            SqlMapping::Skip => Err(ArgumentError::SkipInArray),
            SqlMapping::Composite { .. } => Ok(SqlMapping::Composite { array_brackets: true }),
            SqlMapping::Source { .. } => Ok(SqlMapping::Source { array_brackets: true }),
        }
    }

    fn return_sql() -> Result<Returns, ReturnsError> {
        match T::return_sql()? {
            Returns::One(SqlMapping::As(sql)) => {
                Ok(Returns::One(SqlMapping::As(format!("{sql}[]"))))
            }
            Returns::One(SqlMapping::Composite { array_brackets: _ }) => {
                Ok(Returns::One(SqlMapping::Composite { array_brackets: true }))
            }
            Returns::One(SqlMapping::Source { array_brackets: _ }) => {
                Ok(Returns::One(SqlMapping::Source { array_brackets: true }))
            }
            Returns::One(SqlMapping::Skip) => Err(ReturnsError::SkipInArray),
            Returns::SetOf(_) => Err(ReturnsError::SetOfInArray),
            Returns::Table(_) => Err(ReturnsError::TableInArray),
        }
    }

    fn variadic() -> bool {
        true
    }
}
