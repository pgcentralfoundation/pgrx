use crate::{pg_sys, FromDatum};
use serde::Serializer;
use std::marker::PhantomData;

pub type VariadicArray<'a, T> = Array<'a, T>;

pub struct Array<'a, T: FromDatum<T>> {
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
    pub fn over(elements: *mut pg_sys::Datum, nulls: *mut bool, nelems: usize) -> Array<'a, T> {
        Array::<T> {
            array_type: std::ptr::null_mut(),
            elements,
            nulls,
            typoid: pg_sys::InvalidOid,
            nelems,
            elem_slice: unsafe { std::slice::from_raw_parts(elements, nelems) },
            null_slice: unsafe { std::slice::from_raw_parts(nulls, nelems) },
            _marker: PhantomData,
        }
    }

    fn from_pg(
        array_type: *mut pg_sys::ArrayType,
        elements: *mut pg_sys::Datum,
        nulls: *mut bool,
        typoid: pg_sys::Oid,
        nelems: usize,
    ) -> Self {
        Array::<T> {
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
        if self.array_type.is_null() {
            // don't pfree anything if the values aren't backed by a pg_sys::ArrayType
            return;
        }

        if !self.elements.is_null() {
            unsafe {
                pg_sys::pfree(self.elements as *mut std::os::raw::c_void);
            }
        }

        if !self.nulls.is_null() {
            unsafe {
                pg_sys::pfree(self.nulls as *mut std::os::raw::c_void);
            }
        }

        unsafe {
            pg_sys::pfree(self.array_type as *mut std::os::raw::c_void);
        }
    }
}

impl<'a, T: FromDatum<T>> FromDatum<Array<'a, T>> for Array<'a, T> {
    unsafe fn from_datum(datum: usize, is_null: bool, typoid: u32) -> Option<Array<'a, T>> {
        if is_null {
            None
        } else if datum == 0 {
            panic!("&[32] array was flagged not null but datum is zero");
        } else {
            let array =
                pg_sys::pg_detoast_datum(datum as *mut pg_sys::varlena) as *mut pg_sys::ArrayType;
            let array_ref = array.as_ref().expect("ArrayType * was NULL");

            // outvals for get_typlenbyvalalign()
            let mut typlen = 0;
            let mut typbyval = false;
            let mut typalign = 0;

            // outvals for deconstruct_array()
            let mut elements = 0 as *mut pg_sys::Datum;
            let mut nulls = 0 as *mut bool;
            let mut nelems = 0;

            pg_sys::get_typlenbyvalalign(
                array_ref.elemtype,
                &mut typlen,
                &mut typbyval,
                &mut typalign,
            );

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
                array,
                elements,
                nulls,
                typoid,
                nelems as usize,
            ))
        }
    }
}
