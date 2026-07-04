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
    use std::ffi::CString;
    use std::ffi::c_char;

    use pgrx::guc::*;
    use pgrx::prelude::*;

    #[pg_test]
    fn test_bool_guc() {
        static GUC: GucSetting<bool> = GucSetting::<bool>::new(true);
        GucRegistry::define_bool_guc(
            c"test.bool",
            c"test bool gucs",
            c"test bool gucs",
            &GUC,
            GucContext::Userset,
            GucFlags::default(),
        );
        assert!(GUC.get());

        Spi::run("SET test.bool TO false;").expect("SPI failed");
        assert!(!GUC.get());

        Spi::run("SET test.bool TO true;").expect("SPI failed");
        assert!(GUC.get());
    }

    #[pg_test]
    fn test_int_guc() {
        static GUC: GucSetting<i32> = GucSetting::<i32>::new(42);
        GucRegistry::define_int_guc(
            c"test.int",
            c"test int guc",
            c"test int guc",
            &GUC,
            -1,
            42,
            GucContext::Userset,
            GucFlags::default(),
        );
        assert_eq!(GUC.get(), 42);

        Spi::run("SET test.int = -1").expect("SPI failed");
        assert_eq!(GUC.get(), -1);

        Spi::run("SET test.int = 12").expect("SPI failed");
        assert_eq!(GUC.get(), 12);
    }

    #[pg_test]
    fn test_mb_guc() {
        static GUC: GucSetting<i32> = GucSetting::<i32>::new(42);
        GucRegistry::define_int_guc(
            c"test.megabytes",
            c"test megabytes guc",
            c"test megabytes guc",
            &GUC,
            -1,
            42000,
            GucContext::Userset,
            GucFlags::UNIT_MB,
        );
        assert_eq!(GUC.get(), 42);

        Spi::run("SET test.megabytes = '1GB'").expect("SPI failed");
        assert_eq!(GUC.get(), 1024);
    }

    #[pg_test]
    fn test_float_guc() {
        static GUC: GucSetting<f64> = GucSetting::<f64>::new(42.42);
        GucRegistry::define_float_guc(
            c"test.float",
            c"test float guc",
            c"test float guc",
            &GUC,
            -1.0f64,
            43.0f64,
            GucContext::Userset,
            GucFlags::default(),
        );
        assert_eq!(GUC.get(), 42.42);

        Spi::run("SET test.float = -1").expect("SPI failed");
        assert_eq!(GUC.get(), -1.0);

        Spi::run("SET test.float = 12").expect("SPI failed");
        assert_eq!(GUC.get(), 12.0);

        Spi::run("SET test.float = 3.333").expect("SPI failed");
        assert_eq!(GUC.get(), 3.333);
    }

    #[pg_test]
    fn test_string_guc() {
        static GUC: GucSetting<Option<CString>> =
            GucSetting::<Option<CString>>::new(Some(c"this is a test"));
        GucRegistry::define_string_guc(
            c"test.string_guc",
            c"test string guc",
            c"test string guc",
            &GUC,
            GucContext::Userset,
            GucFlags::default(),
        );
        assert!(GUC.get().is_some());
        assert_eq!(GUC.get().unwrap().to_str().unwrap(), "this is a test");

        Spi::run("SET test.string_guc = 'foo'").expect("SPI failed");
        assert_eq!(GUC.get().unwrap().to_str().unwrap(), "foo");

        Spi::run("SET test.string_guc = DEFAULT").expect("SPI failed");
        assert_eq!(GUC.get().unwrap().to_str().unwrap(), "this is a test");
    }

    #[pg_test]
    fn test_string_guc_null_default() {
        static GUC: GucSetting<Option<CString>> = GucSetting::<Option<CString>>::new(None);
        GucRegistry::define_string_guc(
            c"test.string_guc_null",
            c"test string guc",
            c"test string guc",
            &GUC,
            GucContext::Userset,
            GucFlags::default(),
        );
        assert!(GUC.get().is_none());

        Spi::run("SET test.string_guc_null = 'foo'").expect("SPI failed");
        assert_eq!(GUC.get().unwrap().to_str().unwrap(), "foo");

        Spi::run("SET test.string_guc_null = DEFAULT").expect("SPI failed");
        assert!(GUC.get().is_none());
    }

    #[pg_test]
    fn test_enum_guc() {
        #[derive(PostgresGucEnum, Clone, Copy, PartialEq, Debug)]
        enum TestEnum {
            One,
            Two,
            #[doc = "three"]
            Three,
            #[name = c"five"]
            Four,
            #[hidden = true]
            Six,
        }
        static GUC: GucSetting<TestEnum> = GucSetting::<TestEnum>::new(TestEnum::Two);
        GucRegistry::define_enum_guc(
            c"test.enum",
            c"test enum guc",
            c"test enum guc",
            &GUC,
            GucContext::Userset,
            GucFlags::default(),
        );
        assert_eq!(GUC.get(), TestEnum::Two);

        Spi::run("SET test.enum = 'One'").expect("SPI failed");
        assert_eq!(GUC.get(), TestEnum::One);

        Spi::run("SET test.enum = 'three'").expect("SPI failed");
        assert_eq!(GUC.get(), TestEnum::Three);

        Spi::run("SET test.enum = 'five'").expect("SPI failed");
        assert_eq!(GUC.get(), TestEnum::Four);
    }

    #[pg_test]
    fn test_guc_flags() {
        // variable ensures that GucFlags is Copy, so single name can be used when defining
        // multiple gucs
        let no_show_flag = GucFlags::NO_SHOW_ALL;
        static GUC_NO_SHOW: GucSetting<bool> = GucSetting::<bool>::new(true);
        static GUC_NO_RESET_ALL: GucSetting<bool> = GucSetting::<bool>::new(true);
        GucRegistry::define_bool_guc(
            c"test.no_show",
            c"test no show gucs",
            c"test no show gucs",
            &GUC_NO_SHOW,
            GucContext::Userset,
            no_show_flag,
        );
        GucRegistry::define_bool_guc(
            c"test.no_reset_all",
            c"test no reset gucs",
            c"test no reset gucs",
            &GUC_NO_RESET_ALL,
            GucContext::Userset,
            GucFlags::NO_RESET_ALL,
        );

        // change both, then check that:
        //  1. no_show does not appear in SHOW ALL while no_reset_all does
        //  2. no_reset_all is not reset by RESET ALL, while no_show is
        Spi::run("SET test.no_show TO false;").expect("SPI failed");
        Spi::run("SET test.no_reset_all TO false;").expect("SPI failed");
        assert!(!GUC_NO_RESET_ALL.get());
        Spi::connect_mut(|client| {
            let r = client.update("SHOW ALL", None, &[]).expect("SPI failed");

            let mut no_reset_guc_in_show_all = false;
            for row in r {
                // cols of show all: name, setting, description
                let name: &str = row.get(1).unwrap().unwrap();
                assert!(!name.contains("test.no_show"));
                if name.contains("test.no_reset_all") {
                    no_reset_guc_in_show_all = true;
                }
            }
            assert!(no_reset_guc_in_show_all);

            Spi::run("RESET ALL").expect("SPI failed");
            assert!(
                !GUC_NO_RESET_ALL.get(),
                "'no_reset_all' should remain unchanged after 'RESET ALL'"
            );
            assert!(GUC_NO_SHOW.get(), "'no_show' should reset after 'RESET ALL'");
        });
    }

    #[pg_test]
    #[should_panic(expected = "invalid value for parameter \"test.check_hooks\": 0")]
    fn test_guc_check_hook() {
        static SIDE_EFFECT: std::sync::RwLock<i32> = std::sync::RwLock::new(0);

        #[pg_guard]
        unsafe extern "C-unwind" fn check_hook(
            newval: *mut bool,
            _extra: *mut *mut std::ffi::c_void,
            _source: pg_sys::GucSource::Type,
        ) -> bool {
            if unsafe { *newval } {
                *SIDE_EFFECT.write().unwrap() += 1;
            }
            unsafe { *newval }
        }

        // Create and register GUC with hooks. As default is true, SIDE_EFFECT will be 1.
        static GUC: GucSetting<bool> = GucSetting::<bool>::new(true);
        unsafe {
            GucRegistry::define_bool_guc_with_hooks(
                c"test.check_hooks",
                c"test hooks guc",
                c"test hooks guc",
                &GUC,
                GucContext::Userset,
                GucFlags::default(),
                Some(check_hook),
                None,
                None,
            );
        }

        // Test check hook - should reject false and not initialize the GUC
        assert!(
            Spi::run("SET test.check_hooks TO false").is_err(),
            "Expected panic when setting test.check_hooks to false"
        );
        assert_eq!(*SIDE_EFFECT.read().unwrap(), 1);

        // Test check hook - should accept true and increment SIDE_EFFECT
        assert!(Spi::run("SET test.check_hooks TO true").is_ok());
        assert!(GUC.get());
        assert_eq!(*SIDE_EFFECT.read().unwrap(), 2);
    }

    #[pg_test]
    #[should_panic(expected = "should panic!")]
    fn test_check_hook_fail() {
        #[pg_guard]
        unsafe extern "C-unwind" fn check_hook(
            newval: *mut bool,
            _extra: *mut *mut std::ffi::c_void,
            _source: pg_sys::GucSource::Type,
        ) -> bool {
            if unsafe { *newval } {
                panic!("should panic!");
            }
            unsafe { *newval }
        }

        static GUARDED_GUC: GucSetting<bool> = GucSetting::<bool>::new(true);
        unsafe {
            GucRegistry::define_bool_guc_with_hooks(
                c"test.guarded_hooks",
                c"test guarded hooks guc",
                c"test guarded hooks guc",
                &GUARDED_GUC,
                GucContext::Userset,
                GucFlags::default(),
                Some(check_hook),
                None,
                None,
            );
        }
    }

    #[pg_test]
    fn test_assign_hook() {
        static SIDE_EFFECT: std::sync::RwLock<i32> = std::sync::RwLock::new(0);

        #[pg_guard]
        unsafe extern "C-unwind" fn assign_hook(newval: bool, _extra: *mut ::core::ffi::c_void) {
            if newval {
                *SIDE_EFFECT.write().unwrap() += 1;
            }
        }

        // Create and register GUC with hooks. As default is false, SIDE_EFFECT will be 0.
        static GUC: GucSetting<bool> = GucSetting::<bool>::new(false);
        unsafe {
            GucRegistry::define_bool_guc_with_hooks(
                c"test.assign_hooks",
                c"test hooks guc",
                c"test hooks guc",
                &GUC,
                GucContext::Userset,
                GucFlags::default(),
                None,
                Some(assign_hook),
                None,
            );
        }

        // SIDE_EFFECT should not be updated
        Spi::run("SET test.assign_hooks TO false").unwrap();
        assert_eq!(*SIDE_EFFECT.read().unwrap(), 0);

        // SIDE_EFFECT should be updated
        Spi::run("SET test.assign_hooks TO true").unwrap();
        assert_eq!(*SIDE_EFFECT.read().unwrap(), 1);
    }

    #[pg_test]
    fn test_show_hook() {
        #[pg_guard]
        unsafe extern "C-unwind" fn show_hook() -> *const c_char {
            CString::new("CUSTOM_SHOW_HOOK").unwrap().into_raw() as *const c_char
        }

        // Register GUC
        static GUC: GucSetting<bool> = GucSetting::<bool>::new(false);
        unsafe {
            GucRegistry::define_bool_guc_with_hooks(
                c"test.show_hooks",
                c"test hooks guc",
                c"test hooks guc",
                &GUC,
                GucContext::Userset,
                GucFlags::default(),
                None,
                None,
                Some(show_hook),
            );
        }

        // Test show hook
        Spi::connect_mut(|client| {
            let r = client.update("SHOW test.show_hooks", None, &[]).expect("SPI failed");
            let value: &str = r.first().get_one::<&str>().unwrap().unwrap();
            assert_eq!(value, "CUSTOM_SHOW_HOOK");
        });
    }

    #[derive(PostgresGucEnum, Clone, Copy, PartialEq, Debug)]
    enum HookTestEnum {
        One,
        Two,
    }

    macro_rules! test_pg_guc_hook_macro {
        (
            $test_name:ident,
            $type:ty,
            $initial_val:expr,
            $base_param_name:expr,
            $set_sql_val:expr,
            $expected_val:expr,
            |$guc_setting:ident, $name:ident, $check:ident, $assign:ident, $show:ident| $register_body:expr
        ) => {
            #[pg_test]
            fn $test_name() {
                static CALLED: std::sync::RwLock<Option<$type>> = std::sync::RwLock::new(None);
                static GUC: GucSetting<$type> = GucSetting::<$type>::new($initial_val);

                // Postgres presents show hook return value.
                #[pg_guc_hook(show)]
                fn my_show_hook() -> String {
                    "SHOW_MACRO".to_owned()
                }

                // Check hook boolean true; accepts new value.
                #[pg_guc_hook(check)]
                fn my_check_hook(_newval: $type) -> bool {
                    true
                }

                // Assign hook receives new value.
                #[pg_guc_hook(assign)]
                fn my_assign_hook(newval: $type) {
                    *CALLED.write().unwrap() = Some(newval);
                }

                let c_name = CString::new(format!("{}_main", $base_param_name)).unwrap();
                let leaked_name = Box::leak(c_name.into_boxed_c_str());

                unsafe {
                    let register = |$guc_setting: &'static GucSetting<$type>, $name, $check, $assign, $show| { $register_body };
                    register(&GUC, leaked_name, Some(my_check_hook), Some(my_assign_hook), Some(my_show_hook));
                }

                Spi::connect_mut(|client| {
                    let r = client.update(&format!("SHOW {}", leaked_name.to_str().unwrap()), None, &[]).unwrap();
                    let value: &str = r.first().get_one::<&str>().unwrap().unwrap();
                    assert_eq!(value, "SHOW_MACRO");
                });

                Spi::run(&format!("SET {} = {}", leaked_name.to_str().unwrap(), $set_sql_val)).unwrap();
                assert_eq!(*CALLED.read().unwrap(), Some($expected_val));
            }

            paste::paste! {
                // Check hook Result Ok(); accepts new value.
                #[pg_test]
                fn [<$test_name _check_ok>]() {
                    static CALLED: std::sync::RwLock<Option<$type>> = std::sync::RwLock::new(None);
                    static GUC: GucSetting<$type> = GucSetting::<$type>::new($initial_val);

                    #[pg_guc_hook(check)]
                    fn my_check_hook(_newval: $type) -> Result<(), GucCheckError> {
                        Ok(())
                    }

                    #[pg_guc_hook(assign)]
                    fn my_assign_hook(newval: $type) {
                        *CALLED.write().unwrap() = Some(newval);
                    }

                    let c_name = CString::new(format!("{}_check_ok", $base_param_name)).unwrap();
                    let leaked_name = Box::leak(c_name.into_boxed_c_str());

                    unsafe {
                        let register = |$guc_setting: &'static GucSetting<$type>, $name, $check, $assign, $show| { $register_body };
                        register(&GUC, leaked_name, Some(my_check_hook), Some(my_assign_hook), None);
                    }

                    Spi::run(&format!("SET {} = {}", leaked_name.to_str().unwrap(), $set_sql_val)).unwrap();
                    assert_eq!(*CALLED.read().unwrap(), Some($expected_val));
                }

                // Check hook boolean false; rejects new value.
                #[pg_test]
                #[should_panic(expected = "invalid value")]
                fn [<$test_name _check_false>]() {
                    static GUC: GucSetting<$type> = GucSetting::<$type>::new($initial_val);

                    #[pg_guc_hook(check)]
                    fn my_check_hook(_newval: $type) -> bool {
                        false
                    }

                    let c_name = CString::new(format!("{}_check_false", $base_param_name)).unwrap();
                    let leaked_name = Box::leak(c_name.into_boxed_c_str());

                    unsafe {
                        let register = |$guc_setting: &'static GucSetting<$type>, $name, $check, $assign, $show| { $register_body };
                        register(&GUC, leaked_name, Some(my_check_hook), None, None);
                    }

                    let _ = Spi::run(&format!("SET {} = {}", leaked_name.to_str().unwrap(), $set_sql_val));
                }

                // Check hook Result Err(); rejects with message.
                #[pg_test]
                #[should_panic(expected = "custom message")]
                fn [<$test_name _check_err_message>]() {
                    static GUC: GucSetting<$type> = GucSetting::<$type>::new($initial_val);

                    #[pg_guc_hook(check)]
                    fn my_check_hook(_newval: $type) -> Result<(), GucCheckError> {
                        Err(GucCheckError::new("custom message"))
                    }

                    let c_name = CString::new(format!("{}_check_err_message", $base_param_name)).unwrap();
                    let leaked_name = Box::leak(c_name.into_boxed_c_str());

                    unsafe {
                        let register = |$guc_setting: &'static GucSetting<$type>, $name, $check, $assign, $show| { $register_body };
                        register(&GUC, leaked_name, Some(my_check_hook), None, None);
                    }

                    let _ = Spi::run(&format!("SET {} = {}", leaked_name.to_str().unwrap(), $set_sql_val));
                }

                // Check hook Result Err(); rejects with hint.
                #[pg_test]
                #[should_panic(expected = "positive")]
                fn [<$test_name _check_err_hint>]() {
                    static GUC: GucSetting<$type> = GucSetting::<$type>::new($initial_val);

                    #[pg_guc_hook(check)]
                    fn my_check_hook(_newval: $type) -> Result<(), GucCheckError> {
                        Err(GucCheckError::new("negative").with_hint("positive"))
                    }

                    let c_name = CString::new(format!("{}_check_err_hint", $base_param_name)).unwrap();
                    let leaked_name = Box::leak(c_name.into_boxed_c_str());

                    unsafe {
                        let register = |$guc_setting: &'static GucSetting<$type>, $name, $check, $assign, $show| { $register_body };
                        register(&GUC, leaked_name, Some(my_check_hook), None, None);
                    }

                    let _ = Spi::run(&format!("SET {} = {}", leaked_name.to_str().unwrap(), $set_sql_val));
                }

                // Check hook receives a second argument.
                #[pg_test]
                fn [<$test_name _check_source_argument>]() {
                    static CALLED: std::sync::RwLock<pg_sys::GucSource::Type> = std::sync::RwLock::new(100);
                    static GUC: GucSetting<$type> = GucSetting::<$type>::new($initial_val);

                    #[pg_guc_hook(check)]
                    fn my_check_hook(_newval: $type, source: pg_sys::GucSource::Type) -> bool {
                        *CALLED.write().unwrap() = source;
                        true
                    }

                    let c_name = CString::new(format!("{}_check_source_argument", $base_param_name)).unwrap();
                    let leaked_name = Box::leak(c_name.into_boxed_c_str());

                    unsafe {
                        let register = |$guc_setting: &'static GucSetting<$type>, $name, $check, $assign, $show| { $register_body };
                        register(&GUC, leaked_name, Some(my_check_hook), None, None);
                    }

                    Spi::run(&format!("SET {} = {}", leaked_name.to_str().unwrap(), $set_sql_val)).unwrap();
                    assert_eq!(*CALLED.read().unwrap(), pg_sys::GucSource::PGC_S_SESSION);
                }
            }
        };
    }

    test_pg_guc_hook_macro!(
        test_pg_guc_hook_bool,
        bool,
        false,
        "test.macro_bool",
        "true",
        true,
        |setting, name, check, assign, show| {
            GucRegistry::define_bool_guc_with_hooks(
                name,
                c"test bool macro",
                c"test bool macro",
                setting,
                GucContext::Userset,
                GucFlags::default(),
                check,
                assign,
                show,
            )
        }
    );

    test_pg_guc_hook_macro!(
        test_pg_guc_hook_int,
        i32,
        100,
        "test.macro_int",
        "500",
        500,
        |setting, name, check, assign, show| {
            GucRegistry::define_int_guc_with_hooks(
                name,
                c"test int macro",
                c"test int macro",
                setting,
                -1000,
                1000,
                GucContext::Userset,
                GucFlags::default(),
                check,
                assign,
                show,
            )
        }
    );

    test_pg_guc_hook_macro!(
        test_pg_guc_hook_float,
        f64,
        1.5,
        "test.macro_float",
        "3.15",
        3.15,
        |setting, name, check, assign, show| {
            GucRegistry::define_float_guc_with_hooks(
                name,
                c"test float macro",
                c"test float macro",
                setting,
                -10.0,
                10.0,
                GucContext::Userset,
                GucFlags::default(),
                check,
                assign,
                show,
            )
        }
    );

    test_pg_guc_hook_macro!(
        test_pg_guc_hook_string,
        Option<CString>,
        None,
        "test.macro_string",
        "'hello'",
        Some(CString::new("hello").unwrap()),
        |setting, name, check, assign, show| {
            GucRegistry::define_string_guc_with_hooks(
                name,
                c"test string macro",
                c"test string macro",
                setting,
                GucContext::Userset,
                GucFlags::default(),
                check,
                assign,
                show,
            )
        }
    );

    test_pg_guc_hook_macro!(
        test_pg_guc_hook_enum,
        HookTestEnum,
        HookTestEnum::One,
        "test.macro_enum",
        "'Two'",
        HookTestEnum::Two,
        |setting, name, check, assign, show| {
            GucRegistry::define_enum_guc_with_hooks(
                name,
                c"test enum macro",
                c"test enum macro",
                setting,
                GucContext::Userset,
                GucFlags::default(),
                check,
                assign,
                show,
            )
        }
    );
}
