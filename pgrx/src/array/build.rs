use super::*;
use crate::memcx::{MemCx, PBox};

#[derive(Debug)]
pub struct ArrayBuilder {
    byte_len: Option<u32>,
}

impl ArrayBuilder {
    pub fn new() -> ArrayBuilder {
        ArrayBuilder { byte_len: None }
    }

    pub fn build_in<'mcx, T>(memcx: &MemCx<'mcx>) -> PBox<'mcx, FlatArray<'mcx, T>> {
        let size = todo!();
        let ptr = memcx.alloc_bytes(size);
        unsafe { PBox::from_raw_in(ptr, memcx) }
    }
}
