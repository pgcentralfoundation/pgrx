use crate::{Internal, PgBox};

pub trait Aggregate where Self: Sized {
    type Arg;
    type Finalize;
    type MovingState;

    const PARALLEL: Option<ParallelOption> = None;
    const INITIAL_CONDITION: Option<&'static str> = None;
    const MOVING_INITIAL_CONDITION: Option<&'static str> = None;
    const HYPOTHETICAL: bool = false;

    fn state(&self, v: Self::Arg) -> Self;

    fn finalize(&self) -> Self::Finalize;

    fn combine(&self, _other: Self) -> Self;
    
    fn serial(&self) -> Vec<u8>;

    fn deserial(&self, _buf: Vec<u8>, _internal: PgBox<Self>) -> PgBox<Self>;

    fn moving_state(_mstate: Self::MovingState, _v: Self::Arg) -> Self::MovingState;

    fn moving_finalize(_mstate: Self::MovingState) -> Self::Finalize;

}

pub enum ParallelOption {
    Safe,
    Restricted,
    Unsafe,
}