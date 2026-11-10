//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgrx_unit_tests;

    use pgrx::prelude::*;

    #[pg_extern]
    fn tsearch_vector_ref(v: &TsVector) -> &TsVector {
        v
    }

    #[pg_extern]
    fn tsearch_vector_buf(v: TsVectorBuf) -> TsVectorBuf {
        v
    }

    #[pg_extern]
    fn tsearch_query_ref(q: &TsQuery) -> &TsQuery {
        q
    }

    #[pg_extern]
    fn tsearch_query_buf(q: TsQueryBuf) -> TsQueryBuf {
        q
    }

    #[pg_extern]
    fn tsearch_query_is_none(q: Option<&TsQuery>) -> bool {
        q.is_none()
    }

    #[pg_extern]
    fn tsearch_vector_is_empty(v: &TsVector) -> bool {
        v.is_empty()
    }

    #[pg_extern]
    fn tsearch_vector_varsize(v: &TsVector) -> i32 {
        let ptr = v.as_varlena_ptr().cast_mut();
        assert!(unsafe { pg_sys::VARATT_IS_4B_U!(ptr).get() } != 0);
        unsafe { pg_sys::VARSIZE_4B!(ptr).get() as i32 }
    }

    #[pg_extern]
    fn tsearch_vector_array(a: Array<'_, TsVectorBuf>) -> Vec<TsVectorBuf> {
        a.iter().map(|v| v.unwrap()).collect()
    }

    #[pg_extern]
    fn tsearch_vector_vec(v: Vec<TsVectorBuf>) -> Vec<TsVectorBuf> {
        v
    }

    fn sql_bool(query: &str) -> bool {
        Spi::get_one::<bool>(query).unwrap().unwrap()
    }

    #[pg_test]
    fn tsvector_roundtrips() {
        assert!(sql_bool(
            "SELECT tests.tsearch_vector_ref('a:1,3B b:2C'::tsvector) = 'a:1,3B b:2C'::tsvector"
        ));
        assert!(sql_bool(
            "SELECT tests.tsearch_vector_buf('a:1,3B b:2C'::tsvector) = 'a:1,3B b:2C'::tsvector"
        ));
        assert!(sql_bool("SELECT tests.tsearch_vector_buf(''::tsvector) = ''::tsvector"));
        assert!(sql_bool("SELECT tests.tsearch_vector_is_empty(''::tsvector)"));
        assert!(!sql_bool("SELECT tests.tsearch_vector_is_empty('a'::tsvector)"));
    }

    #[pg_test]
    fn tsquery_roundtrips() {
        assert!(sql_bool(
            "SELECT tests.tsearch_query_ref('a:AB & !b:* | (c <2> d)'::tsquery) = 'a:AB & !b:* | (c <2> d)'::tsquery"
        ));
        assert!(sql_bool(
            "SELECT tests.tsearch_query_buf('a:AB & !b:* | (c <2> d)'::tsquery) = 'a:AB & !b:* | (c <2> d)'::tsquery"
        ));
        assert!(sql_bool("SELECT tests.tsearch_query_is_none(NULL::tsquery)"));
        assert!(!sql_bool("SELECT tests.tsearch_query_is_none('a'::tsquery)"));
    }

    #[pg_test]
    fn tsvector_toasted_argument() {
        Spi::run(
            "CREATE TEMP TABLE tsearch_toast(v tsvector);
             ALTER TABLE tsearch_toast ALTER COLUMN v SET STORAGE EXTERNAL;
             INSERT INTO tsearch_toast SELECT to_tsvector('simple',
                 (SELECT string_agg('w' || i, ' ') FROM generate_series(1, 2000) i));",
        )
        .unwrap();
        assert!(sql_bool(
            "SELECT pg_relation_size(reltoastrelid) > 0 FROM pg_class WHERE oid = 'tsearch_toast'::regclass"
        ));
        assert!(sql_bool("SELECT tests.tsearch_vector_ref(v) = v FROM tsearch_toast"));
        assert!(sql_bool("SELECT tests.tsearch_vector_buf(v) = v FROM tsearch_toast"));
        assert!(sql_bool(
            "SELECT tests.tsearch_vector_varsize(v) = pg_column_size(v::text::tsvector) FROM tsearch_toast"
        ));
    }

    #[pg_test]
    fn tsvector_packed_argument() {
        Spi::run(
            "CREATE TEMP TABLE tsearch_packed(v tsvector);
             INSERT INTO tsearch_packed VALUES ('a:1 b:2'::tsvector);",
        )
        .unwrap();
        assert!(sql_bool("SELECT tests.tsearch_vector_ref(v) = v FROM tsearch_packed"));
        assert!(sql_bool("SELECT tests.tsearch_vector_buf(v) = v FROM tsearch_packed"));
        assert!(sql_bool(
            "SELECT tests.tsearch_vector_varsize(v) = pg_column_size(v) + 3 FROM tsearch_packed"
        ));
    }

    #[pg_test]
    fn tsvector_arrays() {
        assert!(sql_bool(
            "SELECT tests.tsearch_vector_array(ARRAY['a:1'::tsvector, 'b', '']) = ARRAY['a:1'::tsvector, 'b', '']"
        ));
        assert!(sql_bool(
            "SELECT tests.tsearch_vector_vec(ARRAY['a:1'::tsvector, 'b', '']) = ARRAY['a:1'::tsvector, 'b', '']"
        ));
    }

    #[pg_test]
    fn buf_outlives_spi_connection() {
        let v = Spi::get_one::<TsVectorBuf>("SELECT 'a:1 b'::tsvector").unwrap().unwrap();
        let q = Spi::get_one::<TsQueryBuf>("SELECT 'a & b'::tsquery").unwrap().unwrap();
        let matches = Spi::get_one_with_args::<bool>(
            "SELECT $1 = 'a:1 b'::tsvector AND $2 = 'a & b'::tsquery",
            &[v.into(), q.into()],
        );
        assert_eq!(matches, Ok(Some(true)));
    }

    fn equals_sql(value: impl IntoDatum, expr: &str) -> bool {
        Spi::get_one_with_args::<bool>(&format!("SELECT $1 = {expr}"), &[value.into()])
            .unwrap()
            .unwrap()
    }

    #[pg_test]
    fn tsvector_text() {
        let v: TsVectorBuf = "a:1,3B b:2C".parse().unwrap();
        assert_eq!(v.to_string(), "'a':1,3B 'b':2C");
        assert_eq!(format!("{v:?}"), r#"TsVectorBuf("'a':1,3B 'b':2C")"#);
        assert_eq!(format!("{:?}", &*v), r#"TsVector("'a':1,3B 'b':2C")"#);
        assert!(equals_sql(v.clone(), "'a:1,3B b:2C'::tsvector"));
        assert!(equals_sql(
            v.to_string().parse::<TsVectorBuf>().unwrap(),
            "'a:1,3B b:2C'::tsvector"
        ));
        assert!("a:".parse::<TsVectorBuf>().is_err());
        assert!("a\0b".parse::<TsVectorBuf>().is_err());
        assert!(format!("a{}", "b".repeat(3000)).parse::<TsVectorBuf>().is_err());
    }

    #[pg_test]
    fn tsquery_text() {
        let q: TsQueryBuf = "a:AB & !b:* | (c <2> d)".parse().unwrap();
        assert_eq!(q.to_string(), "'a':AB & !'b':* | 'c' <2> 'd'");
        assert!(equals_sql(q.clone(), "'a:AB & !b:* | (c <2> d)'::tsquery"));
        assert!(equals_sql(
            q.to_string().parse::<TsQueryBuf>().unwrap(),
            "'a:AB & !b:* | (c <2> d)'::tsquery"
        ));
        assert!("a &".parse::<TsQueryBuf>().is_err());
        assert!("a <99999> b".parse::<TsQueryBuf>().is_err());
        assert!("a\0b".parse::<TsQueryBuf>().is_err());
    }

    #[pg_test]
    fn empty_text_is_default() {
        let v: TsVectorBuf = "".parse().unwrap();
        let q: TsQueryBuf = "".parse().unwrap();
        assert!(v.is_empty() && q.is_empty());
        assert_eq!(v.to_string(), "");
        assert_eq!(q.to_string(), "");
        assert!(equals_sql(TsVectorBuf::default(), "''::tsvector"));
        assert!(equals_sql(TsQueryBuf::default(), "''::tsquery"));
        assert!(equals_sql(v, "''::tsvector"));
        assert!(equals_sql(q, "''::tsquery"));
    }

    #[pg_test]
    fn serde_roundtrip() {
        let v: TsVectorBuf = "a:1,3B b:2C".parse().unwrap();
        let json = serde_json::to_string(&v).unwrap();
        assert_eq!(json, r#""'a':1,3B 'b':2C""#);
        let back: TsVectorBuf = serde_json::from_str(&json).unwrap();
        assert_eq!(back.to_string(), v.to_string());
        assert_eq!(serde_json::to_string(&*v).unwrap(), json);
        assert!(serde_json::from_str::<TsVectorBuf>(r#""a:""#).is_err());

        let q: TsQueryBuf = "a & b".parse().unwrap();
        let json = serde_json::to_string(&q).unwrap();
        let back: TsQueryBuf = serde_json::from_str(&json).unwrap();
        assert_eq!(back.to_string(), q.to_string());
        assert!(serde_json::from_str::<TsQueryBuf>(r#""""#).unwrap().is_empty());
    }

    #[pg_test]
    fn tsvector_ordering_matches_sql() {
        let values = ["", "a", "a:1", "a:2", "a b", "b", "a:1A", "ab:1 c"];
        for l in values {
            for r in values {
                let (lb, rb): (TsVectorBuf, TsVectorBuf) = (l.parse().unwrap(), r.parse().unwrap());
                let sql = |op: &str| {
                    Spi::get_one_with_args::<bool>(
                        &format!("SELECT $1 {op} $2"),
                        &[lb.clone().into(), rb.clone().into()],
                    )
                    .unwrap()
                    .unwrap()
                };
                assert_eq!(lb < rb, sql("<"), "{l:?} < {r:?}");
                assert_eq!(lb == rb, sql("="), "{l:?} = {r:?}");
                assert_eq!(lb > rb, sql(">"), "{l:?} > {r:?}");
                assert_eq!(*lb == rb, lb == *rb);
                assert_eq!((*lb).partial_cmp(&rb), lb.partial_cmp(&*rb));
            }
        }
    }

    #[pg_test]
    fn tsquery_ordering_matches_sql() {
        let values =
            ["", "a", "b", "a & b", "a | b", "!a", "a <-> b", "a <2> b", "a:*", "a & b & c"];
        for l in values {
            for r in values {
                let (lb, rb): (TsQueryBuf, TsQueryBuf) = (l.parse().unwrap(), r.parse().unwrap());
                let sql = |op: &str| {
                    Spi::get_one_with_args::<bool>(
                        &format!("SELECT $1 {op} $2"),
                        &[lb.clone().into(), rb.clone().into()],
                    )
                    .unwrap()
                    .unwrap()
                };
                assert_eq!(lb < rb, sql("<"), "{l:?} < {r:?}");
                assert_eq!(lb == rb, sql("="), "{l:?} = {r:?}");
                assert_eq!(lb > rb, sql(">"), "{l:?} > {r:?}");
                assert_eq!(*lb == rb, lb == *rb);
            }
        }
    }

    fn sql_value<T: FromDatum + IntoDatum>(
        query: &str,
        args: &[pgrx::datum::DatumWithOid<'_>],
    ) -> T {
        Spi::get_one_with_args::<T>(query, args).unwrap().unwrap()
    }

    #[pg_test]
    fn tsvector_operations() {
        let texts = ["", "a:1 b:2", "b:1A c", "a:3"];
        let queries = ["", "a", "a & b", "a | c", "!a", "a <-> b"];
        for t in texts {
            let v: TsVectorBuf = t.parse().unwrap();
            for q in queries {
                let q: TsQueryBuf = q.parse().unwrap();
                let expected =
                    sql_value::<bool>("SELECT $1 @@ $2", &[v.clone().into(), q.clone().into()]);
                assert_eq!(v.matches(&q), expected, "{v} @@ {q}");
            }
            for u in texts {
                let u: TsVectorBuf = u.parse().unwrap();
                let expected = sql_value::<TsVectorBuf>(
                    "SELECT $1 || $2",
                    &[v.clone().into(), u.clone().into()],
                );
                assert_eq!(v.concat(&u), expected, "{v} || {u}");
            }
        }
    }

    #[pg_test]
    fn tsquery_operations() {
        let queries = ["", "a", "b:*", "a & b", "a | c", "!a", "a <-> b"];
        for l in queries {
            let l: TsQueryBuf = l.parse().unwrap();
            let sql = |op: &str, r: &TsQueryBuf| {
                sql_value::<TsQueryBuf>(
                    &format!("SELECT $1 {op} $2"),
                    &[l.clone().into(), r.clone().into()],
                )
            };
            assert_eq!(l.not(), sql_value::<TsQueryBuf>("SELECT !! $1", &[l.clone().into()]));
            for r in queries {
                let r: TsQueryBuf = r.parse().unwrap();
                assert_eq!(l.and(&r), sql("&&", &r), "{l} && {r}");
                assert_eq!(l.or(&r), sql("||", &r), "{l} || {r}");
                assert_eq!(l.followed_by(&r), sql("<->", &r), "{l} <-> {r}");
                let expected = sql_value::<TsQueryBuf>(
                    "SELECT tsquery_phrase($1, $2, 3)",
                    &[l.clone().into(), r.clone().into()],
                );
                assert_eq!(l.followed_by_at(&r, 3), expected, "{l} <3> {r}");
            }
        }
    }

    #[pg_test]
    fn operators_match_methods() {
        let a: TsVectorBuf = "a:1 b:2".parse().unwrap();
        let b: TsVectorBuf = "b:1 c:2".parse().unwrap();
        assert_eq!(&*a | &*b, a.concat(&b));

        let x: TsQueryBuf = "a & b".parse().unwrap();
        let y: TsQueryBuf = "c".parse().unwrap();
        let empty = TsQueryBuf::default();
        assert_eq!(&*x & &*y, x.and(&y));
        assert_eq!(&*x | &*y, x.or(&y));
        assert_eq!(!&*x, x.not());
        assert_eq!(&*x & &*empty, x.and(&empty));
        assert_eq!(&*empty | &*y, empty.or(&y));
        assert_eq!(x.clone() & y.clone(), x.and(&y));
        assert_eq!(x.clone() | y.clone(), x.or(&y));
        assert_eq!(!x.clone(), x.not());
    }
}
