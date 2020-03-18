use crate::{pg_sys, void_mut_ptr, FromDatum, IntoDatum, PgMemoryContexts};
use serde::Serializer;
use std::marker::PhantomData;

pub type VariadicArray<'a, T> = Array<'a, T>;

pub struct Array<'a, T: FromDatum<T>> {
    ptr: *mut pg_sys::varlena,
    array_type: *mut pg_sys::ArrayType,
    elements: *mut pg_sys::Datum,
    nulls: *mut bool,
    typoid: pg_sys::Oid,
    nelems: usize,
    elem_slice: &'a [pg_sys::Datum],
    null_slice: &'a [bool],
    _marker: PhantomData<T>,
}

impl<'a, T: FromDatum<T> + serde::Serialize> serde::Serialize for Array<'a, T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(self.iter())
    }
}

impl<'a, T: FromDatum<T> + serde::Serialize> serde::Serialize for ArrayTypedIterator<'a, T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(self.array.iter())
    }
}

impl<'a, T: FromDatum<T>> Array<'a, T> {
    /// Create an [`Array`] over an array of [pg_sys::Datum] values and a corresponding array
    /// of "is_null" indicators
    ///
    /// [`T`] can be [pg_sys::Datum] if the elements are not all of the same type
    ///
    /// # Safety
    ///
    /// This function is unsafe as it can't validate the provided pointer are valid or that
    ///
    pub unsafe fn over(
        elements: *mut pg_sys::Datum,
        nulls: *mut bool,
        nelems: usize,
    ) -> Array<'a, T> {
        Array::<T> {
            ptr: std::ptr::null_mut(),
            array_type: std::ptr::null_mut(),
            elements,
            nulls,
            typoid: pg_sys::InvalidOid,
            nelems,
            elem_slice: std::slice::from_raw_parts(elements, nelems),
            null_slice: std::slice::from_raw_parts(nulls, nelems),
            _marker: PhantomData,
        }
    }

    fn from_pg(
        ptr: *mut pg_sys::varlena,
        array_type: *mut pg_sys::ArrayType,
        elements: *mut pg_sys::Datum,
        nulls: *mut bool,
        typoid: pg_sys::Oid,
        nelems: usize,
    ) -> Self {
        Array::<T> {
            ptr,
            array_type,
            elements,
            nulls,
            typoid,
            nelems,
            elem_slice: unsafe { std::slice::from_raw_parts(elements, nelems) },
            null_slice: unsafe { std::slice::from_raw_parts(nulls, nelems) },
            _marker: PhantomData,
        }
    }

    pub fn into_array_type(self) -> *const pg_sys::ArrayType {
        if self.array_type.is_null() {
            panic!("attempt to dereference a NULL array");
        }

        let ptr = self.array_type;
        std::mem::forget(self);
        ptr
    }

    pub fn as_slice(&self) -> &[T] {
        let sizeof_type = std::mem::size_of::<T>();
        let sizeof_datums = std::mem::size_of_val(self.elem_slice);
        unsafe {
            std::slice::from_raw_parts(
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
        if self.array_type.is_null() {
            panic!("array is NULL");
        } else if unsafe { pg_sys::array_contains_nulls(self.array_type) } {
            panic!("array contains NULL");
        }

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
            Some(unsafe { T::from_datum(self.elem_slice[i], self.null_slice[i], self.typoid) })
        }
    }
}

pub struct ArrayTypedIterator<'a, T: 'a + FromDatum<T>> {
    array: &'a Array<'a, T>,
    curr: usize,
}

impl<'a, T: FromDatum<T>> Iterator for ArrayTypedIterator<'a, T> {
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

pub struct ArrayIterator<'a, T: 'a + FromDatum<T>> {
    array: &'a Array<'a, T>,
    curr: usize,
}

impl<'a, T: FromDatum<T>> Iterator for ArrayIterator<'a, T> {
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

pub struct ArrayIntoIterator<'a, T: FromDatum<T>> {
    array: Array<'a, T>,
    curr: usize,
}

impl<'a, T: FromDatum<T>> IntoIterator for Array<'a, T> {
    type Item = Option<T>;
    type IntoIter = ArrayIntoIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        ArrayIntoIterator {
            array: self,
            curr: 0,
        }
    }
}

impl<'a, T: FromDatum<T>> Iterator for ArrayIntoIterator<'a, T> {
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

impl<'a, T: FromDatum<T>> Drop for Array<'a, T> {
    fn drop(&mut self) {
        if !self.elements.is_null() {
            unsafe {
                pg_sys::pfree(self.elements as void_mut_ptr);
            }
        }

        if !self.nulls.is_null() {
            unsafe {
                pg_sys::pfree(self.nulls as void_mut_ptr);
            }
        }

        if !self.array_type.is_null() && self.array_type as *mut pg_sys::varlena != self.ptr {
            unsafe {
                pg_sys::pfree(self.array_type as void_mut_ptr);
            }
        }

        // NB:  we don't pfree(self.ptr) because we don't know if it's actually
        // safe to do that.  It'll be freed whenever Postgres deletes/resets its parent
        // MemoryContext
    }
}

impl<'a, T: FromDatum<T>> FromDatum<Array<'a, T>> for Array<'a, T> {
    #[inline]
    unsafe fn from_datum(datum: usize, is_null: bool, typoid: u32) -> Option<Array<'a, T>> {
        if is_null {
            None
        } else if datum == 0 {
            panic!("array was flagged not null but datum is zero");
        } else {
            let ptr = datum as *mut pg_sys::varlena;
            let array =
                pg_sys::pg_detoast_datum(datum as *mut pg_sys::varlena) as *mut pg_sys::ArrayType;
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
            let mut elements = std::ptr::null_mut();
            let mut nulls = std::ptr::null_mut();
            let mut nelems = 0;

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
                ptr,
                array,
                elements,
                nulls,
                typoid,
                nelems as usize,
            ))
        }
    }
}

impl<T: FromDatum<T>> FromDatum<Vec<Option<T>>> for Vec<Option<T>> {
    #[inline]
    unsafe fn from_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        typoid: u32,
    ) -> Option<Vec<Option<T>>> {
        if is_null {
            None
        } else if datum == 0 {
            panic!("array was flagged not null but datum is zero");
        } else {
            let array = Array::<T>::from_datum(datum, is_null, typoid).unwrap();
            let mut v = Vec::with_capacity(array.len());

            for element in array.iter() {
                v.push(element)
            }
            Some(v)
        }
    }
}

impl<T> IntoDatum<Vec<T>> for Vec<T>
where
    T: IntoDatum<T>,
{
    fn into_datum(self) -> Option<pg_sys::Datum> {
        let mut state = std::ptr::null_mut();
        for s in self {
            let datum = s.into_datum();
            let isnull = datum.is_none();

            unsafe {
                state = pg_sys::accumArrayResult(
                    state,
                    datum.unwrap_or(0usize),
                    isnull,
                    T::type_oid(),
                    PgMemoryContexts::CurrentMemoryContext.value(),
                );
            }
        }

        Some(unsafe {
            pg_sys::makeArrayResult(state, PgMemoryContexts::CurrentMemoryContext.value())
        })
    }

    fn type_oid() -> u32 {
        unsafe { pg_sys::get_array_type(T::type_oid()) }
    }
}
