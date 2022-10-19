// Constants defined in PG13+
mod typalign {
    pub const TYPALIGN_CHAR: u8 = b'c';
    pub const TYPALIGN_SHORT: u8 = b's';
    pub const TYPALIGN_INT: u8 = b'i';
    pub const TYPALIGN_DOUBLE: u8 = b'd';
}

#[cfg(any(feature = "pg10", feature = "pg11", feature = "pg12"))]
pub use typalign::*;
