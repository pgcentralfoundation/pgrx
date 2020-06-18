// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.


pub mod guard;
mod oids;
mod tupdesc;
mod utils;

pub use guard::*;
pub use oids::*;
pub use tupdesc::*;
pub use utils::*;

#[cfg(target_os = "linux")]
extern "C" {
    #[link_name = "__sigsetjmp"]
    pub fn sigsetjmp(
        env: *mut crate::sigjmp_buf,
        savemask: std::os::raw::c_int,
    ) -> std::os::raw::c_int;
}

#[cfg(target_os = "macos")]
extern "C" {
    pub fn sigsetjmp(
        env: *mut crate::sigjmp_buf,
        savemask: std::os::raw::c_int,
    ) -> std::os::raw::c_int;
}
