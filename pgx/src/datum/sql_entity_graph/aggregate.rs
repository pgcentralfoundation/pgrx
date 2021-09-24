

use crate::Internal;

pub trait Aggregate where Self: Sized {
    type Arg;
    type Finalize;
    type MovingState;

    const PARALLEL: Option<ParallelOption> = None;
    const INITIAL_CONDITION: Option<&'static str> = None;
    const MOVING_INITIAL_CONDITION: Option<&'static str> = None;
    const HYPOTHETICAL: bool = false;

    fn state(&self, v: Self::Arg) -> Self;

    fn finalize(&self) -> Self::Finalize {
        unimplemented!("pgx stub, define in impls")
    }

    fn combine(&self, _other: Self) -> Self {
        unimplemented!("pgx stub, define in impls")
    }
    
    fn serial(&self) -> Vec<u8> {
        unimplemented!("pgx stub, define in impls")
    }

    fn deserial(&self, _buf: Vec<u8>, _internal: Internal<Self>) -> Internal<Self> {
        unimplemented!("pgx stub, define in impls")
    }

    fn moving_state(_mstate: Self::MovingState, _v: Self::Arg) -> Self::MovingState {
        unimplemented!("pgx stub, define in impls")
    }

    fn moving_finalize(_mstate: Self::MovingState) -> Self::Finalize {
        unimplemented!("pgx stub, define in impls")
    }

}

pub enum ParallelOption {
    Safe,
    Restricted,
    Unsafe,
}