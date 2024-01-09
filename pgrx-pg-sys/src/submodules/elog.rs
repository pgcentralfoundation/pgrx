//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
//! Access to Postgres' logging system

/// Postgres' various logging levels
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Ord, PartialOrd, PartialEq, Eq)]
pub enum PgLogLevel {
    /// Debugging messages, in categories of decreasing detail
    DEBUG5 = crate::DEBUG5 as isize,

    /// Debugging messages, in categories of decreasing detail
    DEBUG4 = crate::DEBUG4 as isize,

    /// Debugging messages, in categories of decreasing detail
    DEBUG3 = crate::DEBUG3 as isize,

    /// Debugging messages, in categories of decreasing detail
    DEBUG2 = crate::DEBUG2 as isize,

    /// Debugging messages, in categories of decreasing detail
    /// NOTE:  used by GUC debug_* variables
    DEBUG1 = crate::DEBUG1 as isize,

    /// Server operational messages; sent only to server log by default.
    LOG = crate::LOG as isize,

    /// Same as LOG for server reporting, but never sent to client.
    #[allow(non_camel_case_types)]
    LOG_SERVER_ONLY = crate::LOG_SERVER_ONLY as isize,

    /// Messages specifically requested by user (eg VACUUM VERBOSE output); always sent to client
    /// regardless of client_min_messages, but by default not sent to server log.
    INFO = crate::INFO as isize,

    /// Helpful messages to users about query operation; sent to client and not to server log by default.
    NOTICE = crate::NOTICE as isize,

    /// Warnings.  \[NOTICE\] is for expected messages like implicit sequence creation by SERIAL.
    /// \[WARNING\] is for unexpected messages.
    WARNING = crate::WARNING as isize,

    /// user error - abort transaction; return to known state
    ERROR = crate::ERROR as isize,

    /// fatal error - abort process
    FATAL = crate::FATAL as isize,

    /// take down the other backends with me
    PANIC = crate::PANIC as isize,
}

impl From<isize> for PgLogLevel {
    #[inline]
    fn from(i: isize) -> Self {
        if i == PgLogLevel::DEBUG5 as isize {
            PgLogLevel::DEBUG5
        } else if i == PgLogLevel::DEBUG4 as isize {
            PgLogLevel::DEBUG4
        } else if i == PgLogLevel::DEBUG3 as isize {
            PgLogLevel::DEBUG3
        } else if i == PgLogLevel::DEBUG2 as isize {
            PgLogLevel::DEBUG2
        } else if i == PgLogLevel::DEBUG1 as isize {
            PgLogLevel::DEBUG1
        } else if i == PgLogLevel::INFO as isize {
            PgLogLevel::INFO
        } else if i == PgLogLevel::NOTICE as isize {
            PgLogLevel::NOTICE
        } else if i == PgLogLevel::WARNING as isize {
            PgLogLevel::WARNING
        } else if i == PgLogLevel::ERROR as isize {
            PgLogLevel::ERROR
        } else if i == PgLogLevel::FATAL as isize {
            PgLogLevel::FATAL
        } else if i == PgLogLevel::PANIC as isize {
            PgLogLevel::PANIC
        } else {
            // ERROR seems like a good default
            PgLogLevel::ERROR
        }
    }
}

impl From<i32> for PgLogLevel {
    #[inline]
    fn from(i: i32) -> Self {
        (i as isize).into()
    }
}

/// Log to Postgres' `debug5` log level.
///
/// This macro accepts arguments like the [`println`] and [`format`] macros.
/// See [`fmt`](std::fmt) for information about options.
///
/// The output these logs goes to the PostgreSQL log file at `DEBUG5` level, depending on how the
/// [PostgreSQL settings](https://www.postgresql.org/docs/current/runtime-config-logging.html) are configured.
///
/// ## Adding Details and Hints
///
/// Additionally, you can use specify a `detail` and/or `hint` to be included with the report.
/// After the string literal to be used as the message and any formatting arguments, add a semicolon.
/// Then assign a string literal to `detail` or `hint`, optionally followed by a list of
/// comma separated formatting arguments. The `detail` and `hint` options can be used together or
/// separately but `detail` must come before `hint` and both must end with a semicolon.
///
/// ## Examples
///
/// ```rust,no_run
/// // just like `println!` and `format!`
/// debug5!("a simple message");
/// debug5!("or a formatted message: {:?}", "pgrx rocks!");
/// // include details and hints
/// debug5! {
///     "add details if you want";
///     detail = "...";
/// }
/// debug5! {
///     "or a message with just a hint";
///     hint = "...";
/// }
/// debug5! {
///     "put it all together for {} {}", "great", "success!";
///     detail = "extra {}", "info";
///     hint = "{} helpful", "very";
/// }
/// ```
#[macro_export]
macro_rules! debug5 {
    (
        $msg:literal $(, $msg_args:expr)*;
        $(detail = $detail:literal $(, $detail_args:expr)*)?;
        $(hint = $hint:literal $(, $hint_args:expr)*)?;
    ) => (
        {
            extern crate alloc;
            $crate::ereport!(
                $crate::elog::PgLogLevel::DEBUG5,
                $crate::errcodes::PgSqlErrorCode::ERRCODE_SUCCESSFUL_COMPLETION,
                alloc::format!($msg $(, $msg_args)*).as_str();
                $(detail = alloc::format!($detail $(, $detail_args)*).as_str();)?
                $(hint = alloc::format!($hint $(, $hint_args)*).as_str();)?
            );
        }
    );
    ($($arg:tt)*) => (
        {
            extern crate alloc;
            $crate::ereport!($crate::elog::PgLogLevel::DEBUG5, $crate::errcodes::PgSqlErrorCode::ERRCODE_SUCCESSFUL_COMPLETION, alloc::format!($($arg)*).as_str());
        }
    );
}

/// Log to Postgres' `debug4` log level.
///
/// This macro accepts arguments like the [`println`] and [`format`] macros.
/// See [`fmt`](std::fmt) for information about options.
///
/// The output these logs goes to the PostgreSQL log file at `DEBUG4` level, depending on how the
/// [PostgreSQL settings](https://www.postgresql.org/docs/current/runtime-config-logging.html) are configured.
///
/// ## Adding Details and Hints
///
/// Additionally, you can use specify a `detail` and/or `hint` to be included with the report.
/// After the string literal to be used as the message and any formatting arguments, add a semicolon.
/// Then assign a string literal to `detail` or `hint`, optionally followed by a list of
/// comma separated formatting arguments. The `detail` and `hint` options can be used together or
/// separately but `detail` must come before `hint` and both must end with a semicolon.
///
/// ## Examples
///
/// ```rust,no_run
/// // just like `println!` and `format!`
/// debug4!("a simple message");
/// debug4!("or a formatted message: {:?}", "pgrx rocks!");
/// // include details and hints
/// debug4! {
///     "add details if you want";
///     detail = "...";
/// }
/// debug4! {
///     "or a message with just a hint";
///     hint = "...";
/// }
/// debug4! {
///     "put it all together for {} {}", "great", "success!";
///     detail = "extra {}", "info";
///     hint = "{} helpful", "very";
/// }
/// ```
#[macro_export]
macro_rules! debug4 {
    (
        $msg:literal $(, $msg_args:expr)*;
        $(detail = $detail:literal $(, $detail_args:expr)*)?;
        $(hint = $hint:literal $(, $hint_args:expr)*)?;
    ) => (
        {
            extern crate alloc;
            $crate::ereport!(
                $crate::elog::PgLogLevel::DEBUG4,
                $crate::errcodes::PgSqlErrorCode::ERRCODE_SUCCESSFUL_COMPLETION,
                alloc::format!($msg $(, $msg_args)*).as_str();
                $(detail = alloc::format!($detail $(, $detail_args)*).as_str();)?
                $(hint = alloc::format!($hint $(, $hint_args)*).as_str();)?
            );
        }
    );
    ($($arg:tt)*) => (
        {
            extern crate alloc;
            $crate::ereport!($crate::elog::PgLogLevel::DEBUG4, $crate::errcodes::PgSqlErrorCode::ERRCODE_SUCCESSFUL_COMPLETION, alloc::format!($($arg)*).as_str());
        }
    );
}

/// Log to Postgres' `debug3` log level.
///
/// This macro accepts arguments like the [`println`] and [`format`] macros.
/// See [`fmt`](std::fmt) for information about options.
///
/// The output these logs goes to the PostgreSQL log file at `DEBUG3` level, depending on how the
/// [PostgreSQL settings](https://www.postgresql.org/docs/current/runtime-config-logging.html) are configured.
///
/// ## Adding Details and Hints
///
/// Additionally, you can use specify a `detail` and/or `hint` to be included with the report.
/// After the string literal to be used as the message and any formatting arguments, add a semicolon.
/// Then assign a string literal to `detail` or `hint`, optionally followed by a list of
/// comma separated formatting arguments. The `detail` and `hint` options can be used together or
/// separately but `detail` must come before `hint` and both must end with a semicolon.
///
/// ## Examples
///
/// ```rust,no_run
/// // just like `println!` and `format!`
/// debug3!("a simple message");
/// debug3!("or a formatted message: {:?}", "pgrx rocks!");
/// // include details and hints
/// debug3! {
///     "add details if you want";
///     detail = "...";
/// }
/// debug3! {
///     "or a message with just a hint";
///     hint = "...";
/// }
/// debug3! {
///     "put it all together for {} {}", "great", "success!";
///     detail = "extra {}", "info";
///     hint = "{} helpful", "very";
/// }
/// ```
#[macro_export]
macro_rules! debug3 {
    (
        $msg:literal $(, $msg_args:expr)*;
        $(detail = $detail:literal $(, $detail_args:expr)*)?;
        $(hint = $hint:literal $(, $hint_args:expr)*)?;
    ) => (
        {
            extern crate alloc;
            $crate::ereport!(
                $crate::elog::PgLogLevel::DEBUG3,
                $crate::errcodes::PgSqlErrorCode::ERRCODE_SUCCESSFUL_COMPLETION,
                alloc::format!($msg $(, $msg_args)*).as_str();
                $(detail = alloc::format!($detail $(, $detail_args)*).as_str();)?
                $(hint = alloc::format!($hint $(, $hint_args)*).as_str();)?
            );
        }
    );
    ($($arg:tt)*) => (
        {
            extern crate alloc;
            $crate::ereport!($crate::elog::PgLogLevel::DEBUG3, $crate::errcodes::PgSqlErrorCode::ERRCODE_SUCCESSFUL_COMPLETION, alloc::format!($($arg)*).as_str());
        }
    );
}

/// Log to Postgres' `debug2` log level.
///
/// This macro accepts arguments like the [`println`] and [`format`] macros.
/// See [`fmt`](std::fmt) for information about options.
///
/// The output these logs goes to the PostgreSQL log file at `DEBUG2` level, depending on how the
/// [PostgreSQL settings](https://www.postgresql.org/docs/current/runtime-config-logging.html) are configured.
///
/// ## Adding Details and Hints
///
/// Additionally, you can use specify a `detail` and/or `hint` to be included with the report.
/// After the string literal to be used as the message and any formatting arguments, add a semicolon.
/// Then assign a string literal to `detail` or `hint`, optionally followed by a list of
/// comma separated formatting arguments. The `detail` and `hint` options can be used together or
/// separately but `detail` must come before `hint` and both must end with a semicolon.
///
/// ## Examples
///
/// ```rust,no_run
/// // just like `println!` and `format!`
/// debug2!("a simple message");
/// debug2!("or a formatted message: {:?}", "pgrx rocks!");
/// // include details and hints
/// debug2! {
///     "add details if you want";
///     detail = "...";
/// }
/// debug2! {
///     "or a message with just a hint";
///     hint = "...";
/// }
/// debug2! {
///     "put it all together for {} {}", "great", "success!";
///     detail = "extra {}", "info";
///     hint = "{} helpful", "very";
/// }
/// ```
#[macro_export]
macro_rules! debug2 {
    (
        $msg:literal $(, $msg_args:expr)*;
        $(detail = $detail:literal $(, $detail_args:expr)*)?;
        $(hint = $hint:literal $(, $hint_args:expr)*)?;
    ) => (
        {
            extern crate alloc;
            $crate::ereport!(
                $crate::elog::PgLogLevel::DEBUG2,
                $crate::errcodes::PgSqlErrorCode::ERRCODE_SUCCESSFUL_COMPLETION,
                alloc::format!($msg $(, $msg_args)*).as_str();
                $(detail = alloc::format!($detail $(, $detail_args)*).as_str();)?
                $(hint = alloc::format!($hint $(, $hint_args)*).as_str();)?
            );
        }
    );
    ($($arg:tt)*) => (
        {
            extern crate alloc;
            $crate::ereport!($crate::elog::PgLogLevel::DEBUG2, $crate::errcodes::PgSqlErrorCode::ERRCODE_SUCCESSFUL_COMPLETION, alloc::format!($($arg)*).as_str());
        }
    );
}

/// Log to Postgres' `debug1` log level.
///
/// This macro accepts arguments like the [`println`] and [`format`] macros.
/// See [`fmt`](std::fmt) for information about options.
///
/// The output these logs goes to the PostgreSQL log file at `DEBUG1` level, depending on how the
/// [PostgreSQL settings](https://www.postgresql.org/docs/current/runtime-config-logging.html) are configured.
///
/// ## Adding Details and Hints
///
/// Additionally, you can use specify a `detail` and/or `hint` to be included with the report.
/// After the string literal to be used as the message and any formatting arguments, add a semicolon.
/// Then assign a string literal to `detail` or `hint`, optionally followed by a list of
/// comma separated formatting arguments. The `detail` and `hint` options can be used together or
/// separately but `detail` must come before `hint` and both must end with a semicolon.
///
/// ## Examples
///
/// ```rust,no_run
/// // just like `println!` and `format!`
/// debug1!("a simple message");
/// debug1!("or a formatted message: {:?}", "pgrx rocks!");
/// // include details and hints
/// debug1! {
///     "add details if you want";
///     detail = "...";
/// }
/// debug1! {
///     "or a message with just a hint";
///     hint = "...";
/// }
/// debug1! {
///     "put it all together for {} {}", "great", "success!";
///     detail = "extra {}", "info";
///     hint = "{} helpful", "very";
/// }
/// ```
#[macro_export]
macro_rules! debug1 {
    (
        $msg:literal $(, $msg_args:expr)*;
        $(detail = $detail:literal $(, $detail_args:expr)*)?;
        $(hint = $hint:literal $(, $hint_args:expr)*)?;
    ) => (
        {
            extern crate alloc;
            $crate::ereport!(
                $crate::elog::PgLogLevel::DEBUG1,
                $crate::errcodes::PgSqlErrorCode::ERRCODE_SUCCESSFUL_COMPLETION,
                alloc::format!($msg $(, $msg_args)*).as_str();
                $(detail = alloc::format!($detail $(, $detail_args)*).as_str();)?
                $(hint = alloc::format!($hint $(, $hint_args)*).as_str();)?
            );
        }
    );
    ($($arg:tt)*) => (
        {
            extern crate alloc;
            $crate::ereport!($crate::elog::PgLogLevel::DEBUG1, $crate::errcodes::PgSqlErrorCode::ERRCODE_SUCCESSFUL_COMPLETION, alloc::format!($($arg)*).as_str());
        }
    );
}

/// Log to Postgres' `log` log level.
///
/// This macro accepts arguments like the [`println`] and [`format`] macros.
/// See [`fmt`](std::fmt) for information about options.
///
/// The output these logs goes to the PostgreSQL log file at `LOG` level, depending on how the
/// [PostgreSQL settings](https://www.postgresql.org/docs/current/runtime-config-logging.html) are configured.
///
/// ## Adding Details and Hints
///
/// Additionally, you can use specify a `detail` and/or `hint` to be included with the report.
/// After the string literal to be used as the message and any formatting arguments, add a semicolon.
/// Then assign a string literal to `detail` or `hint`, optionally followed by a list of
/// comma separated formatting arguments. The `detail` and `hint` options can be used together or
/// separately but `detail` must come before `hint` and both must end with a semicolon.
///
/// ## Examples
///
/// ```rust,no_run
/// // just like `println!` and `format!`
/// log!("a simple message");
/// log!("or a formatted message: {:?}", "pgrx rocks!");
/// // include details and hints
/// log! {
///     "add details if you want";
///     detail = "...";
/// }
/// log! {
///     "or a message with just a hint";
///     hint = "...";
/// }
/// log! {
///     "put it all together for {} {}", "great", "success!";
///     detail = "extra {}", "info";
///     hint = "{} helpful", "very";
/// }
/// ```
#[macro_export]
macro_rules! log {
    (
        $msg:literal $(, $msg_args:expr)*;
        $(detail = $detail:literal $(, $detail_args:expr)*)?;
        $(hint = $hint:literal $(, $hint_args:expr)*)?;
    ) => (
        {
            extern crate alloc;
            $crate::ereport!(
                $crate::elog::PgLogLevel::LOG,
                $crate::errcodes::PgSqlErrorCode::ERRCODE_SUCCESSFUL_COMPLETION,
                alloc::format!($msg $(, $msg_args)*).as_str();
                $(detail = alloc::format!($detail $(, $detail_args)*).as_str();)?
                $(hint = alloc::format!($hint $(, $hint_args)*).as_str();)?
            );
        }
    );
    ($($arg:tt)*) => (
        {
            extern crate alloc;
            $crate::ereport!($crate::elog::PgLogLevel::LOG, $crate::errcodes::PgSqlErrorCode::ERRCODE_SUCCESSFUL_COMPLETION, alloc::format!($($arg)*).as_str());
        }
    );
}

/// Log to Postgres' `info` log level.
///
/// This macro accepts arguments like the [`println`] and [`format`] macros.
/// See [`fmt`](std::fmt) for information about options.
///
/// ## Adding Details and Hints
///
/// Additionally, you can use specify a `detail` and/or `hint` to be included with the report.
/// After the string literal to be used as the message and any formatting arguments, add a semicolon.
/// Then assign a string literal to `detail` or `hint`, optionally followed by a list of
/// comma separated formatting arguments. The `detail` and `hint` options can be used together or
/// separately but `detail` must come before `hint` and both must end with a semicolon.
///
/// ## Examples
///
/// ```rust,no_run
/// // just like `println!` and `format!`
/// info!("a simple message");
/// info!("or a formatted message: {:?}", "pgrx rocks!");
/// // include details and hints
/// info! {
///     "add details if you want";
///     detail = "...";
/// }
/// info! {
///     "or a message with just a hint";
///     hint = "...";
/// }
/// info! {
///     "put it all together for {} {}", "great", "success!";
///     detail = "extra {}", "info";
///     hint = "{} helpful", "very";
/// }
/// ```
#[macro_export]
macro_rules! info {
    (
        $msg:literal $(, $msg_args:expr)*;
        $(detail = $detail:literal $(, $detail_args:expr)*)?;
        $(hint = $hint:literal $(, $hint_args:expr)*)?;
    ) => (
        {
            extern crate alloc;
            $crate::ereport!(
                $crate::elog::PgLogLevel::INFO,
                $crate::errcodes::PgSqlErrorCode::ERRCODE_SUCCESSFUL_COMPLETION,
                alloc::format!($msg $(, $msg_args)*).as_str();
                $(detail = alloc::format!($detail $(, $detail_args)*).as_str();)?
                $(hint = alloc::format!($hint $(, $hint_args)*).as_str();)?
            );
        }
    );
    ($($arg:tt)*) => (
        {
            extern crate alloc;
            $crate::ereport!($crate::elog::PgLogLevel::INFO, $crate::errcodes::PgSqlErrorCode::ERRCODE_SUCCESSFUL_COMPLETION, alloc::format!($($arg)*).as_str());
        }
    );
}

/// Log to Postgres' `notice` log level.
///
/// This macro accepts arguments like the [`println`] and [`format`] macros.
/// See [`fmt`](std::fmt) for information about options.
///
/// ## Adding Details and Hints
///
/// Additionally, you can use specify a `detail` and/or `hint` to be included with the report.
/// After the string literal to be used as the message and any formatting arguments, add a semicolon.
/// Then assign a string literal to `detail` or `hint`, optionally followed by a list of
/// comma separated formatting arguments. The `detail` and `hint` options can be used together or
/// separately but `detail` must come before `hint` and both must end with a semicolon.
///
/// ## Examples
///
/// ```rust,no_run
/// // just like `println!` and `format!`
/// notice!("a simple message");
/// notice!("or a formatted message: {:?}", "pgrx rocks!");
/// // include details and hints
/// notice! {
///     "add details if you want";
///     detail = "...";
/// }
/// notice! {
///     "or a message with just a hint";
///     hint = "...";
/// }
/// notice! {
///     "put it all together for {} {}", "great", "success!";
///     detail = "extra {}", "info";
///     hint = "{} helpful", "very";
/// }
/// ```
#[macro_export]
macro_rules! notice {
    (
        $msg:literal $(, $msg_args:expr)*;
        $(detail = $detail:literal $(, $detail_args:expr)*)?;
        $(hint = $hint:literal $(, $hint_args:expr)*)?;
    ) => (
        {
            extern crate alloc;
            $crate::ereport!(
                $crate::elog::PgLogLevel::NOTICE,
                $crate::errcodes::PgSqlErrorCode::ERRCODE_SUCCESSFUL_COMPLETION,
                alloc::format!($msg $(, $msg_args)*).as_str();
                $(detail = alloc::format!($detail $(, $detail_args)*).as_str();)?
                $(hint = alloc::format!($hint $(, $hint_args)*).as_str();)?
            );
        }
    );
    ($($arg:tt)*) => (
        {
            extern crate alloc;
            $crate::ereport!($crate::elog::PgLogLevel::NOTICE, $crate::errcodes::PgSqlErrorCode::ERRCODE_SUCCESSFUL_COMPLETION, alloc::format!($($arg)*).as_str());
        }
    );
}

/// Log to Postgres' `warning` log level.
///
/// This macro accepts arguments like the [`println`] and [`format`] macros.
/// See [`fmt`](std::fmt) for information about options.
///
/// ## Adding Details and Hints
///
/// Additionally, you can use specify a `detail` and/or `hint` to be included with the report.
/// After the string literal to be used as the message and any formatting arguments, add a semicolon.
/// Then assign a string literal to `detail` or `hint`, optionally followed by a list of
/// comma separated formatting arguments. The `detail` and `hint` options can be used together or
/// separately but `detail` must come before `hint` and both must end with a semicolon.
///
/// ## Examples
///
/// ```rust,no_run
/// // just like `println!` and `format!`
/// warning!("a simple message");
/// warning!("or a formatted message: {:?}", "pgrx rocks!");
/// // include details and hints
/// warning! {
///     "add details if you want";
///     detail = "...";
/// }
/// warning! {
///     "or a message with just a hint";
///     hint = "...";
/// }
/// warning! {
///     "put it all together for {} {}", "great", "success!";
///     detail = "extra {}", "info";
///     hint = "{} helpful", "very";
/// }
/// ```
#[macro_export]
macro_rules! warning {
    (
        $msg:literal $(, $msg_args:expr)*;
        $(detail = $detail:literal $(, $detail_args:expr)*)?;
        $(hint = $hint:literal $(, $hint_args:expr)*)?;
    ) => (
        {
            extern crate alloc;
            $crate::ereport!(
                $crate::elog::PgLogLevel::WARNING,
                $crate::errcodes::PgSqlErrorCode::ERRCODE_WARNING,
                alloc::format!($msg $(, $msg_args)*).as_str();
                $(detail = alloc::format!($detail $(, $detail_args)*).as_str();)?
                $(hint = alloc::format!($hint $(, $hint_args)*).as_str();)?
            );
        }
    );
    ($($arg:tt)*) => (
        {
            extern crate alloc;
            $crate::ereport!($crate::elog::PgLogLevel::WARNING, $crate::errcodes::PgSqlErrorCode::ERRCODE_WARNING, alloc::format!($($arg)*).as_str());
        }
    );
}

/// Log to Postgres' `error` log level.  This will abort the current Postgres transaction.
///
/// This macro accepts arguments like the [`println`] and [`format`] macros.
/// See [`fmt`](std::fmt) for information about options.
///
/// ## Adding Details and Hints
///
/// Additionally, you can use specify a `detail` and/or `hint` to be included with the report.
/// After the string literal to be used as the message and any formatting arguments, add a semicolon.
/// Then assign a string literal to `detail` or `hint`, optionally followed by a list of
/// comma separated formatting arguments. The `detail` and `hint` options can be used together or
/// separately but `detail` must come before `hint` and both must end with a semicolon.
///
/// ## Examples
///
/// ```rust,no_run
/// // just like `println!` and `format!`
/// error!("a simple message");
/// error!("or a formatted message: {:?}", "pgrx rocks!");
/// // include details and hints
/// error! {
///     "add details if you want";
///     detail = "...";
/// }
/// error! {
///     "or a message with just a hint";
///     hint = "...";
/// }
/// error! {
///     "put it all together for {} {}", "great", "success!";
///     detail = "extra {}", "info";
///     hint = "{} helpful", "very";
/// }
/// ```
#[macro_export]
macro_rules! error {
    (
        $msg:literal $(, $msg_args:expr)*;
        $(detail = $detail:literal $(, $detail_args:expr)*)?;
        $(hint = $hint:literal $(, $hint_args:expr)*)?;
    ) => (
        {
            extern crate alloc;
            $crate::ereport!(
                $crate::elog::PgLogLevel::ERROR,
                $crate::errcodes::PgSqlErrorCode::ERRCODE_INTERNAL_ERROR,
                alloc::format!($msg $(, $msg_args)*).as_str();
                $(detail = alloc::format!($detail $(, $detail_args)*).as_str();)?
                $(hint = alloc::format!($hint $(, $hint_args)*).as_str();)?
            );
            unreachable!()
        }
    );
    ($($arg:tt)*) => (
        {
            extern crate alloc;
            $crate::ereport!($crate::elog::PgLogLevel::ERROR, $crate::errcodes::PgSqlErrorCode::ERRCODE_INTERNAL_ERROR, alloc::format!($($arg)*).as_str());
            unreachable!()
        }
    );
}

/// Log to Postgres' `fatal` log level.  This will abort the current Postgres backend connection process.
///
/// This macro accepts arguments like the [`println`] and [`format`] macros.
/// See [`fmt`](std::fmt) for information about options.
///
/// ## Adding Details and Hints
///
/// Additionally, you can use specify a `detail` and/or `hint` to be included with the report.
/// After the string literal to be used as the message and any formatting arguments, add a semicolon.
/// Then assign a string literal to `detail` or `hint`, optionally followed by a list of
/// comma separated formatting arguments. The `detail` and `hint` options can be used together or
/// separately but `detail` must come before `hint` and both must end with a semicolon.
///
/// ## Examples
///
/// ```rust,no_run
/// // just like `println!` and `format!`
/// FATAL!("a simple message");
/// FATAL!("or a formatted message: {:?}", "pgrx rocks!");
/// // include details and hints
/// FATAL! {
///     "add details if you want";
///     detail = "...";
/// }
/// FATAL! {
///     "or a message with just a hint";
///     hint = "...";
/// }
/// FATAL! {
///     "put it all together for {} {}", "great", "success!";
///     detail = "extra {}", "info";
///     hint = "{} helpful", "very";
/// }
/// ```
#[allow(non_snake_case)]
#[macro_export]
macro_rules! FATAL {
    (
        $msg:literal $(, $msg_args:expr)*;
        $(detail = $detail:literal $(, $detail_args:expr)*)?;
        $(hint = $hint:literal $(, $hint_args:expr)*)?;
    ) => (
        {
            extern crate alloc;
            $crate::ereport!(
                $crate::elog::PgLogLevel::FATAL,
                $crate::errcodes::PgSqlErrorCode::ERRCODE_INTERNAL_ERROR,
                alloc::format!($msg $(, $msg_args)*).as_str();
                $(detail = alloc::format!($detail $(, $detail_args)*).as_str();)?
                $(hint = alloc::format!($hint $(, $hint_args)*).as_str();)?
            );
            unreachable!()
        }
    );
    ($($arg:tt)*) => (
        {
            extern crate alloc;
            $crate::ereport!($crate::elog::PgLogLevel::FATAL, $crate::errcodes::PgSqlErrorCode::ERRCODE_INTERNAL_ERROR, alloc::format!($($arg)*).as_str());
            unreachable!()
        }
    );
}

/// Log to Postgres' `panic` log level.  This will cause the entire Postgres cluster to crash.
///
/// This macro accepts arguments like the [`println`] and [`format`] macros.
/// See [`fmt`](std::fmt) for information about options.
///
/// ## Adding Details and Hints
///
/// Additionally, you can use specify a `detail` and/or `hint` to be included with the report.
/// After the string literal to be used as the message and any formatting arguments, add a semicolon.
/// Then assign a string literal to `detail` or `hint`, optionally followed by a list of
/// comma separated formatting arguments. The `detail` and `hint` options can be used together or
/// separately but `detail` must come before `hint` and both must end with a semicolon.
///
/// ## Examples
///
/// ```rust,no_run
/// // just like `println!` and `format!`
/// PANIC!("a simple message");
/// PANIC!("or a formatted message: {:?}", "pgrx rocks!");
/// // include details and hints
/// PANIC! {
///     "add details if you want";
///     detail = "...";
/// }
/// PANIC! {
///     "or a message with just a hint";
///     hint = "...";
/// }
/// PANIC! {
///     "put it all together for {} {}", "great", "success!";
///     detail = "extra {}", "info";
///     hint = "{} helpful", "very";
/// }
/// ```
#[allow(non_snake_case)]
#[macro_export]
macro_rules! PANIC {
    (
        $msg:literal $(, $msg_args:expr)*;
        $(detail = $detail:literal $(, $detail_args:expr)*)?;
        $(hint = $hint:literal $(, $hint_args:expr)*)?;
    ) => (
        {
            extern crate alloc;
            $crate::ereport!(
                $crate::elog::PgLogLevel::PANIC,
                $crate::errcodes::PgSqlErrorCode::ERRCODE_INTERNAL_ERROR,
                alloc::format!($msg $(, $msg_args)*).as_str();
                $(detail = alloc::format!($detail $(, $detail_args)*).as_str();)?
                $(hint = alloc::format!($hint $(, $hint_args)*).as_str();)?
            );
            unreachable!()
        }
    );
    ($($arg:tt)*) => (
        {
            extern crate alloc;
            $crate::ereport!($crate::elog::PgLogLevel::PANIC, $crate::errcodes::PgSqlErrorCode::ERRCODE_INTERNAL_ERROR, alloc::format!($($arg)*).as_str());
            unreachable!()
        }
    );
}

// shamelessly borrowed from https://docs.rs/stdext/0.2.1/src/stdext/macros.rs.html#61-72
/// This macro returns the name of the enclosing function.
/// As the internal implementation is based on the [`std::any::type_name`], this macro derives
/// all the limitations of this function.
///
/// [`std::any::type_name`]: https://doc.rust-lang.org/std/any/fn.type_name.html
#[macro_export]
macro_rules! function_name {
    () => {{
        // Okay, this is ugly, I get it. However, this is the best we can get on a stable rust.
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            core::any::type_name::<T>()
        }
        let name = type_name_of(f);
        // `3` is the length of the `::f`.
        &name[..name.len() - 3]
    }};
}

/// Sends some kind of message to Postgres, and if it's a [PgLogLevel::ERROR] or greater, Postgres'
/// error handling takes over and, in the case of [PgLogLevel::ERROR], aborts the current transaction.
///
/// This macro is necessary when one needs to supply a specific SQL error code as part of their
/// error message.
///
/// ## Simple Usage
///
/// The argument order is:
/// - `log_level: [PgLogLevel]`
/// - `error_code: [PgSqlErrorCode]`
/// - `message: String`
/// - (optional) `detail: String`
/// - (optional) `hint: String`
///
/// ## Examples
///
/// ```rust,no_run
/// # use pgrx_pg_sys::ereport;
/// # use pgrx_pg_sys::elog::PgLogLevel;
/// # use pgrx_pg_sys::errcodes::PgSqlErrorCode;
/// ereport!(PgLogLevel::ERROR, PgSqlErrorCode::ERRCODE_INTERNAL_ERROR, "oh noes!"); // abort the transaction
/// ```
///
/// ```rust,no_run
/// # use pgrx_pg_sys::ereport;
/// # use pgrx_pg_sys::elog::PgLogLevel;
/// # use pgrx_pg_sys::errcodes::PgSqlErrorCode;
/// ereport!(PgLogLevel::LOG, PgSqlErrorCode::ERRCODE_SUCCESSFUL_COMPLETION, "this is just a message"); // log output only
/// ```
///
/// ## Alternative Usage
///
/// Use this invocation style if you need to include a hint but not a detail. This syntax does not prohibit
/// the use of a detail, but because the simple usage takes it's arguments by position, it isn't possible to
/// use that syntax and include a hint without a detail.
///
/// The argument order is for the first three arguments is the same as the simple usage:
/// - `log_level: [PgLogLevel]`
/// - `error_code: [PgSqlErrorCode]`
/// - `message: String`
///
/// After the message, include a semicolon and then assign an expression resulting in a String to `detail` or `hint`.
///
/// ## Examples
///
/// ```rust,no_run
/// # use pgrx_pg_sys::ereport;
/// # use pgrx_pg_sys::elog::PgLogLevel;
/// # use pgrx_pg_sys::errcodes::PgSqlErrorCode;
/// // abort the transaction
/// ereport! {
///     PgLogLevel::ERROR, PgSqlErrorCode::ERRCODE_INTERNAL_ERROR, "oh noes!";
///     hint = "check earlier logs for more info";
/// }
/// ```
///
/// ```rust,no_run
/// # use pgrx_pg_sys::ereport;
/// # use pgrx_pg_sys::elog::PgLogLevel;
/// # use pgrx_pg_sys::errcodes::PgSqlErrorCode;
/// ereport! {
///     PgLogLevel::LOG, PgSqlErrorCode::ERRCODE_SUCCESSFUL_COMPLETION, "this is just a message";
///     detail = "but wait, there's more!";
///     hint = "there are easier macros to log simple messages like this...";
/// }
/// ```
///
/// > _**NOTE**: the message/detail/hint arguments don't actually need to result in an owned `String`.
/// > The trait bounds for the underlying functions are `Into<String>`, so any type that implements
/// `Into<String>` will work too._
#[macro_export]
macro_rules! ereport {
    (ERROR, $errcode:expr, $message:expr $(, $($tt:tt)*)?) => {
        $crate::ereport!($crate::elog::PgLogLevel::ERROR, $errcode, $message $(, $($tt)*)?);
        unreachable!();
    };

    (ERROR, $errcode:expr, $message:expr $(; $($tt:tt)*)?) => {
        $crate::ereport!($crate::elog::PgLogLevel::ERROR, $errcode, $message $(; $($tt)*)?);
        unreachable!();
    };

    (PANIC, $errcode:expr, $message:expr $(, $($tt:tt)*)?) => {
        $crate::ereport!($crate::elog::PgLogLevel::PANIC, $errcode, $message $(, $($tt)*)?);
        unreachable!();
    };

    (PANIC, $errcode:expr, $message:expr $(; $($tt:tt)*)?) => {
        $crate::ereport!($crate::elog::PgLogLevel::PANIC, $errcode, $message $(; $($tt)*)?);
        unreachable!();
    };

    (FATAL, $errcode:expr, $message:expr $(, $($tt:tt)*)?) => {
        $crate::ereport!($crate::elog::PgLogLevel::FATAL, $errcode, $message $(, $($tt)*)?);
        unreachable!();
    };

    (FATAL, $errcode:expr, $message:expr $(; $($tt:tt)*)?) => {
        $crate::ereport!($crate::elog::PgLogLevel::FATAL, $errcode, $message $(; $($tt)*)?);
        unreachable!();
    };

    (WARNING, $errcode:expr, $message:expr $(, $($tt:tt)*)?) => {
        $crate::ereport!($crate::elog::PgLogLevel::WARNING, $errcode, $message $(, $($tt)*)?)
    };

    (WARNING, $errcode:expr, $message:expr $(; $($tt:tt)*)?) => {
        $crate::ereport!($crate::elog::PgLogLevel::WARNING, $errcode, $message $(; $($tt)*)?)
    };

    (NOTICE, $errcode:expr, $message:expr $(, $($tt:tt)*)?) => {
        $crate::ereport!($crate::elog::PgLogLevel::NOTICE, $errcode, $message $(, $($tt)*)?)
    };

    (NOTICE, $errcode:expr, $message:expr $(; $($tt:tt)*)?) => {
        $crate::ereport!($crate::elog::PgLogLevel::NOTICE, $errcode, $message $(; $($tt)*)?)
    };

    (INFO, $errcode:expr, $message:expr $(, $($tt:tt)*)?) => {
        $crate::ereport!($crate::elog::PgLogLevel::INFO, $errcode, $message $(, $($tt)*)?)
    };

    (INFO, $errcode:expr, $message:expr $(; $($tt:tt)*)?) => {
        $crate::ereport!($crate::elog::PgLogLevel::INFO, $errcode, $message $(; $($tt)*)?)
    };

    (LOG, $errcode:expr, $message:expr $(, $($tt:tt)*)?) => {
        $crate::ereport!($crate::elog::PgLogLevel::LOG, $errcode, $message $(, $($tt)*)?)
    };

    (LOG, $errcode:expr, $message:expr $(; $($tt:tt)*)?) => {
        $crate::ereport!($crate::elog::PgLogLevel::LOG, $errcode, $message $(; $($tt)*)?)
    };

    (DEBUG5, $errcode:expr, $message:expr $(, $($tt:tt)*)?) => {
        $crate::ereport!($crate::elog::PgLogLevel::DEBUG5, $errcode, $message $(, $($tt)*)?)
    };

    (DEBUG5, $errcode:expr, $message:expr $(; $($tt:tt)*)?) => {
        $crate::ereport!($crate::elog::PgLogLevel::DEBUG5, $errcode, $message $(; $($tt)*)?)
    };

    (DEBUG4, $errcode:expr, $message:expr $(, $($tt:tt)*)?) => {
        $crate::ereport!($crate::elog::PgLogLevel::DEBUG4, $errcode, $message $(, $($tt)*)?)
    };

    (DEBUG4, $errcode:expr, $message:expr $(; $($tt:tt)*)?) => {
        $crate::ereport!($crate::elog::PgLogLevel::DEBUG4, $errcode, $message $(; $($tt)*)?)
    };

    (DEBUG3, $errcode:expr, $message:expr $(, $($tt:tt)*)?) => {
        $crate::ereport!($crate::elog::PgLogLevel::DEBUG3, $errcode, $message $(, $($tt)*)?)
    };

    (DEBUG3, $errcode:expr, $message:expr $(; $($tt:tt)*)?) => {
        $crate::ereport!($crate::elog::PgLogLevel::DEBUG3, $errcode, $message $(; $($tt)*)?)
    };

    (DEBUG2, $errcode:expr, $message:expr $(, $($tt:tt)*)?) => {
        $crate::ereport!($crate::elog::PgLogLevel::DEBUG2, $errcode, $message $(, $($tt)*)?)
    };

    (DEBUG2, $errcode:expr, $message:expr $(; $($tt:tt)*)?) => {
        $crate::ereport!($crate::elog::PgLogLevel::DEBUG2, $errcode, $message $(; $($tt)*)?)
    };

    (DEBUG1, $errcode:expr, $message:expr $(, $($tt:tt)*)?) => {
        $crate::ereport!($crate::elog::PgLogLevel::DEBUG1, $errcode, $message $(, $($tt)*)?)
    };

    (DEBUG1, $errcode:expr, $message:expr $(; $($tt:tt)*)?) => {
        $crate::ereport!($crate::elog::PgLogLevel::DEBUG1, $errcode, $message $(; $($tt)*)?)
    };

    (
        $loglevel:expr, $errcode:expr, $message:expr $(;)?
    ) => {
        $crate::panic::ErrorReport::new($errcode, $message, $crate::function_name!())
            .report($loglevel);
    };

    (
        $loglevel:expr, $errcode:expr, $message:expr;
        hint = $hint:expr;
    ) => {
        $crate::panic::ErrorReport::new($errcode, $message, $crate::function_name!())
            .set_hint($hint)
            .report($loglevel);
    };

    (
        $loglevel:expr, $errcode:expr, $message:expr;
        detail = $detail:expr;
    ) => {
        $crate::panic::ErrorReport::new($errcode, $message, $crate::function_name!())
            .set_detail($detail)
            .report($loglevel);
    };

    (
        $loglevel:expr, $errcode:expr, $message:expr;
        detail = $detail:expr;
        hint = $hint:expr;
    ) => {
        $crate::panic::ErrorReport::new($errcode, $message, $crate::function_name!())
            .set_detail($detail)
            .set_hint($hint)
            .report($loglevel);
    };

    (
        $loglevel:expr, $errcode:expr, $message:expr $(,)?
    ) => {
        $crate::panic::ErrorReport::new($errcode, $message, $crate::function_name!())
            .report($loglevel);
    };

    ($loglevel:expr, $errcode:expr, $message:expr, $detail:expr $(,)?) => {
        $crate::panic::ErrorReport::new($errcode, $message, $crate::function_name!())
            .set_detail($detail)
            .report($loglevel);
    };

    ($loglevel:expr, $errcode:expr, $message:expr, $detail:expr, $hint:expr $(,)?) => {
        $crate::panic::ErrorReport::new($errcode, $message, $crate::function_name!())
            .set_detail($detail)
            .set_hint($hint)
            .report($loglevel);
    };
}

/// Is an interrupt pending?
#[inline]
pub fn interrupt_pending() -> bool {
    unsafe { crate::InterruptPending != 0 }
}

/// If an interrupt is pending (perhaps a user-initiated "cancel query" message to this backend),
/// this will safely abort the current transaction
#[macro_export]
macro_rules! check_for_interrupts {
    () => {
        #[allow(unused_unsafe)]
        unsafe {
            if $crate::InterruptPending != 0 {
                $crate::ProcessInterrupts();
            }
        }
    };
}
