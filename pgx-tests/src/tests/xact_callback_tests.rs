// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.


#[cfg(any(test, feature = "pg_test"))]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;

    use pgx::*;

    #[test]
    fn make_idea_happy() {}

    #[pg_test]
    fn test_xact_callback() {
        register_xact_callback(PgXactCallbackEvent::Abort, || {
            info!("TESTMSG: Called on abort")
        });
    }
}
