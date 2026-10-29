use super::Datum;
use super::from::convert_varlena_to_str_memoized;
use super::uuid::Uuid;
use crate::prelude::*;
use crate::varlena::varlena_to_byte_slice;
use crate::{Json, JsonB};
use alloc::ffi::CString;
use core::ffi::CStr;

/// Directly convert a Datum into this type
///
/// Previously, pgrx used FromDatum exclusively, which captures conversion into T, but
/// leaves ambiguous a number of possibilities and allows large swathes of behavior to
/// be folded under a single trait. This provided certain beneficial ergonomics at first,
/// but eventually behavior was incorrectly folded under FromDatum, which provided the
/// illusion of soundness.
///
/// UnboxDatum should only be implemented for a type that CAN be directly converted from a Datum,
/// and it doesn't say whether it can be used directly or if it should be detoasted via MemCx.
/// It's currently just a possibly-temporary shim to make pgrx work.
///
/// # Safety
/// This trait is used to bound the lifetime of certain types: thus the associated type must be
/// this type but "infected" by the Datum's lifetime. By implementing this, you verify that you
/// are implementing this in the way that satisfies that lifetime constraint. There isn't really
/// a good way to constrain lifetimes correctly without forcing from-Datum types to go through a
/// wrapper type bound by the lifetime of the Datum. And what would you use as the bound, hmm?
///
/// If `As<'src>` does not name `'src`, safe code must not be able to read the Datum's memory
/// through the value that `unbox` returns, because `Vec<T>` and `Array::into_iter` keep such
/// values after they free the array they unboxed them from. A raw [`pg_sys::Datum`] meets this,
/// because only `unsafe` code can dereference it, and `PgBox<T>` meets it by unboxing a copy.
/// [`PgHeapTuple::get_by_index`] relies on the same rule but reads through [`FromDatum`], so the
/// value that such a type's `from_polymorphic_datum` returns must meet it too, unless the type
/// sets [`FromDatum::POINTS_INTO_DATUM`] and its `from_datum_in_memory_context` returns a value
/// that does not read the Datum's memory.
///
/// `unbox` must be sound for every value of a type that [`UnboxDatum::can_unbox`] accepts and
/// that the caller's type comparison matched to `Self`. Override `can_unbox` to reject the types
/// whose values `unbox` cannot read.
pub unsafe trait UnboxDatum {
    // TODO: Currently, this doesn't actually get used to identify all the cases where the Datum
    // is actually a pointer type. However, it has been noted that Postgres does yield nullptr
    // on occasion, even when they say something is not supposed to be nullptr. As it is common
    // for Postgres to init [Datum<'_>] with palloc0, it is reasonable to assume nullptr is a risk,
    // even if `is_null == false`.
    //
    // Wait, what are you about, Jubilee? In some cases, the chance of nullness doesn't exist!
    // This is because we are materializing the datums from e.g. pointers to an Array, which
    // requires you to have had a valid base pointer into an ArrayType to start!
    // That's why you started using this goofy GAT scheme in the first place!
    /// Self with the lifetime `'src`
    ///
    /// The lifetime `'src` represents the lifetime of the "root" source of this Datum, which
    /// may be a memory context or a shorter-lived type inside that context.
    ///
    type As<'src>
    where
        Self: 'src;

    /// Can [`UnboxDatum::unbox`] read a value of a type with this length and pass-by-value
    /// flag, PostgreSQL's `typlen` and `typbyval`?
    ///
    /// [`Array`] asks once, for its element type, before it unboxes any element, and panics if
    /// the answer is no. The default accepts every type, leaving the check to the type
    /// comparison that matched the array to `Self`. `PgBox<T>` overrides it, because it copies
    /// `size_of::<T>()` bytes from each element.
    #[inline]
    fn can_unbox(_typlen: i16, _typbyval: bool) -> bool {
        true
    }

    /// Convert from `Datum<'src>` to `T::As<'src>`
    ///
    /// This should be the direct conversion in each case. It is very unsafe to use directly.
    ///
    /// # Safety
    /// Due to the absence of an `is_null` parameter, this does not validate "SQL nullness" of
    /// the Datum in question. The intention is that this core fn eschews an additional branch.
    /// Just... don't use it if it might be null?
    ///
    /// This also should not be used as the primary conversion mechanism if it requires a MemCx,
    /// as this is intended primarily to be used in cases where the datum is guaranteed to be
    /// detoasted already.
    ///
    /// The Datum's type must be one that [`UnboxDatum::can_unbox`] accepts.
    unsafe fn unbox<'src>(datum: Datum<'src>) -> Self::As<'src>
    where
        Self: 'src;
}

// SAFETY: SQL "char" is one byte. DatumGetChar reads its native C representation;
// the final cast preserves that byte's bits in pgrx's signed Rust representation.
unsafe impl UnboxDatum for i8 {
    type As<'src> = i8;
    #[inline]
    unsafe fn unbox<'src>(datum: Datum<'src>) -> Self::As<'src>
    where
        Self: 'src,
    {
        // SAFETY: UnboxDatum's caller guarantees the datum has this initialized scalar type.
        // Where this conversion is a guarded native call, the guard panics off the
        // backend thread.
        #[allow(unused_unsafe, reason = "pure where it is a macro or translated")]
        let value = unsafe { pg_sys::DatumGetChar!(datum.0).get() };
        value as i8
    }
}

// SAFETY: i16 is the native value representation of this PostgreSQL scalar.
unsafe impl UnboxDatum for i16 {
    type As<'src> = i16;
    #[inline]
    unsafe fn unbox<'src>(datum: Datum<'src>) -> Self::As<'src>
    where
        Self: 'src,
    {
        // SAFETY: UnboxDatum's caller guarantees the datum has this initialized scalar type.
        // Where this conversion is a guarded native call, the guard panics off the
        // backend thread.
        #[allow(unused_unsafe, reason = "pure where it is a macro or translated")]
        let value = unsafe { pg_sys::DatumGetInt16!(datum.0).get() };
        value
    }
}

// SAFETY: i32 is the native value representation of this PostgreSQL scalar.
unsafe impl UnboxDatum for i32 {
    type As<'src> = i32;
    #[inline]
    unsafe fn unbox<'src>(datum: Datum<'src>) -> Self::As<'src>
    where
        Self: 'src,
    {
        // SAFETY: UnboxDatum's caller guarantees the datum has this initialized scalar type.
        // Where this conversion is a guarded native call, the guard panics off the
        // backend thread.
        #[allow(unused_unsafe, reason = "pure where it is a macro or translated")]
        let value = unsafe { pg_sys::DatumGetInt32!(datum.0).get() };
        value
    }
}

// SAFETY: i64 is the native value representation of this PostgreSQL scalar.
unsafe impl UnboxDatum for i64 {
    type As<'src> = i64;
    #[inline]
    unsafe fn unbox<'src>(datum: Datum<'src>) -> Self::As<'src>
    where
        Self: 'src,
    {
        // SAFETY: UnboxDatum's caller guarantees the datum has this initialized scalar type.
        // Where this conversion is a guarded native call, the guard panics off the
        // backend thread.
        #[allow(unused_unsafe, reason = "pure where it is a macro or translated")]
        let value = unsafe { pg_sys::DatumGetInt64!(datum.0).get() };
        value
    }
}

// SAFETY: bool is the native value representation of this PostgreSQL scalar.
unsafe impl UnboxDatum for bool {
    type As<'src> = bool;
    #[inline]
    unsafe fn unbox<'src>(datum: Datum<'src>) -> Self::As<'src>
    where
        Self: 'src,
    {
        // SAFETY: UnboxDatum's caller guarantees the datum has this initialized scalar type.
        // Where this conversion is a guarded native call, the guard panics off the
        // backend thread.
        #[allow(unused_unsafe, reason = "pure where it is a macro or translated")]
        let value = unsafe { pg_sys::DatumGetBool!(datum.0).get() };
        value
    }
}

// SAFETY: f32 is the native value representation of this PostgreSQL scalar.
unsafe impl UnboxDatum for f32 {
    type As<'src> = f32;
    #[inline]
    unsafe fn unbox<'src>(datum: Datum<'src>) -> Self::As<'src>
    where
        Self: 'src,
    {
        f32::from_bits(datum.0.value() as u32)
    }
}

// SAFETY: f64 is the native value representation of this PostgreSQL scalar.
unsafe impl UnboxDatum for f64 {
    type As<'src> = f64;
    #[inline]
    unsafe fn unbox<'src>(datum: Datum<'src>) -> Self::As<'src>
    where
        Self: 'src,
    {
        f64::from_bits(datum.0.value() as u64)
    }
}

/// Panics, as `FromDatum` for `&str` does, if the database encoding lets text hold bytes that are
/// not UTF-8 and the value does.
unsafe impl UnboxDatum for str {
    type As<'src> = &'src str;
    #[inline]
    unsafe fn unbox<'src>(datum: Datum<'src>) -> Self::As<'src>
    where
        Self: 'src,
    {
        unsafe { convert_varlena_to_str_memoized(datum.0.cast_mut_ptr()) }
    }
}

unsafe impl UnboxDatum for &str {
    #[rustfmt::skip]
    type As<'src> = &'src str where Self: 'src;
    #[inline]
    unsafe fn unbox<'src>(datum: Datum<'src>) -> Self::As<'src>
    where
        Self: 'src,
    {
        unsafe { convert_varlena_to_str_memoized(datum.0.cast_mut_ptr()) }
    }
}

unsafe impl UnboxDatum for CStr {
    type As<'src> = &'src CStr;
    #[inline]
    unsafe fn unbox<'src>(datum: Datum<'src>) -> Self::As<'src>
    where
        Self: 'src,
    {
        unsafe { CStr::from_ptr(datum.0.cast_mut_ptr()) }
    }
}

unsafe impl UnboxDatum for &CStr {
    #[rustfmt::skip]
    type As<'src> = &'src CStr where Self: 'src;
    #[inline]
    unsafe fn unbox<'src>(datum: Datum<'src>) -> Self::As<'src>
    where
        Self: 'src,
    {
        unsafe { CStr::from_ptr(datum.0.cast_mut_ptr()) }
    }
}

unsafe impl UnboxDatum for [u8] {
    type As<'src> = &'src [u8];
    #[inline]
    unsafe fn unbox<'src>(datum: Datum<'src>) -> Self::As<'src>
    where
        Self: 'src,
    {
        unsafe { varlena_to_byte_slice(datum.0.cast_mut_ptr()) }
    }
}

unsafe impl UnboxDatum for &[u8] {
    #[rustfmt::skip]
    type As<'src> = &'src [u8] where Self: 'src;
    #[inline]
    unsafe fn unbox<'src>(datum: Datum<'src>) -> Self::As<'src>
    where
        Self: 'src,
    {
        unsafe { varlena_to_byte_slice(datum.0.cast_mut_ptr()) }
    }
}

unsafe impl UnboxDatum for pg_sys::Oid {
    type As<'src> = pg_sys::Oid;
    #[inline]
    unsafe fn unbox<'src>(datum: Datum<'src>) -> Self::As<'src>
    where
        Self: 'src,
    {
        // SAFETY: UnboxDatum's caller guarantees this is an initialized OID datum.
        // Where this conversion is a guarded native call, the guard panics off the
        // backend thread.
        #[allow(unused_unsafe, reason = "pure where it is a macro or translated")]
        let value = unsafe { pg_sys::DatumGetObjectId!(datum.0).get() };
        pg_sys::Oid::from(value)
    }
}

unsafe impl UnboxDatum for String {
    type As<'src> = String;
    #[inline]
    unsafe fn unbox<'src>(datum: Datum<'src>) -> Self::As<'src>
    where
        Self: 'src,
    {
        unsafe { str::unbox(datum) }.to_owned()
    }
}

unsafe impl UnboxDatum for CString {
    type As<'src> = CString;
    #[inline]
    unsafe fn unbox<'src>(datum: Datum<'src>) -> Self::As<'src>
    where
        Self: 'src,
    {
        unsafe { CStr::unbox(datum) }.to_owned()
    }
}

unsafe impl UnboxDatum for pg_sys::Datum {
    type As<'src> = pg_sys::Datum;
    #[inline]
    unsafe fn unbox<'src>(datum: Datum<'src>) -> Self::As<'src>
    where
        Self: 'src,
    {
        datum.0
    }
}

unsafe impl UnboxDatum for Uuid {
    type As<'src> = Uuid;
    #[inline]
    unsafe fn unbox<'src>(datum: Datum<'src>) -> Self::As<'src>
    where
        Self: 'src,
    {
        Uuid::from_bytes(datum.0.cast_mut_ptr::<[u8; 16]>().read())
    }
}

unsafe impl UnboxDatum for Date {
    type As<'src> = Date;
    #[inline]
    unsafe fn unbox<'src>(datum: Datum<'src>) -> Self::As<'src>
    where
        Self: 'src,
    {
        Date::try_from(i32::unbox(datum)).unwrap_unchecked()
    }
}

unsafe impl UnboxDatum for Time {
    type As<'src> = Time;
    #[inline]
    unsafe fn unbox<'src>(datum: Datum<'src>) -> Self::As<'src>
    where
        Self: 'src,
    {
        Time::try_from(i64::unbox(datum)).unwrap_unchecked()
    }
}

unsafe impl UnboxDatum for Timestamp {
    type As<'src> = Timestamp;
    #[inline]
    unsafe fn unbox<'src>(datum: Datum<'src>) -> Self::As<'src>
    where
        Self: 'src,
    {
        Timestamp::try_from(i64::unbox(datum)).unwrap_unchecked()
    }
}

unsafe impl UnboxDatum for TimestampWithTimeZone {
    type As<'src> = TimestampWithTimeZone;
    #[inline]
    unsafe fn unbox<'src>(datum: Datum<'src>) -> Self::As<'src>
    where
        Self: 'src,
    {
        TimestampWithTimeZone::try_from(i64::unbox(datum)).unwrap_unchecked()
    }
}

macro_rules! unbox_with_fromdatum {
    ($($from_ty:ty,)*) => {
        $(
            unsafe impl UnboxDatum for $from_ty {
                type As<'src> = $from_ty;
                unsafe fn unbox<'src>(datum: Datum<'src>) -> Self::As<'src> where Self: 'src {
                    Self::from_datum(datum.0, false).unwrap()
                }
            }
        )*
    }
}

unbox_with_fromdatum! {
    TimeWithTimeZone, AnyNumeric, char, pg_sys::Point, Interval, pg_sys::BOX, pg_sys::ItemPointerData,
}

unsafe impl UnboxDatum for PgHeapTuple<'_, crate::AllocatedByRust> {
    #[rustfmt::skip]
    type As<'src> = PgHeapTuple<'src, AllocatedByRust> where Self: 'src;
    #[inline]
    unsafe fn unbox<'src>(d: Datum<'src>) -> Self::As<'src>
    where
        Self: 'src,
    {
        PgHeapTuple::from_datum(d.0, false).unwrap()
    }
}

unsafe impl<T: FromDatum + UnboxDatum> UnboxDatum for Array<'_, T> {
    #[rustfmt::skip]
    type As<'src> = Array<'src, T> where Self: 'src;
    unsafe fn unbox<'src>(d: Datum<'src>) -> Array<'src, T>
    where
        Self: 'src,
    {
        Array::from_datum(d.0, false).unwrap()
    }
}

unsafe impl<T: FromDatum + UnboxDatum> UnboxDatum for VariadicArray<'_, T> {
    #[rustfmt::skip]
    type As<'src> = VariadicArray<'src, T> where Self: 'src;
    unsafe fn unbox<'src>(d: Datum<'src>) -> VariadicArray<'src, T>
    where
        Self: 'src,
    {
        VariadicArray::from_datum(d.0, false).unwrap()
    }
}

unsafe impl<T: FromDatum + UnboxDatum + RangeSubType> UnboxDatum for Range<T> {
    #[rustfmt::skip]
    type As<'src> = Range<T> where Self: 'src;
    unsafe fn unbox<'src>(d: Datum<'src>) -> Self::As<'src>
    where
        Self: 'src,
    {
        Range::<T>::from_datum(d.0, false).unwrap()
    }
}

unsafe impl<const P: u32, const S: u32> UnboxDatum for Numeric<P, S> {
    type As<'src> = Numeric<P, S>;
    #[inline]
    unsafe fn unbox<'src>(d: Datum<'src>) -> Self::As<'src>
    where
        Self: 'src,
    {
        Numeric::from_datum(d.0, false).unwrap()
    }
}

/// Unboxing copies `size_of::<T>()` bytes into a new allocation in `CurrentMemoryContext`.
///
/// The Datum points to a value inside its source, such as an element of an [`Array`], and a
/// `PgBox` has no lifetime to keep that source alive. An `Array` frees its detoasted copy when it
/// drops, so `Vec<PgBox<T>>` and `Array::into_iter` would otherwise return dangling boxes. Each
/// unbox, including each `Array::get`, makes a new copy that lives until its memory context is
/// reset or deleted. Writing through the box does not change the source.
///
/// [`UnboxDatum::can_unbox`] accepts only a fixed-length type passed by reference whose values
/// are `size_of::<T>()` bytes, and only if palloc's MAXALIGN alignment suffices for `T`. A copy of
/// a variable-length value would keep only its first `size_of::<T>()` bytes, and PostgreSQL would
/// read past it when it received the box back. A pass-by-value Datum holds the value itself, not
/// a pointer to it.
// SAFETY: the returned box points to its own allocation, not into the Datum's memory.
unsafe impl<T> UnboxDatum for PgBox<T, AllocatedByPostgres> {
    #[rustfmt::skip]
    type As<'src> = PgBox<T> where Self: 'src;

    #[inline]
    fn can_unbox(typlen: i16, typbyval: bool) -> bool {
        !typbyval
            && usize::try_from(typlen).is_ok_and(|len| len > 0 && len == size_of::<T>())
            && align_of::<T>() <= pg_sys::MAXIMUM_ALIGNOF as usize
    }

    #[inline]
    unsafe fn unbox<'src>(d: Datum<'src>) -> Self::As<'src>
    where
        Self: 'src,
    {
        // SAFETY: the caller passes a Datum of a type that `can_unbox` accepts, so it points to
        // a value of `size_of::<T>()` initialized bytes; `copy_ptr_into` panics on a null pointer
        // and copies the bytes without reading them as a `T`. The copy is a palloc chunk, which
        // `from_pg` may box, aligned to MAXALIGN, which `can_unbox` checked suffices for `T`.
        // That the bytes are a valid `T` rests on the type comparison that matched the Datum's
        // type to `T`.
        unsafe {
            let copy = crate::PgMemoryContexts::CurrentMemoryContext
                .copy_ptr_into(d.0.cast_mut_ptr::<T>(), size_of::<T>());
            PgBox::from_pg(copy)
        }
    }
}

unsafe impl UnboxDatum for Json {
    #[rustfmt::skip]
    type As<'src> = Json where Self: 'src;
    #[inline]
    unsafe fn unbox<'src>(d: Datum<'src>) -> Self::As<'src>
    where
        Self: 'src,
    {
        Json::from_datum(d.0, false).unwrap()
    }
}

unsafe impl UnboxDatum for JsonB {
    #[rustfmt::skip]
    type As<'src> = JsonB where Self: 'src;
    #[inline]
    unsafe fn unbox<'src>(d: Datum<'src>) -> Self::As<'src>
    where
        Self: 'src,
    {
        JsonB::from_datum(d.0, false).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pgbox_unboxes_only_fixed_length_references_of_its_size() {
        type Point = PgBox<pg_sys::Point>;
        assert!(Point::can_unbox(16, false));
        // Passed by value, so the Datum is not a pointer.
        assert!(!Point::can_unbox(16, true));
        assert!(!Point::can_unbox(8, false));
        assert!(!Point::can_unbox(24, false));
        // Variable-length and C string types.
        assert!(!Point::can_unbox(-1, false));
        assert!(!Point::can_unbox(-2, false));
        assert!(!PgBox::<()>::can_unbox(0, false));

        // palloc aligns only to MAXALIGN.
        #[repr(align(32))]
        struct OverAligned {
            _bytes: [u8; 32],
        }
        assert!(!PgBox::<OverAligned>::can_unbox(32, false));

        // Other types leave the check to their type comparison.
        assert!(<pg_sys::Datum as UnboxDatum>::can_unbox(-1, false));
    }
}
