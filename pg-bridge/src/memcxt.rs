//!
//! Provides interfacing into Postgres' `MemoryContext` system.
//!
//! The `PgBox<T>` projects Postgres-allocated memory pointers as if they're first-class Rust types.
//!
//! An enum-based interface (`PgMemoryContexts`) around Postgres' various `MemoryContext`s provides
//! simple accessibility to working with MemoryContexts in a compiler-checked manner
//!
use crate::pg_sys;
use pg_guard::PostgresStruct;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

/// Return a Postgres-allocated pointer to a struct allocated in Postgres' [pg_sys::CurrentMemoryContext].  
/// The struct could be a Postgres struct or even a Rust struct.  In either case, the memory is
/// heap-allocated by Postgres.
///
/// The memory will be freed when its parent MemoryContext is deleted.
#[inline]
pub fn palloc_struct<T>() -> *mut T {
    (unsafe { pg_sys::palloc(std::mem::size_of::<T>()) }) as *mut T
}

/// Return a Postgres-allocated pointer to a struct allocated in Postgres' [pg_sys::CurrentMemoryContext].  
/// The struct could be a Postgres struct or even a Rust struct.  In either case, the memory is
/// heap-allocated by Postgres.
///
/// The memory will be freed when its parent MemoryContext is deleted.
///
/// Also zeros out the allocation block
#[inline]
pub fn palloc0_struct<T>() -> *mut T {
    (unsafe { pg_sys::palloc0(std::mem::size_of::<T>()) }) as *mut T
}

/// Return a Postgres-allocated pointer to a struct allocated in the specified Postgres MemoryContext.  
/// The struct could be a Postgres struct or even a Rust struct.  In either case, the memory is
/// heap-allocated by Postgres.
///
/// The memory will be freed when the specified MemoryContext is deleted.
#[inline]
pub fn palloc_struct_in_memory_context<T>(memory_context: pg_sys::MemoryContext) -> *mut T {
    assert!(!memory_context.is_null());
    (unsafe { pg_sys::MemoryContextAlloc(memory_context, std::mem::size_of::<T>()) }) as *mut T
}

/// Return a Postgres-allocated pointer to a struct allocated in the specified Postgres MemoryContext.  
/// The struct could be a Postgres struct or even a Rust struct.  In either case, the memory is
/// heap-allocated by Postgres.
///
/// The memory will be freed when the specified MemoryContext is deleted.
///
/// Also zeros out the allocation block
#[inline]
pub fn palloc0_struct_in_memory_context<T>(memory_context: pg_sys::MemoryContext) -> *mut T {
    assert!(!memory_context.is_null());
    (unsafe { pg_sys::MemoryContextAllocZero(memory_context, std::mem::size_of::<T>()) }) as *mut T
}

/// A shorter type name for a `*const std::os::raw::c_void`
#[allow(non_camel_case_types)]
pub type void_ptr = *const std::os::raw::c_void;

/// An Enumeration of Postgres top-level MemoryContexts.  Each have their own use and "lifetimes"
/// as defined by Postgres' memory management model.
///
/// It's possible to deference any one of these (except `Transient`) via the `::value()` method if
/// it's necessary to pass the raw pointer to a Postgres function.
///
/// Additionally, the `::switch_to()` function, which takes a closure as its argument, executes the
/// closure within that MemoryContext
#[derive(Debug)]
pub enum PgMemoryContexts {
    /// Because it would be too much notational overhead to always pass an
    /// appropriate memory context to called routines, there always exists the
    /// notion of the current memory context CurrentMemoryContext.  Without it,
    /// for example, the copyObject routines would need to be passed a context, as
    /// would function execution routines that return a pass-by-reference
    /// datatype.  Similarly for routines that temporarily allocate space
    /// internally, but don't return it to their caller?  We certainly don't
    /// want to clutter every call in the system with "here is a context to
    /// use for any temporary memory allocation you might want to do".
    ///
    /// The upshot of that reasoning, though, is that CurrentMemoryContext should
    /// generally point at a short-lifespan context if at all possible.  During
    /// query execution it usually points to a context that gets reset after each
    /// tuple.  Only in *very* circumscribed code should it ever point at a
    /// context having greater than transaction lifespan, since doing so risks
    /// permanent memory leaks.
    CurrentMemoryContext,

    /// this is the actual top level of the context tree;
    /// every other context is a direct or indirect child of this one.  Allocating
    /// here is essentially the same as "malloc", because this context will never
    /// be reset or deleted.  This is for stuff that should live forever, or for
    /// stuff that the controlling module will take care of deleting at the
    /// appropriate time.  An example is fd.c's tables of open files.  Avoid
    /// allocating stuff here unless really necessary, and especially avoid
    /// running with CurrentMemoryContext pointing here.
    TopMemoryContext,

    /// this is not actually a separate context, but a
    /// global variable pointing to the per-portal context of the currently active
    /// execution portal.  This can be used if it's necessary to allocate storage
    /// that will live just as long as the execution of the current portal requires.
    PortalContext,

    /// this permanent context is switched into for error
    /// recovery processing, and then reset on completion of recovery.  We arrange
    /// to have a few KB of memory available in it at all times.  In this way, we
    /// can ensure that some memory is available for error recovery even if the
    /// backend has run out of memory otherwise.  This allows out-of-memory to be
    /// treated as a normal ERROR condition, not a FATAL error.
    ErrorContext,

    /// this is the postmaster's normal working context.
    /// After a backend is spawned, it can delete PostmasterContext to free its
    /// copy of memory the postmaster was using that it doesn't need.
    /// Note that in non-EXEC_BACKEND builds, the postmaster's copy of pg_hba.conf
    /// and pg_ident.conf data is used directly during authentication in backend
    /// processes; so backends can't delete PostmasterContext until that's done.
    /// (The postmaster has only TopMemoryContext, PostmasterContext, and
    /// ErrorContext --- the remaining top-level contexts are set up in each
    /// backend during startup.)
    PostmasterContext,

    /// permanent storage for relcache, catcache, and
    /// related modules.  This will never be reset or deleted, either, so it's
    /// not truly necessary to distinguish it from TopMemoryContext.  But it
    /// seems worthwhile to maintain the distinction for debugging purposes.
    /// (Note: CacheMemoryContext has child contexts with shorter lifespans.
    /// For example, a child context is the best place to keep the subsidiary
    /// storage associated with a relcache entry; that way we can free rule
    /// parsetrees and so forth easily, without having to depend on constructing
    /// a reliable version of freeObject().)
    CacheMemoryContext,

    /// this context holds the current command message from the
    /// frontend, as well as any derived storage that need only live as long as
    /// the current message (for example, in simple-Query mode the parse and plan
    /// trees can live here).  This context will be reset, and any children
    /// deleted, at the top of each cycle of the outer loop of PostgresMain.  This
    /// is kept separate from per-transaction and per-portal contexts because a
    /// query string might need to live either a longer or shorter time than any
    /// single transaction or portal.
    MessageContext,

    /// this holds everything that lives until end of the
    /// top-level transaction.  This context will be reset, and all its children
    /// deleted, at conclusion of each top-level transaction cycle.  In most cases
    /// you don't want to allocate stuff directly here, but in CurTransactionContext;
    /// what does belong here is control information that exists explicitly to manage
    /// status across multiple subtransactions.  Note: this context is NOT cleared
    /// immediately upon error; its contents will survive until the transaction block
    /// is exited by COMMIT/ROLLBACK.
    TopTransactionContext,

    /// this holds data that has to survive until the end
    /// of the current transaction, and in particular will be needed at top-level
    /// transaction commit.  When we are in a top-level transaction this is the same
    /// as TopTransactionContext, but in subtransactions it points to a child context.
    /// It is important to understand that if a subtransaction aborts, its
    /// CurTransactionContext is thrown away after finishing the abort processing;
    /// but a committed subtransaction's CurTransactionContext is kept until top-level
    /// commit (unless of course one of the intermediate levels of subtransaction
    /// aborts).  This ensures that we do not keep data from a failed subtransaction
    /// longer than necessary.  Because of this behavior, you must be careful to clean
    /// up properly during subtransaction abort --- the subtransaction's state must be
    /// delinked from any pointers or lists kept in upper transactions, or you will
    /// have dangling pointers leading to a crash at top-level commit.  An example of
    /// data kept here is pending NOTIFY messages, which are sent at top-level commit,
    /// but only if the generating subtransaction did not abort.
    CurTransactionContext,

    /// This represents a MemoryContext that was likely created via
    /// [pg_sys::AllocSetContextCreateExtended].
    ///
    /// That could be a MemoryContext you created yourself, or it could be one given to you from
    /// Postgres.  For example, the `TupleTableSlot` struct has a field referencing the MemoryContext
    /// in which slots are allocated.
    For(pg_sys::MemoryContext),

    /// Use the MemoryContext in which the specified pointer was allocated.
    ///
    /// It's incredibly important that the specified pointer be one actually allocated by
    /// Postgres' memory management system.  Otherwise, it's undefined behavior and will
    /// **absolutely** crash Postgres
    Of(void_ptr),

    /// Create a temporary MemoryContext for use with [::switch_to()].  It gets deleted as soon
    /// as [::switch_to()] exits.
    ///
    /// Trying to use this context through [::value{}] will result in a panic!().
    Transient {
        parent: pg_sys::MemoryContext,
        name: &'static str,
        min_context_size: usize,
        initial_block_size: usize,
        max_block_size: usize,
    },
}

impl PgMemoryContexts {
    pub fn value(&self) -> pg_sys::MemoryContext {
        match self {
            PgMemoryContexts::CurrentMemoryContext => unsafe { pg_sys::CurrentMemoryContext },
            PgMemoryContexts::TopMemoryContext => unsafe { pg_sys::TopMemoryContext },
            PgMemoryContexts::PortalContext => unsafe { pg_sys::PortalContext },
            PgMemoryContexts::ErrorContext => unsafe { pg_sys::ErrorContext },
            PgMemoryContexts::PostmasterContext => unsafe { pg_sys::PostmasterContext },
            PgMemoryContexts::CacheMemoryContext => unsafe { pg_sys::CacheMemoryContext },
            PgMemoryContexts::MessageContext => unsafe { pg_sys::MessageContext },
            PgMemoryContexts::TopTransactionContext => unsafe { pg_sys::TopTransactionContext },
            PgMemoryContexts::CurTransactionContext => unsafe { pg_sys::CurTransactionContext },
            PgMemoryContexts::For(mc) => *mc,
            PgMemoryContexts::Of(ptr) => PgMemoryContexts::get_context_for_pointer(*ptr),
            PgMemoryContexts::Transient {
                parent: _,
                name: _,
                min_context_size: _,
                initial_block_size: _,
                max_block_size: _,
            } => panic!("cannot use value() to retrieve a Transient PgMemoryContext"),
        }
    }

    /// Run the specified function "within" the `MemoryContext` represented by this enum.
    ///
    /// The important implementation detail is that Postgres' `CurrentMemoryContext` is changed
    /// to be this context, the function is run so that all Postgres memory allocations happen
    /// within that context, and then `CurrentMemoryContext` is restored to what it was before
    /// we started.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use pg_bridge::{PgMemoryContexts, pg_sys, PgBox};
    ///
    /// #[pg_guard]
    /// pub fn do_something() -> pg_sys::ItemPointer {
    ///     PgMemoryContexts::TopTransactionContext.switch_to(|| {
    ///         // allocate a new ItemPointerData, but inside the TopTransactionContext
    ///         let tid = PgBox::<pg_sys::ItemPointerData>::alloc();
    ///         
    ///         // do something with the tid and then return it.
    ///         // Note that it stays allocated here in the TopTransactionContext
    ///         tid.into_pg()
    ///     })
    /// }
    /// ```
    pub fn switch_to<R, F: FnOnce() -> R>(&self, f: F) -> R {
        match self {
            PgMemoryContexts::Transient {
                parent,
                name,
                min_context_size,
                initial_block_size,
                max_block_size,
            } => {
                let context: *mut pg_sys::MemoryContextData = unsafe {
                    let name = std::ffi::CString::new(*name).unwrap();
                    pg_sys::AllocSetContextCreateExtended(
                        *parent,
                        name.into_raw(),
                        *min_context_size,
                        *initial_block_size,
                        *max_block_size,
                    )
                };

                let result = PgMemoryContexts::exec_in_context(context, f);

                unsafe {
                    pg_sys::MemoryContextDelete(context);
                }

                result
            }
            _ => PgMemoryContexts::exec_in_context(self.value(), f),
        }
    }

    /// helper function
    fn exec_in_context<R, F: FnOnce() -> R>(context: pg_sys::MemoryContext, f: F) -> R {
        let prev_context;

        // mimic what palloc.h does for switching memory contexts
        unsafe {
            prev_context = pg_sys::CurrentMemoryContext;
            pg_sys::CurrentMemoryContext = context;
        }

        let result = f();

        // restore our understanding of the current memory context
        unsafe {
            pg_sys::CurrentMemoryContext = prev_context;
        }

        result
    }

    ///
    /// GetMemoryChunkContext
    ///		Given a currently-allocated chunk, determine the context
    ///		it belongs to.
    ///
    /// All chunks allocated by any memory context manager are required to be
    /// preceded by the corresponding MemoryContext stored, without padding, in the
    /// preceding sizeof(void*) bytes.  A currently-allocated chunk must contain a
    /// backpointer to its owning context.  The backpointer is used by pfree() and
    /// repalloc() to find the context to call.
    ///
    fn get_context_for_pointer(ptr: void_ptr) -> pg_sys::MemoryContext {
        // exported from ZomboDB since I failed to convert the macro within a reasonable amount of time
        // TODO:  fix this
        extern "C" {
            pub fn zdb_GetMemoryChunkContext(pointer: void_ptr) -> pg_sys::MemoryContext;
        }
        unsafe { zdb_GetMemoryChunkContext(ptr) }

        //
        // the below causes PG to crash b/c it mis-calculates where the MemoryContext address is
        //
        // I have likely either screwed up max_align()/type_align() or the pointer math at the
        // bottom of the function
        //

        //        // #define MAXALIGN(LEN)			TYPEALIGN(MAXIMUM_ALIGNOF, (LEN))
        //        #[inline]
        //        fn max_align(len: void_ptr) -> void_ptr {
        //            // #define TYPEALIGN(ALIGNVAL,LEN)  \
        //            //	(((uintptr_t) (LEN) + ((ALIGNVAL) - 1)) & ~((uintptr_t) ((ALIGNVAL) - 1)))
        //            #[inline]
        //            fn type_align(
        //                alignval: u32,
        //                len: void_ptr,
        //            ) -> void_ptr {
        //                (((len as usize) + ((alignval) - 1) as usize) & !(((alignval) - 1) as usize))
        //                    as void_ptr
        //            }
        //            type_align(pg_sys::MAXIMUM_ALIGNOF, len)
        //        }
        //
        //        let context;
        //
        //        /*
        //         * Try to detect bogus pointers handed to us, poorly though we can.
        //         * Presumably, a pointer that isn't MAXALIGNED isn't pointing at an
        //         * allocated chunk.
        //         */
        //        assert!(!ptr.is_null());
        //        assert_eq!(
        //            ptr as void_ptr,
        //            max_align(ptr) as void_ptr
        //        );
        //
        //        /*
        //         * OK, it's probably safe to look at the context.
        //         */
        //        //            context = *(MemoryContext *) (((char *) pointer) - sizeof(void *));
        //        context = (((ptr as *const std::os::raw::c_char) as usize)
        //            - std::mem::size_of::<void_ptr>())
        //            as pg_sys::MemoryContext;
        //        context
    }
}

#[derive(Debug)]
enum WhoAllocated {
    Postgres,
    Rust,
}

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
/// ```rust
/// use pg_bridge::{pg_sys, PgBox};
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
/// ```rust
/// use pg_bridge::{pg_sys, PgBox};
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
/// ```rust
/// use pg_bridge::{pg_sys, PgBox};
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
#[derive(Debug)]
pub struct PgBox<T>
where
    T: Sized + Debug + PostgresStruct,
{
    ptr: Option<*mut T>,
    owner: WhoAllocated,
}

impl<T> PgBox<T>
where
    T: Sized + Debug + PostgresStruct,
{
    /// Allocate enough memory for the type'd struct, within Postgres' `CurrentMemoryContext`  The
    /// allocated memory is uninitialized.
    ///
    /// When this object is dropped the backing memory will be pfree'd,
    /// unless it is instead turned `into_pg()`, at which point it will be freeded
    /// when its owning MemoryContext is deleted by Postgres (likely transaction end).
    ///
    /// ## Examples
    /// ```rust
    /// use pg_bridge::{PgBox, pg_sys};
    /// let ctid = PgBox::<pg_sys::ItemPointerData>::alloc();
    /// ```
    pub fn alloc() -> PgBox<T> {
        PgBox::<T> {
            ptr: Some(palloc_struct::<T>()),
            owner: WhoAllocated::Rust,
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
    /// ```rust
    /// use pg_bridge::{PgBox, pg_sys};
    /// let ctid = PgBox::<pg_sys::ItemPointerData>::alloc0();
    /// ```
    pub fn alloc0() -> PgBox<T> {
        PgBox::<T> {
            ptr: Some(palloc0_struct::<T>()),
            owner: WhoAllocated::Rust,
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
    /// ```rust
    /// use pg_bridge::{PgBox, pg_sys, PgMemoryContexts};
    /// let ctid = PgBox::<pg_sys::ItemPointerData>::alloc_in_context(PgMemoryContexts::TopTransactionContext);
    /// ```
    pub fn alloc_in_context(memory_context: PgMemoryContexts) -> PgBox<T> {
        PgBox::<T> {
            ptr: Some(palloc_struct_in_memory_context(memory_context.value())),
            owner: WhoAllocated::Rust,
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
    /// ```rust
    /// use pg_bridge::{PgBox, pg_sys, PgMemoryContexts};
    /// let ctid = PgBox::<pg_sys::ItemPointerData>::alloc0_in_context(PgMemoryContexts::TopTransactionContext);
    /// ```
    pub fn alloc0_in_context(memory_context: PgMemoryContexts) -> PgBox<T> {
        PgBox::<T> {
            ptr: Some(palloc0_struct_in_memory_context(memory_context.value())),
            owner: WhoAllocated::Rust,
        }
    }

    /// Box a pointer that came from Postgres.
    ///
    /// When this `PgBox<T>` is dropped, the boxed memory is **not** freed.  Since Postgres
    /// allocated it, Postgres is responsible for freeing it.
    pub fn from_pg(ptr: *mut T) -> PgBox<T> {
        PgBox::<T> {
            ptr: if ptr.is_null() { None } else { Some(ptr) },
            owner: WhoAllocated::Postgres,
        }
    }

    /// internal use only
    pub(crate) fn from_raw(ptr: *mut T) -> PgBox<T> {
        PgBox::<T> {
            ptr: if ptr.is_null() { None } else { Some(ptr) },
            owner: WhoAllocated::Rust,
        }
    }

    /// Return the boxed pointer, so that it can be passed back into a Postgres function
    pub fn to_pg(&self) -> *mut T {
        let ptr = self.ptr;
        match ptr {
            Some(ptr) => ptr,
            None => 0 as *mut T,
        }
    }

    /// Useful for returning the boxed pointer back to Postgres (as a return value, for example).
    ///
    /// Rust forgets the Box and the boxed pointer is **not** free'd by Rust
    pub fn into_pg(self) -> *mut T {
        let ptr = self.ptr;
        std::mem::forget(self);

        match ptr {
            Some(ptr) => ptr,
            None => 0 as *mut T,
        }
    }

    /// Unwraps the boxed pointer as its underlying type
    pub fn into_inner(self) -> T {
        let ptr = self.ptr;
        match ptr {
            Some(ptr) => {
                std::mem::forget(self);
                unsafe { ptr.read() }
            }
            None => panic!("attempt to dereference a null pointer during PgBox::into_inner()"),
        }
    }
}

impl<T> Deref for PgBox<T>
where
    T: Sized + Debug + PostgresStruct,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self.ptr {
            Some(ptr) => unsafe { &*ptr },
            None => panic!("Attempt to dereference null pointer during Deref of PgBox"),
        }
    }
}

impl<T> DerefMut for PgBox<T>
where
    T: Sized + Debug + PostgresStruct,
{
    fn deref_mut(&mut self) -> &mut T {
        match self.ptr {
            Some(ptr) => unsafe { &mut *ptr },
            None => panic!("Attempt to dereference null pointer during DerefMut of PgBox"),
        }
    }
}

impl<T> From<*mut T> for PgBox<T>
where
    T: Sized + Debug + PostgresStruct,
{
    fn from(ptr: *mut T) -> Self {
        PgBox::from_pg(ptr)
    }
}

impl<T> Drop for PgBox<T>
where
    T: Sized + Debug + PostgresStruct,
{
    fn drop(&mut self) {
        if self.ptr.is_some() {
            match self.owner {
                WhoAllocated::Postgres => { /* do nothing, we'll let Postgres free the pointer */ }
                WhoAllocated::Rust => {
                    // we own it here in rust, so we need to free it too
                    let ptr = self.ptr.unwrap();
                    unsafe {
                        pg_sys::pfree(ptr as *mut std::ffi::c_void);
                    }
                }
            }
        }
    }
}

impl<T> std::fmt::Display for PgBox<T>
where
    T: Sized + Debug + PostgresStruct,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.ptr {
            Some(_) => write!(f, "PgBox {{ {:?} }}", self.deref()), // std::fmt::Display::fmt(self.deref(), f),
            None => std::fmt::Display::fmt("NULL", f),
        }
    }
}
