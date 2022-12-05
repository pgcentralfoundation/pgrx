use pgx_sql_entity_graph::metadata::{
    ArgumentError, Returns, ReturnsError, SqlMapping, SqlTranslatable,
};
use std::iter::once;

use crate::{pg_sys, IntoDatum};

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

pub struct TableIterator<'a, T> {
    iter: Box<dyn Iterator<Item = T> + 'a>,
}

impl<'a, T> TableIterator<'a, T> {
    pub fn new<I>(iter: I) -> Self
    where
        I: Iterator<Item = T> + 'a,
    {
        Self { iter: Box::new(iter) }
    }

    pub fn once(value: T) -> TableIterator<'a, T>
    where
        T: 'a,
    {
        Self { iter: Box::new(once(value)) }
    }
}

impl<'a, T> Iterator for TableIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, T> IntoDatum for TableIterator<'a, T>
where
    T: SqlTranslatable,
{
    fn into_datum(self) -> Option<pg_sys::Datum> {
        todo!()
    }

    fn type_oid() -> pg_sys::Oid {
        todo!()
    }
}

seq_macro::seq!(I in 0..=32 {
    #(
        seq_macro::seq!(N in 0..=I {
            unsafe impl<'a, #(Input~N,)*> SqlTranslatable for TableIterator<'a, (#(Input~N,)*)>
            where
                #(
                    Input~N: SqlTranslatable + 'static,
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
