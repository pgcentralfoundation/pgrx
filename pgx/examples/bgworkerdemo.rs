use pgx::bgworkers::*;
use pgx::*;
use std::time::Duration;

pg_module_magic!();

#[allow(non_snake_case)]
#[pg_guard]
pub extern "C" fn _PG_init() {
    BackgroundWorkerBuilder::new("BGWorker demo")
        .set_function("my_bgw_init")
        .set_library("libbgworkerdemo")
        .enable_spi_access()
        .load();
}

#[allow(non_snake_case)]
#[pg_guard]
pub extern "C" fn my_bgw_init() {
    // These could be hidden with a proc macro on the function. If so you'd need to use the
    // extra field of the bgworker to pass the user/database settings. Maybe not worth it
    BackgroundWorker::attach_signal_handlers(SignalWakeFlags::SIGHUP | SignalWakeFlags::SIGTERM);
    BackgroundWorker::connect_worker_to_spi(Some("postgres"), Some("postgres"));

    log!(
        "Hello from inside the {} BGWorker! ",
        BackgroundWorker::get_name()
    );

    while BackgroundWorker::wait_latch(Some(Duration::from_secs(10))) {
        if BackgroundWorker::sighup_received() {
            //Do sighuppy stuff
        }
        BackgroundWorker::transaction(|| {
            Spi::execute(|client| {
                let tuple_table = client.select(
                    "SELECT 'Hi', id, ''||a FROM (SELECT id, 42 from generate_series(1,10) id) a ",
                    None,
                    None,
                );
                tuple_table.for_each(|tuple| {
                    tuple.get_datum::<String>(1).unwrap();
                    tuple.get_datum::<i32>(2).unwrap();
                    tuple.get_datum::<String>(3).unwrap();
                });
            });
        });
    }

    log!(
        "Goodbye from inside the {} BGWorker! ",
        BackgroundWorker::get_name()
    );
}
