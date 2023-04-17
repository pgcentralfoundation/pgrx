/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
use once_cell::sync::OnceCell;

pub struct PgAtomic<T> {
    inner: OnceCell<*mut T>,
}

impl<T> PgAtomic<T> {
    pub const fn new() -> Self {
        Self { inner: OnceCell::new() }
    }
}

impl<T> PgAtomic<T>
where
    T: atomic_traits::Atomic + Default,
{
    pub fn attach(&self, value: *mut T) {
        self.inner.set(value).expect("This PgAtomic is not empty, can't re-attach");
    }

    pub fn get(&self) -> &T {
        unsafe {
            self.inner.get().expect("This PgAtomic as not been initialized").as_ref().unwrap()
        }
    }
}

unsafe impl<T> Send for PgAtomic<T> where T: atomic_traits::Atomic + Default {}
unsafe impl<T> Sync for PgAtomic<T> where T: atomic_traits::Atomic + Default {}
