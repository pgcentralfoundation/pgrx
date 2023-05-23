// If this code doesn't generate a syntax error in the generated SQL then PR #1134 is working as expected
use pgrx::{prelude::*, Internal};

pub struct Foo;

#[pg_aggregate]
impl Aggregate for Foo {
    const NAME: &'static str = "foo";
    const ORDERED_SET: bool = true;

    type OrderedSetArgs = (name!(a, f64), name!(b, f64));

    type State = Internal;
    type Args = f64;
    type Finalize = f64;

    fn state(
        state: Self::State,
        _value: Self::Args,
        _fcinfo: pg_sys::FunctionCallInfo,
    ) -> Self::State {
        // FIXME create and maintain real state here
        state
    }

    fn finalize(
        _state: Self::State,
        _dontcare: Self::OrderedSetArgs,
        _fcinfo: pg_sys::FunctionCallInfo,
    ) -> Self::Finalize {
        0.0
    }
}
