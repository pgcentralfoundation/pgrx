/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

#[cfg(any(test, feature = "pg_test"))]
#[pgx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;

    use pgx::prelude::*;

    #[pg_extern]
    fn return_bytes() -> &'static [u8] {
        b"bytes"
    }

    #[pg_test]
    fn test_return_bytes() {
        let bytes = Spi::get_one::<&[u8]>("SELECT tests.return_bytes();").unwrap();
        assert_eq!(bytes, b"bytes")
    }

    #[pg_extern]
    fn return_bytes_slice(bytes: &[u8]) -> &[u8] {
        &bytes[1..=3]
    }

    #[pg_test]
    fn test_return_bytes_slice() {
        let slice =
            Spi::get_one::<&[u8]>("SELECT tests.return_bytes_slice('abcdefg'::bytea);").unwrap();
        assert_eq!(slice, b"bcd")
    }

    #[pg_extern]
    fn return_vec_bytes() -> Vec<u8> {
        b"bytes".into_iter().cloned().collect()
    }

    #[pg_test]
    fn test_return_vec_bytes() {
        let vec = Spi::get_one::<Vec<u8>>("SELECT tests.return_vec_bytes();").unwrap();
        assert_eq!(vec.as_slice(), b"bytes")
    }

    #[pg_extern]
    fn return_vec_subvec(bytes: Vec<u8>) -> Vec<u8> {
        (&bytes[1..=3]).into_iter().cloned().collect()
    }

    #[pg_test]
    fn test_return_vec_subvec() {
        let vec =
            Spi::get_one::<Vec<u8>>("SELECT tests.return_vec_subvec('abcdefg'::bytea);").unwrap();
        assert_eq!(vec.as_slice(), b"bcd")
    }
}
