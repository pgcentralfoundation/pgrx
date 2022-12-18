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
    use pgx::{Json, JsonB};

    #[pg_test]
    fn test_json() {
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize)]
        struct User {
            username: String,
            first_name: String,
            last_name: String,
        }

        let json = Spi::get_one::<Json>(
            r#"  SELECT '{"username": "blahblahblah", "first_name": "Blah", "last_name": "McBlahFace"}'::json;  "#,
        ).unwrap();

        let user: User =
            serde_json::from_value(json.0).expect("failed to parse json response from SPI");
        assert_eq!(user.username, "blahblahblah");
        assert_eq!(user.first_name, "Blah");
        assert_eq!(user.last_name, "McBlahFace");
    }

    #[pg_test]
    fn test_jsonb() {
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize)]
        struct User {
            username: String,
            first_name: String,
            last_name: String,
        }

        let json = Spi::get_one::<JsonB>(
            r#"  SELECT '{"username": "blahblahblah", "first_name": "Blah", "last_name": "McBlahFace"}'::jsonb;  "#,
        ).unwrap();

        let user: User =
            serde_json::from_value(json.0).expect("failed to parse json response from SPI");
        assert_eq!(user.username, "blahblahblah");
        assert_eq!(user.first_name, "Blah");
        assert_eq!(user.last_name, "McBlahFace");
    }
}
