use crate::{pg_sys, void_mut_ptr};
use serde::export::PhantomData;

pub struct PgList<T> {
    list: *mut pg_sys::List,
    allocated_by_pg: bool,
    _marker: PhantomData<T>,
}

impl<T> PgList<T> {
    pub fn new() -> Self {
        PgList {
            list: 0 as *mut pg_sys::List, // an empty List is NIL
            allocated_by_pg: false,
            _marker: PhantomData,
        }
    }

    pub fn from_pg(list: *mut pg_sys::List) -> Self {
        PgList {
            list,
            allocated_by_pg: true,
            _marker: PhantomData,
        }
    }

    pub fn as_ptr(&self) -> *mut pg_sys::List {
        self.list
    }

    pub fn into_pg(mut self) -> *mut pg_sys::List {
        self.allocated_by_pg = true;
        self.list
    }

    #[inline]
    pub fn len(&self) -> usize {
        if self.list.is_null() {
            0
        } else {
            unsafe { self.list.as_ref() }.unwrap().length as usize
        }
    }

    #[inline]
    pub fn head(&self) -> Option<*mut T> {
        if self.list.is_null() {
            None
        } else {
            Some(unsafe {
                self.list
                    .as_ref()
                    .unwrap()
                    .head
                    .as_ref()
                    .unwrap()
                    .data
                    .ptr_value
            } as *mut T)
        }
    }

    #[inline]
    pub fn tail(&self) -> Option<*mut T> {
        if self.list.is_null() {
            None
        } else {
            Some(unsafe {
                self.list
                    .as_ref()
                    .unwrap()
                    .tail
                    .as_ref()
                    .unwrap()
                    .data
                    .ptr_value
            } as *mut T)
        }
    }

    #[inline]
    pub fn get(&self, i: usize) -> Option<*mut T> {
        if self.list.is_null() || i >= self.len() {
            None
        } else {
            Some(unsafe { pg_sys::list_nth(self.list, i as i32) } as *mut T)
        }
    }

    #[inline]
    pub fn push(&mut self, ptr: *mut T) {
        self.list = unsafe { pg_sys::lappend(self.list, ptr as void_mut_ptr) };
    }

    #[inline]
    pub fn pop(&mut self) -> Option<*mut T> {
        let tail = self.tail();

        if tail.is_some() {
            self.list = unsafe { pg_sys::list_truncate(self.list, (self.len() - 1) as i32) };
        }

        tail
    }
}

impl<T> Drop for PgList<T> {
    fn drop(&mut self) {
        if !self.allocated_by_pg && !self.list.is_null() {
            unsafe {
                pg_sys::list_free(self.list);
            }
        }
    }
}
