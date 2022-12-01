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

    use pgx::*;

    #[pg_test]
    fn test_string_info_read_full() {
        let mut string_info = StringInfo::from(vec![1, 2, 3, 4, 5]);
        assert_eq!(string_info.read(..), Some(&[1, 2, 3, 4, 5][..]));
        assert_eq!(string_info.read(..), Some(&[][..]));
        assert_eq!(string_info.read(..=1), None);
    }

    #[pg_test]
    fn test_string_info_read_offset() {
        let mut string_info = StringInfo::from(vec![1, 2, 3, 4, 5]);
        assert_eq!(string_info.read(1..), Some(&[2, 3, 4, 5][..]));
        assert_eq!(string_info.read(..), Some(&[][..]));
    }

    #[pg_test]
    fn test_string_info_read_cap() {
        let mut string_info = StringInfo::from(vec![1, 2, 3, 4, 5]);
        assert_eq!(string_info.read(..=1), Some(&[1][..]));
        assert_eq!(string_info.read(1..=2), Some(&[3][..]));
        assert_eq!(string_info.read(..), Some(&[4, 5][..]));
    }
}
