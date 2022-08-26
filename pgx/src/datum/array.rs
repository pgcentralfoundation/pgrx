/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use crate::{pg_sys, FromDatum, IntoDatum, PgMemoryContexts};
use core::ptr::NonNull;
use serde::Serializer;
use std::marker::PhantomData;
use std::{mem, ptr, slice};

pub type VariadicArray<'a, T> = Array<'a, T>;

pub struct Array<'a, T: FromDatum> {
    _ptr: Option<NonNull<pg_sys::varlena>>,
    array_type: Option<NonNull<pg_sys::ArrayType>>,
    nelems: usize,
    elem_slice: &'a [pg_sys::Datum],
    null_slice: &'a [bool],
    _marker: PhantomData<T>,
}

impl<'a, T: FromDatum + serde::Serialize> serde::Serialize for Array<'a, T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(self.iter())
    }
}

impl<'a, T: FromDatum + serde::Serialize> serde::Serialize for ArrayTypedIterator<'a, T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(self.array.iter())
    }
}

impl<'a, T: FromDatum> Array<'a, T> {
    /// Create an [`Array`](crate::datum::Array) over an array of [`pg_sys::Datum`](pg_sys::Datum) values and a corresponding array
    /// of "is_null" indicators
    ///
    /// `T` can be [`pg_sys::Datum`](pg_sys::Datum) if the elements are not all of the same type
    ///
    /// # Safety
    ///
    /// This function requires that:
    /// - `elements` is non-null
    /// - `nulls` is non-null
    /// - both `elements` and `nulls` point to a slice of equal-or-greater length than `nelems`
    #[deprecated(
        since = "0.5.0",
        note = "creating arbitrary Arrays from raw pointers has unsound interactions!
    please open an issue in tcdi/pgx if you need this, with your stated use-case"
    )]
    pub unsafe fn over(
        elements: *mut pg_sys::Datum,
        nulls: *mut bool,
        nelems: usize,
    ) -> Array<'a, T> {
        // FIXME: This function existing prevents simply using NonNull<varlena>
        // or NonNull<ArrayType>. It has also caused issues like tcdi/pgx#633
        // Ideally it would cease being used soon.
        // It can be replaced with ways to make Postgres varlena arrays in Rust,
        // if there are any users who desire such a thing.
        //
        // Remember to remove the Array::over tests in pgx-tests/src/tests/array_tests.rs
        // when you finally kill this off.
        let _ptr: Option<NonNull<pg_sys::varlena>> = None;
        let array_type: Option<NonNull<pg_sys::ArrayType>> = None;
        Array::<T> {
            _ptr,
            array_type,
            nelems,
            elem_slice: slice::from_raw_parts(elements, nelems),
            null_slice: slice::from_raw_parts(nulls, nelems),
            _marker: PhantomData,
        }
    }

    /// # Safety
    ///
    /// This function requires that:
    /// - `elements` is non-null
    /// - `nulls` is non-null
    /// - both `elements` and `nulls` point to a slice of equal-or-greater length than `nelems`
    unsafe fn from_pg(
        _ptr: Option<NonNull<pg_sys::varlena>>,
        array_type: Option<NonNull<pg_sys::ArrayType>>,
        elements: *mut pg_sys::Datum,
        nulls: *mut bool,
        nelems: usize,
    ) -> Self {
        Array::<T> {
            _ptr,
            array_type,
            nelems,
            elem_slice: slice::from_raw_parts(elements, nelems),
            null_slice: slice::from_raw_parts(nulls, nelems),
            _marker: PhantomData,
        }
    }

    pub fn into_array_type(self) -> *const pg_sys::ArrayType {
        let ptr = if let Some(at) = self.array_type {
            at.as_ptr()
        } else {
            ptr::null()
        };
        mem::forget(self);
        ptr
    }

    pub fn as_slice(&self) -> &[T] {
        let sizeof_type = mem::size_of::<T>();
        let sizeof_datums = mem::size_of_val(self.elem_slice);
        unsafe {
            slice::from_raw_parts(
                self.elem_slice.as_ptr() as *const T,
                sizeof_datums / sizeof_type,
            )
        }
    }

    /// Return an Iterator of Option<T> over the contained Datums.
    pub fn iter(&self) -> ArrayIterator<'_, T> {
        ArrayIterator {
            array: self,
            curr: 0,
        }
    }

    /// Return an Iterator of the contained Datums (converted to Rust types).
    ///
    /// This function will panic when called if the array contains any SQL NULL values.
    pub fn iter_deny_null(&self) -> ArrayTypedIterator<'_, T> {
        if let Some(at) = self.array_type {
            if unsafe { pg_sys::array_contains_nulls(at.as_ptr()) } {
                panic!("array contains NULL");
            }
        } else {
            panic!("array is NULL");
        };

        ArrayTypedIterator {
            array: self,
            curr: 0,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.nelems
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.nelems == 0
    }

    #[allow(clippy::option_option)]
    #[inline]
    pub fn get(&self, i: usize) -> Option<Option<T>> {
        if i >= self.nelems {
            None
        } else {
            Some(unsafe { T::from_datum(self.elem_slice[i], self.null_slice[i]) })
        }
    }
}

pub struct ArrayTypedIterator<'a, T: 'a + FromDatum> {
    array: &'a Array<'a, T>,
    curr: usize,
}

impl<'a, T: FromDatum> Iterator for ArrayTypedIterator<'a, T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr >= self.array.nelems {
            None
        } else {
            let element = self
                .array
                .get(self.curr)
                .expect("array index out of bounds")
                .expect("array element was unexpectedly NULL during iteration");
            self.curr += 1;
            Some(element)
        }
    }
}

pub struct ArrayIterator<'a, T: 'a + FromDatum> {
    array: &'a Array<'a, T>,
    curr: usize,
}

impl<'a, T: FromDatum> Iterator for ArrayIterator<'a, T> {
    type Item = Option<T>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr >= self.array.nelems {
            None
        } else {
            let element = self.array.get(self.curr).unwrap();
            self.curr += 1;
            Some(element)
        }
    }
}

pub struct ArrayIntoIterator<'a, T: FromDatum> {
    array: Array<'a, T>,
    curr: usize,
}

impl<'a, T: FromDatum> IntoIterator for Array<'a, T> {
    type Item = Option<T>;
    type IntoIter = ArrayIntoIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        ArrayIntoIterator {
            array: self,
            curr: 0,
        }
    }
}

impl<'a, T: FromDatum> Iterator for ArrayIntoIterator<'a, T> {
    type Item = Option<T>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr >= self.array.nelems {
            None
        } else {
            let element = self.array.get(self.curr).unwrap();
            self.curr += 1;
            Some(element)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.array.nelems))
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.array.nelems
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.array.get(n)
    }
}

impl<'a, T: FromDatum> FromDatum for Array<'a, T> {
    #[inline]
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool) -> Option<Array<'a, T>> {
        if is_null || datum.is_null() {
            None
        } else {
            let ptr = datum.ptr_cast();
            let array = pg_sys::pg_detoast_datum(datum.ptr_cast()) as *mut pg_sys::ArrayType;
            let array_ref = array.as_ref().expect("ArrayType * was NULL");

            // outvals for get_typlenbyvalalign()
            let mut typlen = 0;
            let mut typbyval = false;
            let mut typalign = 0;

            pg_sys::get_typlenbyvalalign(
                array_ref.elemtype,
                &mut typlen,
                &mut typbyval,
                &mut typalign,
            );

            // outvals for deconstruct_array()
            let mut elements = ptr::null_mut();
            let mut nulls = ptr::null_mut();
            let mut nelems = 0;

            // FIXME: This way of getting array buffers causes problems for any Drop impl,
            // and clashes with assumptions of Array being a "zero-copy", lifetime-bound array,
            // some of which are implicitly embedded in other methods (e.g. Array::over).
            // It also risks leaking memory, as deconstruct_array calls palloc.
            // So either we don't use this, we use it more conditionally, or something.
            pg_sys::deconstruct_array(
                array,
                array_ref.elemtype,
                typlen as i32,
                typbyval,
                typalign,
                &mut elements,
                &mut nulls,
                &mut nelems,
            );

            Some(Array::from_pg(
                NonNull::new(ptr),
                NonNull::new(array),
                elements,
                nulls,
                nelems as usize,
            ))
        }
    }
}

impl<T: FromDatum> FromDatum for Vec<T> {
    #[inline]
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool) -> Option<Vec<T>> {
        if is_null {
            None
        } else {
            let array = Array::<T>::from_datum(datum, is_null).unwrap();
            let mut v = Vec::with_capacity(array.len());

            for element in array.iter() {
                v.push(element.expect("array element was NULL"))
            }
            Some(v)
        }
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
            // shoudln't happen
            None
        } else {
            Some(unsafe {
                pg_sys::makeArrayResult(state, PgMemoryContexts::CurrentMemoryContext.value())
            })
        }
    }

    fn type_oid() -> u32 {
        unsafe { pg_sys::get_array_type(T::type_oid()) }
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
            // shoudln't happen
            None
        } else {
            Some(unsafe {
                pg_sys::makeArrayResult(state, PgMemoryContexts::CurrentMemoryContext.value())
            })
        }
    }

    fn type_oid() -> u32 {
        unsafe { pg_sys::get_array_type(T::type_oid()) }
    }

    #[inline]
    fn is_compatible_with(other: pg_sys::Oid) -> bool {
        Self::type_oid() == other || other == unsafe { pg_sys::get_array_type(T::type_oid()) }
    }
}
