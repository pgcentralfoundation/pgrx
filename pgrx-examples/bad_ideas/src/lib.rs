/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
use pgrx::prelude::*;
use pgrx::{check_for_interrupts, info, register_xact_callback, PgRelation, PgXactCallbackEvent};
use std::fs::File;
use std::io::Write;
use std::panic::catch_unwind;
use std::process::Command;

pgrx::pg_module_magic!();

#[pg_extern]
fn panic(s: &str) -> bool {
    catch_unwind(|| {
        PANIC!("{}", s);
    })
    .ok();
    true
}

#[pg_extern]
fn fatal(s: &str) -> bool {
    catch_unwind(|| {
        FATAL!("{}", s);
    })
    .ok();
    true
}

#[pg_extern]
fn error(s: &str) -> bool {
    catch_unwind(|| {
        error!("{}", s);
    })
    .ok();
    true
}

#[pg_extern]
fn warning(s: &str) -> bool {
    catch_unwind(|| {
        warning!("{}", s);
    })
    .ok();
    true
}

#[pg_extern]
fn exec<'a>(
    command: &'a str,
    args: default!(Vec<Option<&'a str>>, "ARRAY[]::text[]"),
) -> TableIterator<'static, (name!(status, Option<i32>), name!(stdout, String))> {
    let mut command = &mut Command::new(command);

    for arg in args {
        if let Some(arg) = arg {
            command = command.arg(arg);
        }
    }

    let output = command.output().expect("command failed");

    if !output.stderr.is_empty() {
        panic!("{}", String::from_utf8(output.stderr).expect("stderr is not valid utf8"))
    }

    TableIterator::once((
        output.status.code(),
        String::from_utf8(output.stdout).expect("stdout is not valid utf8"),
    ))
}

#[pg_extern]
fn write_file(filename: &str, bytes: &[u8]) -> i64 {
    let mut f = File::create(filename).expect("unable to create file");
    f.write_all(bytes).expect("unable to write bytes to file");
    bytes.len() as i64
}

#[pg_extern]
fn http(url: &str) -> String {
    let response = ureq::Agent::new().get(url).call().expect("invalid http response");

    response.into_string().expect("invalid string from response")
}

#[pg_extern]
fn loop_forever() {
    loop {
        check_for_interrupts!();
    }
}

#[pg_extern]
fn random_abort() {
    register_xact_callback(PgXactCallbackEvent::PreCommit, || {
        info!("in xact callback pre-commit");

        if rand::random::<bool>() {
            panic!("aborting transaction");
        }
    });
}

#[pg_guard]
pub unsafe extern "C" fn _PG_init() {
    #[pg_guard]
    extern "C" fn random_abort_callback(event: pg_sys::XactEvent, _arg: *mut std::os::raw::c_void) {
        // info!("in global xact callback: event={}", event);

        if event == pg_sys::XactEvent_XACT_EVENT_PRE_COMMIT {
            if rand::random::<bool>() {
                // panic!("aborting transaction");
            }
        }
    }

    pg_sys::RegisterXactCallback(Some(random_abort_callback), std::ptr::null_mut());
}

/// with `no_guard` we're telling pgrx that we're positive this function
/// won't ever perform a Rust panic!
#[pg_extern(no_guard)]
fn crash_postgres() {
    // so when it does, it'll crash Postgres
    panic!("oh no!")
}

#[pg_extern]
fn drop_struct() {
    struct Foo;
    impl Drop for Foo {
        fn drop(&mut self) {
            info!("Foo was dropped")
        }
    }

    info!("before foo drop");
    {
        let _foo = Foo;
        // panic!("did foo drop anyways?");
        unsafe {
            PgRelation::open_with_name("table doesn't exist").expect("unable to open table");
        }
    }

    info!("after foo drop");
}
