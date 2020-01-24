use crate::nodes::PgNode;
use crate::{pg_sys, void_mut_ptr, PgMemoryContexts};
use std::fmt::{Debug, Error, Formatter};
use std::ops::{Deref, DerefMut};

/// Similar to Rust's `Box<T>` type, `PgBox<T>` represents heap-allocated memory.
///
/// However, it represents a heap-allocated pointer that was allocated by Postgres's memory
/// allocation functions (`palloc`, etc).  Think of `PgBox<T>` as a wrapper around an otherwise
/// opaque Postgres type that is projected as a concrete Rust type.
///
/// Depending on its usage, it'll interoperate correctly with Rust's Drop semantics, such that the
/// backing Postgres-allocated memory is `pfree()'d` when the `PgBox<T>` is dropped, but it is
/// possible to effectively return management of the memory back to Postgres (to free on Transaction
/// end, for example) by calling `::into_pg()`.  This is especially useful for returning values
/// back to Postgres
///
/// ## Examples
///
/// This example allocates a simple Postgres structure, modifies it, and returns it back to Postgres:
///
/// ```rust,no_run
/// use pgx::*;
///
/// #[pg_guard]
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
/// use pgx::*;
///
/// #[pg_guard]
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
/// use pgx::*;
///
/// #[pg_guard]
/// pub fn do_something()  {
///     // open a relation and project it as a pg_sys::Relation
///     let relid: pg_sys::Oid = 42;
///     let lockmode = pg_sys::AccessShareLock as i32;
///     let relation = PgBox::from_pg(unsafe { pg_sys::relation_open(relid, lockmode) });
///
///     // do something with/to 'relation'
///     // ...
///
///     // pass the relation back to Postgres
///     unsafe { pg_sys::relation_close(relation.to_pg(), lockmode); }
///
///     // While the `PgBox` instance gets dropped, the backing Postgres-allocated pointer is
///     // **not** freed since it came "::from_pg()".  We don't own the underlying memory so
///     // we can't free it
/// }
/// ```
///
///
/// ## Safety
///
/// TODO:
///  - Interatctions with Rust's panic!() macro
///  - Interactions with Poastgres' error!() macro
///  - Boxing a null pointer -- it works ::from_pg(), ::into_pg(), and ::to_pg(), but will panic!() on all other uses
///
pub struct PgBox<T> {
    ptr: Option<*mut T>,
    allocated_by_pg: bool,
}

impl<T: Debug> Debug for PgBox<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self.ptr {
            Some(ptr) => f.write_str(&format!(
                "PgBox<{}> (ptr={:?}, owner={:?})",
                std::any::type_name::<T>(),
                unsafe {
                    ptr.as_ref()
                        .expect("impl Debug for PgBox expected self.ptr to be non-NULL")
                },
                self.owner_string()
            )),
            None => f.write_str(&format!(
                "PgBox<{}> (ptr=NULL, owner={:?})",
                std::any::type_name::<T>(),
                self.owner_string()
            )),
        }
    }
}

impl<T> PgBox<T> {
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
    pub fn alloc() -> PgBox<T> {
        PgBox::<T> {
            ptr: Some(unsafe { pg_sys::palloc(std::mem::size_of::<T>()) as *mut T }),
            allocated_by_pg: false,
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
    pub fn alloc0() -> PgBox<T> {
        PgBox::<T> {
            ptr: Some(unsafe { pg_sys::palloc0(std::mem::size_of::<T>()) as *mut T }),
            allocated_by_pg: false,
        }
    }

    /// Allocate enough memory for the type'd struct, within the specified Postgres MemoryContext.  
    /// The allocated memory is uninitalized.
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
    pub fn alloc_in_context(memory_context: PgMemoryContexts) -> PgBox<T> {
        PgBox::<T> {
            ptr: Some(unsafe {
                pg_sys::MemoryContextAlloc(memory_context.value(), std::mem::size_of::<T>())
            } as *mut T),
            allocated_by_pg: false,
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
    pub fn alloc0_in_context(memory_context: PgMemoryContexts) -> PgBox<T> {
        PgBox::<T> {
            ptr: Some(unsafe {
                pg_sys::MemoryContextAllocZero(memory_context.value(), std::mem::size_of::<T>())
            } as *mut T),
            allocated_by_pg: false,
        }
    }

    /// Allocate a struct that can be cast to Postgres' `Node`
    ///
    /// This function automatically fills the struct with zeros and sets
    /// the `type_` field to the specified [PgNode]
    pub fn alloc_node(tag: PgNode) -> PgBox<T> {
        let boxed = PgBox::<T>::alloc0();
        let node = boxed.to_pg() as *mut pg_sys::Node;

        unsafe { node.as_mut() }.unwrap().type_ = tag as u32;

        boxed
    }

    /// Box nothing
    pub fn null() -> PgBox<T> {
        PgBox::<T> {
            ptr: None,
            allocated_by_pg: false,
        }
    }

    /// Box a pointer that came from Postgres.
    ///
    /// When this `PgBox<T>` is dropped, the boxed memory is **not** freed.  Since Postgres
    /// allocated it, Postgres is responsible for freeing it.
    pub fn from_pg(ptr: *mut T) -> PgBox<T> {
        PgBox::<T> {
            ptr: if ptr.is_null() { None } else { Some(ptr) },
            allocated_by_pg: true,
        }
    }

    /// Box a pointer that was allocated within Rust
    ///
    /// When this `PgBox<T>` is dropped, the boxed memory is freed.  Since Rust
    /// allocated it, Rust is responsible for freeing it.
    ///
    /// If you need to give the boxed pointer to Postgres, call `.into_pg()`
    pub fn from_rust(ptr: *mut T) -> PgBox<T> {
        PgBox::<T> {
            ptr: if ptr.is_null() { None } else { Some(ptr) },
            allocated_by_pg: false,
        }
    }

    /// Are we boxing a NULL?
    pub fn is_null(&self) -> bool {
        self.ptr.is_none()
    }

    /// Return the boxed pointer, so that it can be passed back into a Postgres function
    pub fn to_pg(&self) -> *mut T {
        let ptr = self.ptr;
        match ptr {
            Some(ptr) => ptr,
            None => std::ptr::null_mut(),
        }
    }

    /// Consume this instance and return the boxed pointer as a pg_sys::Datum, so that it can be
    /// passed back into a Postgres function
    pub fn convert_to_datum(self) -> pg_sys::Datum {
        self.into_pg() as pg_sys::Datum
    }

    /// Useful for returning the boxed pointer back to Postgres (as a return value, for example).
    ///
    /// Rust forgets the Box and the boxed pointer is **not** free'd by Rust
    pub fn into_pg(mut self) -> *mut T {
        self.allocated_by_pg = true;
        match self.ptr {
            Some(ptr) => ptr,
            None => std::ptr::null_mut(),
        }
    }

    fn owner_string(&self) -> &str {
        if self.allocated_by_pg {
            "Postgres"
        } else {
            "Rust"
        }
    }
}

impl<T> Deref for PgBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self.ptr {
            Some(ptr) => unsafe { &*ptr },
            None => panic!("Attempt to dereference null pointer during Deref of PgBox"),
        }
    }
}

impl<T> DerefMut for PgBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        match self.ptr {
            Some(ptr) => unsafe { &mut *ptr },
            None => panic!("Attempt to dereference null pointer during DerefMut of PgBox"),
        }
    }
}

impl<T> Drop for PgBox<T> {
    fn drop(&mut self) {
        if !self.allocated_by_pg && !self.ptr.is_none() {
            let ptr = self.ptr.expect("PgBox ptr was null during Drop");
            unsafe {
                pg_sys::pfree(ptr as void_mut_ptr);
            }
        }
    }
}

impl<T: Debug> std::fmt::Display for PgBox<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.ptr {
            Some(_) => write!(
                f,
                "PgBox {{ owner={:?}, {:?} }}",
                self.owner_string(),
                self.deref()
            ),
            None => std::fmt::Display::fmt("NULL", f),
        }
    }
}
