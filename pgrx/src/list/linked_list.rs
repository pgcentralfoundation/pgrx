use super::{seal, Enlist, List, ListCell, ListHead};
use crate::pg_sys;
use core::cmp;
use core::ffi;
use core::marker::PhantomData;
use core::mem;
use core::ops::{Bound, Deref, DerefMut, RangeBounds};
use core::ptr::{self, NonNull};

impl<T: Enlist> Deref for ListCell<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: A brief upgrade of readonly &ListCell<T> to writable *mut pg_sys::ListCell
        // may seem sus, but is fine: Enlist::apoptosis is defined as pure casting/arithmetic.
        // So the pointer begins and ends without write permission, and
        // we essentially just reborrow a ListCell as its inner field type
        unsafe { &*T::apoptosis(&self.cell as *const _ as *mut _) }
    }
}

impl<T: Enlist> DerefMut for ListCell<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: we essentially just reborrow a ListCell as its inner field type which
        // only relies on pgrx::list::{Enlist, List, ListCell} maintaining safety invariants
        unsafe { &mut *T::apoptosis(&mut self.cell) }
    }
}

impl seal::Sealed for *mut ffi::c_void {}
unsafe impl Enlist for *mut ffi::c_void {
    const LIST_TAG: pg_sys::NodeTag = pg_sys::NodeTag::T_List;

    unsafe fn apoptosis(cell: *mut pg_sys::ListCell) -> *mut *mut ffi::c_void {
        unsafe { ptr::addr_of_mut!((*cell).data.ptr_value) }
    }

    fn endocytosis(cell: &mut pg_sys::ListCell, value: Self) {
        cell.data.ptr_value = value;
    }

    fn mitosis(cell: &pg_sys::ListCell) -> (&Self, Option<&ListCell<Self>>) {
        let pg_sys::ListCell { data, next } = cell;
        unsafe { (&data.ptr_value, next.cast::<ListCell<Self>>().as_ref()) }
    }

    fn mitosis_mut(cell: &mut pg_sys::ListCell) -> (&mut Self, Option<&mut ListCell<Self>>) {
        let pg_sys::ListCell { data, next } = cell;
        unsafe { (&mut data.ptr_value, next.cast::<ListCell<Self>>().as_mut()) }
    }
}

impl seal::Sealed for ffi::c_int {}
unsafe impl Enlist for ffi::c_int {
    const LIST_TAG: pg_sys::NodeTag = pg_sys::NodeTag::T_IntList;

    unsafe fn apoptosis(cell: *mut pg_sys::ListCell) -> *mut ffi::c_int {
        unsafe { ptr::addr_of_mut!((*cell).data.int_value) }
    }

    fn endocytosis(cell: &mut pg_sys::ListCell, value: Self) {
        cell.data.int_value = value;
    }

    fn mitosis(cell: &pg_sys::ListCell) -> (&Self, Option<&ListCell<Self>>) {
        let pg_sys::ListCell { data, next } = cell;
        unsafe { (&data.int_value, next.cast::<ListCell<Self>>().as_ref()) }
    }

    fn mitosis_mut(cell: &mut pg_sys::ListCell) -> (&mut Self, Option<&mut ListCell<Self>>) {
        let pg_sys::ListCell { data, next } = cell;
        unsafe { (&mut data.int_value, next.cast::<ListCell<Self>>().as_mut()) }
    }
}

impl seal::Sealed for pg_sys::Oid {}
unsafe impl Enlist for pg_sys::Oid {
    const LIST_TAG: pg_sys::NodeTag = pg_sys::NodeTag::T_OidList;

    unsafe fn apoptosis(cell: *mut pg_sys::ListCell) -> *mut pg_sys::Oid {
        unsafe { ptr::addr_of_mut!((*cell).data.oid_value) }
    }

    fn endocytosis(cell: &mut pg_sys::ListCell, value: Self) {
        cell.data.oid_value = value;
    }

    fn mitosis(cell: &pg_sys::ListCell) -> (&Self, Option<&ListCell<Self>>) {
        let pg_sys::ListCell { data, next } = cell;
        unsafe { (&data.oid_value, next.cast::<ListCell<Self>>().as_ref()) }
    }

    fn mitosis_mut(cell: &mut pg_sys::ListCell) -> (&mut Self, Option<&mut ListCell<Self>>) {
        let pg_sys::ListCell { data, next } = cell;
        unsafe { (&mut data.oid_value, next.cast::<ListCell<Self>>().as_mut()) }
    }
}

impl<T: Enlist> List<T> {
    /// Borrow an item from the slice at the index
    pub fn get(&self, index: usize) -> Option<&T> {
        self.iter().nth(index)
    }

    /// Mutably borrow an item from the slice at the index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.iter_mut().nth(index)
    }

    /// Push, and if allocation is needed, allocate in a given context
    /// "Unstable" because this will probably receive breaking changes every week for a few weeks.
    ///
    /// # Safety
    ///
    /// Use the right context, don't play around.
    pub unsafe fn unstable_push_in_context(
        &mut self,
        value: T,
        context: pg_sys::MemoryContext,
    ) -> &mut ListHead<T> {
        match self {
            List::Nil => {
                let list: *mut pg_sys::List =
                    pg_sys::MemoryContextAlloc(context, mem::size_of::<pg_sys::List>()).cast();
                let node: *mut pg_sys::ListCell =
                    pg_sys::MemoryContextAlloc(context, mem::size_of::<pg_sys::ListCell>()).cast();
                (*node).next = ptr::null_mut();
                *T::apoptosis(node) = value;
                (*list).head = node;
                (*list).tail = node;
                (*list).type_ = T::LIST_TAG;
                (*list).length = 1;
                *self = Self::downcast_ptr(list).unwrap();
                match self {
                    List::Cons(head) => head,
                    _ => unreachable!(),
                }
            }
            List::Cons(head) => head.push(value),
        }
    }

    /// Attempt to push or Err if it would allocate
    ///
    /// This exists primarily to allow working with a list with maybe-zero capacity.
    pub fn try_push(&mut self, value: T) -> Result<&mut ListHead<T>, &mut Self> {
        match self {
            List::Nil => Err(self),
            list if list.capacity() - list.len() == 0 => Err(list),
            List::Cons(head) => Ok(head.push(value)),
        }
    }

    /// Try to reserve space for N more items
    pub fn try_reserve(&mut self, items: usize) -> Result<&mut ListHead<T>, &mut Self> {
        match self {
            List::Nil => Err(self),
            List::Cons(head) => Ok(head.reserve(items)),
        }
    }

    // Iterate over part of the List while removing elements from it
    //
    // Note that if this removes the last item, it deallocates the entire list.
    // This is to maintain the Postgres List invariant that a 0-len list is always Nil.
    pub fn drain<R>(&mut self, range: R) -> Drain<'_, T>
    where
        R: RangeBounds<usize>,
    {
        // SAFETY: The Drain invariants are somewhat easier to maintain for List than Vec,
        // however, they have the complication of the Postgres List invariants
        let len = self.len();
        let drain_start = match range.start_bound() {
            Bound::Unbounded | Bound::Included(0) => 0,
            Bound::Included(first) => *first,
            Bound::Excluded(point) => point + 1,
        };
        let tail_start = match range.end_bound() {
            Bound::Unbounded => cmp::min(ffi::c_int::MAX as _, len),
            Bound::Included(last) => last + 1,
            Bound::Excluded(tail) => *tail,
        };
        let Some(tail_len) = len.checked_sub(tail_start) else {
            panic!("index out of bounds of list!")
        };
        let left = tail_start
            .checked_sub(drain_start)
            .expect("start of range has to be less than or equal to end of range!")
            as u32;
        // Let's issue our asserts before mutating state:
        assert!(drain_start <= len);
        assert!(tail_start <= len);

        // Postgres assumes Lists fit into c_int, check before shrinking
        assert!(tail_start <= ffi::c_int::MAX as _);
        assert!(drain_start + tail_len <= ffi::c_int::MAX as _);

        // If draining all, rip it out of place to contain broken invariants from panics
        let raw = if drain_start == 0 {
            mem::take(self).into_ptr()
        } else {
            // Leave it in place, but we need a pointer:
            match self {
                List::Nil => ptr::null_mut(),
                List::Cons(head) => head.list.as_ptr().cast(),
            }
        };

        // Remember to check that our raw ptr is non-null
        if raw != ptr::null_mut() {
            // Shorten the list to prohibit interaction with List's state after drain_start.
            // Note this breaks List repr invariants in the `drain_start == 0` case, but
            // we only consider returning the list ptr to `&mut self` if Drop is completed
            unsafe {
                (*raw).length = drain_start as _;
                let drain_prefix: *mut ListCell<T> = match drain_start as u32 {
                    0 => ptr::null_mut(),
                    start @ 1.. => {
                        // We're guaranteed we have at least one pointer by Postgres
                        let mut ptr = NonNull::new_unchecked((*raw).head);
                        // so we also start counting at 1, because we want to stop "one early"
                        let mut ct = 1;
                        while let Some(next) = NonNull::new((*ptr.as_ptr()).next) {
                            if ct == start {
                                break;
                            }
                            ct += 1;
                            ptr = next;
                        }
                        assert_eq!(ct, start);
                        ptr.as_ptr().cast()
                    }
                };
                let iter = RawCellIter {
                    ptr: if drain_prefix == ptr::null_mut() {
                        (*raw).head.cast()
                    } else {
                        (*drain_prefix).cell.next.cast()
                    },
                };
                Drain { tail_len: tail_len as _, drain_prefix, left, raw, origin: self, iter }
            }
        } else {
            // If it's not, produce the only valid choice: a 0-len iterator pointing to null
            // One last doublecheck for old paranoia's sake:
            assert!(tail_len == 0 && tail_start == 0 && drain_start == 0);
            Drain {
                tail_len: 0,
                drain_prefix: ptr::null_mut(),
                left: 0,
                raw,
                origin: self,
                iter: Default::default(),
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        match self {
            List::Nil => Iter { next: None },
            List::Cons(root) => Iter { next: unsafe { root.as_cells_ptr().as_ref() } },
        }
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        match self {
            List::Nil => IterMut { next: None },
            List::Cons(root) => IterMut { next: unsafe { root.as_mut_cells_ptr().as_mut() } },
        }
    }
}

impl<T> ListHead<T> {
    /// Nonsensical question in Postgres 11-12, but answers as if len
    #[inline]
    pub fn capacity(&self) -> usize {
        self.len()
    }

    pub fn as_cells_ptr(&self) -> *const ListCell<T> {
        unsafe { (*self.list.as_ptr()).head.cast() }
    }

    pub fn as_mut_cells_ptr(&mut self) -> *mut ListCell<T> {
        unsafe { (*self.list.as_ptr()).head.cast() }
    }
}

impl<T: Enlist> ListHead<T> {
    pub fn push(&mut self, value: T) -> &mut Self {
        unsafe {
            let list = self.list.as_mut();
            let cell = cons_cell(list, value);
            (*list.tail).next = cell;
            list.tail = cell;
            list.length += 1;
        }

        // Return `self` for convenience of `List::try_push`
        self
    }

    /// No-op in Postgres 11 and Postgres 12
    pub fn reserve(&mut self, _size: usize) -> &mut Self {
        self
    }
}

unsafe fn cons_cell<T: Enlist>(list: &mut pg_sys::List, value: T) -> *mut pg_sys::ListCell {
    let alloc_size = mem::size_of::<pg_sys::ListCell>();
    // Let's try to maintain all the node cells in the same context, shall we?
    // Even though Postgres won't...
    let context = pg_sys::GetMemoryChunkContext(list as *mut _ as *mut _);
    if context == ptr::null_mut() {
        panic!("Context free list?");
    };
    let buf: *mut pg_sys::ListCell = pg_sys::MemoryContextAlloc(context, alloc_size).cast();
    if buf == ptr::null_mut() {
        panic!("List allocation failure");
    }
    let cell_ptr = T::apoptosis(buf);
    *cell_ptr = value;
    (*buf).next = ptr::null_mut();
    buf
}

unsafe fn destroy_list(list: *mut pg_sys::List) {
    let mut cell = (*list).head;
    while cell != ptr::null_mut() {
        let next = (*cell).next;
        pg_sys::pfree(cell.cast());
        cell = next;
    }
    pg_sys::pfree(list.cast());
}

// #[derive(Debug)]
pub struct Iter<'a, T> {
    next: Option<&'a ListCell<T>>,
}

impl<'a, T: Enlist> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            let (curr, next) = T::mitosis(&node.cell);
            self.next = next;
            curr
        })
    }
}

// #[derive(Debug)]
pub struct IterMut<'a, T> {
    next: Option<&'a mut ListCell<T>>,
}

impl<'a, T: Enlist> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            let (curr, next) = T::mitosis_mut(&mut node.cell);
            self.next = next;
            curr
        })
    }
}

#[derive(Debug)]
pub struct ListIter<T> {
    list: List<T>,
    iter: RawCellIter<T>,
}

/// A list being drained.
#[derive(Debug)]
pub struct Drain<'a, T> {
    /// pointer to the cell to append tail to
    drain_prefix: *mut ListCell<T>,
    /// Length of tail
    tail_len: u32,
    iter: RawCellIter<T>,
    left: u32,
    origin: &'a mut List<T>,
    raw: *mut pg_sys::List,
}

impl<T> Drop for Drain<'_, T> {
    fn drop(&mut self) {
        if self.raw == ptr::null_mut() {
            return;
        }

        // SAFETY: The raw repr accepts null ptrs, but we just checked it's okay.
        unsafe {
            // Note that this may be 0, unlike elsewhere!
            let len = (*self.raw).length;
            if len == 0 && self.tail_len == 0 {
                // Can't simply leave it be due to Postgres List invariants, else it leaks
                destroy_list(self.raw)
            } else {
                // Need to weld over the drained part and fix the length
                // Collect the first deallocation candidate
                let mut to_dealloc = if self.drain_prefix == ptr::null_mut() {
                    let dealloc = (*self.raw).head;
                    (*self.raw).head = self.iter.ptr.cast();
                    dealloc
                } else {
                    let dealloc = (*self.drain_prefix).cell.next;
                    (*self.drain_prefix).cell.next = self.iter.ptr.cast();
                    dealloc
                };
                // actually deallocate the intervening nodes until we catch up
                while to_dealloc != self.iter.ptr.cast() {
                    let next = (*to_dealloc).next;
                    pg_sys::pfree(to_dealloc.cast());
                    to_dealloc = next;
                }
                (*self.raw).length = len + (self.tail_len as ffi::c_int);

                // Put it back now that all invariants have been repaired
                *self.origin = List::Cons(ListHead {
                    list: NonNull::new_unchecked(self.raw),
                    _type: PhantomData,
                });
            }
        }
    }
}

impl<T: Enlist> Iterator for Drain<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.left.checked_sub(1).and_then(|left| {
            self.left = left;
            self.iter.next()
        })
    }
}

impl<T: Enlist> Iterator for ListIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<T: Enlist> IntoIterator for List<T> {
    type IntoIter = ListIter<T>;
    type Item = T;

    fn into_iter(mut self) -> Self::IntoIter {
        let iter = match &mut self {
            List::Nil => Default::default(),
            List::Cons(head) => RawCellIter { ptr: head.as_mut_cells_ptr() },
        };
        ListIter { list: self, iter }
    }
}

impl<T> Drop for ListIter<T> {
    fn drop(&mut self) {
        if let List::Cons(head) = &mut self.list {
            unsafe { destroy_list(head.list.as_ptr()) }
        }
    }
}

/// Needed because otherwise List hits incredibly irritating lifetime issues.
///
/// This must remain a private type, as casual usage of it is wildly unsound.
///
/// # Safety
/// None. Repent that you made this.
///
/// This atrocity assumes pointers passed in are valid or that ptr >= end.
#[derive(Debug, PartialEq)]
struct RawCellIter<T> {
    ptr: *mut ListCell<T>,
}

impl<T> Default for RawCellIter<T> {
    fn default() -> Self {
        RawCellIter { ptr: ptr::null_mut() }
    }
}

impl<T: Enlist> Iterator for RawCellIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        if self.ptr != ptr::null_mut() {
            let ptr = self.ptr;
            // SAFETY: It's assumed that the pointers are valid on construction
            unsafe {
                self.ptr = (*ptr).cell.next.cast::<ListCell<T>>();
                Some(T::apoptosis(ptr.cast()).read())
            }
        } else {
            None
        }
    }
}
