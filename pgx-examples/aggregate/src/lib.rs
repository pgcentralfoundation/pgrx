use pgx::*;
use serde::{Serialize, Deserialize};

pg_module_magic!(); // Uncomment this outside of docs!

#[derive(Copy, Clone, Default, PostgresType, Serialize, Deserialize)]
pub struct DemoSumState {
    count: i32,
}

pub struct DemoSum;

#[pg_aggregate]
impl Aggregate for DemoSum {
    const INITIAL_CONDITION: Option<&'static str> = Some(r#"{ "count": 0 }"#);
    type Args = i32;
    type State = DemoSumState;
    fn state(
        mut current: Self::State,
        arg: Self::Args,
        _fcinfo: pg_sys::FunctionCallInfo
    ) -> Self::State {
        todo!()
    }
}