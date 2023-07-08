/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use pgrx::prelude::*;
use pgrx::{direct_function_call, AnyArray, IntoDatum, Json};

#[pg_extern]
fn anyarray_arg(array: AnyArray) -> Json {
    unsafe { direct_function_call::<Json>(pg_sys::array_to_json, &[array.into_datum()]) }
        .expect("conversion to json returned null")
}

#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgrx_tests;

    use pgrx::prelude::*;
    use pgrx::Json;
    use serde_json::*;

    #[pg_test]
    fn test_anyarray_arg() -> std::result::Result<(), pgrx::spi::Error> {
        let json = Spi::get_one::<Json>("SELECT anyarray_arg(ARRAY[1::integer,2,3]::integer[]);")?
            .expect("datum was null");
        assert_eq!(json.0, json! {[1,2,3]});
        Ok(())
    }
}
