use pgx::*;
use serde::{Serialize, Deserialize};
use std::collections::HashSet;

#[derive(Copy, Clone, Default, Debug, PostgresType, Serialize, Deserialize)]
pub struct DemoSum {
    count: i32,
}

#[pg_aggregate]
impl Aggregate for DemoSum {
    const NAME: &'static str = "demo_sum";
    const PARALLEL: Option<ParallelOption> = Some(pgx::aggregate::ParallelOption::Unsafe);
    const INITIAL_CONDITION: Option<&'static str> = Some(r#"0"#);
    const MOVING_INITIAL_CONDITION: Option<&'static str> = Some(r#"0"#);

    type Args = i32;
    type State = i32;
    type MovingState = i32;

    fn state(
        mut current: Self::State,
        arg: Self::Args,
        _fcinfo: pg_sys::FunctionCallInfo,
    ) -> Self::State {
        current += arg;
        current
    }

    fn moving_state(
        current: Self::State,
        arg: Self::Args,
        fcinfo: pg_sys::FunctionCallInfo,
    ) -> Self::MovingState {
        Self::state(current, arg, fcinfo)
    }

    fn moving_state_inverse(
        mut current: Self::State,
        arg: Self::Args,
        _fcinfo: pg_sys::FunctionCallInfo,
    ) -> Self::MovingState {
        current -= arg;
        current
    }

    fn combine(
        mut first: Self::State,
        second: Self::State,
        _fcinfo: pg_sys::FunctionCallInfo,
    ) -> Self::State {
        first += second;
        first
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct DemoUnique;

#[pg_aggregate]
impl Aggregate for DemoUnique {
    type Args = &'static str;
    type State = Internal;
    type Finalize = i32;

    fn state(
        mut current: Self::State,
        arg: Self::Args,
        _fcinfo: pg_sys::FunctionCallInfo,
    ) -> Self::State {
        let inner = unsafe { current.get_or_insert_default::<HashSet<String>>() };

        inner.insert(arg.to_string());
        current
    }

    fn combine(
        mut first: Self::State,
        mut second: Self::State,
        _fcinfo: pg_sys::FunctionCallInfo
    ) -> Self::State {
        let first_inner = unsafe { first.get_or_insert_default::<HashSet<String>>() };
        let second_inner = unsafe { second.get_or_insert_default::<HashSet<String>>() };

        let unioned: HashSet<_> = first_inner.union(second_inner).collect();
        Internal::new(unioned)
    }

    fn finalize(
        mut current: Self::State,
        _direct_args: Self::OrderedSetArgs,
        _fcinfo: pg_sys::FunctionCallInfo
    ) -> Self::Finalize {
        let inner = unsafe { current.get_or_insert_default::<HashSet<String>>() };

        inner.len() as i32
    }
}

#[derive(Copy, Clone, Default, Debug, PostgresType, Serialize, Deserialize)]
pub struct DemoPercentileDisc;

#[pg_aggregate]
impl Aggregate for DemoPercentileDisc {
    type Args = name!(input, i32);
    type State = Internal;
    type Finalize = i32;
    const ORDERED_SET: bool = true;
    type OrderedSetArgs = name!(percentile, f64);

    fn state(
        mut current: Self::State,
        arg: Self::Args,
        _fcinfo: pg_sys::FunctionCallInfo,
    ) -> Self::State {
        let inner = unsafe { current.get_or_insert_default::<Vec<i32>>() };

        inner.push(arg);
        current
    }

    fn finalize(
        mut current: Self::State,
        direct_arg: Self::OrderedSetArgs,
        _fcinfo: pg_sys::FunctionCallInfo,
    ) -> Self::Finalize {
        let inner = unsafe { current.get_or_insert_default::<Vec<i32>>() };
        // This isn't done for us.
        inner.sort();

        let target_index = (inner.len() as f64 * direct_arg).round() as usize;
        inner[target_index.saturating_sub(1)]
    }
}

#[cfg(any(test, feature = "pg_test"))]
#[pgx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;
    use pgx::*;

    
    #[pg_test]
    fn aggregate_demo_sum() {
        let retval = Spi::get_one::<i32>(
            "SELECT demo_sum(value) FROM UNNEST(ARRAY [1, 1, 2]) as value;"
        ).expect("SQL select failed");
        assert_eq!(retval, 4);

        // Moving-aggregate mode
        let retval = Spi::get_one::<Vec<i32>>("
            SELECT array_agg(calculated) FROM (
                SELECT demo_sum(value) OVER (
                    ROWS BETWEEN 1 PRECEDING AND CURRENT ROW
                ) as calculated FROM UNNEST(ARRAY [1, 20, 300, 4000]) as value
            ) as results;
        ").expect("SQL select failed");
        assert_eq!(retval, vec![1, 21, 320, 4300]);
    }

    #[pg_test]
    fn aggregate_demo_unique() {
        let retval = Spi::get_one::<i32>(
            "SELECT DemoUnique(value) FROM UNNEST(ARRAY ['a', 'a', 'b']) as value;"
        ).expect("SQL select failed");
        assert_eq!(retval, 2);
    }

    #[pg_test]
    fn aggregate_demo_percentile_disc() {
        // Example from https://www.postgresql.org/docs/current/xaggr.html#XAGGR-ORDERED-SET-AGGREGATES
        let retval = Spi::get_one::<i32>(
            "SELECT DemoPercentileDisc(0.5) WITHIN GROUP (ORDER BY income) FROM UNNEST(ARRAY [6000, 70000, 500]) as income;"
        ).expect("SQL select failed");
        assert_eq!(retval, 6000);

        let retval = Spi::get_one::<i32>(
            "SELECT DemoPercentileDisc(0.05) WITHIN GROUP (ORDER BY income) FROM UNNEST(ARRAY [5, 100000000, 6000, 70000, 500]) as income;"
        ).expect("SQL select failed");
        assert_eq!(retval, 5);
    }
}