// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.


#[cfg(any(test, feature = "pg_test"))]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;

    use pgx::*;

    #[pg_extern]
    fn return_bytes() -> &'static [u8] {
        b"bytes"
    }

    #[pg_test]
    fn test_return_bytes() {
        let bytes =
            Spi::get_one::<&[u8]>("SELECT tests.return_bytes();").expect("SPI result was null");
        assert_eq!(bytes, b"bytes")
    }

    #[pg_extern]
    fn return_bytes_slice(bytes: &[u8]) -> &[u8] {
        &bytes[1..=3]
    }

    #[pg_test]
    fn test_return_bytes_slice() {
        let slice = Spi::get_one::<&[u8]>("SELECT tests.return_bytes_slice('abcdefg'::bytea);")
            .expect("SPI result was null");
        assert_eq!(slice, b"bcd")
    }

    #[pg_extern]
    fn return_vec_bytes() -> Vec<u8> {
        b"bytes".into_iter().cloned().collect()
    }

    #[pg_test]
    fn test_return_vec_bytes() {
        let vec = Spi::get_one::<Vec<u8>>("SELECT tests.return_vec_bytes();")
            .expect("SPI result was null");
        assert_eq!(vec.as_slice(), b"bytes")
    }

    #[pg_extern]
    fn return_vec_subvec(bytes: Vec<u8>) -> Vec<u8> {
        (&bytes[1..=3]).into_iter().cloned().collect()
    }

    #[pg_test]
    fn test_return_vec_subvec() {
        let vec = Spi::get_one::<Vec<u8>>("SELECT tests.return_vec_subvec('abcdefg'::bytea);")
            .expect("SPI result was null");
        assert_eq!(vec.as_slice(), b"bcd")
    }
}
