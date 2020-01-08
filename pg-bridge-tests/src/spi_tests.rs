mod tests {
    #[allow(unused_imports)]
    use crate as pg_bridge_tests;

    use pg_bridge::*;

    #[pg_test(error = "syntax error at or near \"THIS\"")]
    fn test_spi_failure() {
        Spi::connect(|client| {
            client.select("THIS IS NOT A VALID QUERY", None, None);

            Ok(().into())
        });
    }

    #[pg_test]
    fn test_spi_can_recurse() {
        Spi::connect(|_| {
            Spi::connect(|_| {
                Spi::connect(|_| {
                    Spi::connect(|_| {
                        Spi::connect(|_| Ok(().into()));
                        Ok(().into())
                    });
                    Ok(().into())
                });
                Ok(().into())
            });
            Ok(().into())
        });
    }

    #[pg_test]
    fn test_spi_returns_primitive() {
        let rc = Spi::connect(|client| {
            Ok(client
                .select("SELECT 42", None, None)
                .get_datum::<i32>(1)
                .unwrap())
        });

        assert_eq!(42, rc.try_into().unwrap())
    }

    #[pg_test]
    fn test_spi_returns_str() {
        let rc = Spi::connect(|client| {
            Ok(client
                .select("SELECT 'this is a test'", None, None)
                .get_datum::<&str>(1)
                .unwrap())
        });

        let rc: &str = rc.try_into().unwrap();
        assert_eq!("this is a test", rc)
    }

    #[pg_test]
    fn test_spi_returns_string() {
        let rc = Spi::connect(|client| {
            Ok(client
                .select("SELECT 'this is a test'", None, None)
                .get_datum::<String>(1)
                .unwrap())
        });

        let rc: String = rc.try_into().unwrap();
        assert_eq!("this is a test", rc)
    }

    #[pg_test]
    fn test_spi_get_one() {
        Spi::connect(|client| {
            let val = client.select("SELECT 42", None, None).get_one::<i64>();

            assert_eq!(42, val);

            Ok(().into())
        });
    }

    #[pg_test]
    fn test_spi_get_two() {
        Spi::connect(|client| {
            let (i, s) = client
                .select("SELECT 42, 'test'", None, None)
                .get_two::<i64, &str>();

            assert_eq!(42, i);
            assert_eq!("test", s);

            Ok(().into())
        });
    }

    #[pg_test]
    fn test_spi_get_three() {
        Spi::connect(|client| {
            let (i, s, b) = client
                .select("SELECT 42, 'test', true", None, None)
                .get_three::<i64, &str, bool>();

            assert_eq!(42, i);
            assert_eq!("test", s);
            assert_eq!(true, b);

            Ok(().into())
        });
    }

    #[pg_test(error = "no value for column #2")]
    fn test_spi_get_two_with_failure() {
        Spi::connect(|client| {
            let (i, s) = client
                .select("SELECT 42", None, None)
                .get_two::<i64, &str>();

            assert_eq!(42, i);
            assert_eq!("test", s);

            Ok(().into())
        });
    }

    #[pg_test(error = "no value for column #3")]
    fn test_spi_get_three_failure() {
        Spi::connect(|client| {
            let (i, s, b) = client
                .select("SELECT 42, 'test'", None, None)
                .get_three::<i64, &str, bool>();

            assert_eq!(42, i);
            assert_eq!("test", s);
            assert_eq!(true, b);

            Ok(().into())
        });
    }
}
