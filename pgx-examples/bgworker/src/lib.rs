use pgx::bgworkers::*;
use pgx::*;
use std::time::Duration;

/*
    In order to use this bgworker with pgx, you'll need to edit the proper `postgresql.conf` file in
    `~/.pgx/data-PGVER/postgresql.conf` and add this line to the end:

    ```
    session_preload_libraries = 'bgworker.so'
    ```

    Background workers **must** be initialized in the extension's `_PG_init()` function, and can **only**
    be started if loaded through the `session_preload_libraries` configuration setting.

    Executing `cargo pgx run <PGVER>` will, when it restarts the specified Postgres instance, also start
    this background worker
*/

pg_module_magic!();

#[pg_guard]
pub extern "C" fn _PG_init() {
    BackgroundWorkerBuilder::new("Background Worker Example")
        .set_function("background_worker_main")
        .set_library("bgworker")
        .set_argument(42i32.into_datum())
        .enable_spi_access()
        .load();
}

#[pg_guard]
pub extern "C" fn background_worker_main(arg: pg_sys::Datum) {
    let arg = unsafe { i32::from_datum(arg, false, pg_sys::INT4OID) };

    // these are the signals we want to receive.  If we don't attach the SIGTERM handler, then
    // we'll never be able to exit via an external notification
    BackgroundWorker::attach_signal_handlers(SignalWakeFlags::SIGHUP | SignalWakeFlags::SIGTERM);

    // we want to be able to use SPI against the specified table (postgres), as the user postgres
    BackgroundWorker::connect_worker_to_spi(Some("postgres"), Some("postgres"));

    log!(
        "Hello from inside the {} BGWorker!  Argument value={}",
        BackgroundWorker::get_name(),
        arg.unwrap()
    );

    // wake up every 10s or if we received a SIGTERM
    while BackgroundWorker::wait_latch(Some(Duration::from_secs(10))) {
        if BackgroundWorker::sighup_received() {
            // on SIGHUP, you might want to reload some external configuration or something
        }

        // within a transaction, execute an SQL statement, and log its results
        BackgroundWorker::transaction(|| {
            Spi::execute(|client| {
                let tuple_table = client.select(
                    "SELECT 'Hi', id, ''||a FROM (SELECT id, 42 from generate_series(1,10) id) a ",
                    None,
                    None,
                );
                tuple_table.for_each(|tuple| {
                    let a = tuple.get_datum::<String>(1).unwrap();
                    let b = tuple.get_datum::<i32>(2).unwrap();
                    let c = tuple.get_datum::<String>(3).unwrap();
                    log!("from bgworker: ({}, {}, {})", a, b, c);
                });
            });
        });
    }

    log!(
        "Goodbye from inside the {} BGWorker! ",
        BackgroundWorker::get_name()
    );
}
