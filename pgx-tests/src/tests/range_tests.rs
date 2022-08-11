/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use pgx::*;
use std::ops::Range;

#[pg_extern]
fn make_pgrange_i32(lower: Option<i32>, upper: Option<i32>) -> Option<PgRange<i32>> {
    Some(PgRange::from_values(lower, upper))
}

// #[pg_extern]
// #[pg_guard]
// fn make_range_i32(lower: Option<i32>, upper: Option<i32>) -> Option<Range<i32>> {
//     match (lower, upper) {
//         (Some(lower_val), Some(upper_val)) => Some(lower_val..upper_val),
//         _ => None,
//     }
// }

#[pg_extern]
fn make_pgrange_i64(lower: Option<i64>, upper: Option<i64>) -> Option<PgRange<i64>> {
    Some(PgRange::from_values(lower, upper))
}

// #[pg_extern]
// fn make_range_i64(lower: Option<i64>, upper: Option<i64>) -> Option<Range<i64>> {
//     match (lower, upper) {
//         (Some(lower_val), Some(upper_val)) => Some(lower_val..upper_val),
//         _ => None,
//     }
// }

#[pg_extern]
fn make_pgrange_date(lower: Option<Date>, upper: Option<Date>) -> Option<PgRange<Date>> {
    Some(PgRange::from_values(lower, upper))
}

// #[pg_extern]
// fn make_range_date(lower: Option<Date>, upper: Option<Date>) -> Option<Range<Date>> {
//     match (lower, upper) {
//         (Some(lower_val), Some(upper_val)) => Some(lower_val..upper_val),
//         _ => None,
//     }
// }

#[pg_extern]
fn make_pgrange_ts(
    lower: Option<Timestamp>,
    upper: Option<Timestamp>,
) -> Option<PgRange<Timestamp>> {
    Some(PgRange::from_values(lower, upper))
}

#[pg_extern]
fn make_pgrange_tstz(
    lower: Option<TimestampWithTimeZone>,
    upper: Option<TimestampWithTimeZone>,
) -> Option<PgRange<TimestampWithTimeZone>> {
    Some(PgRange::from_values(lower, upper))
}

#[pg_extern]
fn make_pgrange_num(lower: Option<Numeric>, upper: Option<Numeric>) -> Option<PgRange<Numeric>> {
    Some(PgRange::from_values(lower, upper))
}

#[pg_extern(name = "describe_pgrange")]
fn describe_pgrange_i32(range: PgRange<i32>) -> String {
    pgrange_to_string(&range)
}

#[pg_extern(name = "describe_pgrange")]
fn describe_pgrange_i64(range: PgRange<i64>) -> String {
    pgrange_to_string(&range)
}

#[pg_extern(name = "describe_pgrange")]
fn describe_pgrange_date(range: PgRange<Date>) -> String {
    pgrange_to_string(&range)
}

#[pg_extern(name = "describe_pgrange")]
fn describe_pgrange_ts(range: PgRange<Timestamp>) -> String {
    pgrange_to_string(&range)
}

#[pg_extern(name = "describe_pgrange")]
fn describe_pgrange_tsz(range: PgRange<TimestampWithTimeZone>) -> String {
    pgrange_to_string(&range)
}

#[pg_extern(name = "describe_pgrange")]
fn describe_pgrange_numeric(range: PgRange<pgx::Numeric>) -> String {
    pgrange_to_string(&range)
}

// #[pg_extern(name = "describe_range")]
// fn describe_range_i32(range: Option<Range<i32>>) -> String {
//     match range {
//         Some(r) => range_to_string(&r),
//         None => "null".into(),
//     }
// }

// #[pg_extern(name = "describe_range")]
// fn describe_range_i64(range: Option<Range<i64>>) -> String {
//     match range {
//         Some(r) => range_to_string(&r),
//         None => "null".into(),
//     }
// }

// #[pg_extern(name = "describe_range")]
// fn describe_range_date(range: Option<Range<Date>>) -> String {
//     match range {
//         Some(r) => range_to_string(&r),
//         None => "null".into(),
//     }
// }

fn pgrange_to_string<T: FromDatum + IntoDatum + std::fmt::Display + RangeSubType>(
    range: &PgRange<T>,
) -> String {
    if range.is_empty {
        return "empty".into();
    }
    let lower_val = if range.lower.infinite {
        "-infinity".into()
    } else {
        format!("{}", range.lower_val().unwrap())
    };
    let upper_val = if range.upper.infinite {
        "infinity".into()
    } else {
        format!("{}", range.upper_val().unwrap())
    };
    format!(
        "{} {} {} {}",
        if range.lower.inclusive { "li" } else { "le" },
        lower_val,
        if range.upper.inclusive { "ui" } else { "ue" },
        upper_val
    )
}

fn range_to_string<T: std::fmt::Display>(range: &Range<T>) -> String {
    format!("li {} ue {}", range.start, range.end)
}

#[cfg(any(test, feature = "pg_test"))]
#[pgx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;

    use pgx::*;

    // i32

    #[pg_test]
    fn pgtest_i32_pgrange_half_open() {
        let desc = Spi::get_one::<String>("SELECT describe_pgrange(int4range'[1,10)')")
            .expect("failed to get SPI result");
        assert_eq!(desc, "li 1 ue 10");
    }

    #[pg_test]
    fn pgtest_i32_pgrange_half_closed() {
        // know that (1,10] will be transformed to canonical form [2, 11)
        let desc = Spi::get_one::<String>("SELECT describe_pgrange(int4range'(1,10]')")
            .expect("failed to get SPI result");
        assert_eq!(desc, "li 2 ue 11");
    }

    #[pg_test]
    fn pgtest_i32_pgrange_inf_inf() {
        let desc = Spi::get_one::<String>("SELECT describe_pgrange(int4range'(,)')")
            .expect("failed to get SPI result");
        assert_eq!(desc, "le -infinity ue infinity");
    }

    #[pg_test]
    fn pgtest_i32_pgrange_empty() {
        let desc = Spi::get_one::<String>("SELECT describe_pgrange(int4range'(1,1]')")
            .expect("failed to get SPI result");
        assert_eq!(desc, "empty");
    }

    #[pg_test]
    fn pgtest_i32_make_pgrange_half_open() {
        let desc = Spi::get_one::<String>("SELECT make_pgrange_i32(1,10)::text")
            .expect("failed to get SPI result");
        assert_eq!(desc, "[1,10)");
    }

    #[pg_test]
    fn pgtest_i32_make_pgrange_val_inf() {
        let desc = Spi::get_one::<String>("SELECT make_pgrange_i32(1,NULL::int)::text")
            .expect("failed to get SPI result");
        assert_eq!(desc, "[1,)");
    }

    #[pg_test]
    fn pgtest_i32_make_pgrange_inf_val() {
        let desc = Spi::get_one::<String>("SELECT make_pgrange_i32(NULL::int, 10)::text")
            .expect("failed to get SPI result");
        assert_eq!(desc, "(,10)");
    }

    #[pg_test]
    fn pgtest_i32_make_pgrange_inf_inf() {
        let desc = Spi::get_one::<String>("SELECT make_pgrange_i32(NULL::int, NULL::int)::text")
            .expect("failed to get SPI result");
        assert_eq!(desc, "(,)");
    }

    #[pg_test]
    fn pgtest_i32_make_pgrange_empty() {
        let desc = Spi::get_one::<String>("SELECT make_pgrange_i32(1,1)::text")
            .expect("failed to get SPI result");
        assert_eq!(desc, "empty");
    }

    // #[pg_test]
    // fn pgtest_i32_range_half_open() {
    //     let desc = Spi::get_one::<String>("SELECT describe_range(int4range'[1,10)')")
    //         .expect("failed to get SPI result");
    //     assert_eq!(desc, "li 1 ue 10");
    // }

    // #[pg_test]
    // fn pgtest_i32_range_half_closed() {
    //     // know that (1,10] will be transformed to canonical form [2, 11)
    //     let desc = Spi::get_one::<String>("SELECT describe_range(int4range'(1,10]')")
    //         .expect("failed to get SPI result");
    //     assert_eq!(desc, "li 2 ue 11");
    // }

    // #[pg_test]
    // fn pgtest_i32_range_inf_inf() {
    //     let desc = Spi::get_one::<String>("SELECT describe_range(int4range'(,)')")
    //         .expect("failed to get SPI result");
    //     assert_eq!(desc, "le -infinity ue infinity");
    // }

    // #[pg_test]
    // fn pgtest_i32_range_empty() {
    //     let desc = Spi::get_one::<String>("SELECT describe_range(int4range'(1,1]')")
    //         .expect("failed to get SPI result");
    //     assert_eq!(desc, "null");
    // }

    // #[pg_test]
    // fn pgtest_i32_make_range_half_open() {
    //     let desc = Spi::get_one::<String>("SELECT COALESCE(make_range_i32(1,10)::text, 'null')")
    //         .expect("failed to get SPI result");
    //     assert_eq!(desc, "[1,10)");
    // }

    // #[pg_test]
    // fn pgtest_i32_make_range_val_inf() {
    //     let desc = Spi::get_one::<String>("SELECT COALESCE(make_range_i32(1,NULL::int)::text, 'null')")
    //         .expect("failed to get SPI result");
    //     assert_eq!(desc, "[1,)");
    // }

    // #[pg_test]
    // fn pgtest_i32_make_range_inf_val() {
    //     let desc = Spi::get_one::<String>("SELECT COALESCE(make_range_i32(NULL::int, 10)::text, 'null')")
    //         .expect("failed to get SPI result");
    //     assert_eq!(desc, "(,10)");
    // }

    // #[pg_test]
    // fn pgtest_i32_make_range_inf_inf() {
    //     let desc = Spi::get_one::<String>("SELECT COALESCE(make_range_i32(NULL::int, NULL::int)::text, 'null')")
    //         .expect("failed to get SPI result");
    //     assert_eq!(desc, "(,)");
    // }

    // #[pg_test]
    // fn pgtest_i32_make_range_empty() {
    //     let desc = Spi::get_one::<String>("SELECT COALESCE(make_range_i32(1,1)::text, 'null')")
    //         .expect("failed to get SPI result");
    //     assert_eq!(desc, "null");
    // }

    // i64

    #[pg_test]
    fn pgtest_i64_pgrange_half_open() {
        let desc = Spi::get_one::<String>("SELECT describe_pgrange(int8range'[1,10)')")
            .expect("failed to get SPI result");
        assert_eq!(desc, "li 1 ue 10");
    }

    #[pg_test]
    fn pgtest_i64_pgrange_half_closed() {
        // know that (1,10] will be transformed to canonical form [2, 11)
        let desc = Spi::get_one::<String>("SELECT describe_pgrange(int8range'(1,10]')")
            .expect("failed to get SPI result");
        assert_eq!(desc, "li 2 ue 11");
    }

    #[pg_test]
    fn pgtest_i64_pgrange_inf_inf() {
        let desc = Spi::get_one::<String>("SELECT describe_pgrange(int8range'(,)')")
            .expect("failed to get SPI result");
        assert_eq!(desc, "le -infinity ue infinity");
    }

    #[pg_test]
    fn pgtest_i64_pgrange_empty() {
        let desc = Spi::get_one::<String>("SELECT describe_pgrange(int8range'(1,1]')")
            .expect("failed to get SPI result");
        assert_eq!(desc, "empty");
    }

    #[pg_test]
    fn pgtest_i64_make_pgrange_half_open() {
        let desc = Spi::get_one::<String>("SELECT make_pgrange_i64(1,10)::text")
            .expect("failed to get SPI result");
        assert_eq!(desc, "[1,10)");
    }

    #[pg_test]
    fn pgtest_i64_make_pgrange_val_inf() {
        // know that (1,10] will be transformed to canonical form [2, 11)
        let desc = Spi::get_one::<String>("SELECT make_pgrange_i64(1,NULL::int8)::text")
            .expect("failed to get SPI result");
        assert_eq!(desc, "[1,)");
    }

    #[pg_test]
    fn pgtest_i64_make_pgrange_inf_val() {
        // know that (1,10] will be transformed to canonical form [2, 11)
        let desc = Spi::get_one::<String>("SELECT make_pgrange_i64(NULL::int8,10)::text")
            .expect("failed to get SPI result");
        assert_eq!(desc, "(,10)");
    }

    #[pg_test]
    fn pgtest_i64_make_pgrange_inf_inf() {
        let desc = Spi::get_one::<String>("SELECT make_pgrange_i64(NULL::int8,NULL::int8)::text")
            .expect("failed to get SPI result");
        assert_eq!(desc, "(,)");
    }

    #[pg_test]
    fn pgtest_i64_make_pgrange_empty() {
        let desc = Spi::get_one::<String>("SELECT make_pgrange_i64(1,1)::text")
            .expect("failed to get SPI result");
        assert_eq!(desc, "empty");
    }

    // date

    #[pg_test]
    fn pgtest_date_pgrange_half_open() {
        let desc =
            Spi::get_one::<String>("SELECT describe_pgrange(daterange'[2000-01-01,2022-01-01)')")
                .expect("failed to get SPI result");
        assert_eq!(desc, "li 2000-01-01 ue 2022-01-01");
    }

    #[pg_test]
    fn pgtest_date_pgrange_half_closed() {
        // know that (2000-01-01,2022-01-01] will be transformed to canonical form [2000-01-02,2022-01-02)
        let desc =
            Spi::get_one::<String>("SELECT describe_pgrange(daterange'(2000-01-01,2022-01-01]')")
                .expect("failed to get SPI result");
        assert_eq!(desc, "li 2000-01-02 ue 2022-01-02");
    }

    #[pg_test]
    fn pgtest_date_pgrange_inf_inf() {
        let desc = Spi::get_one::<String>("SELECT describe_pgrange(daterange'(,)')")
            .expect("failed to get SPI result");
        assert_eq!(desc, "le -infinity ue infinity");
    }

    #[pg_test]
    fn pgtest_date_pgrange_empty() {
        let desc =
            Spi::get_one::<String>("SELECT describe_pgrange(daterange'(2000-01-01,2000-01-01]')")
                .expect("failed to get SPI result");
        assert_eq!(desc, "empty");
    }

    #[pg_test]
    fn pgtest_date_make_pgrange_half_open() {
        let desc = Spi::get_one::<String>(
            "SELECT make_pgrange_date(date'2000-01-01',date'2022-01-01')::text",
        )
        .expect("failed to get SPI result");
        assert_eq!(desc, "[2000-01-01,2022-01-01)");
    }

    #[pg_test]
    fn pgtest_date_make_pgrange_val_inf() {
        let desc =
            Spi::get_one::<String>("SELECT make_pgrange_date(date'2000-01-01',NULL::date)::text")
                .expect("failed to get SPI result");
        assert_eq!(desc, "[2000-01-01,)");
    }

    #[pg_test]
    fn pgtest_date_make_pgrange_val_infinity() {
        let desc = Spi::get_one::<String>(
            "SELECT make_pgrange_date(date'2000-01-01',date'infinity')::text",
        )
        .expect("failed to get SPI result");
        assert_eq!(desc, "[2000-01-01,infinity)");
    }

    #[pg_test]
    fn pgtest_date_make_pgrange_neg_infinity_val() {
        let desc = Spi::get_one::<String>(
            "SELECT make_pgrange_date(date'-infinity',date'2022-01-01')::text",
        )
        .expect("failed to get SPI result");
        assert_eq!(desc, "[-infinity,2022-01-01)");
    }

    #[pg_test]
    fn pgtest_date_make_pgrange_inf_val() {
        let desc =
            Spi::get_one::<String>("SELECT make_pgrange_date(NULL::date,date'2022-01-01')::text")
                .expect("failed to get SPI result");
        assert_eq!(desc, "(,2022-01-01)");
    }

    #[pg_test]
    fn pgtest_date_make_pgrange_inf_inf() {
        let desc = Spi::get_one::<String>("SELECT make_pgrange_date(NULL::date,NULL::date)::text")
            .expect("failed to get SPI result");
        assert_eq!(desc, "(,)");
    }

    #[pg_test]
    fn pgtest_date_make_pgrange_empty() {
        let desc = Spi::get_one::<String>(
            "SELECT make_pgrange_date(date'2000-01-01',date'2000-01-01')::text",
        )
        .expect("failed to get SPI result");
        assert_eq!(desc, "empty");
    }

    // timestamp

    #[pg_test]
    fn pgtest_ts_pgrange_half_open() {
        let desc = Spi::get_one::<String>(
            "SELECT describe_pgrange(tsrange'[2000-01-01 00:00:00-00,2022-01-01 00:00:00-00)')",
        )
        .expect("failed to get SPI result");
        assert_eq!(desc, "li 2000-01-01T00:00:00-00 ue 2022-01-01T00:00:00-00");
    }

    #[pg_test]
    fn pgtest_ts_pgrange_half_closed() {
        let desc = Spi::get_one::<String>(
            "SELECT describe_pgrange(tsrange'(2000-01-01 00:00:00-00,2022-01-01 00:00:00-00]')",
        )
        .expect("failed to get SPI result");
        assert_eq!(desc, "le 2000-01-01T00:00:00-00 ui 2022-01-01T00:00:00-00");
    }

    #[pg_test]
    fn pgtest_ts_pgrange_inf_inf() {
        let desc = Spi::get_one::<String>("SELECT describe_pgrange(tsrange'(,)')")
            .expect("failed to get SPI result");
        assert_eq!(desc, "le -infinity ue infinity");
    }

    #[pg_test]
    fn pgtest_ts_pgrange_empty() {
        let desc = Spi::get_one::<String>(
            "SELECT describe_pgrange(tsrange'(2000-01-01 00:00:00-00,2000-01-01 00:00:00-00]')",
        )
        .expect("failed to get SPI result");
        assert_eq!(desc, "empty");
    }

    #[pg_test]
    fn pgtest_ts_make_pgrange_half_open() {
        let desc = Spi::get_one::<String>(
            "SELECT make_pgrange_ts(timestamp'2000-01-01 00:00:00',timestamp'2022-01-01 00:00:00')::text",
        )
        .expect("failed to get SPI result");
        assert_eq!(desc, "[\"2000-01-01 00:00:00\",\"2022-01-01 00:00:00\")");
    }

    #[pg_test]
    fn pgtest_ts_make_pgrange_val_inf() {
        let desc = Spi::get_one::<String>(
            "SELECT make_pgrange_ts(timestamp'2000-01-01 00:00:00',NULL::timestamp)::text",
        )
        .expect("failed to get SPI result");
        assert_eq!(desc, "[\"2000-01-01 00:00:00\",)");
    }

    #[pg_test]
    fn pgtest_ts_make_pgrange_val_infinity() {
        let desc = Spi::get_one::<String>(
            "SELECT make_pgrange_ts(timestamp'2000-01-01 00:00:00',timestamp'infinity')::text",
        )
        .expect("failed to get SPI result");
        assert_eq!(desc, "[\"2000-01-01 00:00:00\",infinity)");
    }

    #[pg_test]
    fn pgtest_ts_make_pgrange_neg_infinity_val() {
        let desc = Spi::get_one::<String>(
            "SELECT make_pgrange_ts(timestamp'-infinity',timestamp'2022-01-01 00:00:00')::text",
        )
        .expect("failed to get SPI result");
        assert_eq!(desc, "[-infinity,\"2022-01-01 00:00:00\")");
    }

    #[pg_test]
    fn pgtest_ts_make_pgrange_inf_val() {
        let desc = Spi::get_one::<String>(
            "SELECT make_pgrange_ts(NULL::timestamp,timestamp'2022-01-01 00:00:00')::text",
        )
        .expect("failed to get SPI result");
        assert_eq!(desc, "(,\"2022-01-01 00:00:00\")");
    }

    #[pg_test]
    fn pgtest_ts_make_pgrange_inf_inf() {
        let desc =
            Spi::get_one::<String>("SELECT make_pgrange_ts(NULL::timestamp,NULL::timestamp)::text")
                .expect("failed to get SPI result");
        assert_eq!(desc, "(,)");
    }

    #[pg_test]
    fn pgtest_ts_make_pgrange_empty() {
        let desc = Spi::get_one::<String>(
            "SELECT make_pgrange_ts(timestamp'2000-01-01 00:00:00',timestamp'2000-01-01 00:00:00')::text",
        )
        .expect("failed to get SPI result");
        assert_eq!(desc, "empty");
    }

    // timestamp with time zone

    #[pg_test]
    fn pgtest_tstz_pgrange_half_open() {
        let desc = Spi::get_one::<String>(
            "SELECT describe_pgrange(tstzrange'[2000-01-01 00:00:00-00,2022-01-01 00:00:00-00)')",
        )
        .expect("failed to get SPI result");
        assert_eq!(desc, "li 2000-01-01T00:00:00-00 ue 2022-01-01T00:00:00-00");
    }

    #[pg_test]
    fn pgtest_tstz_pgrange_half_closed() {
        let desc = Spi::get_one::<String>(
            "SELECT describe_pgrange(tstzrange'(2000-01-01 00:00:00-00,2022-01-01 00:00:00-00]')",
        )
        .expect("failed to get SPI result");
        assert_eq!(desc, "le 2000-01-01T00:00:00-00 ui 2022-01-01T00:00:00-00");
    }

    #[pg_test]
    fn pgtest_tstz_pgrange_inf_inf() {
        let desc = Spi::get_one::<String>("SELECT describe_pgrange(tstzrange'(,)')")
            .expect("failed to get SPI result");
        assert_eq!(desc, "le -infinity ue infinity");
    }

    #[pg_test]
    fn pgtest_tstz_pgrange_empty() {
        let desc = Spi::get_one::<String>(
            "SELECT describe_pgrange(tstzrange'(2000-01-01 00:00:00-00,2000-01-01 00:00:00-00]')",
        )
        .expect("failed to get SPI result");
        assert_eq!(desc, "empty");
    }

    #[pg_test]
    fn pgtest_tstz_make_pgrange_half_open() {
        let desc = Spi::get_one::<String>(
            "SET LOCAL TIME ZONE 'UTC';SELECT make_pgrange_tstz(timestamp'2000-01-01 00:00:00' at time zone 'UTC',timestamp'2022-01-01 00:00:00' at time zone 'UTC')::text",
        )
        .expect("failed to get SPI result");
        assert_eq!(
            desc,
            "[\"2000-01-01 00:00:00+00\",\"2022-01-01 00:00:00+00\")"
        );
    }

    #[pg_test]
    fn pgtest_tstz_make_pgrange_val_inf() {
        let desc =
            Spi::get_one::<String>("SET LOCAL TIME ZONE 'UTC';SELECT make_pgrange_tstz(timestamp'2000-01-01 00:00:00' at time zone 'UTC',NULL::timestamp)::text")
                .expect("failed to get SPI result");
        assert_eq!(desc, "[\"2000-01-01 00:00:00+00\",)");
    }

    #[pg_test]
    fn pgtest_tstz_make_pgrange_val_infinity() {
        let desc = Spi::get_one::<String>(
            "SET LOCAL TIME ZONE 'UTC';SELECT make_pgrange_tstz(timestamp'2000-01-01 00:00:00' at time zone 'UTC',timestamp'infinity')::text",
        )
        .expect("failed to get SPI result");
        assert_eq!(desc, "[\"2000-01-01 00:00:00+00\",infinity)");
    }

    #[pg_test]
    fn pgtest_tstz_make_pgrange_neg_infinity_val() {
        let desc = Spi::get_one::<String>(
            "SET LOCAL TIME ZONE 'UTC';SELECT make_pgrange_tstz(timestamp'-infinity',timestamp'2022-01-01 00:00:00' at time zone 'UTC')::text",
        )
        .expect("failed to get SPI result");
        assert_eq!(desc, "[-infinity,\"2022-01-01 00:00:00+00\")");
    }

    #[pg_test]
    fn pgtest_tstz_make_pgrange_inf_val() {
        let desc =
            Spi::get_one::<String>("SET LOCAL TIME ZONE 'UTC';SELECT make_pgrange_tstz(NULL::timestamp,timestamp'2022-01-01 00:00:00' at time zone 'UTC')::text")
                .expect("failed to get SPI result");
        assert_eq!(desc, "(,\"2022-01-01 00:00:00+00\")");
    }

    #[pg_test]
    fn pgtest_tstz_make_pgrange_inf_inf() {
        let desc = Spi::get_one::<String>("SET LOCAL TIME ZONE 'UTC';SELECT make_pgrange_tstz(NULL::timestamp,NULL::timestamp)::text")
            .expect("failed to get SPI result");
        assert_eq!(desc, "(,)");
    }

    #[pg_test]
    fn pgtest_tstz_make_pgrange_empty() {
        let desc = Spi::get_one::<String>(
            "SET LOCAL TIME ZONE 'UTC'; SELECT make_pgrange_tstz(timestamp'2000-01-01 00:00:00' at time zone 'UTC',timestamp'2000-01-01 00:00:00' at time zone 'UTC')::text",
        )
        .expect("failed to get SPI result");
        assert_eq!(desc, "empty");
    }

    // numeric

    #[pg_test]
    fn pgtest_num_pgrange_half_open() {
        let desc = Spi::get_one::<String>("SELECT describe_pgrange(numrange'[0.0,1.0)')")
            .expect("failed to get SPI result");
        assert_eq!(desc, "li 0.0 ue 1.0");
    }

    #[pg_test]
    fn pgtest_num_pgrange_half_closed() {
        let desc = Spi::get_one::<String>("SELECT describe_pgrange(numrange'(0.0,1.0]')")
            .expect("failed to get SPI result");
        assert_eq!(desc, "le 0.0 ui 1.0");
    }

    #[pg_test]
    fn pgtest_num_pgrange_inf_inf() {
        let desc = Spi::get_one::<String>("SELECT describe_pgrange(numrange'(,)')")
            .expect("failed to get SPI result");
        assert_eq!(desc, "le -infinity ue infinity");
    }

    #[pg_test]
    fn pgtest_num_pgrange_empty() {
        let desc = Spi::get_one::<String>("SELECT describe_pgrange(numrange'(0.0,0.0]')")
            .expect("failed to get SPI result");
        assert_eq!(desc, "empty");
    }

    #[pg_test]
    fn pgtest_num_make_pgrange_half_open() {
        let desc = Spi::get_one::<String>("SELECT make_pgrange_num(0.0,1.0)::text")
            .expect("failed to get SPI result");
        assert_eq!(desc, "[0.0,1.0)");
    }

    #[pg_test]
    fn pgtest_num_make_pgrange_val_inf() {
        let desc = Spi::get_one::<String>("SELECT make_pgrange_num(0.0,NULL::numeric)::text")
            .expect("failed to get SPI result");
        assert_eq!(desc, "[0.0,)");
    }

    #[pg_test]
    fn pgtest_num_make_pgrange_inf_val() {
        let desc = Spi::get_one::<String>("SELECT make_pgrange_num(NULL::numeric,1.0)::text")
            .expect("failed to get SPI result");
        assert_eq!(desc, "(,1.0)");
    }

    #[pg_test]
    fn pgtest_num_make_pgrange_inf_inf() {
        let desc =
            Spi::get_one::<String>("SELECT make_pgrange_num(NULL::numeric,NULL::numeric)::text")
                .expect("failed to get SPI result");
        assert_eq!(desc, "(,)");
    }

    #[pg_test]
    fn pgtest_num_make_pgrange_empty() {
        let desc = Spi::get_one::<String>("SELECT make_pgrange_num(1.0,1.0)::text")
            .expect("failed to get SPI result");
        assert_eq!(desc, "empty");
    }
}
