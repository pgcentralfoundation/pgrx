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

    use pgrx::pg_sys::panic::CaughtError;
    use pgrx::prelude::*;
    use pgrx::spi;
    use std::panic::UnwindSafe;

    /// `chr(233)` is 'é' in a UTF-8 database. In a single-byte encoding such as LATIN1 or
    /// SQL_ASCII it is the byte 0xE9, which is not UTF-8.
    const ARRAY: &str = "ARRAY['caf' || chr(233)]";

    /// Each read panics in Rust after its query has finished, so catching the panic leaves no
    /// PostgreSQL resources behind.
    fn panic_message(read: impl FnOnce() + UnwindSafe) -> Option<String> {
        PgTryBuilder::new(|| {
            read();
            None
        })
        .catch_rust_panic(|error| match error {
            CaughtError::RustPanic { ereport, .. } => Some(ereport.message().to_owned()),
            other => other.rethrow(),
        })
        .execute()
    }

    /// Run this in a LATIN1 database too: the test databases are UTF-8 or SQL_ASCII.
    #[pg_test]
    fn test_text_array_elements_must_be_utf8() {
        let utf8 = unsafe { pg_sys::GetDatabaseEncoding() } == pg_sys::pg_enc::PG_UTF8 as i32;
        let messages = [
            panic_message(|| {
                Spi::get_one::<Vec<String>>(&format!("SELECT {ARRAY}")).unwrap();
            }),
            panic_message(|| {
                let array = Spi::get_one::<Array<String>>(&format!("SELECT {ARRAY}")).unwrap();
                array.unwrap().iter().for_each(drop);
            }),
            panic_message(|| {
                Spi::connect(|client| {
                    let query = format!("SELECT {ARRAY}");
                    let array =
                        client.select(&query, None, &[])?.first().get_one::<Array<&str>>()?;
                    array.unwrap().iter().for_each(drop);
                    Ok::<_, spi::Error>(())
                })
                .unwrap();
            }),
        ];
        for message in messages {
            if utf8 {
                assert_eq!(message, None);
            } else {
                let message = message.expect("a non-UTF-8 element was read as str");
                assert!(
                    message.starts_with("datums converted to &str should be valid UTF-8"),
                    "{message}"
                );
            }
        }
    }
}
