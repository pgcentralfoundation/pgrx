#[allow(dead_code)]
pub fn elog(level: u32, message: &str) {
    use std::ffi::CString;
    use std::os::raw::c_char;

    unsafe {
        extern "C" {
            fn pg_rs_bridge_elog(level: i32, message: *const c_char);
        }

        match CString::new(message) {
            Ok(s) => crate::guard(|| pg_rs_bridge_elog(level as i32, s.as_ptr())),
            Err(_) => crate::guard(|| {
                pg_rs_bridge_elog(
                    level as i32,
                    b"log message was null\0" as *const c_char,
                )
            }),
        }
    }
}

#[macro_export]
macro_rules! debug5 {
    ($($arg:tt)*) => (
        $crate::log::elog($crate::pg_sys::DEBUG5, format!($($arg)*).as_str());
    )
}

#[macro_export]
macro_rules! debug4 {
    ($($arg:tt)*) => (
        $crate::log::elog($crate::pg_sys::DEBUG4, format!($($arg)*).as_str());
    )
}

#[macro_export]
macro_rules! debug3 {
    ($($arg:tt)*) => (
        $crate::log::elog($crate::pg_sys::DEBUG3, format!($($arg)*).as_str());
    )
}

#[macro_export]
macro_rules! debug2 {
    ($($arg:tt)*) => (
        $crate::log::elog($crate::pg_sys::DEBUG2, format!($($arg)*).as_str());
    )
}

#[macro_export]
macro_rules! debug1 {
    ($($arg:tt)*) => (
        $crate::log::elog($crate::pg_sys::DEBUG1, format!($($arg)*).as_str());
    )
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => (
        $crate::log::elog($crate::pg_sys::LOG, format!($($arg)*).as_str());
    )
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => (
        $crate::log::elog($crate::pg_sys::INFO, format!($($arg)*).as_str());
    )
}

#[macro_export]
macro_rules! notice {
    ($($arg:tt)*) => (
        $crate::log::elog($crate::pg_sys::NOTICE, format!($($arg)*).as_str());
    )
}

#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => (
        $crate::log::elog($crate::pg_sys::WARNING, format!($($arg)*).as_str());
    )
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => (
        { $crate::log::elog($crate::pg_sys::ERROR, format!($($arg)*).as_str()); unreachable!("elog failed"); }
    )
}

#[macro_export]
macro_rules! FATAL {
    ($($arg:tt)*) => (
        { $crate::log::elog($crate::pg_sys::FATAL, format!($($arg)*).as_str()); unreachable!("elog failed"); }
    )
}

#[macro_export]
macro_rules! PANIC {
    ($($arg:tt)*) => (
        { $crate::log::elog($crate::pg_sys::PANIC, format!($($arg)*).as_str()); unreachable!("elog failed"); }
    )
}

#[macro_export]
macro_rules! check_for_interrupts {
    () => {
        #[cfg(any(feature = "pg10", feature = "pg11"))]
        unsafe {
            if $crate::pg_sys::InterruptPending {
                $crate::pg_sys::ProcessInterrupts();
            }
        }

        #[cfg(feature = "pg12")]
        unsafe {
            if $crate::pg_sys::InterruptPending != 0 {
                $crate::pg_sys::ProcessInterrupts();
            }
        }
    };
}
