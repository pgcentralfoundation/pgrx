/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use crate::{array::RawArray, layout::*, pg_sys, FromDatum, IntoDatum, PgMemoryContexts};
use bitvec::slice::BitSlice;
use core::ptr::NonNull;
use pgx_utils::sql_entity_graph::metadata::{
    ArgumentError, ReturnVariant, ReturnVariantError, SqlMapping, SqlTranslatable,
};
use serde::Serializer;
use std::marker::PhantomData;
use std::{mem, ptr, slice};

pub struct Array<'a, T: FromDatum> {
    _ptr: Option<NonNull<pg_sys::varlena>>,
    raw: Option<RawArray>,
    nelems: usize,
    elem_slice: &'a [pg_sys::Datum],
    null_slice: NullKind<'a>,
    elem_layout: Option<Layout>,
    _marker: PhantomData<T>,
}

// FIXME: When Array::over gets removed, this enum can probably be dropped
// since we won't be entertaining ArrayTypes which don't use bitslices anymore.
// However, we could also use a static resolution? Hard to say what's best.
enum NullKind<'a> {
    Bits(&'a BitSlice<u8>),
    Bytes(&'a [bool]),
    Strict(usize),
}

impl<'a> From<&'a [bool]> for NullKind<'a> {
    fn from(b8: &'a [bool]) -> NullKind<'a> {
        NullKind::Bytes(b8)
    }
}

impl NullKind<'_> {
    fn get(&self, index: usize) -> Option<bool> {
        match self {
            Self::Bits(b1) => b1.get(index).map(|b| !b),
            Self::Bytes(b8) => b8.get(index).map(|b| *b),
            Self::Strict(len) => index.le(len).then(|| false),
        }
    }

    fn any(&self) -> bool {
        match self {
            Self::Bits(b1) => !b1.all(),
            Self::Bytes(b8) => b8.into_iter().any(|b| *b),
            Self::Strict(_) => false,
        }
    }
}

impl<'a, T: FromDatum + serde::Serialize> serde::Serialize for Array<'a, T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(self.iter())
    }
}

#[deny(unsafe_op_in_unsafe_fn)]
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
        let raw: Option<RawArray> = None;
        let elem_layout: Option<Layout> = None;
        Array::<T> {
            _ptr,
            raw,
            nelems,
            elem_slice: unsafe { slice::from_raw_parts(elements, nelems) },
            null_slice: unsafe { slice::from_raw_parts(nulls, nelems) }.into(),
            elem_layout,
            _marker: PhantomData,
        }
    }

    /// # Safety
    ///
    /// This function requires that the RawArray was obtained in a properly-constructed form
    /// (probably from Postgres).
    unsafe fn deconstruct_from(
        _ptr: Option<NonNull<pg_sys::varlena>>,
        raw: RawArray,
        layout: Layout,
    ) -> Array<'a, T> {
        let oid = raw.oid();
        let len = raw.len();
        let array = raw.into_ptr().as_ptr();

        // outvals for deconstruct_array
        let mut elements = ptr::null_mut();
        let mut nulls = ptr::null_mut();
        let mut nelems = 0;

        /*
        FIXME(jubilee): This way of getting array buffers causes problems for any Drop impl,
        and clashes with assumptions of Array being a "zero-copy", lifetime-bound array,
        some of which are implicitly embedded in other methods (e.g. Array::over).
        It also risks leaking memory, as deconstruct_array calls palloc.

        TODO(0.6.0): Start implementing Drop again when we no longer have Array::over.
        See tcdi/pgx#627 and #633 for why this is the preferred resolution to this.

        SAFETY: We have already asserted the validity of the RawArray, so
        this only makes mistakes if we mix things up and pass Postgres the wrong data.
        */
        unsafe {
            pg_sys::deconstruct_array(
                array,
                oid,
                layout.size.as_typlen().into(),
                layout.passbyval,
                layout.align.as_typalign(),
                &mut elements,
                &mut nulls,
                &mut nelems,
            )
        };

        let nelems = nelems as usize;

        // Check our RawArray len impl for correctness.
        assert_eq!(nelems, len);
        let mut raw = unsafe { RawArray::from_ptr(NonNull::new_unchecked(array)) };

        let null_slice = raw
            .nulls_bitslice()
            .map(|nonnull| NullKind::Bits(unsafe { &*nonnull.as_ptr() }))
            .unwrap_or(NullKind::Strict(nelems));

        Array {
            _ptr,
            raw: Some(raw),
            nelems,
            elem_slice: /* SAFETY: &[Datum] from palloc'd [Datum] */ unsafe { slice::from_raw_parts(elements, nelems) },
            null_slice,
            elem_layout: Some(layout),
            _marker: PhantomData,
        }
    }

    pub fn into_array_type(mut self) -> *const pg_sys::ArrayType {
        let ptr = mem::take(&mut self.raw).map(|raw| raw.into_ptr().as_ptr() as _);
        mem::forget(self);
        ptr.unwrap_or(ptr::null())
    }

    // # Panics
    //
    // Panics if it detects the slightest misalignment between types,
    // or if a valid slice contains nulls, which may be uninit data.
    #[deprecated(
        since = "0.5.0",
        note = "this function cannot be safe and is not generically sound\n\
        even `unsafe fn as_slice(&self) -> &[T]` is not sound for all `&[T]`\n\
        if you are sure your usage is sound, consider RawArray"
    )]
    pub fn as_slice(&self) -> &[T] {
        if let Some(Layout {
            size, passbyval, ..
        }) = &self.elem_layout
        {
            if self.null_slice.any() {
                panic!("null detected: can't expose potentially uninit data as a slice!")
            }
            const DATUM_SIZE: usize = mem::size_of::<pg_sys::Datum>();
            let sizeof_type = match (passbyval, mem::size_of::<T>(), size.try_as_usize()) {
                (true, rs @ (1 | 2 | 4 | 8), Some(pg @ (1 | 2 | 4 | 8))) if rs == pg => rs,
                (true, _, _) => panic!("invalid sizes for pass-by-value datum"),
                (false, DATUM_SIZE, _) => DATUM_SIZE,
                (false, _, _) => panic!("invalid sizes for pass-by-reference datum"),
            };
            match (sizeof_type, self.raw.as_ref()) {
                // SAFETY: Rust slice layout matches Postgres data layout and this array is "owned"
                (1 | 2 | 4, Some(raw)) => unsafe { raw.assume_init_data_slice::<T>() },
                (DATUM_SIZE, _) => {
                    let sizeof_datums = mem::size_of_val(self.elem_slice);
                    unsafe {
                        slice::from_raw_parts(
                            self.elem_slice.as_ptr() as *const T,
                            sizeof_datums / sizeof_type,
                        )
                    }
                }
                (_, _) => panic!("no correctly-sized slice exists"),
            }
        } else {
            panic!("not enough type information to slice correctly")
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
        if let Some(at) = &self.raw {
            // SAFETY: if Some, then the ArrayType is from Postgres
            if unsafe { at.any_nulls() } {
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
            Some(unsafe { T::from_datum(self.elem_slice[i], self.null_slice.get(i)?) })
        }
    }
}

pub struct VariadicArray<'a, T: FromDatum>(Array<'a, T>);

impl<'a, T: FromDatum + serde::Serialize> serde::Serialize for VariadicArray<'a, T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(self.0.iter())
    }
}

impl<'a, T: FromDatum> VariadicArray<'a, T> {
    pub fn into_array_type(self) -> *const pg_sys::ArrayType {
        self.0.into_array_type()
    }

    // # Panics
    //
    // Panics if it detects the slightest misalignment between types,
    // or if a valid slice contains nulls, which may be uninit data.
    #[deprecated(
        since = "0.5.0",
        note = "this function cannot be safe and is not generically sound\n\
        even `unsafe fn as_slice(&self) -> &[T]` is not sound for all `&[T]`\n\
        if you are sure your usage is sound, consider RawArray"
    )]
    #[allow(deprecated)]
    pub fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }

    /// Return an Iterator of Option<T> over the contained Datums.
    pub fn iter(&self) -> ArrayIterator<'_, T> {
        self.0.iter()
    }

    /// Return an Iterator of the contained Datums (converted to Rust types).
    ///
    /// This function will panic when called if the array contains any SQL NULL values.
    pub fn iter_deny_null(&self) -> ArrayTypedIterator<'_, T> {
        self.0.iter_deny_null()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[allow(clippy::option_option)]
    #[inline]
    pub fn get(&self, i: usize) -> Option<Option<T>> {
        self.0.get(i)
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

impl<'a, T: FromDatum + serde::Serialize> serde::Serialize for ArrayTypedIterator<'a, T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(self.array.iter())
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

impl<'a, T: FromDatum> IntoIterator for VariadicArray<'a, T> {
    type Item = Option<T>;
    type IntoIter = ArrayIntoIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        ArrayIntoIterator {
            array: self.0,
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

impl<'a, T: FromDatum> FromDatum for VariadicArray<'a, T> {
    #[inline]
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool) -> Option<VariadicArray<'a, T>> {
        Array::from_datum(datum, is_null).map(Self)
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
            let raw =
                RawArray::from_ptr(NonNull::new(array).expect("detoast returned null ArrayType*"));
            let ptr = NonNull::new(ptr);
            let oid = raw.oid();
            let layout = Layout::lookup_oid(oid);

            Some(Array::deconstruct_from(ptr, raw, layout))
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

impl<'a, T> SqlTranslatable for Array<'a, T>
where
    T: SqlTranslatable + FromDatum,
{
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        match T::argument_sql()? {
            SqlMapping::As(sql) => Ok(SqlMapping::As(format!("{sql}[]"))),
            SqlMapping::Skip => Err(ArgumentError::SkipInArray),
            SqlMapping::Composite { .. } => Ok(SqlMapping::Composite {
                array_brackets: true,
            }),
            SqlMapping::Source { .. } => Ok(SqlMapping::Source {
                array_brackets: true,
            }),
        }
    }

    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        match T::return_sql()? {
            ReturnVariant::Plain(SqlMapping::As(sql)) => {
                Ok(ReturnVariant::Plain(SqlMapping::As(format!("{sql}[]"))))
            }
            ReturnVariant::Plain(SqlMapping::Composite { array_brackets: _ }) => {
                Ok(ReturnVariant::Plain(SqlMapping::Composite {
                    array_brackets: true,
                }))
            }
            ReturnVariant::Plain(SqlMapping::Source { array_brackets: _ }) => {
                Ok(ReturnVariant::Plain(SqlMapping::Source {
                    array_brackets: true,
                }))
            }
            ReturnVariant::Plain(SqlMapping::Skip) => Err(ReturnVariantError::SkipInArray),
            ReturnVariant::SetOf(_) => Err(ReturnVariantError::SetOfInArray),
            ReturnVariant::Table(_) => Err(ReturnVariantError::TableInArray),
        }
    }
}

impl<'a, T> SqlTranslatable for VariadicArray<'a, T>
where
    T: SqlTranslatable + FromDatum,
{
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        match T::argument_sql()? {
            SqlMapping::As(sql) => Ok(SqlMapping::As(format!("{sql}[]"))),
            SqlMapping::Skip => Err(ArgumentError::SkipInArray),
            SqlMapping::Composite { .. } => Ok(SqlMapping::Composite {
                array_brackets: true,
            }),
            SqlMapping::Source { .. } => Ok(SqlMapping::Source {
                array_brackets: true,
            }),
        }
    }

    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        match T::return_sql()? {
            ReturnVariant::Plain(SqlMapping::As(sql)) => {
                Ok(ReturnVariant::Plain(SqlMapping::As(format!("{sql}[]"))))
            }
            ReturnVariant::Plain(SqlMapping::Composite { array_brackets: _ }) => {
                Ok(ReturnVariant::Plain(SqlMapping::Composite {
                    array_brackets: true,
                }))
            }
            ReturnVariant::Plain(SqlMapping::Source { array_brackets: _ }) => {
                Ok(ReturnVariant::Plain(SqlMapping::Source {
                    array_brackets: true,
                }))
            }
            ReturnVariant::Plain(SqlMapping::Skip) => Err(ReturnVariantError::SkipInArray),
            ReturnVariant::SetOf(_) => Err(ReturnVariantError::SetOfInArray),
            ReturnVariant::Table(_) => Err(ReturnVariantError::TableInArray),
        }
    }

    fn variadic() -> bool {
        true
    }
}
