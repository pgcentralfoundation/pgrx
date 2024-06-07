use pgrx::aggregate::*;
use pgrx::prelude::*;

const DOG_COMPOSITE_TYPE: &str = "Dog";

struct SumScritches {}

#[pg_aggregate]
impl Aggregate for SumScritches {
    type State = i32;
    const INITIAL_CONDITION: Option<&'static str> = Some("0");
    type Args = pgrx::name!(value, pgrx::composite_type!('static, "Dog"));

    fn state(
        current: Self::State,
        arg: Self::Args,
        _fcinfo: pg_sys::FunctionCallInfo,
    ) -> Self::State {
        let arg_scritches: i32 = arg
            .get_by_name("scritches")
            .unwrap() // Unwrap the result of the conversion
            .unwrap_or_default(); // The number of scritches, or 0 if there was none set
        current + arg_scritches
    }
}

/*
Create sum the scritches received by dogs, roughly the equivalent of:

```sql
CREATE FUNCTION scritch_collector_state(state Dog, new integer)
    RETURNS Dog
    LANGUAGE SQL
    STRICT
    RETURN ROW(state.name, state.scritches + new)::Dog;

CREATE AGGREGATE scritch_collector ("value" integer) (
    SFUNC = "sum_scritches_state",
    STYPE = Dog,
)
```
*/
struct ScritchCollector;

#[pg_aggregate]
impl Aggregate for ScritchCollector {
    type State = Option<pgrx::composite_type!('static, "Dog")>;
    type Args = i32;

    fn state(
        current: Self::State,
        arg: Self::Args,
        _fcinfo: pg_sys::FunctionCallInfo,
    ) -> Self::State {
        let mut current = match current {
            Some(v) => v,
            None => PgHeapTuple::new_composite_type(DOG_COMPOSITE_TYPE).unwrap(),
        };
        let current_scritches: i32 = current.get_by_name("scritches").unwrap().unwrap_or_default();
        current.set_by_name("scritches", current_scritches + arg).unwrap();
        Some(current)
    }
}

fn main() {}
