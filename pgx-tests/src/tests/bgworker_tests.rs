/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
use pgx::prelude::*;
use pgx::{FromDatum, IntoDatum, PgOid};

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

    let arg = unsafe { i32::from_datum(arg, false) }.expect("invalid arg");

    if arg > 0 {
        BackgroundWorker::transaction(|| {
            Spi::run("CREATE TABLE tests.bgworker_test (v INTEGER);");
            Spi::execute(|mut client| {
                client.update(
                    "INSERT INTO tests.bgworker_test VALUES ($1);",
                    None,
                    Some(vec![(PgOid::BuiltIn(PgBuiltInOids::INT4OID), arg.into_datum())]),
                );
            });
        });
    }
    while BackgroundWorker::wait_latch(Some(Duration::from_millis(100))) {}
    if arg > 0 {
        BackgroundWorker::transaction(|| {
            Spi::run("UPDATE tests.bgworker_test SET v = v + 1;");
        });
    }
}

#[pg_guard]
#[no_mangle]
/// Here we test that `BackgroundWorker::transaction` can return data from the closure
pub extern "C" fn bgworker_return_value(arg: pg_sys::Datum) {
    use pgx::bgworkers::*;
    use std::time::Duration;
    BackgroundWorker::attach_signal_handlers(SignalWakeFlags::SIGHUP | SignalWakeFlags::SIGTERM);
    BackgroundWorker::connect_worker_to_spi(
        Some(crate::framework::get_pg_dbname()),
        Some(crate::framework::get_pg_user().as_str()),
    );

    let arg = unsafe { i32::from_datum(arg, false) }.expect("invalid arg");

    let val = if arg > 0 {
        BackgroundWorker::transaction(|| {
            Spi::run("CREATE TABLE tests.bgworker_test_return (v INTEGER);");
            Spi::get_one_with_args::<i32>(
                "SELECT $1",
                vec![(PgOid::BuiltIn(PgBuiltInOids::INT4OID), arg.into_datum())],
            )
        })
        .unwrap()
    } else {
        0
    };
    while BackgroundWorker::wait_latch(Some(Duration::from_millis(100))) {}
    BackgroundWorker::transaction(|| {
        Spi::execute(|mut c| {
            c.update(
                "INSERT INTO tests.bgworker_test_return VALUES ($1)",
                None,
                Some(vec![(PgOid::BuiltIn(PgBuiltInOids::INT4OID), val.into_datum())]),
            );
        })
    })
}

#[cfg(any(test, feature = "pg_test"))]
#[pgx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;

    use pgx::bgworkers::*;
    use pgx::prelude::*;
    use pgx::{pg_sys, IntoDatum};

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

    #[pg_test]
    fn test_dynamic_bgworker_untracked() {
        let worker = BackgroundWorkerBuilder::new("dynamic_bgworker")
            .set_library("pgx_tests")
            .set_function("bgworker")
            .set_argument(0i32.into_datum())
            .enable_spi_access()
            .load_dynamic();
        assert!(matches!(worker.wait_for_startup(), Err(BackgroundWorkerStatus::Untracked { .. })));
        assert!(matches!(
            worker.wait_for_shutdown(),
            Err(BackgroundWorkerStatus::Untracked { .. })
        ));
    }

    #[pg_test]
    fn test_dynamic_bgworker_untracked_termination_handle() {
        let worker = BackgroundWorkerBuilder::new("dynamic_bgworker")
            .set_library("pgx_tests")
            .set_function("bgworker")
            .set_argument(0i32.into_datum())
            .enable_spi_access()
            .load_dynamic();
        let handle = worker.terminate();
        assert!(matches!(
            handle.wait_for_shutdown(),
            Err(BackgroundWorkerStatus::Untracked { .. })
        ));
    }

    #[pg_test]
    fn test_background_worker_transaction_return() {
        let worker = BackgroundWorkerBuilder::new("dynamic_bgworker")
            .set_library("pgx_tests")
            .set_function("bgworker_return_value")
            .set_argument(123i32.into_datum())
            .enable_spi_access()
            .set_notify_pid(unsafe { pg_sys::MyProcPid })
            .load_dynamic();
        let pid = worker.wait_for_startup().expect("no PID from the worker");
        assert!(pid > 0);
        let handle = worker.terminate();
        handle.wait_for_shutdown().expect("aborted shutdown");

        assert_eq!(
            123,
            Spi::get_one::<i32>("SELECT v FROM tests.bgworker_test_return;")
                .expect("no return value from the worker")
        );
    }
}
