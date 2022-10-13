/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

#[cfg(any(test, feature = "pg_test"))]
#[pgx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;

    use pgx::prelude::*;
    use pgx::{info, register_xact_callback, PgXactCallbackEvent};

    #[test]
    fn make_idea_happy() {}

    #[pg_test]
    fn test_xact_callback() {
        register_xact_callback(PgXactCallbackEvent::Abort, || info!("TESTMSG: Called on abort"));
    }
}
