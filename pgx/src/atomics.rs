use crate::pg_sys;
use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::ops::{BitAnd, Deref};

pub struct PgAtomic<T, V>
where
    T: atomic_traits::Atomic<Type = V> + Default,
    V: BitAnd + Copy,
{
    pub(crate) name: UnsafeCell<Option<*const std::os::raw::c_char>>,
    data: UnsafeCell<Option<*mut std::os::raw::c_void>>,
    pub(crate) default: Option<V>,
    __marker: PhantomData<T>,
}

impl<T, V> PgAtomic<T, V>
where
    T: atomic_traits::Atomic<Type = V> + Default,
    V: BitAnd + Copy,
{
    pub fn new(default: V) -> Self {
        PgAtomic {
            name: UnsafeCell::new(None),
            data: UnsafeCell::new(None),
            default: Some(default),
            __marker: PhantomData,
        }
    }

    pub fn from_named(name: &str) -> Self {
        let shm_name = std::ffi::CString::new(name).expect("CString::new() failed");
        let atomic = PgAtomic {
            name: UnsafeCell::new(Some(shm_name.as_ptr())),
            data: UnsafeCell::new(None),
            default: None,
            __marker: PhantomData,
        };
        std::mem::forget(shm_name);

        atomic
    }
}

impl<T, V> Deref for PgAtomic<T, V>
where
    T: atomic_traits::Atomic<Type = V> + Default,
    V: BitAnd + Copy,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            if self.data.get().as_ref().unwrap().is_none() {
                let addin_shmem_init_lock: *mut pg_sys::LWLock =
                    &mut (*pg_sys::MainLWLockArray.add(21)).lock;

                let shm_name = self
                    .name
                    .get()
                    .as_ref()
                    .unwrap()
                    .expect("atomic was not initialized");

                let (shmem_address, found) = {
                    let mut found = false;
                    pg_sys::LWLockAcquire(addin_shmem_init_lock, pg_sys::LWLockMode_LW_EXCLUSIVE);
                    let shmem_address =
                        pg_sys::ShmemInitStruct(shm_name, std::mem::size_of::<T>(), &mut found)
                            as *mut T;
                    pg_sys::LWLockRelease(addin_shmem_init_lock);
                    (shmem_address, found)
                };

                if !found {
                    panic!("Unable to locate atomic in shared memory");
                }

                *self.data.get() = Some(shmem_address as *mut std::os::raw::c_void);
            }

            let atomic_ptr = self.data.get().as_ref().unwrap();
            (atomic_ptr.unwrap() as *const T).as_ref().unwrap()
        }
    }
}
