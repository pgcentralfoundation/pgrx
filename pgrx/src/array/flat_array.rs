use crate::datum::{Array, BorrowDatum, Datum};
use crate::layout::{Align, Layout};
use crate::memcx::MemCx;
use crate::nullable::Nullable;
use crate::palloc::PBox;
use crate::pgrx_sql_entity_graph::metadata::{
    ArgumentError, Returns, ReturnsError, SqlMapping, SqlTranslatable,
};
use crate::toast::{Toast, Toasty};
use crate::{layout, pg_sys, varlena};
use bitvec::ptr::{self as bitptr, BitPtr, BitPtrError, Const, Mut};
use bitvec::slice::{self as bitslice, BitSlice};
use core::iter::{ExactSizeIterator, FusedIterator};
use core::marker::PhantomData;
use core::{ffi, mem, ptr, slice};

use super::port;
use super::{Element, RawArray, Scalar};

/** `pg_sys::ArrayType` and its unsized varlena

# Safety
`&FlatArray<'_, T>` assumes its tail is the remainder of a Postgres array of element `T`.
*/
#[repr(C)]
#[derive(Debug)]
pub struct FlatArray<'mcx, T: ?Sized> {
    scalar: PhantomData<&'mcx T>,
    head: pg_sys::ArrayType,
    tail: [u8],
}

impl<'mcx, T> FlatArray<'mcx, T>
where
    T: ?Sized,
{
    fn as_raw(&self) -> RawArray {
        unsafe {
            let ptr = ptr::NonNull::new_unchecked(ptr::from_ref(self).cast_mut());
            RawArray::from_ptr(ptr.cast())
        }
    }

    /// Number of elements in the array, including nulls
    ///
    /// Note that for many arrays, this doesn't have a linear relationship with array byte-len.
    #[doc(alias = "cardinality")]
    pub fn nelems(&self) -> usize {
        self.as_raw().len()
    }

    /// Number of dimensions the array has
    pub fn ndims(&self) -> usize {
        self.head.ndim as _
    }

    pub fn contains_nulls(&self) -> bool {
        // SAFETY: Constructive validity from ref and function is non-mutating
        unsafe { pg_sys::array_contains_nulls((&raw const self.head).cast_mut()) }
    }
}

// TODO: remove `non_exhaustive` when the errors have been worked out
#[non_exhaustive]
#[derive(Debug)]
pub enum ArrayAllocError {
    TooManyBytes {
        over: usize,
    },
    TooManyElems {
        over: usize,
    },
    /// One or more dimensions are zero
    ZeroLenDim(usize),
}

const MAX_ALLOC_SIZE: usize = 0x3fffffff;
const MAX_ARRAY_SIZE: usize = MAX_ALLOC_SIZE / size_of::<pg_sys::Datum>();
const MAX_DIMS: usize = pg_sys::MAXDIM as usize;
// COMPAT: this has to be the last field of ArrayType
const _ARRAY_TYPE_IS_PADDING_FREE: () = assert!(
    size_of::<pg_sys::ArrayType>()
        == (mem::offset_of!(pg_sys::ArrayType, elemtype) + size_of::<pg_sys::Oid>())
);

impl<'mcx, T> FlatArray<'mcx, T>
where
    T: Scalar + Sized,
{
    pub fn new_zeroed_in<'cx, const N: usize>(
        dims: [usize; N],
        has_nulls: bool,
        memcx: &MemCx<'cx>,
    ) -> Result<PBox<'cx, FlatArray<'cx, T>>, ArrayAllocError> {
        let base_size = size_of::<pg_sys::ArrayType>();
        let ndims = N;
        let dims_size = size_of::<ffi::c_int>() * ndims;
        const { assert!(N != 0) };
        const { assert!(N <= MAX_DIMS) };
        let dims = dims.map(|i| ffi::c_int::try_from(i).unwrap());
        let mut product = 1i32;
        let lbounds = dims.map(|dim| {
            product = product.checked_mul(dim).unwrap();
            product + 1
        });
        let nelems = product as usize;

        let null_size = if has_nulls { nelems.div_ceil(8) } else { 0 };

        let prefix_size = base_size + dims_size * 2 + null_size;
        const MAX_ELEM_ALIGN: usize = pg_sys::MAXIMUM_ALIGNOF as _;
        const { assert!(align_of::<T>() <= MAX_ELEM_ALIGN) };
        let prefix_size = prefix_size.next_multiple_of(MAX_ELEM_ALIGN);
        let size = prefix_size + size_of::<T>() * nelems;

        if let Some((zlen_dim, _)) = dims.into_iter().enumerate().find(|(i, len)| *len == 0) {
            return Err(ArrayAllocError::ZeroLenDim(zlen_dim));
        }
        if let Some(over) = nelems.checked_sub(MAX_ARRAY_SIZE + 1) {
            return Err(ArrayAllocError::TooManyElems { over });
        }
        if let Some(over) = size.checked_sub(MAX_ALLOC_SIZE + 1) {
            return Err(ArrayAllocError::TooManyBytes { over });
        }

        if let Ok(nbytes) = i32::try_from(size)
            && let Ok(dataoffset) = i32::try_from(prefix_size)
        {
            let ptr = memcx.alloc_zeroed_bytes(size).as_ptr();

            let dataoffset = if has_nulls { dataoffset } else { 0 };
            let elemtype = <T as Scalar>::OID;

            let head_ptr = ptr.cast::<pg_sys::ArrayType>();
            // SAFETY: we've allocated enough space so we can initialize everything
            unsafe {
                // COMPAT: assign so fields must be initialized even if ArrayType changes
                // SAFETY: _ARRAY_TYPE_IS_PADDING_FREE means we will not deinitialize any bytes
                (*head_ptr) = pg_sys::ArrayType {
                    vl_len_: varlena::encode_vlen_4b(nbytes) as i32,
                    ndim: ndims as ffi::c_int,
                    dataoffset,
                    elemtype,
                };
                *(head_ptr.add(base_size).cast()) = dims;
                *(head_ptr.add(base_size + dims_size).cast()) = lbounds;
            }
            let ptr = ptr::slice_from_raw_parts_mut(ptr, size - base_size);
            let ptr = ptr as *mut FlatArray<_>;
            let ptr = ptr::NonNull::new(ptr).unwrap();

            // SAFETY: size of the metadata matches the bytes of the varlena header,
            // and there is no padding in ArrayType to make any offsets incorrect
            Ok(unsafe { PBox::from_raw_in(ptr, memcx) })
        } else {
            // Shouldn't happen?
            unreachable!()
        }
    }
}

impl<'mcx, T> FlatArray<'mcx, T>
where
    T: ?Sized + Element,
{
    /// Iterate the array
    #[doc(alias = "unnest")]
    pub fn iter(&self) -> ArrayIter<'_, T> {
        let nelems = self.nelems();
        let raw = self.as_raw();
        let nulls =
            raw.nulls_bitptr().map(|p| unsafe { bitslice::from_raw_parts(p, nelems).unwrap() });

        let data = unsafe { ptr::NonNull::new_unchecked(raw.data_ptr().cast_mut()) };
        let arr = self;
        let index = 0;
        let offset = 0;
        let align = Layout::lookup_oid(self.head.elemtype).align;

        ArrayIter { data, nulls, nelems, arr, index, offset, align }
    }

    pub fn iter_non_null(&self) -> impl Iterator<Item = &T> {
        self.iter().filter_map(|elem| elem.into_option())
    }

    /// Borrow the nth element (0-indexed)
    ///
    /// `FlatArray::get` may have to iterate elements, so this is `O(n)` in the general case
    pub fn get(&self, index: usize) -> Option<Nullable<&T>> {
        self.iter().nth(index)
    }

    /// Obtain `&[T]` if the array has no nulls
    pub fn as_non_null_slice(&self) -> Option<&[T]>
    where
        T: Scalar,
    {
        if self.contains_nulls() {
            None
        } else {
            let raw = self.as_raw();
            // SAFETY: Sound if the bound of `T: Scalar` holds and the type fulfills those requirements
            Some(unsafe { slice::from_raw_parts(raw.data_ptr() as *const _, raw.len()) })
        }
    }

    /// Obtain `&mut [T]` if the array has no nulls
    pub fn as_non_null_slice_mut(&mut self) -> Option<&mut [T]>
    where
        T: Scalar,
    {
        if self.contains_nulls() {
            None
        } else {
            let elements = self.nelems();
            // SAFETY: We start with a valid ArrayType
            let data_ptr = unsafe { port::ARR_DATA_PTR(&raw mut self.head as _) };
            // SAFETY: Sound if the bound of `T: Scalar` holds and there are no nulls
            Some(unsafe { slice::from_raw_parts_mut(data_ptr.cast(), elements) })
        }
    }

    pub fn nullbitmap_bytes(&self) -> Option<&[u8]> {
        let len = self.nelems().div_ceil(8);

        // SAFETY: This obtains the nulls pointer from a function that must either
        // return a null pointer or a pointer to a valid null bitmap.
        unsafe {
            let nulls_ptr = port::ARR_NULLBITMAP(ptr::addr_of!(self.head).cast_mut());
            ptr::slice_from_raw_parts(nulls_ptr, len).as_ref()
        }
    }
}

unsafe impl<T: ?Sized> BorrowDatum for FlatArray<'_, T> {
    const PASS: layout::PassBy = layout::PassBy::Ref;
    unsafe fn point_from(ptr: ptr::NonNull<u8>) -> ptr::NonNull<Self> {
        unsafe {
            let len =
                varlena::varsize_any(ptr.as_ptr().cast()) - mem::size_of::<pg_sys::ArrayType>();
            ptr::NonNull::new_unchecked(
                ptr::slice_from_raw_parts_mut(ptr.as_ptr(), len) as *mut Self
            )
        }
    }
}

// `T[]` in Postgres
//
// # Safety
// Note that this is currently only implemented for `&FlatArray<'_, T>`, because we cannot assume
// that any datum passed from the outside is mutable
unsafe impl<T> SqlTranslatable for FlatArray<'_, T>
where
    T: ?Sized + SqlTranslatable + Element,
{
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        match T::argument_sql()? {
            SqlMapping::As(sql) => Ok(SqlMapping::As(format!("{sql}[]"))),
            SqlMapping::Skip => Err(ArgumentError::SkipInArray),
            SqlMapping::Composite { .. } => Ok(SqlMapping::Composite { array_brackets: true }),
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
            Returns::One(SqlMapping::Skip) => Err(ReturnsError::SkipInArray),
            Returns::SetOf(_) => Err(ReturnsError::SetOfInArray),
            Returns::Table(_) => Err(ReturnsError::TableInArray),
        }
    }
}

/// Iterator for arrays
#[derive(Clone)]
pub struct ArrayIter<'arr, T>
where
    T: ?Sized + Element,
{
    arr: &'arr FlatArray<'arr, T>,
    data: ptr::NonNull<u8>,
    nulls: Option<&'arr BitSlice<u8>>,
    nelems: usize,
    index: usize,
    offset: usize,
    align: Align,
}

impl<'arr, T> Iterator for ArrayIter<'arr, T>
where
    T: ?Sized + Element,
{
    type Item = Nullable<&'arr T>;

    fn next(&mut self) -> Option<Nullable<&'arr T>> {
        if self.index >= self.nelems {
            return None;
        }
        let is_null = match self.nulls {
            Some(nulls) => !nulls.get(self.index).unwrap(),
            None => false,
        };
        // note the index freezes when we reach the end, fusing the iterator
        self.index += 1;

        if is_null {
            // note that we do NOT offset when the value is a null!
            Some(Nullable::Null)
        } else {
            let borrow = unsafe { T::borrow_unchecked(self.data.add(self.offset)) };
            // As we always have a borrow, we just ask Rust what the array element's size is
            self.offset += self.align.pad(mem::size_of_val(borrow));
            Some(Nullable::Valid(borrow))
        }
    }
}

impl<'arr, 'mcx, T> IntoIterator for &'arr FlatArray<'mcx, T>
where
    T: ?Sized + Element,
{
    type IntoIter = ArrayIter<'arr, T>;
    type Item = Nullable<&'arr T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'arr, T> ExactSizeIterator for ArrayIter<'arr, T> where T: ?Sized + Element {}
impl<'arr, T> FusedIterator for ArrayIter<'arr, T> where T: ?Sized + Element {}
