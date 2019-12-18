//
// we allow improper_ctypes just to eliminate these warnings:
//      = note: `#[warn(improper_ctypes)]` on by default
//      = note: 128-bit integers don't currently have a known stable ABI

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(non_upper_case_globals)]
#[allow(improper_ctypes)]
#[cfg(feature = "pg10")]
pub mod pg10;

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(non_upper_case_globals)]
#[allow(improper_ctypes)]
#[cfg(feature = "pg11")]
pub mod pg11;

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(non_upper_case_globals)]
#[allow(improper_ctypes)]
#[cfg(feature = "pg12")]
pub mod pg12;

#[cfg(feature = "pg10")]
pub use pg10::*;

#[cfg(feature = "pg11")]
pub use pg11::*;

#[cfg(feature = "pg12")]
pub use pg12::*;
