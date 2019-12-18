#![allow(dead_code)]

use std::ffi::CString;
use std::os::raw::c_char;

pub static DEBUG5: i32 = 10;
pub static DEBUG4: i32 = 11;
pub static DEBUG3: i32 = 12;
pub static DEBUG2: i32 = 13;
pub static DEBUG1: i32 = 14;
pub static LOG: i32 = 15;
pub static LOG_SERVER_ONLY: i32 = 16;
pub static COMMERROR: i32 = LOG_SERVER_ONLY;
pub static INFO: i32 = 17;
pub static NOTICE: i32 = 18;
pub static WARNING: i32 = 19;
pub static ERROR: i32 = 20;
pub static PGERROR: i32 = 20;
pub static FATAL: i32 = 21;
pub static PANIC: i32 = 22;

pub fn elog(level: i32, message: &str) {
    unsafe {
        extern "C" {
            fn zdb_log_proxy(level: i32, message: *const c_char);
        }

        match CString::new(message) {
            Ok(s) => crate::guard(|| zdb_log_proxy(level, s.as_ptr())),
            Err(_) => crate::guard(|| {
                zdb_log_proxy(level, b"log message was null\0".as_ptr() as *const c_char)
            }),
        }
    }
}

fn check_for_interrupts() {}

#[macro_export]
macro_rules! debug5 {
    ($($arg:tt)*) => (
        $crate::log::elog($crate::log::DEBUG5, format!($($arg)*).as_str());
    )
}

#[macro_export]
macro_rules! debug4 {
    ($($arg:tt)*) => (
        $crate::log::elog($crate::log::DEBUG4, format!($($arg)*).as_str());
    )
}

#[macro_export]
macro_rules! debug3 {
    ($($arg:tt)*) => (
        $crate::log::elog($crate::log::DEBUG3, format!($($arg)*).as_str());
    )
}

#[macro_export]
macro_rules! debug2 {
    ($($arg:tt)*) => (
        $crate::log::elog($crate::log::DEBUG2, format!($($arg)*).as_str());
    )
}

#[macro_export]
macro_rules! debug1 {
    ($($arg:tt)*) => (
        $crate::log::elog($crate::log::DEBUG1, format!($($arg)*).as_str());
    )
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => (
        $crate::log::elog($crate::log::LOG, format!($($arg)*).as_str());
    )
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => (
        $crate::log::elog($crate::log::INFO, format!($($arg)*).as_str());
    )
}

#[macro_export]
macro_rules! notice {
    ($($arg:tt)*) => (
        $crate::log::elog($crate::log::NOTICE, format!($($arg)*).as_str());
    )
}

#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => (
        $crate::log::elog($crate::log::WARNING, format!($($arg)*).as_str());
    )
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => (
        { $crate::log::elog($crate::log::ERROR, format!($($arg)*).as_str()); unreachable!("elog failed"); }
    )
}

#[macro_export]
macro_rules! FATAL {
    ($($arg:tt)*) => (
        { $crate::log::elog($crate::log::FATAL, format!($($arg)*).as_str()); unreachable!("elog failed"); }
    )
}

#[macro_export]
macro_rules! PANIC {
    ($($arg:tt)*) => (
        { $crate::log::elog($crate::log::PANIC, format!($($arg)*).as_str()); unreachable!("elog failed"); }
    )
}

#[macro_export]
macro_rules! check_for_interrupts {
    () => {
        unsafe {
            use pg_bridge;
            if pg_bridge::externs::InterruptPending {
                pg_bridge::externs::ProcessInterrupts();
            }
        }
    };
}
