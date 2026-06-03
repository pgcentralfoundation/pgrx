//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgrx_unit_tests;

    use pgrx::prelude::*;

    #[pg_extern]
    fn return_bytes() -> &'static [u8] {
        b"bytes"
    }

    #[pg_test]
    fn test_return_bytes() {
        let bytes = Spi::get_one::<&[u8]>("SELECT tests.return_bytes();");
        assert_eq!(bytes, Ok(Some(b"bytes".as_slice())));
    }

    #[pg_extern]
    fn return_bytes_slice(bytes: &[u8]) -> &[u8] {
        &bytes[1..=3]
    }

    #[pg_test]
    fn test_return_bytes_slice() {
        let slice = Spi::get_one::<&[u8]>("SELECT tests.return_bytes_slice('abcdefg'::bytea);");
        assert_eq!(slice, Ok(Some(b"bcd".as_slice())));
    }

    #[pg_extern]
    fn return_vec_bytes() -> Vec<u8> {
        b"bytes".to_vec()
    }

    #[pg_test]
    fn test_return_vec_bytes() {
        let vec = Spi::get_one::<Vec<u8>>("SELECT tests.return_vec_bytes();");
        assert_eq!(vec, Ok(Some(vec![b'b', b'y', b't', b'e', b's'])));
    }

    #[pg_extern]
    fn return_vec_subvec(bytes: Vec<u8>) -> Vec<u8> {
        bytes[1..=3].to_vec()
    }

    #[pg_test]
    fn test_return_vec_subvec() {
        let vec = Spi::get_one::<Vec<u8>>("SELECT tests.return_vec_subvec('abcdefg'::bytea);");
        assert_eq!(vec, Ok(Some(vec![b'b', b'c', b'd'])));
    }

    #[pg_extern]
    fn bytea_arg_length(data: Bytea<'_>) -> i32 {
        data.len() as i32
    }

    #[pg_test]
    fn test_bytea_arg_length() {
        let result = Spi::get_one::<i32>("SELECT tests.bytea_arg_length('abcdef'::bytea);");
        assert_eq!(result, Ok(Some(6)));
    }

    #[pg_extern]
    fn bytea_roundtrip<'fcx>(input: Bytea<'fcx>) -> Bytea<'fcx> {
        input
    }

    #[pg_test]
    fn test_bytea_roundtrip() {
        let result = Spi::get_one::<&[u8]>("SELECT tests.bytea_roundtrip('roundtrip'::bytea);");
        assert_eq!(result, Ok(Some(b"roundtrip".as_slice())));
    }

    #[pg_extern]
    fn bytea_is_empty(data: Bytea<'_>) -> bool {
        data.is_empty()
    }

    #[pg_test]
    fn test_bytea_is_empty() {
        let empty = Spi::get_one::<bool>("SELECT tests.bytea_is_empty(''::bytea);");
        assert_eq!(empty, Ok(Some(true)));
        let non_empty = Spi::get_one::<bool>("SELECT tests.bytea_is_empty('x'::bytea);");
        assert_eq!(non_empty, Ok(Some(false)));
    }
}
