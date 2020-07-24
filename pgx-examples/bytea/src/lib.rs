use libflate::gzip::{Decoder, Encoder};
use pgx::*;
use std::io::{Read, Write};

pg_module_magic!();

#[pg_extern]
fn gzip_text(input: &str) -> Vec<u8> {
    let mut encoder = Encoder::new(Vec::new()).expect("failed to construct gzip Encoder");
    encoder
        .write_all(input.as_bytes())
        .expect("failed to write input to gzip encoder");
    encoder.finish().into_result().unwrap()
}

#[pg_extern]
fn gzip_bytes(input: &[u8]) -> Vec<u8> {
    let mut encoder = Encoder::new(Vec::new()).expect("failed to construct gzip Encoder");
    encoder
        .write_all(input)
        .expect("failed to write input to gzip encoder");
    encoder.finish().into_result().unwrap()
}

#[pg_extern]
fn gunzip_bytes(mut bytes: &[u8]) -> Vec<u8> {
    let mut decoder = Decoder::new(&mut bytes).expect("failed to construct gzip Decoder");
    let mut buf = Vec::new();
    decoder
        .read_to_end(&mut buf)
        .expect("failed to decode gzip data");
    buf
}

#[pg_extern]
fn gunzip_text(mut bytes: &[u8]) -> String {
    let mut decoder = Decoder::new(&mut bytes).expect("failed to construct gzip Decoder");
    let mut buf = Vec::new();
    decoder
        .read_to_end(&mut buf)
        .expect("failed to decode gzip data");
    String::from_utf8(buf).expect("decoded text is not valid utf8")
}

#[cfg(any(test, feature = "pg_test"))]
mod tests {
    use pgx::*;

    #[pg_test]
    fn test_gzip_text() {
        let result = Spi::get_one::<String>("SELECT gunzip_text(gzip_text('hi there'));")
            .expect("SPI result was null");
        assert_eq!(result, "hi there");
    }

    #[pg_test]
    fn test_gzip_bytes() {
        let result = Spi::get_one::<&[u8]>("SELECT gunzip_bytes(gzip_bytes('hi there'::bytea));")
            .expect("SPI result was null");
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
