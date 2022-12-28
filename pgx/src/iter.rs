use std::iter::once;

use crate::IntoHeapTuple;
use pgx_sql_entity_graph::metadata::{
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
/// use pgx::prelude::*;
/// #[pg_extern]
/// fn return_ints() -> SetOfIterator<'static, i32> {
///     SetOfIterator::new(1..=5)
/// }
/// ```
///
/// Here we return a set of `&str`s, borrowed from an argument:
///
/// ```rust,no_run
/// use pgx::prelude::*;
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
/// must also be "named" using pgx' [`name!`] macro.  See the examples below.
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
/// use pgx::prelude::*;
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
/// use pgx::prelude::*;
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

seq_macro::seq!(I in 0..=32 {
    #(
        seq_macro::seq!(N in 0..=I {
            unsafe impl<'a, #(Input~N,)*> SqlTranslatable for TableIterator<'a, (#(Input~N,)*)>
            where
                #(
                    Input~N: SqlTranslatable + 'a,
                )*
            {
                fn argument_sql() -> Result<SqlMapping, ArgumentError> {
                    Err(ArgumentError::Table)
                }
                fn return_sql() -> Result<Returns, ReturnsError> {
                    let mut vec = Vec::new();
                    #(
                        vec.push(match Input~N::return_sql() {
                            Ok(Returns::One(sql)) => sql,
                            Ok(Returns::SetOf(_)) => return Err(ReturnsError::TableContainingSetOf),
                            Ok(Returns::Table(_)) => return Err(ReturnsError::NestedTable),
                            Err(err) => return Err(err),
                        });
                    )*
                    Ok(Returns::Table(vec))
                }
            }
        });
    )*
});
