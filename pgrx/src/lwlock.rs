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

/// Holds PostgreSQL's `AddinShmemInitLock` while an extension sets up its shared memory.
struct AddinShmemInitLock(*mut crate::pg_sys::LWLock);

impl AddinShmemInitLock {
    /// # Safety
    ///
    /// The main shared memory segment must be attached, as it is in `shmem_startup_hook`.
    unsafe fn exclusive() -> Self {
        // `AddinShmemInitLock` is entry 21 of the main LWLock array in every supported version.
        const ADDIN_SHMEM_INIT_LOCK_POS: usize = 21;
        // SAFETY: the caller guarantees that `MainLWLockArray` points to the attached main array,
        // which holds more than `ADDIN_SHMEM_INIT_LOCK_POS` locks.
        unsafe {
            let lock =
                &raw mut (*crate::pg_sys::MainLWLockArray.add(ADDIN_SHMEM_INIT_LOCK_POS)).lock;
            crate::pg_sys::LWLockAcquire(lock, crate::pg_sys::LWLockMode::LW_EXCLUSIVE);
            Self(lock)
        }
    }
}

impl Drop for AddinShmemInitLock {
    fn drop(&mut self) {
        // SAFETY: `self.0` is the lock that `exclusive` acquired.
        unsafe { release_unless_elog_unwinding(self.0) }
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
    use super::{AddinShmemInitLock, PgLwLockExclusiveGuard, PgLwLockShareGuard};
    use std::ffi::{CStr, c_int, c_void};
    use std::sync::atomic::{AtomicI32, Ordering};

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
    /// wraps. Data is byte-wise copied from the reference passed to [DsmLwLock::init] to the DSM.
    /// Its type must be self-contained, avoiding any reference or pointer to local memory, like
    /// heap-allocated data structures. The user must guarantee that this requirement is met by
    /// using a type implementing the [crate::PGRXSharedMemory] and [Copy] traits, or by providing
    /// its own implementation.
    ///
    /// After the lock is initialized on the DSM, it must be registered in every other involved
    /// process with [DsmLwLock::register], which returns a [DsmLwLockHandle] that can be stored in
    /// the process local memory. The handle provides methods to obtain
    /// [exclusive](DsmLwLockHandle::exclusive) or [shared](DsmLwLockHandle::share) lock guards.
    /// When dropped, a guard releases the lock. Quoting the PostgreSQL documentation, each process
    /// using the tranche must register it separately, as "dynamic shared memory segments aren't
    /// guaranteed to be mapped at the same address in all coordinating backends, so storing the
    /// registration in the main shared memory segment wouldn't work for that case".
    ///
    /// A handle points into the segment, so the segment must stay mapped for as long as the
    /// handle is used. [DsmLwLock::init] and [DsmLwLock::register] require this of their callers.
    #[repr(C)]
    pub struct DsmLwLock<T> {
        lock: crate::pg_sys::LWLock,
        data: T,
    }

    /// Handle to a DSM LWLock.
    /// It can be obtained from [DsmLwLock::register] after the DSM is initialized by
    /// [DsmLwLock::init].
    pub struct DsmLwLockHandle<T> {
        /// Non-null and aligned. The callers of `init` and `register` guarantee that it points to
        /// an initialized lock for as long as the handle is used.
        dsm: *mut DsmLwLock<T>,
    }

    impl<T: crate::PGRXSharedMemory + Copy> DsmLwLock<T> {
        /// Memory size in bytes required to store a lock instance, along its wrapped value.
        pub const fn mem_size() -> usize {
            size_of::<DsmLwLock<T>>()
        }

        fn handle(dsm: *mut c_void) -> DsmLwLockHandle<T> {
            let dsm = dsm.cast::<DsmLwLock<T>>();
            assert!(!dsm.is_null(), "dynamic shared memory pointer is null");
            assert!(dsm.is_aligned(), "dynamic shared memory is not aligned");
            DsmLwLockHandle { dsm }
        }

        /// Initialize the DSM with a lock and a copy of the wrapped value.
        ///
        /// `tranche_id` should come from [new_lwlock_tranche_id] or [DsmLwLockTranche].
        ///
        /// # Safety
        ///
        /// * `dsm` must point to at least [Self::mem_size] bytes that are valid for writes and
        ///   not in use by anything else.
        /// * `data` must not reference a value inside the `dsm` memory allocation.
        /// * The memory must stay mapped for as long as any handle to it is used, including a
        ///   lock guard leaked with [std::mem::forget], which PostgreSQL releases only at
        ///   transaction end.
        ///
        /// # Panics
        ///
        /// Panics if `dsm` is null or not aligned for `DsmLwLock<T>`.
        pub unsafe fn init(dsm: *mut c_void, tranche_id: c_int, data: &T) {
            let handle = Self::handle(dsm);
            // SAFETY: the caller guarantees that `dsm` is valid for writes of a `DsmLwLock<T>`
            // that nothing else uses, and that `data` lies outside it.
            unsafe {
                crate::pg_sys::LWLockInitialize(&raw mut (*handle.dsm).lock, tranche_id);
                (&raw mut (*handle.dsm).data).copy_from_nonoverlapping(data, 1);
            }
        }

        /// Returns a handle to a lock initialized with [Self::init].
        ///
        /// PostgreSQL 18 and earlier need each process to register the tranche `name`, which
        /// this does. PostgreSQL 19 keeps the name given to [new_lwlock_tranche_id] instead.
        ///
        /// # Safety
        ///
        /// * `dsm` must point to a lock that [Self::init] initialized for this same `T`.
        /// * The memory must stay mapped for as long as the handle is used, as [Self::init]
        ///   describes.
        ///
        /// # Panics
        ///
        /// Panics if `dsm` is null or not aligned for `DsmLwLock<T>`.
        pub unsafe fn register(dsm: *mut c_void, name: &'static CStr) -> DsmLwLockHandle<T> {
            let handle = Self::handle(dsm);
            // SAFETY: the caller guarantees that `dsm` holds an initialized lock, and `name`
            // is static, as `LWLockRegisterTranche` requires.
            #[cfg(not(feature = "pg19"))]
            unsafe {
                crate::pg_sys::LWLockRegisterTranche(
                    (*handle.dsm).lock.tranche.into(),
                    name.as_ptr(),
                );
            }
            #[cfg(feature = "pg19")]
            let _ = name;
            handle
        }
    }

    impl<T> DsmLwLockHandle<T> {
        /// Obtain a shared lock (which comes with `&T` access).
        pub fn share(&self) -> PgLwLockShareGuard<'_, T> {
            // SAFETY: `self.dsm` points to an initialized lock while the handle is used, as the
            // callers of `init` and `register` guarantee. The shared lock keeps writers out for
            // as long as the guard lends `&T`.
            unsafe {
                let lock = &raw mut (*self.dsm).lock;
                crate::pg_sys::LWLockAcquire(lock, crate::pg_sys::LWLockMode::LW_SHARED);
                PgLwLockShareGuard { data: &(*self.dsm).data, lock }
            }
        }

        /// Obtain an exclusive lock (which comes with `&mut T` access).
        pub fn exclusive(&self) -> PgLwLockExclusiveGuard<'_, T> {
            // SAFETY: as in `share`; the exclusive lock keeps every other holder out for as long
            // as the guard lends `&mut T`.
            unsafe {
                let lock = &raw mut (*self.dsm).lock;
                crate::pg_sys::LWLockAcquire(lock, crate::pg_sys::LWLockMode::LW_EXCLUSIVE);
                PgLwLockExclusiveGuard { data: &mut (*self.dsm).data, lock }
            }
        }
    }

    /// Request a new tranche ID for dynamically allocated LWLocks.
    ///
    /// PostgreSQL 19 stores `name` with the new ID. Earlier versions ignore it here, and each
    /// process registers the name through [DsmLwLock::register] instead.
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
    ///
    /// # Safety
    ///
    /// Must be called from a PostgreSQL process after the main shared memory is initialized, e.g.
    /// from the shared memory startup hook [crate::PgSharedMemoryInitialization::on_shmem_startup].
    pub unsafe fn new_lwlock_tranche_id(name: &CStr) -> c_int {
        // SAFETY: the caller guarantees that the main shared memory is initialized.
        #[cfg(feature = "pg19")]
        let tranche_id = unsafe { crate::pg_sys::LWLockNewTrancheId(name.as_ptr()) };
        #[cfg(not(feature = "pg19"))]
        let tranche_id = {
            let _ = name;
            // SAFETY: as above.
            unsafe { crate::pg_sys::LWLockNewTrancheId() }
        };
        if tranche_id > c_int::from(u16::MAX) {
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
        /// Zero until the shared memory startup hook stores the ID. PostgreSQL never assigns
        /// zero to an extension tranche, because the built-in tranches come first.
        tranche_id: AtomicI32,
    }

    impl DsmLwLockTranche {
        /// Define a LWLock tranche, along with the tranche name that backends will associate locks
        /// to when created from this tranche.
        pub const fn new(name: &'static CStr) -> Self {
            Self { name, tranche_id: AtomicI32::new(0) }
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
            match self.tranche_id.load(Ordering::Relaxed) {
                0 => panic!(
                    "uninitialized DSM LWLock tranche (use pg_shmem_init!() in _PG_init() to initialize it)"
                ),
                tranche_id => tranche_id,
            }
        }

        /// Initializes the DSM with a lock and a copy of the wrapped value, and registers the lock
        /// tranche.
        ///
        /// # Safety
        ///
        /// The requirements of [DsmLwLock::init].
        ///
        /// # Panics
        ///
        /// Panics if the tranche is uninitialized, or if `dsm` is null or not aligned for
        /// `DsmLwLock<T>`.
        pub unsafe fn init<T>(&self, dsm: *mut c_void, data: &T) -> DsmLwLockHandle<T>
        where
            T: crate::PGRXSharedMemory + Copy,
        {
            // SAFETY: the caller upholds the requirements of `DsmLwLock::init`, and that call
            // initializes the lock for `DsmLwLock::register`.
            unsafe {
                DsmLwLock::<T>::init(dsm, self.tranche_id(), data);
                DsmLwLock::register(dsm, self.name)
            }
        }

        /// Registers the lock tranche to associate its ID with a name.
        ///
        /// # Safety
        ///
        /// The requirements of [DsmLwLock::register]: another process initialized `dsm` with
        /// [Self::init] for this same `T`.
        ///
        /// # Panics
        ///
        /// Panics if `dsm` is null or not aligned for `DsmLwLock<T>`.
        pub unsafe fn register<T>(&self, dsm: *mut c_void) -> DsmLwLockHandle<T>
        where
            T: crate::PGRXSharedMemory + Copy,
        {
            // SAFETY: the caller upholds the requirements of `DsmLwLock::register`.
            unsafe { DsmLwLock::register(dsm, self.name) }
        }
    }

    impl crate::PgSharedMemoryInitialization for DsmLwLockTranche {
        type Value = ();

        unsafe fn on_shmem_request(&'static self) {
            // SAFETY: the trait requires this to run inside `shmem_request_hook`.
            unsafe { crate::pg_sys::RequestAddinShmemSpace(size_of::<c_int>()) };
        }

        unsafe fn on_shmem_startup(&'static self, _value: Self::Value) {
            // SAFETY: the trait requires this to run inside `shmem_startup_hook`, where the main
            // shared memory is attached.
            let addin_shmem_init_lock = unsafe { AddinShmemInitLock::exclusive() };

            // Looking up the tranche id in shared memory to guarantee that it is requested once
            // per shared memory lifetime, by the first process running the hook (the postmaster),
            // and reused by every backend.
            // On platforms where PostgreSQL is built with `EXEC_BACKEND` (e.g. Windows), backends
            // are spawned as new processes that run `_PG_init()` and the shmem startup hook again:
            // without this, every backend would request its own tranche id until we exhaust them.
            let mut found = false;
            // SAFETY: `ShmemInitStruct` returns aligned shared memory for a `c_int` or raises an
            // ERROR. `AddinShmemInitLock` makes the first process initialize the value before any
            // other process reads it.
            let tranche_id = unsafe {
                let shared_tranche_id = crate::pg_sys::ShmemInitStruct(
                    self.name.as_ptr(),
                    size_of::<c_int>(),
                    &mut found,
                )
                .cast::<c_int>();
                if !found {
                    shared_tranche_id.write(new_lwlock_tranche_id(self.name));
                }
                shared_tranche_id.read()
            };
            self.tranche_id.store(tranche_id, Ordering::Relaxed);
            drop(addin_shmem_init_lock);
        }
    }
}

/// LWLock for dynamic shared memory (DSM) during parallel foreign scans.
pub mod scan {
    use super::dsm::{DsmLwLock, DsmLwLockHandle, DsmLwLockTranche};
    use super::{PgLwLockExclusiveGuard, PgLwLockShareGuard};
    use std::borrow::BorrowMut;
    use std::ffi::{CStr, c_void};
    use std::ops::{Deref, DerefMut};

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
    /// because PostgreSQL can run a parallel plan without parallel mode, for example when the
    /// client fetches the result in batches, and then it never calls the FDW routines dedicated
    /// to the DSM initialization.
    ///
    /// PostgreSQL can destroy the DSM right after `ShutdownForeignScan`, before
    /// `ExplainForeignScan` and `EndForeignScan` run. Those routines must not use a lock that was
    /// moved to shared memory; copy anything they need out of it in `ShutdownForeignScan`.
    ///
    /// The shared state type must implement the [crate::PGRXSharedMemory] and [Copy] traits. Then,
    /// a reference to the LWLock-wrapped state can be stored in the parallel scan state with
    /// [ParallelScanLwLock].
    ///
    /// ```rust,no_run
    /// use ::pgrx::*;
    /// use ::pgrx_pg_sys::*;
    /// use ::pgrx::lwlock::scan::*;
    ///
    /// #[derive(Clone, Copy)]
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
    /// #[derive(Clone, Copy)]
    /// struct MyLargeSharedBuffer {
    ///     start: usize,
    ///     end: usize,
    ///     buffer: [u8; 10 * 1024 * 1024], // 10MB on the stack -> ☠️
    /// }
    ///
    /// unsafe impl PGRXSharedMemory for MyLargeSharedBuffer {} // yet DSM-safe
    ///
    /// impl MyLargeSharedBuffer {
    ///     pub fn boxed() -> Box<Self> {
    ///         // SAFETY: all zeroes is a valid value for every field.
    ///         unsafe { Box::new_zeroed().assume_init() }
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
    /// #[derive(Clone, Copy)]
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
    ///     // SAFETY: `fdw_state` is the scan state from `pgrx_begin_foreign_scan`, and PostgreSQL
    ///     // sized `shared_mem` with `pgrx_estimate_dsm_foreign_scan`.
    ///     unsafe {
    ///         let scan_state = &mut *((*foreign_scan_state).fdw_state as *mut MyParallelScanState);
    ///         // Initialize DSM and update the scan state to refer to the DSM-stored LWLock
    ///         scan_state.shared_state.initialize_dsm_and_register_leader(shared_mem);
    ///     }
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
    ///     // SAFETY: `fdw_state` is this worker's scan state, and the leader initialized
    ///     // `coordinate` in `pgrx_initialize_dsm_foreign_scan`.
    ///     unsafe {
    ///         let scan_state = &mut *((*foreign_scan_state).fdw_state as *mut MyParallelScanState);
    ///         // Update scan_state, replacing the invalid pointer from the parent process with the remapped DSM pointer
    ///         scan_state.shared_state.register_parallel_worker(coordinate);
    ///     }
    ///     // Initialize parallel worker local state, if needed
    /// }
    ///
    /// #[pg_guard]
    /// unsafe extern "C-unwind" fn pgrx_shutdown_foreign_scan(_node: *mut ForeignScanState) {
    ///     // PostgreSQL may destroy the DSM right after this returns. Copy out anything that
    ///     // `ExplainForeignScan` or `EndForeignScan` will need from the lock.
    /// }
    /// ```
    pub struct ParallelScanLwLock<T, A = T>
    where
        A: BorrowMut<T>,
    {
        tranche: &'static DsmLwLockTranche,
        data: ParallelScanSharedState<T, A>,
    }

    /// A shared LWLock guard that skips locking if the lock was not moved to shared memory yet.
    pub enum ParallelScanLwLockShareGuard<'a, T> {
        Local(&'a T),
        Shared(PgLwLockShareGuard<'a, T>),
    }

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
        Shared(PgLwLockExclusiveGuard<'a, T>),
    }

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
            Self { tranche, data: ParallelScanSharedState::Local(value) }
        }

        /// Obtain a shared lock (which comes with `&T` access).
        pub fn share(&self) -> ParallelScanLwLockShareGuard<'_, T> {
            match &self.data {
                ParallelScanSharedState::Local(value) => {
                    ParallelScanLwLockShareGuard::Local(value.borrow())
                }
                ParallelScanSharedState::Shared(handle) => {
                    ParallelScanLwLockShareGuard::Shared(handle.share())
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

    impl<T: crate::PGRXSharedMemory + Copy, A> ParallelScanLwLock<T, A>
    where
        A: BorrowMut<T>,
    {
        /// Bytes that `EstimateDSMForeignScan` must request for this lock.
        pub const fn mem_size() -> usize {
            DsmLwLock::<T>::mem_size()
        }

        /// To be called by the leader process of a parallel foreign scan within the
        /// `pgrx_initialize_dsm_foreign_scan` function, passing the `coordinate` memory that
        /// PostgreSQL provides. Copies the value into `dsm` and locks it there from now on.
        ///
        /// # Safety
        ///
        /// * `dsm` must satisfy the requirements of [DsmLwLock::init] for `T`: it holds at least
        ///   [ParallelScanLwLock::mem_size] bytes that nothing else uses, and it stays mapped for
        ///   as long as this lock is used. PostgreSQL can destroy the parallel query's DSM right
        ///   after `ShutdownForeignScan`, so `ExplainForeignScan` and `EndForeignScan` must not
        ///   use this lock.
        /// * The local value must not reference the `dsm` memory.
        ///
        /// # Panics
        ///
        /// This method panics if it or [Self::register_parallel_worker] were already called, or
        /// if `dsm` is null or not aligned for `DsmLwLock<T>`.
        pub unsafe fn initialize_dsm_and_register_leader(&mut self, dsm: *mut c_void) {
            let ParallelScanSharedState::Local(value) = &self.data else {
                panic!("DSM LWLock already initialized");
            };
            // SAFETY: the caller upholds the requirements of `DsmLwLockTranche::init`.
            let handle = unsafe { self.tranche.init(dsm, value.borrow()) };
            self.data = ParallelScanSharedState::Shared(handle);
        }

        /// To be called by parallel worker processes of a parallel foreign scan within the
        /// `pgrx_initialize_worker_foreign_scan` function, passing the `coordinate` memory that
        /// PostgreSQL provides.
        ///
        /// # Safety
        ///
        /// * The leader process must have passed the same memory to
        ///   [Self::initialize_dsm_and_register_leader], for a lock of the same `T`.
        /// * The memory must stay mapped for as long as this lock is used, as
        ///   [Self::initialize_dsm_and_register_leader] describes.
        ///
        /// # Panics
        ///
        /// This method panics if it or [Self::initialize_dsm_and_register_leader] were already
        /// called, or if `dsm` is null or not aligned for `DsmLwLock<T>`.
        pub unsafe fn register_parallel_worker(&mut self, dsm: *mut c_void) {
            assert!(
                matches!(self.data, ParallelScanSharedState::Local(_)),
                "DSM LWLock already initialized"
            );
            // SAFETY: the caller upholds the requirements of `DsmLwLockTranche::register`.
            self.data = ParallelScanSharedState::Shared(unsafe { self.tranche.register(dsm) });
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
            T: crate::PGRXSharedMemory + Copy,
            A: BorrowMut<T>,
        {
            ParallelScanLwLock::new(&self.0, value)
        }
    }

    impl crate::PgSharedMemoryInitialization for ParallelScanLwLockTranche {
        type Value = ();

        unsafe fn on_shmem_request(&'static self) {
            // SAFETY: the caller upholds the same requirements for the inner tranche.
            unsafe { self.0.on_shmem_request() }
        }

        unsafe fn on_shmem_startup(&'static self, value: Self::Value) {
            // SAFETY: the caller upholds the same requirements for the inner tranche.
            unsafe { self.0.on_shmem_startup(value) }
        }
    }
}
