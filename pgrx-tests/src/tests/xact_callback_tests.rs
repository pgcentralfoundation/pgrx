#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgrx_tests;

    use pgrx::prelude::*;
    use pgrx::{info, register_xact_callback, PgXactCallbackEvent};

    #[test]
    fn make_idea_happy() {}

    #[pg_test]
    fn test_xact_callback() {
        register_xact_callback(PgXactCallbackEvent::Abort, || info!("TESTMSG: Called on abort"));
    }
}
