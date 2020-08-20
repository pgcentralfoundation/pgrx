//#[macro_use]
//extern crate static_assertions;
use crate::lwlock::*;
use crate::pg_sys;
//use impls::impls;
use uuid::Uuid;

pub unsafe trait PGXSharedMemory {}

#[macro_export]
macro_rules! pgx_sharedmem_locked {
    ($item_name:ident, $item_type:ty) => {
        //This doesn't give a nice error message -> would use if it did
        //const_assert!(impls!($item_type: PGXSharedMemory));

        thread_local! {
            static $item_name: PgLwLock<&'static mut $item_type> = PgLwLock::empty(stringify!($item_name));
        }
    };
}

/// This macro is used to create a global which will be used to access shared memory components
#[macro_export]
macro_rules! pgx_sharedmem_atomic {
    ($item_name:ident, $item_type:ty = $value:expr) => {
        static $item_name: $item_type = <$item_type>::new($value);
    };
}

/// This struct contains methods to drive creation of types in shared memory
pub struct PgSharedMem {}

impl PgSharedMem {
    /// Must be run from PG_init, use for types which are guarded by a LWLock
    pub fn pg_init_locked<T: Default + PGXSharedMemory>(
        pgstatic: &'static std::thread::LocalKey<PgLwLock<&'static mut T>>,
    ) {
        unsafe {
            pgstatic.with(|lock| {
                let lock = std::ffi::CString::new(lock.get_name()).expect("CString::new failed");
                pg_sys::RequestAddinShmemSpace(std::mem::size_of::<T>());
                pg_sys::RequestNamedLWLockTranche(lock.as_ptr(), 1);
            });
        }
    }

    // Test version
    pub fn pg_init_locked_sized<T: PGXSharedMemory>(
        pgstatic: &'static std::thread::LocalKey<PgLwLock<&'static mut T>>,
        size: usize,
    ) {
        unsafe {
            pgstatic.with(|lock| {
                let lock = std::ffi::CString::new(lock.get_name()).expect("CString::new failed");
                pg_sys::RequestAddinShmemSpace(std::mem::size_of::<T>() + size);
                pg_sys::RequestNamedLWLockTranche(lock.as_ptr(), 1);
            });
        }
    }

    /// Must be run from PG_init for atomics
    pub fn pg_init_atomic<T: atomic_traits::Atomic + Default>(_pgstatic: &'static T) {
        unsafe {
            pg_sys::RequestAddinShmemSpace(std::mem::size_of::<T>());
        }
    }

    /// Must be run from the shared memory init hook, use for types which are guarded by a LWLock
    pub fn shmem_init_locked<T: Default + PGXSharedMemory>(
        pgstatic: &'static std::thread::LocalKey<PgLwLock<&'static mut T>>,
    ) {
        let mut found = false;
        pgstatic.with(|lock| unsafe {
            let shm_name = std::ffi::CString::new(lock.get_name()).expect("CString::new failed");
            let addin_shmem_init_lock: *mut pg_sys::LWLock =
                &mut (*pg_sys::MainLWLockArray.add(21)).lock;
            pg_sys::LWLockAcquire(addin_shmem_init_lock, pg_sys::LWLockMode_LW_EXCLUSIVE);

            let fv_shmem =
                pg_sys::ShmemInitStruct(shm_name.into_raw(), std::mem::size_of::<T>(), &mut found)
                    as *mut T;

            *fv_shmem = std::mem::zeroed();
            let fv = <T>::default();

            std::ptr::copy(&fv, fv_shmem, 1);

            lock.attach(&mut *fv_shmem);
            pg_sys::LWLockRelease(addin_shmem_init_lock);
        });
    }

    /// Test version
    pub fn shmem_init_locked_sized<T: PGXSharedMemory>(
        pgstatic: &'static std::thread::LocalKey<PgLwLock<&'static mut T>>,
        size: usize,
        data: T,
    ) {
        let mut found = false;
        pgstatic.with(|lock| unsafe {
            let shm_name = std::ffi::CString::new(lock.get_name()).expect("CString::new failed");
            let addin_shmem_init_lock: *mut pg_sys::LWLock =
                &mut (*pg_sys::MainLWLockArray.add(21)).lock;
            pg_sys::LWLockAcquire(addin_shmem_init_lock, pg_sys::LWLockMode_LW_EXCLUSIVE);

            let fv_shmem = pg_sys::ShmemInitStruct(
                shm_name.into_raw(),
                std::mem::size_of::<T>() + size,
                &mut found,
            ) as *mut T;

            *fv_shmem = std::mem::zeroed();

            std::ptr::copy(&data, fv_shmem, 1);

            lock.attach(&mut *fv_shmem);
            pg_sys::LWLockRelease(addin_shmem_init_lock);
        });
    }

    /// Must be run from the shared memory init hook, use for atomics
    pub fn shmem_init_atomic<T: atomic_traits::Atomic + Default>(pgstatic: &'static T) {
        unsafe {
            let mut found = false;
            let shm_name =
                std::ffi::CString::new(Uuid::new_v4().to_string()).expect("CString::new failed");
            let addin_shmem_init_lock: *mut pg_sys::LWLock =
                &mut (*pg_sys::MainLWLockArray.add(21)).lock;
            pg_sys::LWLockAcquire(addin_shmem_init_lock, pg_sys::LWLockMode_LW_EXCLUSIVE);

            let fv_shmem =
                pg_sys::ShmemInitStruct(shm_name.into_raw(), std::mem::size_of::<T>(), &mut found)
                    as *mut T;

            std::ptr::copy(pgstatic, fv_shmem, 1);

            pg_sys::LWLockRelease(addin_shmem_init_lock);
        }
    }
}

//unsafe impl PGXSharedMemory for dyn num_traits::Num {}
unsafe impl PGXSharedMemory for bool {}
unsafe impl PGXSharedMemory for char {}
unsafe impl PGXSharedMemory for str {}
unsafe impl PGXSharedMemory for () {}
unsafe impl PGXSharedMemory for i8 {}
unsafe impl PGXSharedMemory for i16 {}
unsafe impl PGXSharedMemory for i32 {}
unsafe impl PGXSharedMemory for i64 {}
unsafe impl PGXSharedMemory for i128 {}
unsafe impl PGXSharedMemory for u8 {}
unsafe impl PGXSharedMemory for u16 {}
unsafe impl PGXSharedMemory for u32 {}
unsafe impl PGXSharedMemory for u64 {}
unsafe impl PGXSharedMemory for u128 {}
unsafe impl PGXSharedMemory for usize {}
unsafe impl PGXSharedMemory for isize {}
unsafe impl PGXSharedMemory for f32 {}
unsafe impl PGXSharedMemory for f64 {}
unsafe impl<T> PGXSharedMemory for [T] where T: PGXSharedMemory + Default {}
unsafe impl<A, B> PGXSharedMemory for (A, B)
where
    A: PGXSharedMemory + Default,
    B: PGXSharedMemory + Default,
{
}
unsafe impl<A, B, C> PGXSharedMemory for (A, B, C)
where
    A: PGXSharedMemory + Default,
    B: PGXSharedMemory + Default,
    C: PGXSharedMemory + Default,
{
}
unsafe impl<A, B, C, D> PGXSharedMemory for (A, B, C, D)
where
    A: PGXSharedMemory + Default,
    B: PGXSharedMemory + Default,
    C: PGXSharedMemory + Default,
    D: PGXSharedMemory + Default,
{
}
unsafe impl<A, B, C, D, E> PGXSharedMemory for (A, B, C, D, E)
where
    A: PGXSharedMemory + Default,
    B: PGXSharedMemory + Default,
    C: PGXSharedMemory + Default,
    D: PGXSharedMemory + Default,
    E: PGXSharedMemory + Default,
{
}
unsafe impl<N: Default + PGXSharedMemory, T: heapless::ArrayLength<N>> PGXSharedMemory
    for heapless::Vec<N, T>
{
}
unsafe impl<
        K: Eq + hash32::Hash,
        V: Default,
        N: heapless::ArrayLength<heapless::Bucket<K, V>>
            + heapless::ArrayLength<Option<heapless::Pos>>,
        S,
    > PGXSharedMemory for heapless::IndexMap<K, V, N, S>
{
}
