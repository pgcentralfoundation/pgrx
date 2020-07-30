// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.


#[cfg(any(test, feature = "pg_test"))]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;

    use pgx::*;

    #[pg_test]
    fn test_deserialize_inet() {
        let inet =
            serde_json::from_str::<Inet>("\"192.168.0.1\"").expect("failed to deserialize inet");
        assert_eq!("192.168.0.1", &inet.0)
    }

    #[pg_test]
    fn test_serialize_inet() {
        let json = serde_json::to_string(&Inet("192.168.0.1".to_owned()))
            .expect("failed to serialize inet");
        assert_eq!("\"192.168.0.1\"", &json);
    }

    #[pg_extern]
    fn take_and_return_inet(inet: Inet) -> Inet {
        inet
    }

    #[pg_test]
    fn test_take_and_return_inet() {
        let rc = Spi::get_one::<bool>(
            "SELECT tests.take_and_return_inet('192.168.0.1') = '192.168.0.1'::inet;",
        )
        .expect("failed to get SPI result");
        assert!(rc)
    }
}
