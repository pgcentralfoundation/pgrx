use super::*;
use crate::memcx::{MemCx, PBox};
use core::{mem, ptr};

#[derive(Debug)]
pub struct ArrayBuilder {
    byte_len: Option<u32>,
}

impl ArrayBuilder {
    pub fn new() -> ArrayBuilder {
        ArrayBuilder { byte_len: None }
    }

    pub fn build_in<'mcx, T>(self, memcx: &MemCx<'mcx>) -> PBox<'mcx, FlatArray<'mcx, T>> {
        let base_size = mem::size_of::<pg_sys::ArrayType>();
        let size = todo!();
        let ptr = memcx.alloc_bytes(size);
        let ptr = ptr::slice_from_raw_parts_mut(ptr, size);

        // SAFETY: eh, what's a little unsoundness between friends?
        unsafe { PBox::from_raw_in(mem::transmute(ptr), memcx) }
    }
}
