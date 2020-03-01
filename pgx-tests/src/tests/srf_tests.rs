use pgx::*;

#[pg_extern]
fn example_generate_series(
    start: i32,
    end: i32,
    step: default!(i32, 1),
) -> impl std::iter::Iterator<Item = i32> {
    (start..=end).step_by(step as usize)
}

#[pg_extern]
fn example_composite_set(
) -> impl std::iter::Iterator<Item = (name!(idx, i32), name!(value, &'static str))> {
    vec!["a", "b", "c"]
        .into_iter()
        .enumerate()
        .map(|(idx, value)| ((idx + 1) as i32, value))
}

#[cfg(any(test, feature = "pg_test"))]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;

    use pgx::*;

    #[pg_test]
    fn test_generate_series() {
        let cnt = Spi::connect(|client| {
            let mut table =
                client.select("SELECT * FROM example_generate_series(1, 10)", None, None);

            let mut expect = 0;
            while table.next().is_some() {
                let value = table.get_one::<i32>().expect("value was NULL");

                expect += 1;
                assert_eq!(value, expect);
            }

            Ok(Some(expect))
        });

        assert_eq!(cnt.unwrap(), 10)
    }

    #[pg_test]
    fn test_composite_set() {
        let cnt = Spi::connect(|client| {
            let mut table = client.select("SELECT * FROM example_composite_set()", None, None);

            let mut expect = 0;
            while table.next().is_some() {
                let (idx, value) = table.get_two::<i32, &str>();
                let idx = idx.expect("idx was null");
                let value = value.expect("value was null");

                expect += 1;
                assert_eq!(idx, expect);
                match idx {
                    1 => assert_eq!("a", value),
                    2 => assert_eq!("b", value),
                    3 => assert_eq!("c", value),
                    _ => panic!("unexpected idx={}", idx),
                }
            }

            Ok(Some(expect))
        });

        assert_eq!(cnt.unwrap(), 3)
    }
}
