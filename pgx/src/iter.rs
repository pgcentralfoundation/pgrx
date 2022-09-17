use pgx_utils::sql_entity_graph::metadata::{
    ArgumentError, ReturnVariant, ReturnVariantError, SqlMapping, SqlTranslatable,
};
use std::panic::{RefUnwindSafe, UnwindSafe};

use crate::{pg_sys, IntoDatum};

pub struct SetOfIterator<'a, T>
where
    T: UnwindSafe + RefUnwindSafe,
{
    iter: Box<dyn Iterator<Item = T> + UnwindSafe + RefUnwindSafe + 'a>,
}

impl<'a, T> SetOfIterator<'a, T>
where
    T: UnwindSafe + RefUnwindSafe,
{
    pub fn new<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T> + UnwindSafe + 'a,
        <I as IntoIterator>::IntoIter: UnwindSafe + RefUnwindSafe,
    {
        Self {
            iter: Box::new(iter.into_iter()),
        }
    }
}

impl<'a, T> Iterator for SetOfIterator<'a, T>
where
    T: UnwindSafe + RefUnwindSafe,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, T> SqlTranslatable for SetOfIterator<'a, T>
where
    T: SqlTranslatable + UnwindSafe + RefUnwindSafe,
{
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        T::argument_sql()
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        match T::return_sql() {
            Ok(ReturnVariant::Plain(sql)) => Ok(ReturnVariant::SetOf(sql)),
            Ok(ReturnVariant::SetOf(_)) => Err(ReturnVariantError::NestedSetOf),
            Ok(ReturnVariant::Table(_)) => Err(ReturnVariantError::SetOfContainingTable),
            err @ Err(_) => err,
        }
    }
}

pub struct TableIterator<'a, T>
where
    T: UnwindSafe + RefUnwindSafe,
{
    iter: Box<dyn Iterator<Item = T> + UnwindSafe + RefUnwindSafe + 'a>,
}

impl<'a, T> TableIterator<'a, T>
where
    T: UnwindSafe + RefUnwindSafe,
{
    pub fn new<I>(iter: I) -> Self
    where
        I: Iterator<Item = T> + UnwindSafe + RefUnwindSafe + 'a,
    {
        Self {
            iter: Box::new(iter),
        }
    }
}

impl<'a, T> Iterator for TableIterator<'a, T>
where
    T: UnwindSafe + RefUnwindSafe,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, T> IntoDatum for TableIterator<'a, T>
where
    T: SqlTranslatable + UnwindSafe + RefUnwindSafe,
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
            impl<'a, #(Input~N,)*> SqlTranslatable for TableIterator<'a, (#(Input~N,)*)>
            where
                #(
                    Input~N: SqlTranslatable + UnwindSafe + RefUnwindSafe + 'static,
                )*
            {
                fn argument_sql() -> Result<SqlMapping, ArgumentError> {
                    Err(ArgumentError::Table)
                }
                fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
                    let mut vec = Vec::new();
                    #(
                        vec.push(match Input~N::return_sql() {
                            Ok(ReturnVariant::Plain(sql)) => sql,
                            Ok(ReturnVariant::SetOf(_)) => return Err(ReturnVariantError::TableContainingSetOf),
                            Ok(ReturnVariant::Table(_)) => return Err(ReturnVariantError::NestedTable),
                            Err(err) => return Err(err),
                        });
                    )*
                    Ok(ReturnVariant::Table(vec))
                }
            }
        });
    )*
});
