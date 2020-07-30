// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.


#[cfg(any(test, feature = "pg_test"))]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;

    use pgx::*;

    #[pg_test]
    fn test_bool_guc() {
        static GUC: GucSetting<bool> = GucSetting::new(true);
        GucRegistry::define_bool_guc(
            "test.bool",
            "test bool gucs",
            "test bool gucs",
            &GUC,
            GucContext::Userset,
        );
        assert_eq!(GUC.get(), true);

        Spi::run("SET test.bool TO false;");
        assert_eq!(GUC.get(), false);

        Spi::run("SET test.bool TO true;");
        assert_eq!(GUC.get(), true);
    }

    #[pg_test]
    fn test_int_guc() {
        static GUC: GucSetting<i32> = GucSetting::new(42);
        GucRegistry::define_int_guc(
            "test.int",
            "test int guc",
            "test int guc",
            &GUC,
            -1,
            42,
            GucContext::Userset,
        );
        assert_eq!(GUC.get(), 42);

        Spi::run("SET test.int = -1");
        assert_eq!(GUC.get(), -1);

        Spi::run("SET test.int = 12");
        assert_eq!(GUC.get(), 12);
    }

    #[pg_test]
    fn test_float_guc() {
        static GUC: GucSetting<f64> = GucSetting::new(42.42);
        GucRegistry::define_float_guc(
            "test.float",
            "test float guc",
            "test float guc",
            &GUC,
            -1.0f64,
            43.0f64,
            GucContext::Userset,
        );
        assert_eq!(GUC.get(), 42.42);

        Spi::run("SET test.float = -1");
        assert_eq!(GUC.get(), -1.0);

        Spi::run("SET test.float = 12");
        assert_eq!(GUC.get(), 12.0);

        Spi::run("SET test.float = 3.333");
        assert_eq!(GUC.get(), 3.333);
    }

    #[pg_test]
    fn test_string_guc() {
        static GUC: GucSetting<Option<&'static str>> = GucSetting::new(Some("this is a test"));
        GucRegistry::define_string_guc(
            "test.string",
            "test string guc",
            "test string guc",
            &GUC,
            GucContext::Userset,
        );
        assert!(GUC.get().is_some());
        assert_eq!(GUC.get().unwrap(), "this is a test");

        Spi::run("SET test.string = 'foo'");
        assert_eq!(GUC.get().unwrap(), "foo");

        Spi::run("SET test.string = DEFAULT");
        assert_eq!(GUC.get().unwrap(), "this is a test");
    }

    #[pg_test]
    fn test_string_guc_null_default() {
        static GUC: GucSetting<Option<&'static str>> = GucSetting::new(None);
        GucRegistry::define_string_guc(
            "test.string",
            "test string guc",
            "test string guc",
            &GUC,
            GucContext::Userset,
        );
        assert!(GUC.get().is_none());

        Spi::run("SET test.string = 'foo'");
        assert_eq!(GUC.get().unwrap(), "foo");

        Spi::run("SET test.string = DEFAULT");
        assert!(GUC.get().is_none());
    }

    #[pg_test]
    fn test_enum_guc() {
        #[derive(PostgresGucEnum, Clone, Copy, PartialEq, Debug)]
        enum TestEnum {
            One,
            Two,
            Three,
        }
        static GUC: GucSetting<TestEnum> = GucSetting::new(TestEnum::Two);
        GucRegistry::define_enum_guc(
            "test.enum",
            "test enum guc",
            "test enum guc",
            &GUC,
            GucContext::Userset,
        );
        assert_eq!(GUC.get(), TestEnum::Two);

        Spi::run("SET test.enum = 'One'");
        assert_eq!(GUC.get(), TestEnum::One);

        Spi::run("SET test.enum = 'three'");
        assert_eq!(GUC.get(), TestEnum::Three);
    }
}
