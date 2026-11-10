//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgrx_unit_tests;

    use pgrx::datum::tsearch::Weight;
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

    type Unnested = Vec<(String, Option<Vec<i16>>, Option<Vec<String>>)>;

    fn unnest(v: &TsVectorBuf) -> Unnested {
        Spi::connect(|client| {
            let rows = client.select(
                "SELECT lexeme, positions, weights FROM unnest($1)",
                None,
                &[v.clone().into()],
            )?;
            let mut out = Vec::new();
            for row in rows {
                out.push((
                    row.get::<String>(1)?.unwrap(),
                    row.get::<Vec<i16>>(2)?,
                    row.get::<Vec<String>>(3)?,
                ));
            }
            Ok::<_, spi::Error>(out)
        })
        .unwrap()
    }

    #[pg_test]
    fn tsvector_lexemes_match_unnest() {
        let texts = [
            "a:1,3B b:2C",
            "a b",
            "",
            "h\u{e9}llo:1A \u{65e5}\u{672c}\u{8a9e}:2,16383D",
            "a:1,2,3,4 b:5D c:6A,7B,8C,9",
        ];
        for text in texts {
            let v: TsVectorBuf = text.parse().unwrap();
            let expected = unnest(&v);
            assert_eq!(v.lexemes().len(), v.len());
            assert_eq!(v.len(), expected.len());

            let actual: Unnested = v
                .lexemes()
                .map(|lexeme| {
                    let positions: Vec<_> = lexeme.positions().collect();
                    let weights = positions.iter().map(|p| char::from(p.weight).to_string());
                    (
                        lexeme.to_str().unwrap().to_owned(),
                        (!positions.is_empty())
                            .then(|| positions.iter().map(|p| p.pos as i16).collect()),
                        (!positions.is_empty()).then(|| weights.collect()),
                    )
                })
                .collect();
            assert_eq!(actual, expected, "{text:?}");
        }
    }

    fn image(lexemes: &[(&str, &[(u16, Weight)])]) -> TsVectorBuf {
        let mut strings = Vec::new();
        let mut entries = Vec::new();
        for (text, positions) in lexemes {
            let pos = strings.len() as u32;
            strings.extend_from_slice(text.as_bytes());
            if !positions.is_empty() {
                if strings.len() % 2 == 1 {
                    strings.push(0);
                }
                strings.extend_from_slice(&(positions.len() as u16).to_ne_bytes());
                for &(p, w) in *positions {
                    strings.extend_from_slice(&(((w as u16) << 14) | p).to_ne_bytes());
                }
            }
            let bits = pg_sys::WordEntry::new_bitfield_1(
                !positions.is_empty() as u32,
                text.len() as u32,
                pos,
            );
            let entry = pg_sys::WordEntry { _bindgen_align: [], _bitfield_1: bits };
            // SAFETY: `WordEntry` is four initialized bytes.
            entries.extend_from_slice(unsafe {
                std::slice::from_raw_parts(std::ptr::addr_of!(entry).cast::<u8>(), 4)
            });
        }
        let mut bytes = vec![0; 8];
        bytes[4..8].copy_from_slice(&(lexemes.len() as i32).to_ne_bytes());
        bytes.extend_from_slice(&entries);
        bytes.extend_from_slice(&strings);
        // SAFETY: `ptr` is a fresh allocation of `bytes.len()` bytes with a 4-byte header.
        unsafe {
            let ptr = pg_sys::palloc0(bytes.len()).cast::<u8>();
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, bytes.len());
            pg_sys::SET_VARSIZE_4B!(ptr.cast::<pg_sys::varlena>(), bytes.len() as i32).get();
            let datum = pg_sys::Datum::from(ptr);
            let buf = TsVectorBuf::from_datum(datum, false).unwrap();
            pg_sys::pfree(ptr.cast());
            buf
        }
    }

    #[pg_test]
    fn tsvector_image_matches_unnest() {
        use Weight::*;
        let cases: [&[(&str, &[(u16, Weight)])]; 4] = [
            &[],
            &[("a", &[(1, D), (3, B)]), ("b", &[(2, C)])],
            &[("abc", &[]), ("d", &[(16383, A)])],
            &[("h\u{e9}llo", &[(1, A)]), ("\u{65e5}\u{672c}", &[(2, D), (9, C)])],
        ];
        for case in cases {
            let expected: Unnested = case
                .iter()
                .map(|&(text, positions)| {
                    (
                        text.to_owned(),
                        (!positions.is_empty())
                            .then(|| positions.iter().map(|p| p.0 as i16).collect()),
                        (!positions.is_empty()).then(|| {
                            positions.iter().map(|p| char::from(p.1).to_string()).collect()
                        }),
                    )
                })
                .collect();
            assert_eq!(unnest(&image(case)), expected);
        }
    }

    #[pg_test]
    fn tsquery_items() {
        use pgrx::datum::tsearch::{Item, Operator};

        let q: TsQueryBuf = "a:AB & !b:* | (c <2> d) & e <-> f".parse().unwrap();
        let mut shape = Vec::new();
        for item in q.items() {
            shape.push(match item {
                Item::Operator(Operator::Not) => "!".to_owned(),
                Item::Operator(Operator::And) => "&".to_owned(),
                Item::Operator(Operator::Or) => "|".to_owned(),
                Item::Operator(Operator::Phrase { distance }) => format!("<{distance}>"),
                Item::Operand(op) => {
                    let mut weights: Vec<_> = op.weights().iter().map(char::from).collect();
                    weights.reverse();
                    let weights: String = weights.into_iter().collect();
                    let prefix = if op.is_prefix() { "*" } else { "" };
                    format!("{}:{weights}{prefix}", op.to_str().unwrap())
                }
            });
        }
        assert_eq!(
            shape,
            ["|", "&", "<1>", "f:", "e:", "<2>", "d:", "c:", "&", "!", "b:*", "a:AB"]
        );
        assert_eq!(q.items().len(), 12);
        assert_eq!(q.operands().count(), 6);
        assert_eq!(q.to_string(), "'a':AB & !'b':* | 'c' <2> 'd' & 'e' <-> 'f'");
    }

    #[pg_test]
    fn tsquery_items_of_empty() {
        let q = TsQueryBuf::default();
        assert_eq!(q.items().count(), 0);
    }

    #[pg_test]
    fn tsquery_items_unicode() {
        let q: TsQueryBuf = "h\u{e9}llo | \u{65e5}\u{672c}".parse().unwrap();
        let words: Vec<_> = q.operands().map(|o| o.to_str().unwrap().to_owned()).collect();
        assert_eq!(words, ["\u{65e5}\u{672c}", "h\u{e9}llo"]);
    }

    fn hash_of<T: std::hash::Hash + ?Sized>(value: &T) -> u64 {
        use std::hash::Hasher;
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    #[pg_test]
    fn tsvector_hash_follows_eq() {
        let texts = ["a:1,3B b:2C", "a:1,3 b:2C", "a b", "b a", "", "a:1", "a:1A", "h\u{e9}llo:1"];
        let values: Vec<TsVectorBuf> = texts.iter().map(|t| t.parse().unwrap()).collect();
        for a in &values {
            for b in &values {
                if a == b {
                    assert_eq!(hash_of(a), hash_of(b));
                }
            }
            assert_eq!(hash_of(a), hash_of(&**a));
        }
        let set: std::collections::HashSet<_> = values.iter().cloned().collect();
        assert_eq!(set.len(), texts.len() - 1);
    }

    #[pg_test]
    fn tsquery_hash_follows_eq() {
        let texts = ["a", "a:A", "a:*", "a & b", "b & a", "a | b", "a <-> b", "a <2> b", "!a", ""];
        let values: Vec<TsQueryBuf> = texts.iter().map(|t| t.parse().unwrap()).collect();
        for a in &values {
            for b in &values {
                if a == b {
                    assert_eq!(hash_of(a), hash_of(b));
                }
            }
            assert_eq!(hash_of(a), hash_of(&**a));
        }
        assert!(values[0] == values[1] && values[1] == values[2]);
        let set: std::collections::HashSet<_> = values.iter().cloned().collect();
        assert_eq!(set.len(), texts.len() - 2);
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

    fn sql_text<T: FromDatum + IntoDatum>(query: &str) -> T {
        Spi::get_one::<T>(query).unwrap().unwrap()
    }

    #[pg_test]
    fn tsvector_from_text() {
        use pgrx::datum::tsearch::TsConfig;

        let text = "The quick brown foxes jumped";
        let current: TsVectorBuf = sql_text(&format!("SELECT to_tsvector('{text}')"));
        assert_eq!(TsVectorBuf::from_text(text), current);
        let english = TsConfig::by_name("english").unwrap();
        let simple = TsConfig::by_name("pg_catalog.simple").unwrap();
        assert_ne!(english, simple);
        let expected: TsVectorBuf = sql_text(&format!("SELECT to_tsvector('simple', '{text}')"));
        assert_eq!(TsVectorBuf::from_text_with(simple, text), expected);
        let expected: TsVectorBuf = sql_text(&format!("SELECT to_tsvector('english', '{text}')"));
        assert_eq!(TsVectorBuf::from_text_with(english, text), expected);
        assert_ne!(TsVectorBuf::from_text_with(simple, text), expected);
        assert!(TsVectorBuf::from_text("").is_empty());
    }

    #[pg_test]
    fn ts_config_lookup() {
        use pgrx::datum::tsearch::TsConfig;

        assert!(TsConfig::by_name("nope").is_none());
        assert!(TsConfig::by_name("nope\0").is_none());
        for malformed in
            ["", "-", "0", "12345", "a.b.c.d", "nope.simple", "other_db.pg_catalog.simple"]
        {
            assert!(TsConfig::by_name(malformed).is_none(), "{malformed:?}");
        }
        assert_eq!(TsConfig::by_name("simple"), TsConfig::by_name("pg_catalog.simple"));
        let current = Spi::get_one::<String>("SHOW default_text_search_config").unwrap().unwrap();
        let name = current.trim_start_matches("pg_catalog.");
        assert_eq!(Some(TsConfig::current()), TsConfig::by_name(name));
    }

    #[pg_test]
    fn tsquery_from_text() {
        use pgrx::datum::tsearch::TsConfig;

        let simple = TsConfig::by_name("simple").unwrap();
        let text = "Fat & (Rats | cats)";
        let expected: TsQueryBuf = sql_text(&format!("SELECT to_tsquery('simple', '{text}')"));
        assert_eq!(TsQueryBuf::from_text_with(simple, text).unwrap(), expected);
        let current: TsQueryBuf = sql_text(&format!("SELECT to_tsquery('{text}')"));
        assert_eq!(TsQueryBuf::from_text(text).unwrap(), current);
        assert!(TsQueryBuf::from_text("a &").is_err());
        assert!(TsQueryBuf::from_text_with(simple, "a <99999> b").is_err());
    }

    #[pg_test]
    fn tsquery_from_free_text() {
        use pgrx::datum::tsearch::TsConfig;

        let simple = TsConfig::by_name("simple").unwrap();
        let text = "Fat rats: \"cats\" -dogs";
        let cases = [
            (TsQueryBuf::from_plain_text_with(simple, text), "plainto_tsquery"),
            (TsQueryBuf::from_phrase_text_with(simple, text), "phraseto_tsquery"),
            (TsQueryBuf::from_websearch_text_with(simple, text), "websearch_to_tsquery"),
        ];
        for (actual, function) in cases {
            let expected: TsQueryBuf =
                sql_text(&format!("SELECT {function}('simple', $${text}$$)"));
            assert_eq!(actual, expected, "{function}");
        }
        let current: TsQueryBuf = sql_text(&format!("SELECT plainto_tsquery($${text}$$)"));
        assert_eq!(TsQueryBuf::from_plain_text(text), current);
        let current: TsQueryBuf = sql_text(&format!("SELECT phraseto_tsquery($${text}$$)"));
        assert_eq!(TsQueryBuf::from_phrase_text(text), current);
        let current: TsQueryBuf = sql_text(&format!("SELECT websearch_to_tsquery($${text}$$)"));
        assert_eq!(TsQueryBuf::from_websearch_text(text), current);
    }

    #[pg_test]
    fn weight_labels() {
        assert_eq!(Weight::try_from('a'), Ok(Weight::A));
        assert_eq!(Weight::try_from('D'), Ok(Weight::D));
        assert!(Weight::try_from('E').is_err());
        assert!(Weight::D < Weight::C && Weight::C < Weight::B && Weight::B < Weight::A);
        assert_eq!(char::from(Weight::B), 'B');
        assert_eq!(Weight::default(), Weight::D);
    }
}
