// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

//! Safely create Postgres Background Workers, including with full SPI support
//!
//! See: [https://www.postgresql.org/docs/12/bgworker.html](https://www.postgresql.org/docs/12/bgworker.html)
use crate::pg_sys;
use std::convert::TryInto;
use std::ffi::CStr;
use std::ffi::CString;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

pub static mut PREV_SHMEM_STARTUP_HOOK: Option<unsafe extern "C" fn()> = None;
static GOT_SIGHUP: AtomicBool = AtomicBool::new(false);
static GOT_SIGTERM: AtomicBool = AtomicBool::new(false);

bitflags! {
    struct BGWflags: i32 {
        const BGWORKER_SHMEM_ACCESS                = pg_sys::BGWORKER_SHMEM_ACCESS as i32;
        const BGWORKER_BACKEND_DATABASE_CONNECTION = pg_sys::BGWORKER_BACKEND_DATABASE_CONNECTION as i32;
    }
}

bitflags! {
    /// Flags to indicate when a BackgroundWorker should be awaken
    pub struct SignalWakeFlags: i32 {
        const SIGHUP = 0x1;
        const SIGTERM = 0x2;
    }
}

bitflags! {
    struct WLflags: i32 {
        const WL_LATCH_SET         = pg_sys::WL_LATCH_SET as i32;
        const WL_SOCKET_READABLE   = pg_sys::WL_SOCKET_READABLE as i32;
        const WL_SOCKET_WRITEABLE  = pg_sys::WL_SOCKET_WRITEABLE as i32;
        const WL_TIMEOUT           = pg_sys::WL_TIMEOUT as i32;
        const WL_POSTMASTER_DEATH  = pg_sys::WL_POSTMASTER_DEATH as i32;
        const WL_SOCKET_CONNECTED  = pg_sys::WL_SOCKET_WRITEABLE as i32;
        const WL_SOCKET_MASK       = (pg_sys::WL_SOCKET_READABLE | pg_sys::WL_SOCKET_WRITEABLE | pg_sys::WL_SOCKET_CONNECTED) as i32;
        #[cfg(any(feature = "pg12"))]
        const WL_EXIT_ON_PM_DEATH  = pg_sys::WL_EXIT_ON_PM_DEATH  as i32;

    }
}

/// The various points in which a BackgroundWorker can be started by Postgres
pub enum BgWorkerStartTime {
    PostmasterStart = pg_sys::BgWorkerStartTime_BgWorkerStart_PostmasterStart as isize,
    ConsistentState = pg_sys::BgWorkerStartTime_BgWorkerStart_ConsistentState as isize,
    RecoveryFinished = pg_sys::BgWorkerStartTime_BgWorkerStart_RecoveryFinished as isize,
}

/// Static interface into a running Background Worker
///
/// It also provides a few helper functions as wrappers around the global `pgx::pg_sys::MyBgworkerEntry`
pub struct BackgroundWorker {}

impl BackgroundWorker {
    /// What is our name?
    pub fn get_name() -> &'static str {
        #[cfg(feature = "pg10")]
        const LEN: usize = 64;
        #[cfg(any(feature = "pg11", feature = "pg12", feature = "pg13"))]
        const LEN: usize = 96;

        unsafe {
            CStr::from_ptr(std::mem::transmute::<&[i8; LEN], *const i8>(
                &(*pg_sys::MyBgworkerEntry).bgw_name,
            ))
        }
        .to_str()
        .expect("should not have non UTF8")
    }

    /// Retrieve the `extra` data provided to the `BackgroundWorkerBuilder`
    pub fn get_extra() -> &'static str {
        const LEN: usize = 128;

        unsafe {
            CStr::from_ptr(std::mem::transmute::<&[i8; LEN], *const i8>(
                &(*pg_sys::MyBgworkerEntry).bgw_extra,
            ))
        }
        .to_str()
        .expect("'extra' is not valid UTF8")
    }

    /// Have we received a SIGUP?
    pub fn sighup_received() -> bool {
        // toggle the bool to false, returning whatever it was
        GOT_SIGHUP.swap(false, Ordering::SeqCst)
    }

    /// Have we received a SIGTERM?
    pub fn sigterm_received() -> bool {
        // toggle the bool to false, returning whatever it was
        GOT_SIGTERM.swap(false, Ordering::SeqCst)
    }

    /// Wait for the specified amount of time on the background worker's latch
    ///
    /// Returns true if we're still supposed to be alive and haven't received a SIGTERM
    pub fn wait_latch(timeout: Option<Duration>) -> bool {
        match timeout {
            Some(t) => wait_latch(
                t.as_millis().try_into().unwrap(),
                WLflags::WL_LATCH_SET | WLflags::WL_TIMEOUT | WLflags::WL_POSTMASTER_DEATH,
            ),
            None => wait_latch(0, WLflags::WL_LATCH_SET | WLflags::WL_POSTMASTER_DEATH),
        };
        !BackgroundWorker::sigterm_received()
    }

    /// Is this `BackgroundWorker` allowed to continue?
    pub fn worker_continue() -> bool {
        pg_sys::WL_POSTMASTER_DEATH as i32 != 0
    }

    /// Intended to be called once to indicate the database and user to use to
    /// connect to via SPI
    pub fn connect_worker_to_spi(dbname: Option<&str>, username: Option<&str>) {
        let db = dbname.and_then(|rs| CString::new(rs).ok());
        let db: *const i8 = db.as_ref().map_or(std::ptr::null(), |i| i.as_ptr());

        let user = username.and_then(|rs| CString::new(rs).ok());
        let user: *const i8 = user.as_ref().map_or(std::ptr::null(), |i| i.as_ptr());

        unsafe {
            #[cfg(feature = "pg10")]
            pg_sys::BackgroundWorkerInitializeConnection(db as *mut i8, user as *mut i8);

            #[cfg(any(feature = "pg11", feature = "pg12", feature = "pg13"))]
            pg_sys::BackgroundWorkerInitializeConnection(db, user, 0);
        };
    }

    /// Indicate the set of signal handlers we want to receive.
    ///
    /// You likely always want to do this:
    ///
    /// ```rust,no_run
    /// use pgx::bgworkers::{BackgroundWorker, SignalWakeFlags};
    /// BackgroundWorker::attach_signal_handlers(SignalWakeFlags::SIGHUP | SignalWakeFlags::SIGTERM);
    /// ```
    pub fn attach_signal_handlers(wake: SignalWakeFlags) {
        unsafe {
            if wake.contains(SignalWakeFlags::SIGHUP) {
                pg_sys::pqsignal(pg_sys::SIGHUP as i32, Some(worker_spi_sighup));
            }
            if wake.contains(SignalWakeFlags::SIGTERM) {
                pg_sys::pqsignal(pg_sys::SIGTERM as i32, Some(worker_spi_sigterm));
            }
            pg_sys::BackgroundWorkerUnblockSignals();
        }
    }

    /// Once connected to SPI via `connect_worker_to_spi()`, begin a transaction to
    /// use the `pgx::Spi` interface.
    pub fn transaction<F: FnOnce() + std::panic::UnwindSafe + std::panic::RefUnwindSafe>(
        transaction_body: F,
    ) {
        unsafe {
            pg_sys::SetCurrentStatementStartTimestamp();
            pg_sys::StartTransactionCommand();
            pg_sys::PushActiveSnapshot(pg_sys::GetTransactionSnapshot());
        }
        pg_sys::guard(|| transaction_body());
        unsafe {
            pg_sys::PopActiveSnapshot();
            pg_sys::CommitTransactionCommand();
        }
    }
}

unsafe extern "C" fn worker_spi_sighup(_signal_args: i32) {
    GOT_SIGHUP.store(true, Ordering::SeqCst);
    pg_sys::ProcessConfigFile(pg_sys::GucContext_PGC_SIGHUP);
    pg_sys::SetLatch(pg_sys::MyLatch);
}

unsafe extern "C" fn worker_spi_sigterm(_signal_args: i32) {
    GOT_SIGTERM.store(true, Ordering::SeqCst);
    pg_sys::SetLatch(pg_sys::MyLatch);
}

/// A builder-style interface for creating a new Background Worker
///
/// This must be used from within your extension's `_PG_init()` function,
/// finishing with the `.load()` function.
///
/// ## Example
///
/// ```rust,no_run
/// use pgx::bgworkers::BackgroundWorkerBuilder;
/// use pgx::*;
///
/// pg_module_magic!();
///
/// #[pg_guard]
/// pub extern "C" fn _PG_init() {
///     BackgroundWorkerBuilder::new("My Example BGWorker")
///         .set_function("background_worker_main")
///         .set_library("example")
///         .enable_spi_access()
///         .load();
/// }
///
/// #[pg_guard]
/// pub extern "C" fn background_worker_main(_arg: pg_sys::Datum) {
///     // do bgworker stuff here
/// }
/// ```
pub struct BackgroundWorkerBuilder {
    bgw_name: String,
    bgw_type: String,
    bgw_flags: BGWflags,
    bgw_start_time: BgWorkerStartTime,
    bgw_restart_time: Option<Duration>,
    bgw_library_name: String,
    bgw_function_name: String,
    bgw_main_arg: pg_sys::Datum,
    bgw_extra: String,
    bgw_notify_pid: pg_sys::pid_t,
    shared_memory_startup_fn: Option<unsafe extern "C" fn()>,
}

impl BackgroundWorkerBuilder {
    /// Construct a new BackgroundWorker of the specified name
    ///
    /// By default, its `type` is also set to the specified name
    /// and it is configured to
    ///     - start at `BgWorkerStartTime::PostmasterStart`.
    ///     - never restart in the event it crashes
    pub fn new(name: &str) -> BackgroundWorkerBuilder {
        BackgroundWorkerBuilder {
            bgw_name: name.to_string(),
            bgw_type: name.to_string(),
            bgw_flags: BGWflags::empty(),
            bgw_start_time: BgWorkerStartTime::PostmasterStart,
            bgw_restart_time: None,
            bgw_library_name: name.to_string(),
            bgw_function_name: name.to_string(),
            bgw_main_arg: 0,
            bgw_extra: "".to_string(),
            bgw_notify_pid: 0,
            shared_memory_startup_fn: None,
        }
    }

    /// What is the type of this BackgroundWorker
    pub fn set_type(mut self: Self, input: &str) -> Self {
        self.bgw_type = input.to_string();
        self
    }

    /// Does this BackgroundWorker want Shared Memory access?
    pub fn enable_shmem_access(mut self: Self, startup: Option<unsafe extern "C" fn()>) -> Self {
        self.bgw_flags = self.bgw_flags | BGWflags::BGWORKER_SHMEM_ACCESS;
        self.shared_memory_startup_fn = startup;
        self
    }

    /// Does this BackgroundWorker intend to use SPI?
    ///
    /// If set, then the configured start time becomes `BgWorkerStartTIme::RecoveryFinished`
    /// as accessing SPI prior to possible database recovery is not possible
    pub fn enable_spi_access(mut self: Self) -> Self {
        self.bgw_flags = self.bgw_flags
            | BGWflags::BGWORKER_SHMEM_ACCESS
            | BGWflags::BGWORKER_BACKEND_DATABASE_CONNECTION;
        self.bgw_start_time = BgWorkerStartTime::RecoveryFinished;
        self
    }

    /// When should this BackgroundWorker be started by Postgres?
    pub fn set_start_time(mut self: Self, input: BgWorkerStartTime) -> Self {
        self.bgw_start_time = input;
        self
    }

    /// the interval, in seconds, that postgres should wait before restarting the process,
    /// in case it crashes. It can be `Some(any positive duration value), or
    /// `None`, indicating not to restart the process in case of a crash.
    pub fn set_restart_time(mut self: Self, input: Option<Duration>) -> Self {
        self.bgw_restart_time = input;
        self
    }

    /// What is the library name that contains the "main" function?
    ///
    /// Typically, this will just be your extension's name
    pub fn set_library(mut self: Self, input: &str) -> Self {
        self.bgw_library_name = input.to_string();
        self
    }

    /// What is the "main" function that should be run when the BackgroundWorker
    /// process is started?  
    ///
    /// The specified function **must** be:
    ///     - `extern "C"`,
    ///     - guarded with `#[pg_guard]`,
    ///     - take 1 argument of type `pgx::pg_sys::Datum`, and
    ///     - return "void"
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use pgx::*;
    ///
    /// #[pg_guard]
    /// pub extern "C" fn background_worker_main(_arg: pg_sys::Datum) {
    /// }
    /// ```
    pub fn set_function(mut self: Self, input: &str) -> Self {
        self.bgw_function_name = input.to_string();
        self
    }

    /// Datum argument to the background worker main function. This main function should
    /// take a single argument of type Datum and return void. bgw_main_arg will be passed
    /// as the argument. In addition, the global variable MyBgworkerEntry points to a copy
    /// of the BackgroundWorker structure passed at registration time; the worker may find
    /// it helpful to examine this structure.
    ///
    /// On Windows (and anywhere else where EXEC_BACKEND is defined) or in dynamic
    /// background workers it is not safe to pass a Datum by reference, only by value.
    /// If an argument is required, it is safest to pass an int32 or other small value and
    /// use that as an index into an array allocated in shared memory.
    ///
    /// ## Important
    /// If a value like a cstring or text is passed then the pointer won't be valid from
    /// the new background worker process.
    ///
    /// In general, this means that you should stick to primitive Rust types such as `i32`,
    /// `bool`, etc.
    ///
    /// You you use `pgx`'s `IntoDatum` trait to make the conversion into a datum easy:
    ///
    /// ```rust,no_run
    /// use pgx::bgworkers::BackgroundWorkerBuilder;
    /// use pgx::IntoDatum;
    /// BackgroundWorkerBuilder::new("Example")
    ///     .set_function("background_worker_main")
    ///     .set_library("example")
    ///     .set_argument(42i32.into_datum())
    ///     .load();
    /// ```
    pub fn set_argument(mut self: Self, input: Option<pg_sys::Datum>) -> Self {
        self.bgw_main_arg = input.unwrap_or(0);
        self
    }

    /// extra data to be passed to the background worker. Unlike bgw_main_arg, this
    /// data is not passed as an argument to the worker's main function, but it can be
    /// accessed via the `BackgroundWorker` struct.
    pub fn set_extra(mut self: Self, input: &str) -> Self {
        self.bgw_extra = input.to_string();
        self
    }

    /// PID of a PostgreSQL backend process to which the postmaster should send SIGUSR1
    /// when the process is started or exits. It should be 0 for workers registered at
    /// postmaster startup time, or when the backend registering the worker does not wish
    /// to wait for the worker to start up. Otherwise, it should be initialized to
    /// `pgx::pg_sys::MyProcPid`
    pub fn set_notify_pid(mut self: Self, input: i32) -> Self {
        self.bgw_notify_pid = input;
        self
    }

    /// Once properly configured, call `load()` to get the BackgroundWorker registered and
    /// started at the proper time by Postgres.
    pub fn load(self: Self) {
        #[cfg(feature = "pg10")]
        let mut bgw = pg_sys::BackgroundWorker {
            bgw_name: RpgffiChar::from(&self.bgw_name[..]).0,
            bgw_flags: self.bgw_flags.bits(),
            bgw_start_time: self.bgw_start_time as u32,
            bgw_restart_time: match self.bgw_restart_time {
                None => pg_sys::BGW_NEVER_RESTART,
                Some(d) => d.as_secs() as i32,
            },
            bgw_library_name: RpgffiChar::from(&self.bgw_library_name[..]).0,
            bgw_function_name: RpgffiChar::from(&self.bgw_function_name[..]).0,
            bgw_main_arg: self.bgw_main_arg,
            bgw_extra: RpgffiChar128::from(&self.bgw_extra[..]).0,
            bgw_notify_pid: self.bgw_notify_pid,
        };

        #[cfg(any(feature = "pg11", feature = "pg12", feature = "pg13"))]
        let mut bgw = pg_sys::BackgroundWorker {
            bgw_name: RpgffiChar::from(&self.bgw_name[..]).0,
            bgw_type: RpgffiChar::from(&self.bgw_type[..]).0,
            bgw_flags: self.bgw_flags.bits(),
            bgw_start_time: self.bgw_start_time as u32,
            bgw_restart_time: match self.bgw_restart_time {
                None => -1,
                Some(d) => d.as_secs() as i32,
            },
            bgw_library_name: RpgffiChar::from(&self.bgw_library_name[..]).0,
            bgw_function_name: RpgffiChar::from(&self.bgw_function_name[..]).0,
            bgw_main_arg: self.bgw_main_arg,
            bgw_extra: RpgffiChar128::from(&self.bgw_extra[..]).0,
            bgw_notify_pid: self.bgw_notify_pid,
        };

        unsafe {
            pg_sys::RegisterBackgroundWorker(&mut bgw);
            if self.bgw_flags.contains(BGWflags::BGWORKER_SHMEM_ACCESS)
                && self.shared_memory_startup_fn.is_some()
            {
                PREV_SHMEM_STARTUP_HOOK = pg_sys::shmem_startup_hook;
                pg_sys::shmem_startup_hook = self.shared_memory_startup_fn;
            }
        };
    }
}

fn wait_latch(timeout: i64, wakeup_flags: WLflags) -> i32 {
    unsafe {
        let latch = pg_sys::WaitLatch(
            pg_sys::MyLatch,
            wakeup_flags.bits(),
            timeout,
            pg_sys::PG_WAIT_EXTENSION,
        );
        pg_sys::ResetLatch(pg_sys::MyLatch);
        check_for_interrupts!();

        latch
    }
}

#[cfg(feature = "pg10")]
type RpgffiChar = RpgffiChar64;

#[cfg(any(feature = "pg11", feature = "pg12", feature = "pg13"))]
type RpgffiChar = RpgffiChar96;

struct RpgffiChar64([i8; 64]);

impl<'a> From<&'a str> for RpgffiChar64 {
    fn from(string: &str) -> Self {
        let mut r = [0; 64];
        for (dest, src) in r.iter_mut().zip(string.as_bytes()) {
            *dest = *src as i8;
        }
        RpgffiChar64(r)
    }
}

struct RpgffiChar96([i8; 96]);

impl<'a> From<&'a str> for RpgffiChar96 {
    fn from(string: &str) -> Self {
        let mut r = [0; 96];
        for (dest, src) in r.iter_mut().zip(string.as_bytes()) {
            *dest = *src as i8;
        }
        RpgffiChar96(r)
    }
}

struct RpgffiChar128([i8; 128]);

impl<'a> From<&'a str> for RpgffiChar128 {
    fn from(string: &str) -> Self {
        let mut r = [0; 128];
        for (dest, src) in r.iter_mut().zip(string.as_bytes()) {
            *dest = *src as i8;
        }
        RpgffiChar128(r)
    }
}
