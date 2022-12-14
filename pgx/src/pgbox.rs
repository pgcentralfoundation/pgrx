/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

/// Similar to Rust's `Box<T>` type, `PgBox<T>` also represents heap-allocated memory.
use crate::{pg_sys, PgMemoryContexts};
//use std::fmt::{Debug, Error, Formatter};
use pgx_sql_entity_graph::metadata::{
    ArgumentError, Returns, ReturnsError, SqlMapping, SqlTranslatable,
};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

/// Similar to Rust's `Box<T>` type, `PgBox<T>` also represents heap-allocated memory.
///
/// However, it represents a heap-allocated pointer that was allocated by **Postgres's** memory
/// allocation functions (`palloc`, etc).  Think of `PgBox<T>` as a wrapper around an otherwise
/// opaque Postgres type that is projected as a concrete Rust type.
///
/// Depending on its usage, it'll interoperate correctly with Rust's Drop semantics, such that the
/// backing Postgres-allocated memory is `pfree()'d` when the `PgBox<T>` is dropped, but it is
/// possible to effectively return management of the memory back to Postgres (to free on Transaction
/// end, for example) by calling `::into_pg()` or ``::into_pg_boxed()`.  This is especially useful
/// for returning values back to Postgres.
///
/// ## Examples
///
/// This example allocates a simple Postgres structure, modifies it, and returns it back to Postgres:
///
/// ```rust,no_run
/// use pgx::prelude::*;
///
/// pub fn do_something() -> pg_sys::ItemPointer {
///     // postgres-allocate an ItemPointerData structure
///     let mut tid = PgBox::<pg_sys::ItemPointerData>::alloc();
///
///     // set its position to 42
///     tid.ip_posid = 42;
///
///     // return it to Postgres
///     tid.into_pg()
/// }
/// ```
///
/// A similar example, but instead the `PgBox<T>`'s backing memory gets freed when the box is
/// dropped:
///
/// ```rust,no_run
/// use pgx::prelude::*;
///
/// pub fn do_something()  {
///     // postgres-allocate an ItemPointerData structure
///     let mut tid = PgBox::<pg_sys::ItemPointerData>::alloc();
///
///     // set its position to 42
///     tid.ip_posid = 42;
///
///     // tid gets dropped here and as such, gets immediately pfree()'d
/// }
/// ```
///
/// Alternatively, perhaps you want to work with a pointer Postgres gave you as if it were a Rust type,
/// but it can't be freed on Drop since you don't own it -- Postgres does:
///
/// ```rust,no_run
/// use pgx::prelude::*;
///
/// pub fn do_something()  {
///     // open a relation and project it as a pg_sys::Relation
///     let relid: pg_sys::Oid = 42;
///     let lockmode = pg_sys::AccessShareLock as i32;
///     let relation = unsafe { PgBox::from_pg(pg_sys::relation_open(relid, lockmode)) };
///
///     // do something with/to 'relation'
///     // ...
///
///     // pass the relation back to Postgres
///     unsafe { pg_sys::relation_close(relation.as_ptr(), lockmode); }
///
///     // While the `PgBox` instance gets dropped, the backing Postgres-allocated pointer is
///     // **not** freed since it came "::from_pg()".  We don't own the underlying memory so
///     // we can't free it
/// }
/// ```
#[repr(transparent)]
pub struct PgBox<T, AllocatedBy: WhoAllocated = AllocatedByPostgres> {
    ptr: Option<NonNull<T>>,
    __marker: PhantomData<AllocatedBy>,
}

/// A trait to track if the contents of a [PgBox] were allocated by Rust or Postgres.
pub trait WhoAllocated {
    /// Implementations can decide if they want to [`pg_sys::pfree`] the specified pointer
    /// or not.  As such, the specified pointer must be a valid, [`pg_sys::palloc`]'d pointer.
    ///
    /// # Safety
    ///
    /// This function is unsafe as it cannot determine if the specified pointer is valid and was
    /// allocated by Postgres.
    unsafe fn maybe_pfree(ptr: *mut std::os::raw::c_void);
}

/// Indicates the [PgBox] contents were allocated by Postgres.  This is also PgBox' default
/// understanding.
pub struct AllocatedByPostgres;

/// Indicates the [PgBox] contents were allocated by Rust.
pub struct AllocatedByRust;

impl WhoAllocated for AllocatedByPostgres {
    /// Doesn't do anything
    unsafe fn maybe_pfree(_ptr: *mut std::os::raw::c_void) {}
}
impl WhoAllocated for AllocatedByRust {
    /// Uses [pg_sys::pfree] to free the specified pointer
    #[inline]
    unsafe fn maybe_pfree(ptr: *mut std::os::raw::c_void) {
        pg_sys::pfree(ptr.cast());
    }
}

impl<T> PgBox<T, AllocatedByPostgres> {
    /// Box a pointer that cames from Postgres.
    ///
    /// When this `PgBox<T>` is dropped, the boxed memory is **not** freed.  Since Postgres
    /// allocated it, Postgres is responsible for freeing it.
    #[inline]
    pub unsafe fn from_pg(ptr: *mut T) -> PgBox<T, AllocatedByPostgres> {
        PgBox::<T, AllocatedByPostgres> { ptr: NonNull::new(ptr), __marker: PhantomData }
    }
}

impl<T> PgBox<T, AllocatedByRust> {
    /**
    Allocates memory in PostgreSQL and then places `val` into it.

    This value is managed by Rust, so gets dropped via normal [`Drop`][std::ops::Drop]
    semantics.

    If you need to give the boxed pointer to Postgres, call [`.into_pg()`][PgBox::into_pg].

    ```rust,no_run
    use pgx::{PgBox, AllocatedByRust};

    let ptr: PgBox<i32, AllocatedByRust> = PgBox::new(5);
    assert_eq!(*ptr, 5);

    let mut ptr: PgBox<Vec<i32>, AllocatedByRust> = PgBox::new(vec![]);
    assert_eq!(*ptr, Vec::<i32>::default());

    ptr.push(1);
    assert_eq!(*ptr, vec![1]);

    ptr.push(2);
    assert_eq!(*ptr, vec![1, 2]);

    ptr.push(3);
    assert_eq!(*ptr, vec![1, 2, 3]);

    let drained = ptr.drain(..).collect::<Vec<_>>();
    assert_eq!(drained, vec![1, 2, 3])
    ```
     */
    pub fn new(val: T) -> PgBox<T, AllocatedByRust> {
        let ptr = Self::alloc0();
        unsafe { core::ptr::write(ptr.as_ptr(), val) };
        ptr
    }

    /**
    Allocates memory in PostgreSQL and then places `val` into it.

    This value is managed by Rust, so gets dropped via normal [`Drop`][std::ops::Drop]
    semantics.

    If you need to give the boxed pointer to Postgres, call [`.into_pg()`][PgBox::into_pg].

    ```rust,no_run
    use pgx::{PgBox, PgMemoryContexts, AllocatedByRust};

    let ptr: PgBox<i32, AllocatedByRust> = PgBox::new_in_context(5, PgMemoryContexts::CurrentMemoryContext);
    assert_eq!(*ptr, 5);
    ```
     */
    pub fn new_in_context(val: T, memory_context: PgMemoryContexts) -> PgBox<T, AllocatedByRust> {
        let ptr = Self::alloc0_in_context(memory_context);
        unsafe { core::ptr::write(ptr.as_ptr(), val) };
        ptr
    }
}

impl<T, AllocatedBy: WhoAllocated> PgBox<T, AllocatedBy> {
    /// Box a pointer that was allocated within Rust
    ///
    /// When this `PgBox<T>` is dropped, the boxed memory is freed.  Since Rust
    /// allocated it, Rust is responsible for freeing it.
    ///
    /// If you need to give the boxed pointer to Postgres, call [`.into_pg()`][PgBox::into_pg]
    #[inline]
    pub unsafe fn from_rust(ptr: *mut T) -> PgBox<T, AllocatedByRust> {
        PgBox::<T, AllocatedByRust> { ptr: NonNull::new(ptr), __marker: PhantomData }
    }

    /// Allocate enough memory for the type'd struct, within Postgres' `CurrentMemoryContext`  The
    /// allocated memory is uninitialized.
    ///
    /// When this object is dropped the backing memory will be pfree'd,
    /// unless it is instead turned `into_pg()`, at which point it will be freeded
    /// when its owning MemoryContext is deleted by Postgres (likely transaction end).
    ///
    /// ## Examples
    /// ```rust,no_run
    /// use pgx::{PgBox, pg_sys};
    /// let ctid = PgBox::<pg_sys::ItemPointerData>::alloc();
    /// ```
    #[inline]
    pub fn alloc() -> PgBox<T, AllocatedByRust> {
        PgBox::<T, AllocatedByRust> {
            ptr: Some(unsafe {
                NonNull::new_unchecked(pg_sys::palloc(std::mem::size_of::<T>()) as *mut T)
            }),
            __marker: PhantomData,
        }
    }

    /// Allocate enough memory for the type'd struct, within Postgres' `CurrentMemoryContext`  The
    /// allocated memory is zero-filled.
    ///
    /// When this object is dropped the backing memory will be pfree'd,
    /// unless it is instead turned `into_pg()`, at which point it will be freeded
    /// when its owning MemoryContext is deleted by Postgres (likely transaction end).
    ///
    /// ## Examples
    /// ```rust,no_run
    /// use pgx::{PgBox, pg_sys};
    /// let ctid = PgBox::<pg_sys::ItemPointerData>::alloc0();
    /// ```
    #[inline]
    pub fn alloc0() -> PgBox<T, AllocatedByRust> {
        PgBox::<T, AllocatedByRust> {
            ptr: Some(unsafe {
                NonNull::new_unchecked(pg_sys::palloc0(std::mem::size_of::<T>()) as *mut T)
            }),
            __marker: PhantomData,
        }
    }

    /// Allocate enough memory for the type'd struct, within the specified Postgres MemoryContext.
    /// The allocated memory is uninitialized.
    ///
    /// When this object is dropped the backing memory will be pfree'd,
    /// unless it is instead turned `into_pg()`, at which point it will be freeded
    /// when its owning MemoryContext is deleted by Postgres (likely transaction end).
    ///
    /// ## Examples
    /// ```rust,no_run
    /// use pgx::{PgBox, pg_sys, PgMemoryContexts};
    /// let ctid = PgBox::<pg_sys::ItemPointerData>::alloc_in_context(PgMemoryContexts::TopTransactionContext);
    /// ```
    #[inline]
    pub fn alloc_in_context(memory_context: PgMemoryContexts) -> PgBox<T, AllocatedByRust> {
        PgBox::<T, AllocatedByRust> {
            ptr: Some(unsafe {
                NonNull::new_unchecked(pg_sys::MemoryContextAlloc(
                    memory_context.value(),
                    std::mem::size_of::<T>(),
                ) as *mut T)
            }),
            __marker: PhantomData,
        }
    }

    /// Allocate enough memory for the type'd struct, within the specified Postgres MemoryContext.
    /// The allocated memory is zero-filled.
    ///
    /// When this object is dropped the backing memory will be pfree'd,
    /// unless it is instead turned `into_pg()`, at which point it will be freeded
    /// when its owning MemoryContext is deleted by Postgres (likely transaction end).
    ///
    /// ## Examples
    /// ```rust,no_run
    /// use pgx::{PgBox, pg_sys, PgMemoryContexts};
    /// let ctid = PgBox::<pg_sys::ItemPointerData>::alloc0_in_context(PgMemoryContexts::TopTransactionContext);
    /// ```
    #[inline]
    pub fn alloc0_in_context(memory_context: PgMemoryContexts) -> PgBox<T, AllocatedByRust> {
        PgBox::<T, AllocatedByRust> {
            ptr: Some(unsafe {
                NonNull::new_unchecked(pg_sys::MemoryContextAllocZero(
                    memory_context.value(),
                    std::mem::size_of::<T>(),
                ) as *mut T)
            }),
            __marker: PhantomData,
        }
    }

    /// Allocate a Postgres `pg_sys::Node` subtype, using `palloc` in the `CurrentMemoryContext`.
    ///
    /// The allocated node will have it's `type_` field set to the `node_tag` argument, and will
    /// otherwise be initialized with all zeros
    ///
    /// ## Examples
    /// ```rust,no_run
    /// use pgx::{PgBox, pg_sys};
    /// let create_trigger_statement = PgBox::<pg_sys::CreateTrigStmt>::alloc_node(pg_sys::NodeTag_T_CreateTrigStmt);
    /// ```
    #[inline]
    pub fn alloc_node(node_tag: pg_sys::NodeTag) -> PgBox<T, AllocatedByRust> {
        let node = PgBox::<T>::alloc0();
        let ptr = node.as_ptr();

        unsafe {
            (ptr as *mut _ as *mut pg_sys::Node).as_mut().unwrap().type_ = node_tag;
        }
        node
    }

    /// Box nothing
    #[inline]
    pub fn null() -> PgBox<T, AllocatedBy> {
        PgBox::<T, AllocatedBy> { ptr: None, __marker: PhantomData }
    }

    /// Are we boxing a NULL?
    #[inline]
    pub fn is_null(&self) -> bool {
        self.ptr.is_none()
    }

    /// Return the boxed pointer, so that it can be passed back into a Postgres function
    #[inline]
    pub fn as_ptr(&self) -> *mut T {
        match self.ptr.as_ref() {
            Some(ptr) => unsafe { ptr.clone().as_mut() as *mut T },
            None => std::ptr::null_mut(),
        }
    }

    /// Useful for returning the boxed pointer back to Postgres (as a return value, for example).
    ///
    /// The boxed pointer is **not** free'd by Rust
    #[inline]
    pub fn into_pg(mut self) -> *mut T {
        match self.ptr.take() {
            Some(ptr) => ptr.as_ptr(),
            None => std::ptr::null_mut(),
        }
    }

    /// Useful for returning the boxed pointer back to Postgres (as a return value, for example).
    ///
    /// The boxed pointer is **not** free'd by Rust
    #[inline]
    pub fn into_pg_boxed(mut self) -> PgBox<T, AllocatedByPostgres> {
        // SAFETY:  we know our internal pointer is good so we can now make it owned by Postgres
        unsafe {
            PgBox::from_pg(match self.ptr.take() {
                Some(ptr) => ptr.as_ptr(),
                None => std::ptr::null_mut(),
            })
        }
    }

    /// Execute a closure with a mutable, `PgBox`'d form of the specified `ptr`
    #[inline]
    pub unsafe fn with<F: FnOnce(&mut PgBox<T>)>(ptr: *mut T, func: F) {
        func(&mut PgBox::from_pg(ptr))
    }
}

impl<T, AllocatedBy: WhoAllocated> Deref for PgBox<T, AllocatedBy> {
    type Target = T;

    #[track_caller]
    fn deref(&self) -> &Self::Target {
        match self.ptr.as_ref() {
            Some(ptr) => unsafe { ptr.as_ref() },
            None => panic!("Attempt to dereference null pointer during Deref of PgBox"),
        }
    }
}

impl<T, AllocatedBy: WhoAllocated> DerefMut for PgBox<T, AllocatedBy> {
    #[track_caller]
    fn deref_mut(&mut self) -> &mut T {
        match self.ptr.as_mut() {
            Some(ptr) => unsafe { ptr.as_mut() },
            None => panic!("Attempt to dereference null pointer during DerefMut of PgBox"),
        }
    }
}

impl<T, AllocatedBy: WhoAllocated> Drop for PgBox<T, AllocatedBy> {
    fn drop(&mut self) {
        if let Some(ptr) = self.ptr {
            unsafe {
                // SAFETY:  we know ptr is a valid, non-null, Postgres allocated pointer
                AllocatedBy::maybe_pfree(ptr.as_ptr().cast());
            }
        }
    }
}

unsafe impl<T: SqlTranslatable> SqlTranslatable for PgBox<T, AllocatedByPostgres> {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        T::argument_sql()
    }
    fn return_sql() -> Result<Returns, ReturnsError> {
        T::return_sql()
    }
}

unsafe impl<T: SqlTranslatable> SqlTranslatable for PgBox<T, AllocatedByRust> {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        T::argument_sql()
    }
    fn return_sql() -> Result<Returns, ReturnsError> {
        T::return_sql()
    }
}
