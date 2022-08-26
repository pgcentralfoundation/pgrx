use crate::datum::{Array, FromDatum};
use crate::pg_sys;
use core::ptr::{slice_from_raw_parts_mut, NonNull};
use core::marker::PhantomData;
use pgx_pg_sys::*;

extern "C" {
    /// # Safety
    /// Does a field access, but doesn't deref out of bounds of ArrayType
    fn pgx_ARR_DATA_PTR(arrayType: *mut ArrayType) -> *mut u8;
    /// # Safety
    /// Does a field access, but doesn't deref out of bounds of ArrayType
    fn pgx_ARR_DIMS(arrayType: *mut ArrayType) -> *mut libc::c_int;
    /// # Safety
    /// Must only be used on a "valid" (Postgres-constructed) ArrayType
    fn pgx_ARR_NELEMS(arrayType: *mut ArrayType) -> i32;
    /// # Safety
    /// Does a field access, but doesn't deref out of bounds of ArrayType
    fn pgx_ARR_NULLBITMAP(arrayType: *mut ArrayType) -> *mut bits8;
}

/**
An aligned, dereferenceable `NonNull<ArrayType>` with low-level accessors.

It offers technically-safe accessors to the "dynamic" fields of a Postgres varlena array,
as well as any fields not covered by such, but does not require any more validity.
This also means that the [NonNull] pointers that are returned may not be valid to read.
Validating the correctness of the entire array requires a bit more effort.

# On sizes and subscripts

Postgres uses C's `int` (`c_int` in Rust) for sizes, and Rust uses [usize].
Thus various functions of ArrayPtr return `c_int` values, but you must convert to usize.
On 32-bit or 64-bit machines with 32-bit `c_int`s, you may losslessly upgrade `as usize`,
except with negative indices, which Postgres asserts against creating.
PGX currently only intentionally supports 64-bit machines,
and while support for ILP32 or I64LP128 C data models may become possible,
PGX will **not** support 16-bit machines in any practical case, even though Rust does.
*/
#[repr(transparent)]
#[derive(Debug)]
pub struct ArrayPtr {
    at: NonNull<ArrayType>,
}

#[deny(unsafe_op_in_unsafe_fn)]
impl ArrayPtr {
    // General implementation notes:
    // ArrayPtr is not Copy or Clone, making it harder to misuse versus *mut ArrayType.
    // But this also offers safe accessors to the fields, like &ArrayType,
    // so it requires validity assertions in order to be constructed.
    // The main reason it uses NonNull and denies Clone, however, is access soundness:
    // It is not sound to go from &mut Type to *mut T if *mut T is not a field of Type.
    // This creates an obvious complication for handing out pointers into varlenas.
    // Thus also why this does not use lifetime-bounded borrows.

    /// Returns a handle to the raw array header.
    ///
    /// # Safety
    ///
    /// When calling this method, you have to ensure that all of the following is true:
    /// * The pointer must be properly aligned.
    /// * It must be "dereferenceable" in the sense defined in [the std documentation].
    /// * The pointer must point to an initialized instance of `ArrayType`.
    /// * This can be considered a unique, "owning pointer",
    ///   so it won't be aliased while ArrayPtr is held,
    ///   and it points to data in the Postgres ArrayType format.
    ///
    /// It should be noted as ArrayPtr is not inherently lifetime-bound, it can be racy and unsafe!
    ///
    /// [the std documentation]: core::ptr#safety
    pub unsafe fn from_ptr(at: NonNull<ArrayType>) -> ArrayPtr {
        ArrayPtr { at }
    }

    /// # Safety
    ///
    /// Array must have been constructed from an ArrayType pointer.
    pub unsafe fn from_array<T: FromDatum>(arr: Array<T>) -> Option<ArrayPtr> {
        let array_type = arr.into_array_type() as *mut _;
        Some(ArrayPtr {
            at: NonNull::new(array_type)?,
        })
    }

    /// Returns the inner raw pointer to the ArrayType.
    pub fn into_raw(self) -> NonNull<ArrayType> {
        self.at
    }

    /// Get the number of dimensions.
    pub fn ndims(&self) -> libc::c_int {
        // SAFETY: Validity asserted on construction.
        unsafe {
            (*self.at.as_ptr()).ndim
            // FIXME: While this is a c_int, the max ndim is normally 6
            // While the value can be set higher, it is... unlikely
            // that it is going to actually challenge even 16-bit pointer widths.
            // It would be preferable to return a usize instead,
            // however, PGX has trouble with that, unfortunately.
            as _
        }
    }

    /// A raw slice of the dimensions.
    /// Oxidized form of ARR_DIMS(ArrayType*)
    ///
    /// # Safety
    ///
    /// This requires &mut to collect the slice, but be aware: this is a raw pointer!
    /// If you find ways to store it, you are probably violating ownership.
    /// Raw pointer validity is **asserted on dereference, not construction**,
    /// so remember, this slice is only guaranteed to be valid almost immediately
    /// after obtaining it, or if you continue to hold the ArrayPtr.
    pub fn dims(&mut self) -> NonNull<[libc::c_int]> {
        // for expected behavior, see:
        // postgres/src/include/utils/array.h
        // #define ARR_DIMS

        // SAFETY: Welcome to the infernal bowels of FFI.
        // Because the initial ptr was NonNull, we can assume this is also NonNull.
        // As validity of the initial ptr was asserted on construction of ArrayPtr,
        // this can assume the dims ptr is also valid, allowing making the slice.
        // This code doesn't assert validity per se, but in practice,
        // the caller will probably immediately turn this into a borrowed slice,
        // opening up the methods that are available on borrowed slices.
        //
        // So, to be clear, yes, everything done so far allows the caller to do so,
        // though it is possible the caller can misuse this in various ways.
        // Only the "naive" case is well-guarded.
        unsafe {
            let len = self.ndims() as usize;
            NonNull::new_unchecked(slice_from_raw_parts_mut(
                pgx_ARR_DIMS(self.at.as_ptr()),
                len,
            ))
        }
    }

    /// The flattened length of the array.
    pub fn len(&self) -> libc::c_int {
        // SAFETY: Validity asserted on construction, and...
        // ...well, hopefully Postgres knows what it's doing.
        unsafe {
            pgx_ARR_NELEMS(self.at.as_ptr())
            // FIXME: While this was indeed a function that returns a c_int,
            // using a usize is more idiomatic in Rust, to say the least.
            // In addition, the actual sizes are under various restrictions,
            // so we probably can further constrain the value, honestly.
            // However, PGX has trouble with returning usizes
            as _
        }
    }

    pub fn oid(&self) -> pg_sys::Oid {
        // SAFETY: Validity asserted on construction.
        unsafe { (*self.at.as_ptr()).elemtype }
    }

    /// Gets the offset to the ArrayType's data.
    /// Note that this should not be "taken literally".
    fn data_offset(&self) -> i32 {
        // SAFETY: Validity asserted on construction.
        unsafe { (*self.at.as_ptr()).dataoffset }
        // This field is an "int32" in Postgres
    }

    /// Equivalent to ARR_HASNULL(ArrayType*)
    /// Note this means that it only asserts that there MIGHT be a null
    pub fn nullable(&self) -> bool {
        // for expected behavior, see:
        // postgres/src/include/utils/array.h
        // #define ARR_HASNULL
        self.data_offset() != 0
    }

    /// Oxidized form of ARR_NULLBITMAP(ArrayType*)
    pub fn nulls(&self) -> Option<NonNull<[u8]>> {
        // for expected behavior, see:
        // postgres/src/include/utils/array.h
        // #define ARR_NULLBITMAP
        let len = (self.len() as usize + 7) >> 3; // Obtains 0 if len was 0.

        // SAFETY: This obtains the nulls pointer, which is valid to obtain because
        // the validity was asserted on construction. However, unlike the other cases,
        // it isn't correct to trust it. Instead, this gets null-checked.
        // This is because, while the initial pointer is NonNull,
        // ARR_NULLBITMAP can return a nullptr!
        let nulls = unsafe { slice_from_raw_parts_mut(pgx_ARR_NULLBITMAP(self.at.as_ptr()), len) };
        Some(NonNull::new(nulls)?)
    }

    /// The slice of the data.
    /// Oxidized form of ARR_DATA_PTR(ArrayType*)
    ///
    /// # Safety
    ///
    /// This is not inherently typesafe!
    /// Thus you must know the implied type of the underlying ArrayType when calling this.
    /// In addition, the raw slice is not guaranteed to be legible at any given index,
    /// e.g. it may be an "SQL null" if so indicated in the null bitmap.
    /// But even if the index is not marked as null, the value may be equal to nullptr,
    /// thus leaving it correct to read the value but incorrect to then dereference.
    ///
    /// That is why this returns [`NonNull<[T]>`]: if it returned `&mut [T]`,
    /// then for many possible types that can be **undefined behavior**,
    /// as it would assert that each particular index was a valid `T`.
    /// A Rust borrow, including of a slice, will always be
    /// - non-null
    /// - aligned
    /// - **validly initialized**, except in the case of [MaybeUninit] types
    /// It can be incorrect to assume that data that Postgres has marked "null"
    /// follows Rust-level initialization requirements.
    ///
    /// [MaybeUninit]: core::mem::MaybeUninit
    /// [`NonNull<[T]>`]: core::ptr::NonNull
    pub unsafe fn data<T>(&mut self) -> NonNull<[T]> {
        let len = self.len() as usize;

        // SAFETY: Welcome to the infernal bowels of FFI.
        // Because the initial ptr was NonNull, we can assume this is also NonNull.
        // As validity of the initial ptr was asserted on construction of ArrayPtr,
        // this can assume the data ptr is also valid, allowing making the slice.
        // This code doesn't assert validity per se, but in practice,
        // the caller will probably immediately turn this into a borrowed slice,
        // opening up the methods that are available on borrowed slices.
        //
        // Most importantly, the caller has asserted this is in fact a valid [T],
        // by calling this function, so both this code and the caller can rely
        // on that assertion, only requiring that it is correct.
        unsafe {
            NonNull::new_unchecked(slice_from_raw_parts_mut(
                pgx_ARR_DATA_PTR(self.at.as_ptr()).cast(),
                len,
            ))
        }
    }
}

/**
A pointer into a Postgres varlena.
*/
pub struct RawArray<T> {
    _ptr: ArrayPtr,
    _len: usize,
    ty: PhantomData<T>,
}
