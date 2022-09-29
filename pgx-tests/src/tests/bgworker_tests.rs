/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
use pgx::prelude::*;
use pgx::{pg_guard, pg_sys, FromDatum, IntoDatum, PgOid, Spi};

#[pg_guard]
#[no_mangle]
pub extern "C" fn bgworker(arg: pg_sys::Datum) {
    use pgx::bgworkers::*;
    use std::time::Duration;
    BackgroundWorker::attach_signal_handlers(SignalWakeFlags::SIGHUP | SignalWakeFlags::SIGTERM);
    BackgroundWorker::connect_worker_to_spi(
        Some(crate::framework::get_pg_dbname()),
        Some(crate::framework::get_pg_user().as_str()),
    );

    let arg = unsafe { i32::from_datum(arg, false, pg_sys::INT4OID) }.expect("invalid arg");

    BackgroundWorker::transaction(|| {
        Spi::run("CREATE TABLE tests.bgworker_test (v INTEGER);");
        Spi::execute(|mut client| {
            client.update(
                "INSERT INTO tests.bgworker_test VALUES ($1);",
                None,
                Some(vec![(
                    PgOid::BuiltIn(PgBuiltInOids::INT4OID),
                    arg.into_datum(),
                )]),
            );
        });
    });
    while BackgroundWorker::wait_latch(Some(Duration::from_millis(100))) {}
    BackgroundWorker::transaction(|| {
        Spi::run("UPDATE tests.bgworker_test SET v = v + 1;");
    });
}

#[cfg(any(test, feature = "pg_test"))]
#[pgx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;

    use pgx::bgworkers::*;
    use pgx::prelude::*;
    use pgx::{pg_guard, pg_sys, IntoDatum};

    #[pg_test]
    fn test_dynamic_bgworker() {
        let worker = BackgroundWorkerBuilder::new("dynamic_bgworker")
            .set_library("pgx_tests")
            .set_function("bgworker")
            .set_argument(123i32.into_datum())
            .enable_spi_access()
            .set_notify_pid(unsafe { pg_sys::MyProcPid })
            .load_dynamic();
        let pid = worker.wait_for_startup().expect("no PID from the worker");
        assert!(pid > 0);
        let handle = worker.terminate();
        handle.wait_for_shutdown().expect("aborted shutdown");

        assert_eq!(
            124,
            Spi::get_one::<i32>("SELECT v FROM tests.bgworker_test;")
                .expect("no return value from the worker")
        );
    }
}
