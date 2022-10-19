/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

mod datum;
pub mod guard;
mod oids;
mod polyfill;
pub mod setjmp;
mod tupdesc;
mod utils;
// Various SqlTranslatable mappings for SQL generation
mod sql_translatable;

pub use datum::Datum;
// OnDrop(feature = "pg11"): remove this cfg if all supported versions of Postgres
// now include NullableDatum.
#[cfg(any(feature = "pg12", feature = "pg13", feature = "pg14"))]
pub use datum::NullableDatum;
pub use guard::*;
pub use oids::*;
pub use polyfill::*;
pub use tupdesc::*;
pub use utils::*;

#[cfg(target_os = "linux")]
extern "C" {
    #[link_name = "__sigsetjmp"]
    pub(crate) fn sigsetjmp(
        env: *mut crate::sigjmp_buf,
        savemask: std::os::raw::c_int,
    ) -> std::os::raw::c_int;
}

#[cfg(any(target_os = "macos", target_os = "freebsd", target_os = "openbsd"))]
extern "C" {
    pub(crate) fn sigsetjmp(
        env: *mut crate::sigjmp_buf,
        savemask: std::os::raw::c_int,
    ) -> std::os::raw::c_int;
}
