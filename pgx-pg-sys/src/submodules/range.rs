use std::hash::{Hash, Hasher};

impl PartialEq for crate::RangeBound {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
            && self.infinite == other.infinite
            && self.inclusive == other.inclusive
            && self.lower == other.lower
    }
}

impl Eq for crate::RangeBound {}

impl Hash for crate::RangeBound {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.val.value());
        state.write_u8(self.infinite as u8);
        state.write_u8(self.inclusive as u8);
        state.write_u8(self.lower as u8);
    }
}
