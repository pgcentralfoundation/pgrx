


pub trait Aggregate where Self: Sized {
    type Arg;
    type Finalize;
    type MovingState;

    const PARALLEL: Option<ParallelOption> = None;
    const INITIAL_CONDITION: Option<&str> = None;
    const MOVING_INITIAL_CONDITION: Option<&str> = None;
    const HYPOTHETICAL: bool = false;

    fn state(&self, v: Self::Arg) -> Self;

    fn finalize(&self) -> Self::Finalize {
        unimplemented!("pgx stub, define in impls")
    }

    fn combine(&self, other: Self) -> Self {
        unimplemented!("pgx stub, define in impls")
    }
    
    fn serial(&self) -> Vec<u8> {
        unimplemented!("pgx stub, define in impls")
    }

    fn deserial(&self, buf: Vec<u8>, internal: Internal<Self>) -> Internal<Self> {
        unimplemented!("pgx stub, define in impls")
    }

    fn moving_state(mstate: Self::MovingState, v: Self::Arg) -> Self::MovingState {
        unimplemented!("pgx stub, define in impls")
    }

    fn moving_finalize(mstate: Self::MovingState) -> Self::Finalize {
        unimplemented!("pgx stub, define in impls")
    }

}

pub enum ParallelOption {
    Safe,
    Restricted,
    Unsafe,
}