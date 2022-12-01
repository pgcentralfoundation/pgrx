/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use libflate::gzip::{Decoder, Encoder};
use pgx::prelude::*;
use std::io::{Read, Write};

pgx::pg_module_magic!();

/// gzip bytes.  Postgres will automatically convert `text`/`varchar` data into `bytea`
#[pg_extern]
fn gzip(input: &[u8]) -> Vec<u8> {
    let mut encoder = Encoder::new(Vec::new()).expect("failed to construct gzip Encoder");
    encoder.write_all(input).expect("failed to write input to gzip encoder");
    encoder.finish().into_result().unwrap()
}

/// gunzip previously gzipped bytes
#[pg_extern]
fn gunzip(mut bytes: &[u8]) -> Vec<u8> {
    let mut decoder = Decoder::new(&mut bytes).expect("failed to construct gzip Decoder");
    let mut buf = Vec::new();
    decoder.read_to_end(&mut buf).expect("failed to decode gzip data");
    buf
}

/// gunzip previously gzipped bytes as a String
#[pg_extern]
fn gunzip_as_text(bytes: &[u8]) -> String {
    String::from_utf8(gunzip(bytes)).expect("decompressed text is not valid utf8")
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::prelude::*;

    #[pg_test]
    fn test_gzip_text() {
        let result = Spi::get_one::<String>("SELECT gunzip_as_text(gzip('hi there'));").unwrap();
        assert_eq!(result, "hi there");
    }

    #[pg_test]
    fn test_gzip_bytes() {
        let result = Spi::get_one::<&[u8]>("SELECT gunzip(gzip('hi there'::bytea));").unwrap();
        assert_eq!(result, b"hi there");
    }
}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
