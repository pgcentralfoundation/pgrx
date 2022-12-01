//! Provides a safe interface to Postgres `HeapTuple` objects.
//!
//! [`PgHeapTuple`]s also describe composite types as defined by [`pgx::composite_type!()`][crate::composite_type].
use crate::pg_sys::{Datum, Oid};
use crate::{
    heap_getattr_raw, pg_sys, AllocatedByPostgres, AllocatedByRust, FromDatum, IntoDatum, PgBox,
    PgMemoryContexts, PgTupleDesc, TriggerTuple, TryFromDatumError, WhoAllocated,
};
use pgx_sql_entity_graph::metadata::{
    ArgumentError, Returns, ReturnsError, SqlMapping, SqlTranslatable,
};
use std::num::NonZeroUsize;

/// Describes errors that can occur when trying to create a new [PgHeapTuple].
#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum PgHeapTupleError {
    #[error("Incorrect attribute count, found {0}, descriptor had {1}")]
    IncorrectAttributeCount(usize, usize),

    #[error("The specified composite type, {0}, does not exist")]
    NoSuchType(String),
}

/// A [`PgHeapTuple`] is a lightweight wrapper around Postgres' [`pg_sys::HeapTuple`] object and a [`PgTupleDesc`].
///
/// In order to access the attributes within a [`pg_sys::HeapTuple`], the [`PgTupleDesc`] is required
/// to describe its structure.
///
/// [`PgHeapTuple`]s can be created from existing (Postgres-provided) [`pg_sys::HeapTuple`] pointers, from
/// [`pg_sys::TriggerData`] pointers, from a composite datum, or created from scratch using raw Datums.
///
/// A [`PgHeapTuple`] can either be considered to be allocated by Postgres or by the Rust runtime. If
/// allocated by Postgres, it is not mutable until [`PgHeapTuple::into_owned`] is called.
///
/// [`PgHeapTuple`]s also describe composite types as defined by [`pgx::composite_type!()`][crate::composite_type].
pub struct PgHeapTuple<'a, AllocatedBy: WhoAllocated> {
    tuple: PgBox<pg_sys::HeapTupleData, AllocatedBy>,
    tupdesc: PgTupleDesc<'a>,
}

impl<'a> FromDatum for PgHeapTuple<'a, AllocatedByRust> {
    unsafe fn from_polymorphic_datum(
        composite: pg_sys::Datum,
        is_null: bool,
        _oid: pg_sys::Oid,
    ) -> Option<Self> {
        if is_null {
            None
        } else {
            Some(PgHeapTuple::from_composite_datum(composite))
        }
    }

    unsafe fn from_datum_in_memory_context(
        mut memory_context: PgMemoryContexts,
        composite: Datum,
        is_null: bool,
        _oid: pg_sys::Oid,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        if is_null {
            None
        } else {
            memory_context.switch_to(|_| {
                // we're copying the composite datum into this memory context
                let tuple = PgHeapTuple::from_composite_datum(composite);
                let datum = tuple.into_composite_datum();
                Some(PgHeapTuple::from_composite_datum(datum.unwrap()))
            })
        }
    }
}

impl<'a> PgHeapTuple<'a, AllocatedByPostgres> {
    /// Creates a new [PgHeapTuple] from a [PgTupleDesc] and a [pg_sys::HeapTuple] pointer.  The
    /// returned [PgHeapTuple] will be considered by have been allocated by Postgres and is not mutable
    /// until [PgHeapTuple::into_owned] is called.
    ///
    /// ## Safety
    ///
    /// This function is unsafe as we cannot guarantee that the [pg_sys::HeapTuple] pointer is valid,
    /// nor can we guaratee that the provided [PgTupleDesc] properly describes the structure of
    /// the heap tuple.
    pub unsafe fn from_heap_tuple(tupdesc: PgTupleDesc<'a>, heap_tuple: pg_sys::HeapTuple) -> Self {
        Self { tuple: PgBox::from_pg(heap_tuple), tupdesc }
    }

    /// Creates a new [PgHeapTuple] from one of the two (`Current` or `New`) trigger tuples.  The returned
    /// [PgHeapTuple] will be considered by have been allocated by Postgres and is not mutable until
    /// [PgHeapTuple::into_owned] is called.  
    ///
    /// ## Safety
    ///
    /// This function is unsafe as we cannot guarantee that any pointers in the `trigger_data`
    /// argument are valid.
    pub unsafe fn from_trigger_data(
        trigger_data: &'a pg_sys::TriggerData,
        which_tuple: TriggerTuple,
    ) -> Option<PgHeapTuple<'a, AllocatedByPostgres>> {
        let tupdesc =
            PgTupleDesc::from_pg_unchecked(trigger_data.tg_relation.as_ref().unwrap().rd_att);

        let tuple = match which_tuple {
            TriggerTuple::Current => trigger_data.tg_trigtuple,
            TriggerTuple::New => trigger_data.tg_newtuple,
        };

        if tuple.is_null() {
            return None;
        }

        Some(PgHeapTuple::from_heap_tuple(tupdesc, tuple))
    }

    /// Consumes a `[PgHeapTuple]` considered to be allocated by Postgres and transforms it into one
    /// that is considered allocated by Rust.  This is accomplished by copying the underlying [pg_sys::HeapTupleData].
    pub fn into_owned(self) -> PgHeapTuple<'a, AllocatedByRust> {
        let copy = unsafe { pg_sys::heap_copytuple(self.tuple.into_pg()) };
        PgHeapTuple {
            tuple: unsafe { PgBox::<pg_sys::HeapTupleData, AllocatedByRust>::from_rust(copy) },
            tupdesc: self.tupdesc,
        }
    }
}

impl<'a> PgHeapTuple<'a, AllocatedByRust> {
    /** Create a new heap tuple in the shape of a defined composite type

    ```rust,no_run
    use pgx::prelude::*;

    Spi::run("CREATE TYPE dog AS (name text, age int);");
    let mut heap_tuple = PgHeapTuple::new_composite_type("dog").unwrap();

    assert_eq!(heap_tuple.get_by_name::<String>("name").unwrap(), None);
    assert_eq!(heap_tuple.get_by_name::<i32>("age").unwrap(), None);

    heap_tuple
        .set_by_name("name", "Brandy".to_string())
        .unwrap();
    heap_tuple.set_by_name("age", 42).unwrap();

    assert_eq!(
        heap_tuple.get_by_name("name").unwrap(),
        Some("Brandy".to_string())
    );
    assert_eq!(heap_tuple.get_by_name("age").unwrap(), Some(42i32));
    ```
    */
    pub fn new_composite_type(
        type_name: &str,
    ) -> Result<PgHeapTuple<'a, AllocatedByRust>, PgHeapTupleError> {
        let tuple_desc = PgTupleDesc::for_composite_type(type_name)
            .ok_or_else(|| PgHeapTupleError::NoSuchType(type_name.to_string()))?;
        let natts = tuple_desc.len();
        unsafe {
            let datums =
                pg_sys::palloc0(natts * std::mem::size_of::<pg_sys::Datum>()) as *mut pg_sys::Datum;
            let mut is_null = (0..natts).map(|_| true).collect::<Vec<_>>();

            let heap_tuple =
                pg_sys::heap_form_tuple(tuple_desc.as_ptr(), datums, is_null.as_mut_ptr());

            Ok(PgHeapTuple {
                tuple: PgBox::<pg_sys::HeapTupleData, AllocatedByRust>::from_rust(heap_tuple),
                tupdesc: tuple_desc,
            })
        }
    }

    /// Create a new [PgHeapTuple] from a [PgTupleDesc] from an iterator of Datums.
    ///
    /// ## Errors
    /// - [PgHeapTupleError::IncorrectAttributeCount] if the number of items in the iterator
    /// does not match the number of attributes in the [PgTupleDesc].
    pub fn from_datums<I: IntoIterator<Item = Option<pg_sys::Datum>>>(
        tupdesc: PgTupleDesc<'a>,
        datums: I,
    ) -> Result<PgHeapTuple<'a, AllocatedByRust>, PgHeapTupleError> {
        let iter = datums.into_iter();
        let mut datums = Vec::<pg_sys::Datum>::with_capacity(iter.size_hint().1.unwrap_or(1));
        let mut nulls = Vec::<bool>::with_capacity(iter.size_hint().1.unwrap_or(1));
        iter.for_each(|datum| {
            nulls.push(datum.is_none());
            datums.push(datum.unwrap_or(0.into()));
        });
        if datums.len() != tupdesc.len() {
            return Err(PgHeapTupleError::IncorrectAttributeCount(datums.len(), tupdesc.len()));
        }

        unsafe {
            let formed_tuple =
                pg_sys::heap_form_tuple(tupdesc.as_ptr(), datums.as_mut_ptr(), nulls.as_mut_ptr());

            Ok(Self {
                tuple: PgBox::<pg_sys::HeapTupleData, AllocatedByRust>::from_rust(formed_tuple),
                tupdesc,
            })
        }
    }

    /// Creates a new [PgHeapTuple] from an opaque Datum that should be a "composite" type.
    ///
    /// The Datum should be a pointer to a [pg_sys::HeapTupleHeader].  Typically, this will be used
    /// in situations when working with SQL `ROW(...)` constructors, or a composite SQL type such as
    ///
    /// ```sql
    /// CREATE TYPE my_composite AS (name text, age i32);
    /// ```
    ///
    /// ## Safety
    ///
    /// This function is unsafe as we cannot guarantee that the provided Datum is a valid [pg_sys::HeapTupleHeader]
    /// pointer.
    pub unsafe fn from_composite_datum(composite: pg_sys::Datum) -> Self {
        let htup_header =
            pg_sys::pg_detoast_datum(composite.cast_mut_ptr()) as pg_sys::HeapTupleHeader;
        let tup_type = crate::heap_tuple_header_get_type_id(htup_header);
        let tup_typmod = crate::heap_tuple_header_get_typmod(htup_header);
        let tupdesc = pg_sys::lookup_rowtype_tupdesc(tup_type, tup_typmod);

        let mut data = PgBox::<pg_sys::HeapTupleData>::alloc0();

        data.t_len = crate::heap_tuple_header_get_datum_length(htup_header) as u32;
        data.t_data = htup_header;

        Self { tuple: data, tupdesc: PgTupleDesc::from_pg(tupdesc) }
    }

    /// Given the name for an attribute in this [PgHeapTuple], change its value.
    ///
    /// Attribute names are case sensitive.
    ///
    /// ## Errors
    ///
    /// - return [TryFromDatumError::NoSuchAttributeName] if the attribute does not exist
    /// - return [TryFromDatumError::IncompatibleTypes] if the Rust type of the `value` is not
    /// compatible with the attribute's Postgres type
    pub fn set_by_name<T: IntoDatum>(
        &mut self,
        attname: &str,
        value: T,
    ) -> Result<(), TryFromDatumError> {
        match self.get_attribute_by_name(attname) {
            None => Err(TryFromDatumError::NoSuchAttributeName(attname.to_string())),
            Some((attnum, _)) => self.set_by_index(attnum, value),
        }
    }

    /// Given the index for an attribute in this [PgHeapTuple], change its value.
    ///
    /// Attribute numbers start at 1, not 0.
    ///
    /// ## Errors
    /// - return [TryFromDatumError::NoSuchAttributeNumber] if the attribute does not exist
    /// - return [TryFromDatumError::IncompatibleTypes] if the Rust type of the `value` is not
    /// compatible with the attribute's Postgres type
    pub fn set_by_index<T: IntoDatum>(
        &mut self,
        attno: NonZeroUsize,
        value: T,
    ) -> Result<(), TryFromDatumError> {
        unsafe {
            match self.get_attribute_by_index(attno) {
                None => return Err(TryFromDatumError::NoSuchAttributeNumber(attno)),
                Some(att) => {
                    let type_oid = T::type_oid();
                    let composite_type_oid = value.composite_type_oid();
                    let is_compatible_composite_types =
                        type_oid == pg_sys::RECORDOID && composite_type_oid == Some(att.atttypid);
                    if !is_compatible_composite_types && !T::is_compatible_with(att.atttypid) {
                        return Err(TryFromDatumError::IncompatibleTypes);
                    }
                }
            }

            let mut datums =
                (0..self.tupdesc.len()).map(|i| pg_sys::Datum::from(i)).collect::<Vec<_>>();
            let mut nulls = (0..self.tupdesc.len()).map(|_| false).collect::<Vec<_>>();
            let mut do_replace = (0..self.tupdesc.len()).map(|_| false).collect::<Vec<_>>();

            let datum = value.into_datum();
            let attno = attno.get() - 1;

            nulls[attno] = datum.is_none();
            datums[attno] = datum.unwrap_or(0.into());
            do_replace[attno] = true;

            let new_tuple = PgBox::<pg_sys::HeapTupleData, AllocatedByRust>::from_rust(
                pg_sys::heap_modify_tuple(
                    self.tuple.as_ptr(),
                    self.tupdesc.as_ptr(),
                    datums.as_mut_ptr(),
                    nulls.as_mut_ptr(),
                    do_replace.as_mut_ptr(),
                ),
            );
            let old_tuple = std::mem::replace(&mut self.tuple, new_tuple);
            drop(old_tuple);
            Ok(())
        }
    }
}

impl<'a, AllocatedBy: WhoAllocated> IntoDatum for PgHeapTuple<'a, AllocatedBy> {
    // Delegate to `into_composite_datum()` as this will normally be used with composite types.
    // See `into_trigger_datum()` if using as a trigger.
    fn into_datum(self) -> Option<pg_sys::Datum> {
        self.into_composite_datum()
    }

    fn type_oid() -> pg_sys::Oid {
        crate::pg_sys::RECORDOID
    }

    fn composite_type_oid(&self) -> Option<Oid> {
        Some(self.tupdesc.oid())
    }

    fn is_compatible_with(other: pg_sys::Oid) -> bool {
        fn is_composite(oid: pg_sys::Oid) -> bool {
            unsafe {
                let entry = pg_sys::lookup_type_cache(oid, pg_sys::TYPECACHE_TUPDESC as _);
                (*entry).typtype == pg_sys::RELKIND_COMPOSITE_TYPE as i8
            }
        }
        Self::type_oid() == other || is_composite(other)
    }
}

impl<'a, AllocatedBy: WhoAllocated> PgHeapTuple<'a, AllocatedBy> {
    /// Consume this [`PgHeapTuple`] and return a composite Datum representation, containing the tuple
    /// data and the corresponding tuple descriptor information.
    pub fn into_composite_datum(self) -> Option<pg_sys::Datum> {
        unsafe {
            Some(pg_sys::heap_copy_tuple_as_datum(self.tuple.as_ptr(), self.tupdesc.as_ptr()))
        }
    }

    /// Consume this [`PgHeapTuple`] and return a Datum representation appropriate for returning from
    /// a trigger function
    pub fn into_trigger_datum(self) -> Option<pg_sys::Datum> {
        self.tuple.into_datum()
    }

    /// Returns the number of attributes in this [`PgHeapTuple`].
    #[inline]
    pub fn len(&self) -> usize {
        self.tupdesc.len()
    }

    /// Returns an iterator over the attributes in this [`PgHeapTuple`].
    ///
    /// The return value is `(attribute_number: NonZeroUsize, attribute_info: &pg_sys::FormData_pg_attribute)`.
    pub fn attributes(
        &'a self,
    ) -> impl std::iter::Iterator<Item = (NonZeroUsize, &'a pg_sys::FormData_pg_attribute)> {
        self.tupdesc.iter().enumerate().map(|(i, att)| (NonZeroUsize::new(i + 1).unwrap(), att))
    }

    /// Get the attribute information for the specified attribute number.  
    ///
    /// Returns `None` if the attribute number is out of bounds.
    #[inline]
    pub fn get_attribute_by_index(
        &'a self,
        index: NonZeroUsize,
    ) -> Option<&'a pg_sys::FormData_pg_attribute> {
        self.tupdesc.get(index.get() - 1)
    }

    /// Get the attribute information for the specified attribute, by name.
    ///
    /// Returns `None` if the attribute name is not found.
    pub fn get_attribute_by_name(
        &'a self,
        name: &str,
    ) -> Option<(NonZeroUsize, &'a pg_sys::FormData_pg_attribute)> {
        for i in 0..self.len() {
            let i = NonZeroUsize::new(i + 1).unwrap();
            let att = self.get_attribute_by_index(i).unwrap();
            if att.name() == name {
                return Some((i, att));
            }
        }

        None
    }

    /// Retrieve the value of the specified attribute, by name.
    ///
    /// Attribute names are case-insensitive.
    ///
    /// ## Errors
    /// - return [`TryFromDatumError::NoSuchAttributeName`] if the attribute does not exist
    /// - return [`TryFromDatumError::IncompatibleTypes`] if the Rust type of the `value` is not
    /// compatible with the attribute's Postgres type
    pub fn get_by_name<T: FromDatum + IntoDatum + 'static>(
        &self,
        attname: &str,
    ) -> Result<Option<T>, TryFromDatumError> {
        // find the attribute number by name
        for att in self.tupdesc.iter() {
            if att.name() == attname {
                // we found the named attribute, so go get it from the HeapTuple
                return self.get_by_index(NonZeroUsize::new(att.attnum as usize).unwrap());
            }
        }

        // no attribute with the specified name
        Err(TryFromDatumError::NoSuchAttributeName(attname.to_owned()))
    }

    /// Retrieve the value of the specified attribute, by index.
    ///
    /// Attribute numbers start at 1, not 0.
    ///
    /// ## Errors
    /// - return [`TryFromDatumError::NoSuchAttributeNumber`] if the attribute does not exist
    /// - return [`TryFromDatumError::IncompatibleTypes`] if the Rust type of the `value` is not
    /// compatible with the attribute's Postgres type
    pub fn get_by_index<T: FromDatum + IntoDatum + 'static>(
        &self,
        attno: NonZeroUsize,
    ) -> Result<Option<T>, TryFromDatumError> {
        unsafe {
            // tuple descriptor attribute numbers are zero-based
            match self.tupdesc.get(attno.get() - 1) {
                // it's an attribute number outside the bounds of the tuple descriptor
                None => Err(TryFromDatumError::NoSuchAttributeNumber(attno)),

                // it's a valid attribute number
                Some(att) => {
                    let datum = heap_getattr_raw(self.tuple.as_ptr(), attno, self.tupdesc.as_ptr());
                    if datum.is_none() {
                        return Ok(None);
                    }
                    (match T::type_oid() {
                        record @ pg_sys::RECORDOID => {
                            T::try_from_datum(datum.unwrap(), false, record)
                        }
                        _ => T::try_from_datum(datum.unwrap(), false, att.type_oid().value()),
                    })
                    .map(Some)
                }
            }
        }
    }
}

/** Composite type support

Support for working with types defined by SQL statements like:

```sql
CREATE TYPE Dog AS (
    name TEXT,
    scritches INT
);
```

To PostgreSQL, these types are a [`pgx::pg_sys::HeapTuple`][crate::pg_sys::HeapTuple], which is a
pointer to a [`pgx::pg_sys::HeapTupleData`][crate::pg_sys::HeapTupleData]. `pgx` provides more idiomatic
wrapping of this type with [`pgx::heap_tuple::PgHeapTuple`][crate::heap_tuple::PgHeapTuple].

This `composite_type!()` macro expands into a [`pgx::heap_tuple::PgHeapTuple`][crate::heap_tuple::PgHeapTuple].

```rust
assert_eq!(
    core::any::TypeId::of::<pgx::composite_type!("Dog")>(),
    core::any::TypeId::of::<pgx::heap_tuple::PgHeapTuple<'static, ::pgx::AllocatedByRust>>(),
);
assert_eq!(
    core::any::TypeId::of::<pgx::composite_type!('static, "Dog")>(),
    core::any::TypeId::of::<pgx::heap_tuple::PgHeapTuple<'static, ::pgx::AllocatedByRust>>(),
);
const DOG_COMPOSITE_TYPE_IDENT: &str = "Dog";
assert_eq!(
    core::any::TypeId::of::<pgx::composite_type!('static, DOG_COMPOSITE_TYPE_IDENT)>(),
    core::any::TypeId::of::<pgx::heap_tuple::PgHeapTuple<'static, ::pgx::AllocatedByRust>>(),
);
```

# Inside a `#[pg_extern]`

Used inside of a [`#[pg_extern]`][crate::pg_extern] definition, this macro alters the generated SQL to use the given
composite type name.

Meaning that this function:

```rust,no_run
use pgx::{prelude::*, AllocatedByRust};

#[pg_extern]
fn scritch(
    maybe_dog: Option<::pgx::composite_type!("Dog")>,
) -> Option<pgx::composite_type!("Dog")> {
    // Gets resolved to:
    let maybe_dog: Option<PgHeapTuple<AllocatedByRust>> = maybe_dog;

    let maybe_dog = if let Some(mut dog) = maybe_dog {
        dog.set_by_name("scritches", dog.get_by_name::<i32>("scritches").unwrap())
            .unwrap();
        Some(dog)
    } else {
        None
    };

    maybe_dog
}
```

Would generate SQL similar to this:

```SQL
-- a_bunch_of_dog_functions/src/lib.rs:3
-- a_bunch_of_dog_functions::scritch
CREATE FUNCTION "scritch"(
        "maybe_dog" Dog /* core::option::Option<pgx::heap_tuple::PgHeapTuple<pgx::pgbox::AllocatedByRust>> */
) RETURNS Dog /* core::option::Option<pgx::heap_tuple::PgHeapTuple<pgx::pgbox::AllocatedByRust>> */
LANGUAGE c /* Rust */
AS 'MODULE_PATHNAME', 'scritch_wrapper';
```

It's possibly to use `composite_type!()` inside a `default!()` macro:

```rust
use pgx::{prelude::*, AllocatedByRust};

#[pg_extern]
fn this_dog_name_or_your_favorite_dog_name(
    dog: pgx::default!(pgx::composite_type!("Dog"), "ROW('Nami', 0)::Dog"),
) -> &str {
    // Gets resolved to:
    let dog: PgHeapTuple<AllocatedByRust> = dog;

    dog.get_by_name("name").unwrap().unwrap()
}
```

Composite types are very **runtime failure** heavy, as opposed to using PostgreSQL types `pgx` has
a builtin compatible type for, or a [`#[derive(pgx::PostgresType)`][crate::PostgresType] type. Those options
 can have their shape and API reasoned about at build time.

This runtime failure model is because the shape and layout, or even the name of the type could change during
the runtime of the extension.

For example, a user of the extension could do something like:

```sql
CREATE TYPE Dog AS (
    name TEXT,
    scritches INT
);

CREATE EXTENSION a_bunch_of_dog_functions;

SELECT scritch(ROW('Nami', 0)::Dog);

ALTER TYPE Dog ADD ATTRIBUTE tail_wags INT;

SELECT scritch(ROW('Nami', 0, 0)::Dog);
```

Because of this, all interaction with composite types requires runtime lookup and type checking.

# Creating composite types

It's possible to create composite types of a given identifier with [`pgx::heap_tuple::PgHeapTuple::new_composite_type`][crate::heap_tuple::PgHeapTuple::new_composite_type].

 */
#[macro_export]
macro_rules! composite_type {
    ($lt:lifetime, $composite_type:expr) => {
        ::pgx::heap_tuple::PgHeapTuple<$lt, ::pgx::AllocatedByRust>
    };
    ($composite_type:expr) => {
        ::pgx::heap_tuple::PgHeapTuple<'static, ::pgx::AllocatedByRust>
    };
}

unsafe impl SqlTranslatable for crate::heap_tuple::PgHeapTuple<'static, AllocatedByPostgres> {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::Composite { array_brackets: false })
    }
    fn return_sql() -> Result<Returns, ReturnsError> {
        Ok(Returns::One(SqlMapping::Composite { array_brackets: false }))
    }
}

unsafe impl SqlTranslatable for crate::heap_tuple::PgHeapTuple<'static, AllocatedByRust> {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::Composite { array_brackets: false })
    }
    fn return_sql() -> Result<Returns, ReturnsError> {
        Ok(Returns::One(SqlMapping::Composite { array_brackets: false }))
    }
}
