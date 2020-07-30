// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.


use pgx::*;

#[pg_extern]
fn anyarray_arg(array: AnyArray) -> Json {
    unsafe { direct_function_call::<Json>(pg_sys::array_to_json, vec![array.into_datum()]) }
        .expect("conversion to json returned null")
}

#[cfg(any(test, feature = "pg_test"))]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;

    use pgx::*;
    use serde_json::*;

    #[pg_test]
    fn test_anyarray_arg() {
        let json = Spi::get_one::<Json>("SELECT anyarray_arg(ARRAY[1::integer,2,3]::integer[]);")
            .expect("anyarray_arg() returned null");
        assert_eq!(json.0, json! {[1,2,3]})
    }
}
