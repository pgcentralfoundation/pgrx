//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
//! A safe wrapper around Postgres' internal [`List`][crate::pg_sys::List] structure.
//!
//! It functions similarly to a Rust [`Vec`][std::vec::Vec], including iterator support, but provides separate
//! understandings of [`List`][crate::pg_sys::List]s of [`Oid`][crate::pg_sys::Oid]s, Integers, and Pointers.

use crate::{is_a, pg_sys, void_mut_ptr};
use core::ffi;
use core::ops::{Index, IndexMut};
use core::ptr::NonNull;
use std::marker::PhantomData;

/// The List type from Postgres, lifted into Rust
/// Note: you may want the ListHead type
pub enum List<T> {
    Nil,
    Cons(ListHead<T>),
}

pub enum ListErr {
    Nil,
    WrongType,
    WrongNodeKind,
}

pub struct ListHead<T> {
    list: NonNull<pg_sys::List>,
    _type: PhantomData<[T]>,
}

mod seal {
    pub trait Sealed {}
}

pub unsafe trait Listable: seal::Sealed + Sized {
    fn matching_tag(tag: pg_sys::NodeTag) -> bool;
}

impl seal::Sealed for *mut ffi::c_void {}
unsafe impl Listable for *mut ffi::c_void {
    fn matching_tag(tag: pg_sys::NodeTag) -> bool {
        matches!(tag, pg_sys::NodeTag::T_List)
    }
}

impl seal::Sealed for ffi::c_int {}
unsafe impl Listable for ffi::c_int {
    fn matching_tag(tag: pg_sys::NodeTag) -> bool {
        matches!(tag, pg_sys::NodeTag::T_IntList)
    }
}

impl seal::Sealed for pg_sys::Oid {}
unsafe impl Listable for pg_sys::Oid {
    fn matching_tag(tag: pg_sys::NodeTag) -> bool {
        matches!(tag, pg_sys::NodeTag::T_OidList)
    }
}

#[cfg(feature = "pg16")]
impl seal::Sealed for pg_sys::TransactionId {}
#[cfg(feature = "pg16")]
unsafe impl Listable for pg_sys::TransactionId {
    fn matching_tag(tag: pg_sys::NodeTag) -> bool {
        matches!(tag, pg_sys::NodeTag::T_XidList)
    }
}

/// Note the absence of `impl Default for ListHead`:
/// it must initialize at least 1 element to be created at all
impl<T> Default for List<T> {
    fn default() -> List<T> {
        List::Nil
    }
}

impl<T: Listable> List<T> {
    /// Attempt to obtain a `List<T>` from a `*mut pg_sys::List`
    ///
    /// This may be somewhat confusing:
    /// A valid List of any type is the null pointer, as in the Lisp `(car, cdr)` representation.
    /// This remains true even after significant reworks of the List type in Postgres 13, which
    /// cause it to internally use a "flat array" representation.
    ///
    /// Thus, this returns `Some` even if the List is NULL, because it is `Some(List::Nil)`,
    /// and returns `None` only if the List is non-NULL but has an invalid tag!
    ///
    /// # Safety
    /// This assumes the pointer is either NULL or the NodeTag is valid to read
    pub unsafe fn from_ptr(ptr: *mut pg_sys::List) -> Option<List<T>> {
        match NonNull::new(ptr) {
            None => Some(List::Nil),
            Some(list) => T::matching_tag((*ptr).type_)
                .then_some(List::Cons(ListHead { list, _type: PhantomData })),
        }
    }
}

impl<T> List<T> {
    pub fn len(&self) -> usize {
        match self {
            List::Nil => 0,
            List::Cons(head) => head.len(),
        }
    }
}

impl<T> ListHead<T> {
    pub fn len(&self) -> usize {
        unsafe { self.list.as_ref().length as usize }
    }

    pub fn capacity(&self) -> usize {
        unsafe { self.list.as_ref().max_length as usize }
    }
}
// pub unsafe fn downcast_nullable(list: *mut pg_sys::List) -> Result<ListHead<T>, ListErr> {

// }
impl<T: Listable> ListHead<T> {
    pub unsafe fn downcast_ptr(list: NonNull<pg_sys::List>) -> Option<ListHead<T>> {
        T::matching_tag((*list.as_ptr()).type_).then_some(ListHead { list, _type: PhantomData })
    }
}

impl<T> Index<usize> for ListHead<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        todo!()
    }
}

impl<T> IndexMut<usize> for ListHead<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        todo!()
    }
}

impl<T> Index<ffi::c_int> for ListHead<T> {
    type Output = T;

    fn index(&self, index: ffi::c_int) -> &Self::Output {
        todo!()
    }
}

impl<T> IndexMut<ffi::c_int> for ListHead<T> {
    fn index_mut(&mut self, index: ffi::c_int) -> &mut Self::Output {
        todo!()
    }
}

pub struct PgList<T> {
    list: *mut pg_sys::List,
    allocated_by_pg: bool,
    _marker: PhantomData<T>,
}
impl<T> Default for PgList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> PgList<T> {
    pub fn new() -> Self {
        PgList {
            list: std::ptr::null_mut(), // an empty List is NIL
            allocated_by_pg: false,
            _marker: PhantomData,
        }
    }

    pub unsafe fn from_pg(list: *mut pg_sys::List) -> Self {
        PgList { list, allocated_by_pg: true, _marker: PhantomData }
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
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn head(&self) -> Option<*mut T> {
        if self.list.is_null() {
            None
        } else {
            Some(unsafe { pg_sys::pgrx_list_nth(self.list, 0) } as *mut T)
        }
    }

    #[inline]
    pub fn tail(&self) -> Option<*mut T> {
        if self.list.is_null() {
            None
        } else {
            Some(unsafe { pg_sys::pgrx_list_nth(self.list, (self.len() - 1) as i32) } as *mut T)
        }
    }

    #[inline]
    pub fn get_ptr(&self, i: usize) -> Option<*mut T> {
        if !self.is_empty()
            && unsafe { !is_a(self.list as *mut pg_sys::Node, pg_sys::NodeTag::T_List) }
        {
            panic!("PgList does not contain pointers")
        }
        if self.list.is_null() || i >= self.len() {
            None
        } else {
            Some(unsafe { pg_sys::pgrx_list_nth(self.list, i as i32) } as *mut T)
        }
    }

    #[inline]
    pub fn get_int(&self, i: usize) -> Option<i32> {
        if !self.is_empty()
            && unsafe { !is_a(self.list as *mut pg_sys::Node, pg_sys::NodeTag::T_IntList) }
        {
            panic!("PgList does not contain ints")
        }

        if self.list.is_null() || i >= self.len() {
            None
        } else {
            Some(unsafe { pg_sys::pgrx_list_nth_int(self.list, i as i32) })
        }
    }

    #[inline]
    pub fn get_oid(&self, i: usize) -> Option<pg_sys::Oid> {
        if !self.is_empty()
            && unsafe { !is_a(self.list as *mut pg_sys::Node, pg_sys::NodeTag::T_OidList) }
        {
            panic!("PgList does not contain oids")
        }

        if self.list.is_null() || i >= self.len() {
            None
        } else {
            Some(unsafe { pg_sys::pgrx_list_nth_oid(self.list, i as i32) })
        }
    }

    #[cfg(any(feature = "pg11", feature = "pg12"))]
    #[inline]
    pub unsafe fn replace_ptr(&mut self, i: usize, with: *mut T) {
        let cell = pg_sys::pgrx_list_nth_cell(self.list, i as i32);
        cell.as_mut().expect("cell is null").data.ptr_value = with as void_mut_ptr;
    }

    #[cfg(any(feature = "pg13", feature = "pg14", feature = "pg15", feature = "pg16"))]
    #[inline]
    pub unsafe fn replace_ptr(&mut self, i: usize, with: *mut T) {
        let cell = pg_sys::pgrx_list_nth_cell(self.list, i as i32);
        cell.as_mut().expect("cell is null").ptr_value = with as void_mut_ptr;
    }

    #[cfg(any(feature = "pg11", feature = "pg12"))]
    #[inline]
    pub fn replace_int(&mut self, i: usize, with: i32) {
        unsafe {
            let cell = pg_sys::pgrx_list_nth_cell(self.list, i as i32);
            cell.as_mut().expect("cell is null").data.int_value = with;
        }
    }

    #[cfg(any(feature = "pg13", feature = "pg14", feature = "pg15", feature = "pg16"))]
    #[inline]
    pub fn replace_int(&mut self, i: usize, with: i32) {
        unsafe {
            let cell = pg_sys::pgrx_list_nth_cell(self.list, i as i32);
            cell.as_mut().expect("cell is null").int_value = with;
        }
    }

    #[cfg(any(feature = "pg11", feature = "pg12"))]
    #[inline]
    pub fn replace_oid(&mut self, i: usize, with: pg_sys::Oid) {
        unsafe {
            let cell = pg_sys::pgrx_list_nth_cell(self.list, i as i32);
            cell.as_mut().expect("cell is null").data.oid_value = with;
        }
    }

    #[cfg(any(feature = "pg13", feature = "pg14", feature = "pg15", feature = "pg16"))]
    #[inline]
    pub fn replace_oid(&mut self, i: usize, with: pg_sys::Oid) {
        unsafe {
            let cell = pg_sys::pgrx_list_nth_cell(self.list, i as i32);
            cell.as_mut().expect("cell is null").oid_value = with;
        }
    }

    #[inline]
    pub fn iter_ptr(&self) -> impl Iterator<Item = *mut T> + '_ {
        PgListIteratorPtr { list: &self, pos: 0 }
    }

    #[inline]
    pub fn iter_oid(&self) -> impl Iterator<Item = pg_sys::Oid> + '_ {
        PgListIteratorOid { list: &self, pos: 0 }
    }

    #[inline]
    pub fn iter_int(&self) -> impl Iterator<Item = i32> + '_ {
        PgListIteratorInt { list: &self, pos: 0 }
    }

    /// Add a pointer value to the end of this list
    ///
    /// ## Safety
    ///
    /// We cannot guarantee the specified pointer is valid, but we assume it is as we only store it,
    /// we don't dereference it
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

struct PgListIteratorPtr<'a, T> {
    list: &'a PgList<T>,
    pos: usize,
}

struct PgListIteratorOid<'a, T> {
    list: &'a PgList<T>,
    pos: usize,
}

struct PgListIteratorInt<'a, T> {
    list: &'a PgList<T>,
    pos: usize,
}

impl<'a, T> Iterator for PgListIteratorPtr<'a, T> {
    type Item = *mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.list.get_ptr(self.pos);
        self.pos += 1;
        result
    }
}

impl<'a, T> Iterator for PgListIteratorOid<'a, T> {
    type Item = pg_sys::Oid;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.list.get_oid(self.pos);
        self.pos += 1;
        result
    }
}

impl<'a, T> Iterator for PgListIteratorInt<'a, T> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.list.get_int(self.pos);
        self.pos += 1;
        result
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
