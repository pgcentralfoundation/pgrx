/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use pgx::prelude::*;

#[pg_extern]
fn extern_func() -> bool {
    extern_func_impl::<u8>()
}

// This ensures that parameterized function compiles when it has `pg_guard` attached to it
#[pg_guard]
extern "C" fn extern_func_impl<T>() -> bool {
    true
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;
}
