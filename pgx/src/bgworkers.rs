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
    pub struct BGWflags: i32 {
        const BGWORKER_SHMEM_ACCESS                = pg_sys::BGWORKER_SHMEM_ACCESS as i32;
        const BGWORKER_BACKEND_DATABASE_CONNECTION = pg_sys::BGWORKER_BACKEND_DATABASE_CONNECTION as i32;
    }
}

bitflags! {
    pub struct SignalWakeFlags: i32 {
        const SIGHUP = 0x1;
        const SIGTERM = 0x2;
    }
}

bitflags! {
    pub struct WLflags: i32 {
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

pub enum BgWorkerStartTime {
    PostmasterStart = 0,
    ConsistentState = 1,
    RecoveryFinished = 2,
}

pub struct BackgroundWorker {}

impl BackgroundWorker {
    pub fn get_name() -> &'static str {
        unsafe {
            CStr::from_ptr(std::mem::transmute::<&[i8; 96], *const i8>(
                &(*pg_sys::MyBgworkerEntry).bgw_name,
            ))
        }
        .to_str()
        .expect("should not have non UTF8")
    }

    pub fn sighup_received() -> bool {
        // toggle the bool to false, returning whatever it was
        GOT_SIGHUP.swap(false, Ordering::SeqCst)
    }

    pub fn sigterm_received() -> bool {
        // toggle the bool to false, returning whatever it was
        GOT_SIGTERM.swap(false, Ordering::SeqCst)
    }

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

    pub fn worker_continue() -> bool {
        pg_sys::WL_POSTMASTER_DEATH as i32 != 0
    }

    pub fn connect_worker_to_spi(dbname: Option<&str>, username: Option<&str>) {
        let db = dbname.and_then(|rs| CString::new(rs).ok());
        let db: *const i8 = db.as_ref().map_or(std::ptr::null(), |i| i.as_ptr());

        let user = username.and_then(|rs| CString::new(rs).ok());
        let user: *const i8 = user.as_ref().map_or(std::ptr::null(), |i| i.as_ptr());

        unsafe {
            pg_sys::BackgroundWorkerInitializeConnection(db, user, 0);
        };
    }

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

#[allow(non_snake_case)]
pub unsafe extern "C" fn worker_spi_sighup(_signal_args: i32) {
    GOT_SIGHUP.store(true, Ordering::SeqCst);
    pg_sys::ProcessConfigFile(pg_sys::GucContext_PGC_SIGHUP);
    pg_sys::SetLatch(pg_sys::MyLatch);
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn worker_spi_sigterm(_signal_args: i32) {
    GOT_SIGTERM.store(true, Ordering::SeqCst);
    pg_sys::SetLatch(pg_sys::MyLatch);
}

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
    pub fn new(name: &str) -> BackgroundWorkerBuilder {
        BackgroundWorkerBuilder {
            bgw_name: name.to_string(),
            bgw_type: name.to_string(),
            bgw_flags: BGWflags::empty(),
            bgw_start_time: BgWorkerStartTime::PostmasterStart,
            bgw_restart_time: Some(Duration::from_secs(10)),
            bgw_library_name: name.to_string(),
            bgw_function_name: name.to_string(),
            bgw_main_arg: 0,
            bgw_extra: "".to_string(),
            bgw_notify_pid: 0,
            shared_memory_startup_fn: None,
        }
    }

    pub fn set_name(mut self: Self, input: &str) -> Self {
        self.bgw_name = input.to_string();
        self
    }

    pub fn set_type(mut self: Self, input: &str) -> Self {
        self.bgw_type = input.to_string();
        self
    }

    pub fn enable_shmem_access(mut self: Self, startup: Option<unsafe extern "C" fn()>) -> Self {
        self.bgw_flags = self.bgw_flags | BGWflags::BGWORKER_SHMEM_ACCESS;
        self.shared_memory_startup_fn = startup;
        self
    }

    pub fn enable_spi_access(mut self: Self) -> Self {
        self.bgw_flags = self.bgw_flags
            | BGWflags::BGWORKER_SHMEM_ACCESS
            | BGWflags::BGWORKER_BACKEND_DATABASE_CONNECTION;
        self.bgw_start_time = BgWorkerStartTime::RecoveryFinished;
        self
    }

    pub fn set_start_time(mut self: Self, input: BgWorkerStartTime) -> Self {
        self.bgw_start_time = input;
        self
    }

    pub fn set_restart_time(mut self: Self, input: Duration) -> Self {
        self.bgw_restart_time = Some(input);
        self
    }

    pub fn set_library(mut self: Self, input: &str) -> Self {
        self.bgw_library_name = input.to_string();
        self
    }

    pub fn set_function(mut self: Self, input: &str) -> Self {
        self.bgw_function_name = input.to_string();
        self
    }

    pub fn set_argument(mut self: Self, input: pg_sys::Datum) -> Self {
        self.bgw_main_arg = input;
        self
    }

    pub fn set_extra(mut self: Self, input: &str) -> Self {
        self.bgw_extra = input.to_string();
        self
    }

    pub fn set_notify_pid(mut self: Self, input: i32) -> Self {
        self.bgw_notify_pid = input;
        self
    }

    pub fn load(self: Self) {
        let mut bgw = pg_sys::BackgroundWorker {
            bgw_name: RpgffiChar96::from(&self.bgw_name[..]).0,
            bgw_type: RpgffiChar96::from(&self.bgw_type[..]).0,
            bgw_flags: self.bgw_flags.bits(),
            bgw_start_time: self.bgw_start_time as u32,
            bgw_restart_time: match self.bgw_restart_time {
                None => -1,
                Some(d) => d.as_secs() as i32,
            },
            bgw_library_name: RpgffiChar96::from(&self.bgw_library_name[..]).0,
            bgw_function_name: RpgffiChar96::from(&self.bgw_function_name[..]).0,
            /*
            bgw_function_name: RpgffiChar96::from(
                &format!("{}_wrapper", &self.bgw_function_name)[..],
            )
            */
            .0,
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

pub struct RpgffiChar96([i8; 96]);

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
