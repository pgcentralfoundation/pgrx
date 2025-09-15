//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use crate::{PGRXSharedMemory, PgSharedMemoryInitialization};
use core::ops::{Deref, DerefMut};
use std::cell::UnsafeCell;
use std::ffi::CStr;

/// A Rust locking mechanism which uses a PostgreSQL LWLock to lock the data.
///
/// This type of lock allows a number of readers or at most one writer at any
/// point in time. The write portion of this lock typically allows modification
/// of the underlying data (exclusive access) and the read portion of this lock
/// typically allows for read-only access (shared access).
///
/// The lock is valid across processes as the LWLock is managed by Postgres. Data
/// mutability once a lock is obtained is handled by Rust giving out `&` or `&mut`
/// pointers.
///
/// When a lock is given out it is wrapped in a PgLwLockShareGuard or
/// PgLwLockExclusiveGuard, which releases the lock on drop
///
/// # Poisoning
/// This lock can not be poisoned from Rust. Panic and Abort are handled by
/// PostgreSQL cleanly.
pub struct PgLwLock<T> {
    name: &'static CStr,
    inner: UnsafeCell<*mut Shared<T>>,
}

unsafe impl<T: PGRXSharedMemory> Sync for PgLwLock<T> {}

impl<T> PgLwLock<T> {
    /// Create a pointer of a lock that points to nothing.
    ///
    /// # Safety
    ///
    /// * Caller must be confident that there are no name conflicts.
    pub const unsafe fn new(name: &'static CStr) -> Self {
        Self { name, inner: UnsafeCell::new(std::ptr::null_mut()) }
    }

    /// Get the name of the atomic.
    pub const fn name(&self) -> &'static CStr {
        self.name
    }
}

impl<T: PGRXSharedMemory> PgLwLock<T> {
    /// Obtain a shared lock (which comes with `&T` access).
    pub fn share(&self) -> PgLwLockShareGuard<'_, T> {
        unsafe {
            let shared = self.inner.get().read().as_ref().expect("PgLwLock was not initialized");
            crate::pg_sys::LWLockAcquire(shared.lock, crate::pg_sys::LWLockMode::LW_SHARED);
            PgLwLockShareGuard { data: &*shared.data.get(), lock: shared.lock }
        }
    }

    /// Obtain an exclusive lock (which comes with `&mut T` access).
    pub fn exclusive(&self) -> PgLwLockExclusiveGuard<'_, T> {
        unsafe {
            let shared = self.inner.get().read().as_ref().expect("PgLwLock was not initialized");
            crate::pg_sys::LWLockAcquire(shared.lock, crate::pg_sys::LWLockMode::LW_EXCLUSIVE);
            PgLwLockExclusiveGuard { data: &mut *shared.data.get(), lock: shared.lock }
        }
    }
}

struct AddinShmemInitLock(*mut crate::pg_sys::LWLock);

impl AddinShmemInitLock {
    unsafe fn exclusive() -> Self {
        const ADDIN_SHMEM_INIT_LOCK_POS: usize = 21;
        let lock = &raw mut (*crate::pg_sys::MainLWLockArray.add(ADDIN_SHMEM_INIT_LOCK_POS)).lock;
        crate::pg_sys::LWLockAcquire(lock, crate::pg_sys::LWLockMode::LW_EXCLUSIVE);
        Self(lock)
    }
}

impl Drop for AddinShmemInitLock {
    fn drop(&mut self) {
        unsafe {
            if !self.0.is_null() {
                crate::pg_sys::LWLockRelease(self.0);
            }
        }
    }
}

impl<T: PGRXSharedMemory> PgSharedMemoryInitialization for PgLwLock<T> {
    type Value = T;

    unsafe fn on_shmem_request(&'static self) {
        unsafe {
            crate::pg_sys::RequestAddinShmemSpace(size_of::<Shared<T>>());
            crate::pg_sys::RequestNamedLWLockTranche(self.name.as_ptr(), 1);
        }
    }

    unsafe fn on_shmem_startup(&'static self, value: T) {
        unsafe {
            use crate::pg_sys;

            let shm_name = self.name;
            let addin_shmem_init_lock = AddinShmemInitLock::exclusive();

            let mut found = false;
            let fv_shmem =
                pg_sys::ShmemInitStruct(shm_name.as_ptr(), size_of::<Shared<T>>(), &mut found)
                    .cast::<Shared<T>>();
            assert!(fv_shmem.is_aligned(), "shared memory is not aligned");
            if !found {
                fv_shmem.write(Shared {
                    data: UnsafeCell::new(value),
                    lock: &raw mut (*pg_sys::GetNamedLWLockTranche(shm_name.as_ptr())).lock,
                });
            }

            *self.inner.get() = fv_shmem;

            drop(addin_shmem_init_lock);
        }
    }
}

#[repr(C)]
struct Shared<T> {
    data: UnsafeCell<T>,
    lock: *mut crate::pg_sys::LWLock,
}

pub struct PgLwLockShareGuard<'a, T> {
    data: &'a T,
    lock: *mut crate::pg_sys::LWLock,
}

unsafe impl<T: PGRXSharedMemory> Sync for PgLwLockShareGuard<'_, T> {}

impl<T> Drop for PgLwLockShareGuard<'_, T> {
    fn drop(&mut self) {
        // SAFETY: self.lock is always valid
        unsafe { release_unless_elog_unwinding(self.lock) }
    }
}

impl<T> Deref for PgLwLockShareGuard<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        self.data
    }
}

pub struct PgLwLockExclusiveGuard<'a, T> {
    data: &'a mut T,
    lock: *mut crate::pg_sys::LWLock,
}

unsafe impl<T: PGRXSharedMemory> Sync for PgLwLockExclusiveGuard<'_, T> {}

impl<T> Deref for PgLwLockExclusiveGuard<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        self.data
    }
}

impl<T> DerefMut for PgLwLockExclusiveGuard<'_, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        self.data
    }
}

impl<T> Drop for PgLwLockExclusiveGuard<'_, T> {
    fn drop(&mut self) {
        // SAFETY: self.lock is always valid
        unsafe { release_unless_elog_unwinding(self.lock) }
    }
}

/// Releases the given lock, unless we are unwinding due to an `error` in postgres code
///
/// `elog(ERROR)` from postgres code resets `pg_sys::InterruptHoldoffCount` to zero, and
/// `LWLockRelease` fails an assertion if called in this case.
/// If we detect this condition, we skip releasing the lock; all lwlocks will be released
/// on (sub)transaction abort anyway.
///
/// SAFETY: the given lock must be valid
unsafe fn release_unless_elog_unwinding(lock: *mut crate::pg_sys::LWLock) {
    // SAFETY: mut static access is ok from a single (main) thread.
    if crate::pg_sys::InterruptHoldoffCount > 0 {
        crate::pg_sys::LWLockRelease(lock);
    }
}

/// LWLock for dynamic shared memory (DSM).
pub mod dsm {
    use crate::lwlock::AddinShmemInitLock;
    use std::cell::UnsafeCell;
    use std::ffi::{CStr, c_int, c_void};

    /// A PostgreSQL LWLock-backed locking mechanism for dynamic shared memory (DSM).
    ///
    /// This is a lower level component which defines operations close to the PostgreSQL LWLock API.
    /// If you are interested in locking the shared memory of a foreign parallel scan, please refer
    /// to [ParallelScanLwLock](crate::lwlock::scan::ParallelScanLwLock) and
    /// [ParallelScanLwLockTranche](crate::lwlock::scan::ParallelScanLwLockTranche).
    ///
    /// # Usage
    ///
    /// First, the user may need to obtain a new tranche ID, which is a marker for a family of
    /// LWLocks. It can be retrieved by calling [new_lwlock_tranche_id], and it's recommended
    /// that a tranche ID for a LWLock family is retrieved once per server instance. This function
    /// can be called during the extension setup in the SHMEM hooks (which are made available by
    /// pgrx through the [crate::PgSharedMemoryInitialization] trait and the [crate::pg_shmem_init!]
    /// macro). If you find more convenient a tranche ID provided at startup time, refer to the
    /// documentation of the [DsmLwLockTranche] type.
    ///
    /// In addition to the tranche ID, the DSM lock must be initialized along with the data it
    /// wraps. Data is byte-wise copied from the pointer passed to [DsmLwLock::init] to the DSM. Its
    /// type must be self-contained, avoiding any reference or pointer to local memory, like
    /// heap-allocated data structures. The user must guarantee that this requirement is met by
    /// using a type implementing the [crate::PGRXSharedMemory] trait, or by providing its own
    /// implementation.
    ///
    /// After the lock is initialized on the DSM, it must be registered in every involved process
    /// with [DsmLwLock::register], which returns a [DsmLwLockHandle] that can be stored in the
    /// process local memory. The handle provides methods to obtain
    /// [exclusive](DsmLwLockHandle::exclusive) or [shared](DsmLwLockHandle::shared) lock guards.
    /// When dropped, a guard releases the lock. Quoting the PostgreSQL documentation, each process
    /// using the tranche must register it separately, as "dynamic shared memory segments aren't
    /// guaranteed to be mapped at the same address in all coordinating backends, so storing the
    /// registration in the main shared memory segment wouldn't work for that case".
    #[repr(C)]
    pub struct DsmLwLock<T> {
        lock: crate::pg_sys::LWLock,
        data: T,
    }

    /// Handle to a DSM LWLock.
    /// It can be obtained from [DsmLwLock::register] after the DSM is initialized by
    /// [DsmLwLock::init].
    pub struct DsmLwLockHandle<T> {
        handle: *mut DsmLwLock<T>,
    }

    impl<T: crate::PGRXSharedMemory> DsmLwLock<T> {
        /// Memory size in bytes required to store a lock instance, along its wrapped value.
        pub const fn mem_size() -> usize {
            size_of::<DsmLwLock<T>>()
        }

        /// Initialize the DSM with a lock and a copy of the wrapped value.
        ///
        /// # Safety
        ///
        /// * `dsm` must not be null.
        /// * `dsm` must be aligned.
        /// * `dsm` must have at least [Self::mem_size] space.
        /// * `data` must not be null.
        /// * `data` must not point to an address inside the `dsm` memory allocation.
        pub unsafe fn init(dsm: *mut c_void, tranche_id: c_int, data: *const T) {
            assert!(dsm.is_aligned(), "dynamic shared memory is not aligned");
            let dsm = dsm as *mut DsmLwLock<T>;
            (&raw mut (*dsm).lock).write(crate::pg_sys::LWLock::default());
            crate::pg_sys::LWLockInitialize(&raw mut (*dsm).lock, tranche_id);
            (&raw mut (*dsm).data).copy_from_nonoverlapping(data, 1);
        }

        /// Register the lock tranche to associate its ID with a name.
        ///
        /// # Safety
        ///
        /// * `dsm` was already initialized with [Self::init] (and therefore all its safety requirements are met).
        pub unsafe fn register(dsm: *mut c_void, name: &'static CStr) -> DsmLwLockHandle<T> {
            assert!(dsm.is_aligned(), "dynamic shared memory is not aligned");
            let dsm = dsm as *mut DsmLwLock<T>;
            crate::pg_sys::LWLockRegisterTranche((*dsm).lock.tranche as _, name.as_ptr());
            DsmLwLockHandle { handle: dsm }
        }
    }

    impl<T> DsmLwLockHandle<T> {
        /// Obtain a shared lock (which comes with `&T` access).
        pub fn shared(&self) -> super::PgLwLockShareGuard<'_, T> {
            assert!(!self.handle.is_null(), "unregistered DSM LWLock handle");
            unsafe {
                let lock_ptr = (&raw mut (*self.handle).lock);
                crate::pg_sys::LWLockAcquire(lock_ptr, crate::pg_sys::LWLockMode::LW_SHARED);
                super::PgLwLockShareGuard {
                    data: (&raw const (*self.handle).data)
                        .as_ref()
                        .expect("Unexpected null raw pointer to field"),
                    lock: lock_ptr,
                }
            }
        }

        /// Obtain an exclusive lock (which comes with `&mut T` access).
        pub fn exclusive(&self) -> super::PgLwLockExclusiveGuard<'_, T> {
            assert!(!self.handle.is_null(), "unregistered DSM LWLock handle");
            unsafe {
                let lock_ptr = (&raw mut (*self.handle).lock);
                crate::pg_sys::LWLockAcquire(lock_ptr, crate::pg_sys::LWLockMode::LW_EXCLUSIVE);
                super::PgLwLockExclusiveGuard {
                    data: (&raw mut (*self.handle).data)
                        .as_mut()
                        .expect("Unexpected null raw pointer to field"),
                    lock: lock_ptr,
                }
            }
        }
    }

    /// Request a new tranche ID for dynamically allocated LWLocks.
    ///
    /// # Caution
    ///
    /// Use parsimoniously, for locks store tranche IDs in 16-bit unsigned integers. The user
    /// should not request a tranche ID per LWLock instance, but per LWLock family instead,
    /// which groups instances of locks created for the same purpose.
    ///
    /// # Panics
    ///
    /// This function checks whether the next tranche ID exceeds the unsigned 16-bit boundary,
    /// to avoid subtle errors inside PostgreSQL LWLock API that may associate a new lock to a
    /// completely different tranche because of the ID truncation.
    pub fn new_lwlock_tranche_id() -> c_int {
        let tranche_id = unsafe { crate::pg_sys::LWLockNewTrancheId() };
        if tranche_id > (u16::MAX as i32) {
            panic!(
                "all valid LWLock tranche IDs have been consumed: this or any other extension is probably requesting a new tranche ID on every dynamic LWLock creation"
            );
        }
        tranche_id
    }

    /// Component that obtains a LWLock tranche ID on Postgres Shared Memory initialization.
    ///
    /// To be used as the type for a static global, initialized by the [crate::pg_shmem_init!] macro
    /// in the extension `_PG_init()` function.
    ///
    /// ```rust,no_run
    /// use ::pgrx::*;
    /// use ::pgrx_pg_sys::*;
    /// use ::pgrx::lwlock::dsm::*;
    ///
    /// static LOCKS_FOR_MY_TASK: DsmLwLockTranche = DsmLwLockTranche::new(c"my_task_lock");
    ///
    /// #[allow(non_snake_case)]
    /// #[pg_guard]
    /// pub extern "C-unwind" fn _PG_init() {
    ///     //...
    ///     pg_shmem_init!(LOCKS_FOR_MY_TASK);
    /// }
    /// ```
    ///
    /// This type provides convenience methods for initializing and registering LWLocks on the DSM.
    /// Safety requirements are those specified in the respective [DsmLwLock] functions.
    pub struct DsmLwLockTranche {
        name: &'static CStr,
        lock: UnsafeCell<Option<c_int>>,
    }

    /// UnsafeCell cannot be shared between threads safely, we allow its use within static globals.
    unsafe impl Sync for DsmLwLockTranche {}

    impl DsmLwLockTranche {
        /// Define a LWLock tranche, along with the tranche name that backends will associate locks
        /// to when created from this tranche.
        pub const fn new(name: &'static CStr) -> Self {
            Self { name, lock: UnsafeCell::new(None) }
        }

        /// The name assigned to this tranche.
        pub const fn name(&self) -> &'static CStr {
            self.name
        }

        /// Get the tranche ID.
        /// Make sure that the static global is initialized by the [crate::pg_shmem_init!] macro in
        /// `_PG_init()`.
        ///
        /// # Panics
        ///
        /// This method must not be invoked on an uninitialized tranche, otherwise it will panic.
        pub fn tranche_id(&self) -> c_int {
            unsafe {
                (*self.lock.get()).expect("uninitialized DSM LWLock tranche (use pg_shmem_init!() in _PG_init() to initialize it)")
            }
        }

        /// Initialize the DSM with a lock and a copy of the wrapped value.
        ///
        /// # Panics
        ///
        /// This method must not be invoked on an uninitialized tranche, otherwise it will panic.
        ///
        /// # Safety
        ///
        /// * `dsm` must not be null.
        /// * `dsm` must have at least [DsmLwLock::mem_size] space.
        /// * `data` must not be null.
        /// * `data` must not point to an address inside the `dsm` memory allocation.
        pub unsafe fn init<T>(&self, dsm: *mut c_void, data: *const T)
        where
            T: crate::PGRXSharedMemory,
        {
            DsmLwLock::<T>::init(dsm, self.tranche_id(), data)
        }

        /// Register the lock tranche to associate its ID with a name.
        ///
        /// # Safety
        ///
        /// * `dsm` was already initialized with [Self::init] (and therefore all its safety
        ///   requirements are met).
        pub unsafe fn register<T>(&self, dsm: *mut c_void) -> DsmLwLockHandle<T>
        where
            T: crate::PGRXSharedMemory,
        {
            DsmLwLock::<T>::register(dsm, self.name)
        }
    }

    impl crate::PgSharedMemoryInitialization for DsmLwLockTranche {
        type Value = ();

        unsafe fn on_shmem_request(&'static self) {
            // Nothing to do here
        }

        unsafe fn on_shmem_startup(&'static self, _value: Self::Value) {
            let addin_shmem_init_lock = AddinShmemInitLock::exclusive();
            if (*self.lock.get()).is_none() {
                *self.lock.get() = Some(new_lwlock_tranche_id());
            }
            drop(addin_shmem_init_lock);
        }
    }
}

/// LWLock for dynamic shared memory (DSM) during parallel foreign scans.
pub mod scan {
    use super::dsm::{DsmLwLock, DsmLwLockHandle, DsmLwLockTranche};
    use std::borrow::{Borrow, BorrowMut};
    use std::ffi::{CStr, c_void};
    use std::ops::{Deref, DerefMut};
    use std::panic::AssertUnwindSafe;

    enum ParallelScanSharedState<T, A> {
        Local(A),
        Shared(DsmLwLockHandle<T>),
    }

    /// This type of lock is designed to manage access to the DSM allocated for parallel foreign
    /// scans, to coordinate work among parallel workers. Some of the FDW routines exposed by
    /// PostgreSQL provide a template for setting up shared state once, in the leader process, and
    /// then use it to initialize parallel workers local state. Creating a LWLock in the DSM from
    /// the leader process guarantees that the lock is initialized once. When using this locking
    /// mechanism for other types of shared memory, the user must guarantee that the lock
    /// initialization is run once by one of the participating worker processes.
    ///
    /// The value wrapped by this lock is initially stored within the local memory, then it's copied
    /// to the shared memory upon its initialization. This transitional local state is necessary,
    /// because the FDW routines dedicated to the DSM initialization may not be called for parallel
    /// foreign scans with the participation of a single parallel worker process.
    ///
    /// The shared state type must implement the [crate::PGRXSharedMemory] trait. Then, a reference
    /// to the LWLock-wrapped state can be stored in the parallel scan state with
    /// [ParallelScanLwLock].
    ///
    /// ```rust,no_run
    /// use ::pgrx::*;
    /// use ::pgrx_pg_sys::*;
    /// use ::pgrx::lwlock::scan::*;
    ///
    /// struct MySharedState {
    ///     a: usize,
    ///     b: i64,
    /// }
    ///
    /// unsafe impl PGRXSharedMemory for MySharedState {}
    ///
    /// struct MyParallelScanState {
    ///     local_data: Vec<u8>,
    ///     shared_data: ParallelScanLwLock<MySharedState>,
    /// }
    /// ```
    ///
    /// The user may need to store large datatypes in the DSM, e.g. buffers or other data structures
    /// with a pre-allocated capacity. As the wrapped type must not contain any reference or pointer
    /// (see [DsmLwLock]), the value to be allocated may be too large to fit on the stack. To
    /// overcome such kind of memory issues, the user should use instead a [Box] or any other boxing
    /// type that implements [BorrowMut] for its type.
    ///
    /// ```rust,no_run
    /// use ::pgrx::*;
    /// use ::pgrx_pg_sys::*;
    /// use ::pgrx::lwlock::scan::*;
    ///
    /// struct MyLargeSharedBuffer {
    ///     start: usize,
    ///     end: usize,
    ///     buffer: [u8; 10 * 1024 * 1024], // 10MB on the stack -> ☠️
    /// }
    ///
    /// unsafe impl PGRXSharedMemory for MyLargeSharedBuffer {} // yet DSM-safe
    ///
    /// impl MyLargeSharedBuffer {
    ///
    ///     pub fn boxed() -> Box<Self> {
    ///         unsafe {
    ///             let ptr = std::alloc::alloc_zeroed(std::alloc::Layout::new::<MyLargeSharedBuffer>()) as *mut MyLargeSharedBuffer;
    ///             (&raw mut (*ptr).start).write(0usize);
    ///             (&raw mut (*ptr).end).write(0usize);
    ///             Box::from_raw(ptr)
    ///         }
    ///     }
    /// }
    ///
    /// struct MyParallelScanState {
    ///     local_buffer: Vec<u8>,
    ///     shared_buffer: ParallelScanLwLock<MyLargeSharedBuffer, Box<MyLargeSharedBuffer>>,
    /// }
    /// ```
    ///
    /// # Lock initialization
    ///
    /// The user should declare a static global for a LWLock tranche of type
    /// [ParallelScanLwLockTranche] to be initialized with the [crate::pg_shmem_init!] macro in the
    /// extension `_PG_init()` function. Then the lock methods must be called within the PostgreSQL
    /// parallel foreign scan routines.
    ///
    /// ```rust,no_run
    /// use ::pgrx::*;
    /// use ::pgrx_pg_sys::*;
    /// use ::pgrx::lwlock::dsm::*;
    /// use ::pgrx::lwlock::scan::*;
    ///
    /// struct MySharedState {
    /// //...
    /// }
    ///
    /// unsafe impl PGRXSharedMemory for MySharedState {}
    ///
    /// struct MyParallelScanState {
    ///     local_state: Vec<u8>,
    ///     shared_state: ParallelScanLwLock<MySharedState, Box<MySharedState>>,
    /// }
    ///
    /// static PARALLEL_SCAN_LWLOCKS: ParallelScanLwLockTranche = ParallelScanLwLockTranche::new(c"parallel_scan_lock");
    ///
    /// #[allow(non_snake_case)]
    /// #[pg_guard]
    /// pub extern "C-unwind" fn _PG_init() {
    ///     // other required initialization
    ///     pg_shmem_init!(PARALLEL_SCAN_LWLOCKS);
    /// }
    ///
    /// #[pg_guard]
    /// extern "C-unwind" fn pgrx_begin_foreign_scan(foreign_scan_state: *mut ForeignScanState, _eflags: ::std::os::raw::c_int) {
    ///     // Other foreign scan setup...
    ///
    ///     let scan_state = MyParallelScanState {
    ///         local_state: vec![],
    ///         shared_state: PARALLEL_SCAN_LWLOCKS.lock_for(Box::new(MySharedState { /* ... */ } )),
    ///     };
    ///
    ///     // We rely on Postgres memory context to drop our value on delete
    ///     unsafe { (*foreign_scan_state).fdw_state = PgMemoryContexts::CurrentMemoryContext.leak_and_drop_on_delete(scan_state) as *mut std::os::raw::c_void };
    /// }
    ///
    /// #[pg_guard]
    /// unsafe extern "C-unwind" fn pgrx_is_foreign_scan_parallel_safe(_root: *mut PlannerInfo, _rel: *mut RelOptInfo, _rte: *mut RangeTblEntry) -> bool {
    ///     true
    /// }
    ///
    /// #[pg_guard]
    /// unsafe extern "C-unwind" fn pgrx_estimate_dsm_foreign_scan(_foreign_scan_state: *mut ForeignScanState, _pcxt: *mut ParallelContext) -> Size {
    ///     ParallelScanLwLock::<MySharedState, Box<_>>::mem_size()
    /// }
    ///
    /// #[pg_guard]
    /// unsafe extern "C-unwind" fn pgrx_initialize_dsm_foreign_scan(foreign_scan_state: *mut ForeignScanState, _pcxt: *mut ParallelContext, shared_mem: *mut ::std::os::raw::c_void) {
    ///     let scan_state = &mut *((*foreign_scan_state).fdw_state as *mut MyParallelScanState);
    ///     // Initialize DSM and update the scan state to refer to the DSM-stored LWLock
    ///     scan_state.shared_state.initialize_dsm_and_register_leader(shared_mem);
    ///     // Initialize leader process local state, if needed
    /// }
    ///
    /// #[pg_guard]
    /// unsafe extern "C-unwind" fn pgrx_reinitialize_dsm_foreign_scan(_node: *mut ForeignScanState, _pcxt: *mut ParallelContext, _coordinate: *mut ::std::os::raw::c_void) {
    ///     // Do some re-initialization, if needed
    /// }
    ///
    /// #[pg_guard]
    /// unsafe extern "C-unwind" fn pgrx_initialize_worker_foreign_scan(foreign_scan_state: *mut ForeignScanState, _toc: *mut shm_toc, coordinate: *mut ::std::os::raw::c_void) {
    ///     let scan_state = &mut *((*foreign_scan_state).fdw_state as *mut MyParallelScanState);
    ///     // Update scan_state, replacing the invalid pointer from the parent process with the remapped DSM pointer
    ///     scan_state.shared_state.register_parallel_worker(coordinate);
    ///     // Initialize parallel worker local state, if needed
    /// }
    ///
    /// #[pg_guard]
    /// unsafe extern "C-unwind" fn pgrx_shutdown_foreign_scan(_node: *mut ForeignScanState) {
    ///     // Invoked when the node will not be executed to completion, if you wish to take some action
    ///     // before the DSM segment is destroyed. Look at FDW callbacks documentation for more info.
    /// }
    /// ```
    pub struct ParallelScanLwLock<T, A = T>
    where
        A: BorrowMut<T>,
    {
        tranche: AssertUnwindSafe<&'static DsmLwLockTranche>,
        data: ParallelScanSharedState<T, A>,
    }

    /// A shared LWLock guard that skips locking if the lock was not moved to shared memory yet.
    pub enum ParallelScanLwLockShareGuard<'a, T> {
        Local(&'a T),
        Shared(super::PgLwLockShareGuard<'a, T>),
    }

    unsafe impl<T: crate::PGRXSharedMemory> Sync for ParallelScanLwLockShareGuard<'_, T> {}

    impl<T> Deref for ParallelScanLwLockShareGuard<'_, T> {
        type Target = T;

        #[inline]
        fn deref(&self) -> &T {
            match self {
                Self::Local(value) => value,
                Self::Shared(guard) => guard.deref(),
            }
        }
    }

    /// An exclusive LWLock guard that skips locking if the lock was not moved to shared memory yet.
    pub enum ParallelScanLwLockExclusiveGuard<'a, T> {
        Local(&'a mut T),
        Shared(super::PgLwLockExclusiveGuard<'a, T>),
    }

    unsafe impl<T: crate::PGRXSharedMemory> Sync for ParallelScanLwLockExclusiveGuard<'_, T> {}

    impl<T> Deref for ParallelScanLwLockExclusiveGuard<'_, T> {
        type Target = T;

        #[inline]
        fn deref(&self) -> &T {
            match self {
                Self::Local(value) => value,
                Self::Shared(guard) => guard.deref(),
            }
        }
    }

    impl<T> DerefMut for ParallelScanLwLockExclusiveGuard<'_, T> {
        #[inline]
        fn deref_mut(&mut self) -> &mut T {
            match self {
                Self::Local(value) => value,
                Self::Shared(guard) => guard.deref_mut(),
            }
        }
    }

    impl<T, A> ParallelScanLwLock<T, A>
    where
        A: BorrowMut<T>,
    {
        /// Constructs a new LWLock, given a tranche and an initial value (or a box type from which
        /// you can [BorrowMut] it).
        pub fn new(tranche: &'static DsmLwLockTranche, value: A) -> Self {
            Self { tranche: AssertUnwindSafe(tranche), data: ParallelScanSharedState::Local(value) }
        }

        /// Obtain a shared lock (which comes with `&T` access).
        pub fn shared(&self) -> ParallelScanLwLockShareGuard<'_, T> {
            match &self.data {
                ParallelScanSharedState::Local(value) => {
                    ParallelScanLwLockShareGuard::Local(value.borrow())
                }
                ParallelScanSharedState::Shared(handle) => {
                    ParallelScanLwLockShareGuard::Shared(handle.shared())
                }
            }
        }

        /// Obtain an exclusive lock (which comes with `&mut T` access).
        pub fn exclusive(&mut self) -> ParallelScanLwLockExclusiveGuard<'_, T> {
            match &mut self.data {
                ParallelScanSharedState::Local(value) => {
                    ParallelScanLwLockExclusiveGuard::Local(value.borrow_mut())
                }
                ParallelScanSharedState::Shared(handle) => {
                    ParallelScanLwLockExclusiveGuard::Shared(handle.exclusive())
                }
            }
        }
    }

    impl<T: crate::PGRXSharedMemory, A> ParallelScanLwLock<T, A>
    where
        A: BorrowMut<T>,
    {
        pub const fn mem_size() -> usize {
            DsmLwLock::<T>::mem_size()
        }

        /// To be called by the leader process of a parallel foreign scan within the
        /// `pgrx_initialize_dsm_foreign_scan` function.
        ///
        /// # Panics
        ///
        /// This method panics if it or [Self::register_parallel_worker] were already called.
        ///
        /// # Safety
        ///
        /// * `dsm` must not be null.
        /// * `dsm` must have at least [ParallelScanLwLock::mem_size] space.
        pub unsafe fn initialize_dsm_and_register_leader(&mut self, dsm: *mut c_void) {
            match &self.data {
                ParallelScanSharedState::Local(value) => {
                    self.tranche.init(dsm, <A as Borrow<T>>::borrow(value) as *const T);
                    self.data = ParallelScanSharedState::Shared(self.tranche.register(dsm));
                }
                ParallelScanSharedState::Shared(_) => {
                    panic!("DSM LWLock already initialized");
                }
            }
        }

        /// To be called by parallel worker processes of a parallel foreign scan within the
        /// `pgrx_initialize_worker_foreign_scan` function.
        ///
        /// # Safety
        ///
        /// * The leader process must have invoked [Self::initialize_dsm_and_register_leader] on
        ///   `dsm` first (and therefore all its safety requirements are met).
        pub unsafe fn register_parallel_worker(&mut self, dsm: *mut c_void) {
            self.data = ParallelScanSharedState::Shared(self.tranche.register(dsm));
        }
    }

    /// LWLock tranche for foreign parallel scans.
    /// Similar to the [DsmLwLockTranche] type, offering a more convenient method to create a LWLock
    /// directly from the tranche instance.
    pub struct ParallelScanLwLockTranche(DsmLwLockTranche);

    impl ParallelScanLwLockTranche {
        /// Define a LWLock tranche, along with the tranche name that backends will associate locks
        /// to when created from this tranche.
        pub const fn new(name: &'static CStr) -> Self {
            Self(DsmLwLockTranche::new(name))
        }

        /// Creates a new LWLock associated to this tranche.
        pub fn lock_for<T, A>(&'static self, value: A) -> ParallelScanLwLock<T, A>
        where
            T: crate::PGRXSharedMemory,
            A: BorrowMut<T>,
        {
            ParallelScanLwLock::new(&self.0, value)
        }
    }

    impl crate::PgSharedMemoryInitialization for ParallelScanLwLockTranche {
        type Value = ();

        unsafe fn on_shmem_request(&'static self) {
            self.0.on_shmem_request()
        }

        unsafe fn on_shmem_startup(&'static self, value: Self::Value) {
            self.0.on_shmem_startup(value)
        }
    }
}
