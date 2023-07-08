use std::hash::{Hash, Hasher};

use crate::{direct_function_call, pg_sys, AnyNumeric, Numeric};

impl Hash for AnyNumeric {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            let hash = direct_function_call(pg_sys::hash_numeric, &[self.as_datum()]).unwrap();
            state.write_i32(hash)
        }
    }
}

impl<const P: u32, const S: u32> Hash for Numeric<P, S> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_anynumeric().hash(state)
    }
}
