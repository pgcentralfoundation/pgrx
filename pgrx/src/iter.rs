//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
#![allow(clippy::vec_init_then_push)]
use std::iter::once;

use crate::{pg_sys, IntoDatum, IntoHeapTuple};
use pgrx_sql_entity_graph::metadata::{
    ArgumentError, Returns, ReturnsError, SqlMapping, SqlTranslatable,
};

/// Support for returning a `SETOF T` from an SQL function.
///
/// [`SetOfIterator`] is typically used as a return type on `#[pg_extern]`-style functions
/// and indicates that the SQL function should return a `SETOF` over the generic argument `T`.
///
/// It is a lightweight wrapper around an iterator, which you provide during construction.  The
/// iterator *can* borrow from its environment, following Rust's normal borrowing rules.  If no
/// borrowing is necessary, the `'static` lifetime should be used.
///
/// # Examples
///
/// This example simply returns a set of integers in the range `1..=5`.
///
/// ```rust,no_run
/// use pgrx::prelude::*;
/// #[pg_extern]
/// fn return_ints() -> SetOfIterator<'static, i32> {
///     SetOfIterator::new(1..=5)
/// }
/// ```
///
/// Here we return a set of `&str`s, borrowed from an argument:
///
/// ```rust,no_run
/// use pgrx::prelude::*;
/// #[pg_extern]
/// fn split_string<'a>(input: &'a str) -> SetOfIterator<'a, &'a str> {
///     SetOfIterator::new(input.split_whitespace())
/// }
/// ```
pub struct SetOfIterator<'a, T> {
    iter: Box<dyn Iterator<Item = T> + 'a>,
}

impl<'a, T> SetOfIterator<'a, T> {
    pub fn new<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T> + 'a,
    {
        Self { iter: Box::new(iter.into_iter()) }
    }
}

impl<'a, T> Iterator for SetOfIterator<'a, T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

unsafe impl<'a, T> SqlTranslatable for SetOfIterator<'a, T>
where
    T: SqlTranslatable,
{
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        T::argument_sql()
    }
    fn return_sql() -> Result<Returns, ReturnsError> {
        match T::return_sql() {
            Ok(Returns::One(sql)) => Ok(Returns::SetOf(sql)),
            Ok(Returns::SetOf(_)) => Err(ReturnsError::NestedSetOf),
            Ok(Returns::Table(_)) => Err(ReturnsError::SetOfContainingTable),
            err @ Err(_) => err,
        }
    }
}

/// Support for a `TABLE (...)` from an SQL function.
///
/// [`TableIterator`] is typically used as the return type of a `#[pg_extern]`-style function,
/// indicating that the function returns a table of named columns.  [`TableIterator`] is
/// generic over `T`, but that `T` must be a Rust tuple containing one or more elements.  They
/// must also be "named" using pgrx's [`name!`][crate::name] macro.  See the examples below.
///
/// It is a lightweight wrapper around an iterator, which you provide during construction.  The
/// iterator *can* borrow from its environment, following Rust's normal borrowing rules.  If no
/// borrowing is necessary, the `'static` lifetime should be used.
///
/// # Examples
///
/// This example returns a table of employee information.
///
/// ```rust,no_run
/// use pgrx::prelude::*;
/// #[pg_extern]
/// fn employees() -> TableIterator<'static,
///         (
///             name!(id, i64),
///             name!(dept_code, String),
///             name!(full_text, &'static str)
///         )
/// > {
///     TableIterator::new(vec![
///         (42, "ARQ".into(), "John Hammond"),
///         (87, "EGA".into(), "Mary Kinson"),
///         (3,  "BLA".into(), "Perry Johnson"),
///     ])
/// }
/// ```
///
/// And here we return a simple numbered list of words, borrowed from the input `&str`.
///
/// ```rust,no_run
/// use pgrx::prelude::*;
/// #[pg_extern]
/// fn split_string<'a>(input: &'a str) -> TableIterator<'a, ( name!(num, i32), name!(word, &'a str) )> {
///     TableIterator::new(input.split_whitespace().enumerate().map(|(n, w)| (n as i32, w)))
/// }
/// ```
pub struct TableIterator<'a, T> {
    iter: Box<dyn Iterator<Item = T> + 'a>,
}

impl<'a, T> TableIterator<'a, T>
where
    T: IntoHeapTuple + 'a,
{
    pub fn new<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T> + 'a,
    {
        Self { iter: Box::new(iter.into_iter()) }
    }

    pub fn once(value: T) -> Self {
        Self::new(once(value))
    }
}

impl<'a, T> Iterator for TableIterator<'a, T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

macro_rules! impl_table_iter {
    ($($C:ident),* $(,)?) => {
        unsafe impl<'iter, $($C,)*> SqlTranslatable for TableIterator<'iter, ($($C,)*)>
        where
            $($C: SqlTranslatable + 'iter,)*
        {
            fn argument_sql() -> Result<SqlMapping, ArgumentError> {
                Err(ArgumentError::Table)
            }
            fn return_sql() -> Result<Returns, ReturnsError> {
                let vec = vec![
                $(
                    match $C::return_sql() {
                        Ok(Returns::One(sql)) => sql,
                        Ok(Returns::SetOf(_)) => return Err(ReturnsError::TableContainingSetOf),
                        Ok(Returns::Table(_)) => return Err(ReturnsError::NestedTable),
                        err => return err,
                    },
                )*
                ];
                Ok(Returns::Table(vec))
            }
        }

        impl<$($C: IntoDatum),*> IntoHeapTuple for ($($C,)*) {
            unsafe fn into_heap_tuple(self, tupdesc: pg_sys::TupleDesc) -> *mut pg_sys::HeapTupleData {
                // shadowing the type names with these identifiers
                #[allow(nonstandard_style)]
                let ($($C,)*) = self;
                let datums = [$($C.into_datum(),)*];
                let mut nulls = datums.map(|option| option.is_none());
                let mut datums = datums.map(|option| option.unwrap_or(pg_sys::Datum::from(0)));


                unsafe {
                    // SAFETY:  Caller has asserted that `tupdesc` is valid, and we just went
                    // through a little bit of effort to setup properly sized arrays for
                    // `datums` and `nulls`
                    pg_sys::heap_form_tuple(tupdesc, datums.as_mut_ptr(), nulls.as_mut_ptr())
                }
            }
        }

    }
}

impl_table_iter!(T0);
impl_table_iter!(T0, T1);
impl_table_iter!(T0, T1, T2);
impl_table_iter!(T0, T1, T2, T3);
impl_table_iter!(T0, T1, T2, T3, T4);
impl_table_iter!(T0, T1, T2, T3, T4, T5);
impl_table_iter!(T0, T1, T2, T3, T4, T5, T6);
impl_table_iter!(T0, T1, T2, T3, T4, T5, T6, T7);
impl_table_iter!(T0, T1, T2, T3, T4, T5, T6, T7, T8);
impl_table_iter!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_table_iter!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_table_iter!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_table_iter!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
impl_table_iter!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
impl_table_iter!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
impl_table_iter!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
impl_table_iter!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
impl_table_iter!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
impl_table_iter!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18
);
impl_table_iter!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19
);
impl_table_iter!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20
);
impl_table_iter!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
    T21
);
impl_table_iter!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
    T21, T22
);
impl_table_iter!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
    T21, T22, T23
);
impl_table_iter!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
    T21, T22, T23, T24
);
impl_table_iter!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
    T21, T22, T23, T24, T25
);
impl_table_iter!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
    T21, T22, T23, T24, T25, T26
);
impl_table_iter!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
    T21, T22, T23, T24, T25, T26, T27
);
impl_table_iter!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
    T21, T22, T23, T24, T25, T26, T27, T28
);
impl_table_iter!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
    T21, T22, T23, T24, T25, T26, T27, T28, T29
);
impl_table_iter!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
    T21, T22, T23, T24, T25, T26, T27, T28, T29, T30
);
impl_table_iter!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
    T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31
);
