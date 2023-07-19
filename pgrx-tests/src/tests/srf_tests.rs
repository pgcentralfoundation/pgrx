//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use pgrx::prelude::*;

#[pg_extern]
fn example_generate_series(
    start: i32,
    end: i32,
    step: default!(i32, 1),
) -> SetOfIterator<'static, i32> {
    SetOfIterator::new((start..=end).step_by(step as usize).into_iter())
}

#[pg_extern]
fn example_composite_set() -> TableIterator<'static, (name!(idx, i32), name!(value, &'static str))>
{
    TableIterator::new(
        vec!["a", "b", "c"].into_iter().enumerate().map(|(idx, value)| ((idx + 1) as i32, value)),
    )
}

#[pg_extern]
fn return_some_iterator(
) -> Option<TableIterator<'static, (name!(idx, i32), name!(some_value, &'static str))>> {
    Some(TableIterator::new(
        vec!["a", "b", "c"].into_iter().enumerate().map(|(idx, value)| ((idx + 1) as i32, value)),
    ))
}

#[pg_extern]
fn return_none_iterator(
) -> Option<TableIterator<'static, (name!(idx, i32), name!(some_value, &'static str))>> {
    if true {
        None
    } else {
        Some(TableIterator::new(
            vec!["a", "b", "c"]
                .into_iter()
                .enumerate()
                .map(|(idx, value)| ((idx + 1) as i32, value)),
        ))
    }
}

#[pg_extern]
fn return_some_setof_iterator() -> Option<SetOfIterator<'static, i32>> {
    Some(SetOfIterator::new(vec![1, 2, 3].into_iter()))
}

#[pg_extern]
fn return_none_setof_iterator() -> Option<SetOfIterator<'static, i32>> {
    if true {
        None
    } else {
        Some(SetOfIterator::new(vec![1, 2, 3].into_iter()))
    }
}

#[pg_extern]
fn return_none_result_setof_iterator(
) -> Result<Option<SetOfIterator<'static, String>>, Box<dyn std::error::Error>> {
    Ok(None)
}

// TODO:  We don't yet support returning Result<Option<TableIterator>> because the code generator
//        is inscrutable. But when we do, this function will help ensure it works
//
// #[pg_extern]
// fn return_none_result_tableiterator_iterator() -> Result<
//     Option<TableIterator<'static, (name!(idx, i32), name!(some_value, &'static str))>>,
//     Box<dyn std::error::Error>,
// > {
//     Ok(None)
// }

#[pg_extern]
fn split_set_with_borrow<'a>(input: &'a str, pattern: &'a str) -> SetOfIterator<'a, &'a str> {
    SetOfIterator::new(input.split_terminator(pattern))
}

#[pg_extern]
fn split_table_with_borrow<'a>(
    input: &'a str,
    pattern: &'a str,
) -> TableIterator<'a, (name!(i, i32), name!(s, &'a str))> {
    TableIterator::new(input.split_terminator(pattern).enumerate().map(|(i, s)| (i as i32, s)))
}

#[pg_extern]
fn result_table_1<'a>() -> Result<
    Option<::pgrx::iter::TableIterator<'a, (name!(a, Option<i32>), name!(b, Option<i32>))>>,
    Box<dyn std::error::Error + Send + Sync + 'static>,
> {
    Ok(Some(TableIterator::new(vec![(Some(1), Some(2))])))
}

#[pg_extern]
fn result_table_2<'a>() -> Result<
    Option<pgrx::iter::TableIterator<'a, (name!(a, Option<i32>), name!(b, Option<i32>))>>,
    Box<dyn std::error::Error + Send + Sync + 'static>,
> {
    Ok(Some(TableIterator::new(vec![(Some(1), Some(2))])))
}

#[pg_extern]
fn result_table_3<'a>() -> Result<
    Option<TableIterator<'a, (name!(a, Option<i32>), name!(b, Option<i32>))>>,
    Box<dyn std::error::Error + Send + Sync + 'static>,
> {
    Ok(Some(TableIterator::new(vec![(Some(1), Some(2))])))
}

#[pg_extern]
fn result_table_4_err<'a>() -> Result<
    Option<TableIterator<'a, (name!(a, Option<i32>), name!(b, Option<i32>))>>,
    Box<dyn std::error::Error + Send + Sync + 'static>,
> {
    Err("oh no")?
}

#[pg_extern]
fn result_table_5_none<'a>() -> Result<
    Option<TableIterator<'a, (name!(a, Option<i32>), name!(b, Option<i32>))>>,
    Box<dyn std::error::Error + Send + Sync + 'static>,
> {
    Ok(None)
}

#[pg_extern]
fn one_col<'a>() -> TableIterator<'a, (name!(a, i32),)> {
    TableIterator::new(std::iter::once((42,)))
}

#[pg_extern]
fn one_col_option<'a>() -> Option<TableIterator<'a, (name!(a, i32),)>> {
    Some(TableIterator::new(std::iter::once((42,))))
}

#[pg_extern]
fn one_col_result<'a>() -> Result<TableIterator<'a, (name!(a, i32),)>, Box<dyn std::error::Error>> {
    Ok(TableIterator::new(std::iter::once((42,))))
}

#[pg_extern]
fn one_col_result_option<'a>(
) -> Result<Option<TableIterator<'a, (name!(a, i32),)>>, Box<dyn std::error::Error>> {
    Ok(Some(TableIterator::new(std::iter::once((42,)))))
}

#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgrx_tests;

    use pgrx::prelude::*;

    #[pg_test]
    fn test_generate_series() {
        let cnt = Spi::connect(|client| {
            let mut table =
                client.select("SELECT * FROM example_generate_series(1, 10)", None, None)?;

            let mut expect = 0;
            while table.next().is_some() {
                let value = table.get_one::<i32>()?;

                expect += 1;
                assert_eq!(value, Some(expect));
            }

            Ok::<_, spi::Error>(expect)
        })
        .unwrap();

        assert_eq!(cnt, 10)
    }

    #[pg_test]
    fn test_composite_set() {
        let cnt = Spi::connect(|client| {
            let mut table = client.select("SELECT * FROM example_composite_set()", None, None)?;

            let mut expect = 0;
            while table.next().is_some() {
                let (idx, value) = table.get_two::<i32, &str>()?;

                expect += 1;
                assert_eq!(idx, Some(expect));
                match idx {
                    Some(1) => assert_eq!(Some("a"), value),
                    Some(2) => assert_eq!(Some("b"), value),
                    Some(3) => assert_eq!(Some("c"), value),
                    _ => panic!("unexpected idx={:?}", idx),
                }
            }

            Ok::<_, spi::Error>(expect)
        })
        .unwrap();

        assert_eq!(cnt, 3)
    }

    #[pg_test]
    fn test_return_some_iterator() {
        let cnt = Spi::connect(|client| {
            let table = client.select("SELECT * from return_some_iterator();", None, None)?;

            Ok::<_, spi::Error>(table.len() as i64)
        });

        assert_eq!(cnt, Ok(3))
    }

    #[pg_test]
    fn test_return_none_iterator() {
        let cnt = Spi::connect(|client| {
            let table = client.select("SELECT * from return_none_iterator();", None, None)?;

            Ok::<_, spi::Error>(table.len() as i64)
        });

        assert_eq!(cnt, Ok(0))
    }

    #[pg_test]
    fn test_return_some_setof_iterator() {
        let cnt = Spi::connect(|client| {
            let table = client.select("SELECT * from return_some_setof_iterator();", None, None)?;

            Ok::<_, spi::Error>(table.len() as i64)
        });

        assert_eq!(cnt, Ok(3))
    }

    #[pg_test]
    fn test_return_none_setof_iterator() {
        let cnt = Spi::connect(|client| {
            let table = client.select("SELECT * from return_none_setof_iterator();", None, None)?;

            Ok::<_, spi::Error>(table.len() as i64)
        });

        assert_eq!(cnt, Ok(0))
    }

    #[pg_test]
    fn test_srf_setof_datum_detoasting_with_borrow() {
        let cnt = Spi::connect(|mut client| {
            // build up a table with one large column that Postgres will be forced to TOAST
            client.update("CREATE TABLE test_srf_datum_detoasting AS SELECT array_to_string(array_agg(g),' ') s FROM (SELECT 'a' g FROM generate_series(1, 1000)) x;", None, None)?;

            // and make sure we can use the DETOASTED value with our SRF function
            let table = client.select(
                "SELECT split_set_with_borrow(s, ' ') FROM test_srf_datum_detoasting",
                None,
                None,
            )?;

            Ok::<_, spi::Error>(table.len() as i64)
        });
        assert_eq!(cnt, Ok(1000))
    }

    #[pg_test]
    fn test_srf_table_datum_detoasting_with_borrow() {
        let cnt = Spi::connect(|mut client| {
            // build up a table with one large column that Postgres will be forced to TOAST
            client.update("CREATE TABLE test_srf_datum_detoasting AS SELECT array_to_string(array_agg(g),' ') s FROM (SELECT 'a' g FROM generate_series(1, 1000)) x;", None, None)?;

            // and make sure we can use the DETOASTED value with our SRF function
            let table = client.select(
                "SELECT split_table_with_borrow(s, ' ') FROM test_srf_datum_detoasting",
                None,
                None,
            )?;

            Ok::<_, spi::Error>(table.len() as i64)
        });
        assert_eq!(cnt, Ok(1000))
    }

    #[pg_test(error = "column \"cause_an_error\" does not exist")]
    pub fn spi_in_iterator(
    ) -> TableIterator<'static, (name!(id, i32), name!(relname, Result<Option<String>, spi::Error>))>
    {
        let oids = vec![1213, 1214, 1232, 1233, 1247, 1249, 1255];

        TableIterator::new(oids.into_iter().map(|oid| {
            (oid, Spi::get_one(&format!("SELECT CAUSE_AN_ERROR FROM pg_class WHERE oid = {oid}")))
        }))
    }

    #[pg_test(error = "column \"cause_an_error\" does not exist")]
    pub fn spi_in_setof() -> SetOfIterator<'static, Result<Option<String>, spi::Error>> {
        let oids = vec![1213, 1214, 1232, 1233, 1247, 1249, 1255];

        SetOfIterator::new(oids.into_iter().map(|oid| {
            Spi::get_one(&format!("SELECT CAUSE_AN_ERROR FROM pg_class WHERE oid = {oid}"))
        }))
    }

    #[pg_test]
    pub fn test_result_table_1() {
        let result = Spi::get_two::<i32, i32>("SELECT * from result_table_1()");
        assert_eq!(result, Ok((Some(1), Some(2))));
    }

    #[pg_test]
    pub fn test_result_table_2() {
        let result = Spi::get_two::<i32, i32>("SELECT * from result_table_2()");
        assert_eq!(result, Ok((Some(1), Some(2))));
    }

    #[pg_test]
    pub fn test_result_table_3() {
        let result = Spi::get_two::<i32, i32>("SELECT * from result_table_3()");
        assert_eq!(result, Ok((Some(1), Some(2))));
    }

    #[pg_test(error = "oh no")]
    pub fn test_result_table_4_err() {
        let _ = Spi::get_two::<i32, i32>("SELECT * from result_table_4_err()");
    }

    #[pg_test]
    pub fn test_result_table_5_none() {
        let result = Spi::get_two::<i32, i32>("SELECT * from result_table_5_none()");
        assert_eq!(result, Err(spi::Error::InvalidPosition));
    }

    #[pg_test]
    pub fn test_one_col_table() {
        assert_eq!(Spi::get_one::<i32>("SELECT * from one_col()"), Ok(Some(42)));
        assert_eq!(Spi::get_one::<i32>("SELECT * from one_col_option()"), Ok(Some(42)));
        assert_eq!(Spi::get_one::<i32>("SELECT * from one_col_result()"), Ok(Some(42)));
        assert_eq!(Spi::get_one::<i32>("SELECT * from one_col_result_option()"), Ok(Some(42)));
    }
}
