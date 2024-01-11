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

// A helper macro to define the various elog macros.
macro_rules! define_elog {
    (
        $name:tt = {
            loglevel = $loglevel:tt
            errcode = $errcode:tt
            docs = $doc_str:tt
            extra_docs = $extra_docs:tt
            extra_stmts = $extra_stmts:tt
        }
    ) => {
        define_elog!{
            @define_macro
            name = $name
            loglevel = $loglevel
            errcode = $errcode
            docs = $doc_str
            extra_docs = $extra_docs
            extra_stmts = $extra_stmts
            dollar = $
        }
    };
    (@get_first_doc_line name = $name:tt $doc_str:tt) => {
        concat!("Log to Postgres' `", stringify!($name), "` log level. ", $doc_str)
    };
    (@get_extra_docs loglevel = $loglevel:tt false) => {""};
    (@get_extra_docs loglevel = $loglevel:tt true) => {
        concat!(
            "The output these logs goes to the PostgreSQL log file at `", stringify!($loglevel), "` level, depending on how the\n",
            "[PostgreSQL settings](https://www.postgresql.org/docs/current/runtime-config-logging.html) are configured.",
            "\n"
        )
    };
    (
        @define_macro
        name = $name:tt
        loglevel = $loglevel:tt
        errcode = $errcode:tt
        docs = $doc_str:tt
        extra_docs = $extra_docs:tt
        extra_stmts = $extra_stmts:tt
        dollar = $_:tt
    ) => {
        #[allow(non_snake_case)]
        #[macro_export]
        #[doc=concat!(
            define_elog!(@get_first_doc_line name = $name $doc_str),
            "\n",
            "This macro accepts arguments like the [`println`] and [`format`] macros.\n",
            "See [`fmt`](std::fmt) for information about options.\n",
            "\n",
            define_elog!(@get_extra_docs loglevel = $loglevel $extra_docs),
            "## Adding Details and Hints\n",
            "\n",
            "An alternative invocation style is available if you want to include a `detail` and/or `hint`.\n",
            "The `message`, `detail`, and `hint` components can take formatting args just like [`format`].\n",
            "\n",
            "## Examples\n",
            "\n",
            "```rust,no_run\n",
            "# use pgrx_pg_sys::", stringify!($name), ";\n",
            "\n",
            "// just like `println!` and `format!`\n",
            stringify!($name), "!(\"a simple message\");\n",
            stringify!($name), "!(\"or a formatted message: {:?}\", \"pgrx rocks!\");\n",
            "// include details and hints\n",
            stringify!($name), "! {\n",
            "    message = \"add details if you want\";\n",
            "    detail = \"...\";\n",
            "}\n",
            stringify!($name), "! {\n",
            "    message = \"or a message with just a hint\";\n",
            "    hint = \"...\";\n",
            "}\n",
            stringify!($name), "! {\n",
            "    message = \"put it all together for {} {}\", \"great\", \"success!\";\n",
            "    detail = \"extra {}\", \"info\";\n",
            "    hint = \"{} helpful\", \"very\";\n",
            "}\n",
            "```",
        )]
        macro_rules! $name{
            (detail = $_ ($_ _tt:tt)*) => (
                compile_error!(concat!("The alternative invocation requires `message` to be specified first."));
            );

            (hint = $_ ($_ _tt:tt)*) => (
                compile_error!(concat!("The alternative invocation requires `message` to be specified first."));
            );

            (
                message = $_ msg:literal $_ (, $_ msg_args:expr)*;
                $_ (detail = $_ detail:literal $_ (, $_ detail_args:expr)* ;)?
            ) => (
                {
                    extern crate core;
                    extern crate alloc;
                    $_ crate::ereport!(
                        loglevel = $_ crate::elog::PgLogLevel::$loglevel;
                        errcode = $_ crate::errcodes::PgSqlErrorCode::$errcode;
                        message = alloc::format!("{}", core::format_args!($_ msg $_ (, $_ msg_args)*));
                        $_ (detail = alloc::format!("{}", core::format_args!($_ detail $_ (, $_ detail_args)*));)?
                    );
                    $extra_stmts
                }
            );

            (
                message = $_ msg:literal $_ (, $_ msg_args:expr)*;
                $_ (hint = $_ hint:literal $_ (, $_ hint_args:expr)* ;)?
            ) => (
                {
                    extern crate core;
                    extern crate alloc;
                    $_ crate::ereport!(
                        loglevel = $_ crate::elog::PgLogLevel::$loglevel;
                        errcode = $_ crate::errcodes::PgSqlErrorCode::$errcode;
                        message = alloc::format!("{}", core::format_args!($_ msg $_ (, $_ msg_args)*));
                        $_ (hint = alloc::format!("{}", core::format_args!($_ hint $_ (, $_ hint_args)*));)?
                    );
                    $extra_stmts
                }
            );

            (
                message = $_ msg:literal $_ (, $_ msg_args:expr)*;
                $_ (hint = $_ hint:literal $_ (, $_ hint_args:expr)* ;)?
                $_ (detail = $_ detail:literal $_ (, $_ detail_args:expr)* ;)?
            ) => (
                {
                    extern crate core;
                    extern crate alloc;
                    $_ crate::ereport!(
                        loglevel = $_ crate::elog::PgLogLevel::$loglevel;
                        errcode = $_ crate::errcodes::PgSqlErrorCode::$errcode;
                        message = alloc::format!("{}", core::format_args!($_ msg $_ (, $_ msg_args)*));
                        $_ (detail = alloc::format!("{}", core::format_args!($_ detail $_ (, $_ detail_args)*));)?
                        $_ (hint = alloc::format!("{}", core::format_args!($_ hint $_ (, $_ hint_args)*));)?
                    );
                    $extra_stmts
                }
            );

            (
                message = $_ msg:literal $_ (, $_ msg_args:expr)*;
                $_ (detail = $_ detail:literal $_ (, $_ detail_args:expr)* ;)?
                $_ (hint = $_ hint:literal $_ (, $_ hint_args:expr)* ;)?
            ) => (
                {
                    extern crate core;
                    extern crate alloc;
                    $_ crate::ereport!(
                        loglevel = $_ crate::elog::PgLogLevel::$loglevel;
                        errcode = $_ crate::errcodes::PgSqlErrorCode::$errcode;
                        message = alloc::format!("{}", core::format_args!($_ msg $_ (, $_ msg_args)*));
                        $_ (detail = alloc::format!("{}", core::format_args!($_ detail $_ (, $_ detail_args)*));)?
                        $_ (hint = alloc::format!("{}", core::format_args!($_ hint $_ (, $_ hint_args)*));)?
                    );
                    $extra_stmts
                }
            );

            ($_ s:literal $_ (, $_ arg:expr)* $_ (,)?) => (
                {
                    extern crate alloc;
                    $_ crate::ereport!($_ crate::elog::PgLogLevel::$loglevel, $_ crate::errcodes::PgSqlErrorCode::$errcode, alloc::format!($_ s $_ (, $_ arg)*));
                    $extra_stmts
                }
            );

            ($_ ($_ tt:tt)*) => {
                compile_error!(
                    concat!(
                        "Invalid invocation\n",
                        "\n",
                        "# Simple Usage\n",
                        "\n",
                        "## Required arguments:\n",
                        "  - string literal\n",
                        "\n",
                        "## Optional arguments:\n",
                        "  - formatting arguments\n",
                        "\n",
                        "## Example\n",
                        "  ", stringify!($name), "!(\"{}\", 42);\n",
                        "\n",
                        "# Alternative Usage\n",
                        "\n",
                        "## Required arguments:\n",
                        "  - message: impl Into<String>\n",
                        "\n",
                        "## Optional arguments:\n",
                        "  - detail: impl Into<String>\n",
                        "  - hint: impl Into<String>\n",
                        "\n",
                        "## Example\n",
                        "  ", stringify!($name), "! {\n",
                        "    message = \"...\";\n",
                        "    detail = \"extra detail\";\n",
                        "    hint = \"plus a hint\";\n",
                        "  }\n",
                        "\n",
                    )
                )
            };
        }
    };
}

define_elog! {
    debug5 = {
        loglevel = DEBUG5
        errcode = ERRCODE_SUCCESSFUL_COMPLETION
        docs = ""
        extra_docs = true
        extra_stmts = {}
    }
}

define_elog! {
    debug4 = {
        loglevel = DEBUG4
        errcode = ERRCODE_SUCCESSFUL_COMPLETION
        docs = ""
        extra_docs = true
        extra_stmts = {}
    }
}

define_elog! {
    debug3 = {
        loglevel = DEBUG3
        errcode = ERRCODE_SUCCESSFUL_COMPLETION
        docs = ""
        extra_docs = true
        extra_stmts = {}
    }
}

define_elog! {
    debug2 = {
        loglevel = DEBUG2
        errcode = ERRCODE_SUCCESSFUL_COMPLETION
        docs = ""
        extra_docs = true
        extra_stmts = {}
    }
}

define_elog! {
    debug1 = {
        loglevel = DEBUG1
        errcode = ERRCODE_SUCCESSFUL_COMPLETION
        docs = ""
        extra_docs = true
        extra_stmts = {}
    }
}

define_elog! {
    log = {
        loglevel = LOG
        errcode = ERRCODE_SUCCESSFUL_COMPLETION
        docs = ""
        extra_docs = true
        extra_stmts = {}
    }
}

define_elog! {
    info = {
        loglevel = INFO
        errcode = ERRCODE_SUCCESSFUL_COMPLETION
        docs = ""
        extra_docs = false
        extra_stmts = {}
    }
}

define_elog! {
    notice = {
        loglevel = NOTICE
        errcode = ERRCODE_SUCCESSFUL_COMPLETION
        docs = ""
        extra_docs = false
        extra_stmts = {}
    }
}

define_elog! {
    warning = {
        loglevel = WARNING
        errcode = ERRCODE_WARNING
        docs = ""
        extra_docs = false
        extra_stmts = {}
    }
}

define_elog! {
    error = {
        loglevel = ERROR
        errcode = ERRCODE_INTERNAL_ERROR
        docs = "This will abort the current Postgres transaction."
        extra_docs = false
        extra_stmts = {unreachable!()}
    }
}

define_elog! {
    FATAL = {
        loglevel = FATAL
        errcode = ERRCODE_INTERNAL_ERROR
        docs = "This will abort the current Postgres backend connection process."
        extra_docs = false
        extra_stmts = {unreachable!()}
    }
}

define_elog! {
    PANIC = {
        loglevel = PANIC
        errcode = ERRCODE_INTERNAL_ERROR
        docs = "This will cause the entire Postgres cluster to crash."
        extra_docs = false
        extra_stmts = {unreachable!()}
    }
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

macro_rules! define_ereport {
    (
        macro_name = $name:tt
        $(#[$attr:meta])*
    ) => {
        define_ereport! {
            @define_macro
            $(#[$attr])*
            name = $name
            fail_levels = [
                ERROR,
                FATAL,
                PANIC,
            ]
            info_levels = [
                DEBUG5,
                DEBUG4,
                DEBUG3,
                DEBUG2,
                DEBUG1,
                LOG,
                INFO,
                NOTICE,
                WARNING,
            ]
            dollar = $
        }
    };
    (
        @define_macro
        $(#[$attr:meta])*
        name = $name:tt
        fail_levels = [$($fail_level:tt),+ $(,)?]
        info_levels = [$($info_level:tt),+ $(,)?]
        dollar = $_:tt
    ) => {
        $(#[$attr])*
        macro_rules! $name {
            // error cases
            (errcode = $_ errcode:expr; $_ ($_ tt:tt)*) => {
                compile_error!("The alternative invocation style requires `loglevel` argument to be specified first.");
            };

            (loglevel = $_ loglevel:expr; message = $_ ($_ tt:tt)*) => {
                compile_error!("The alternative invocation style requires `errcode` argument to be specified after `loglevel`.");
            };

            (loglevel = $_ loglevel:expr; detail = $_ ($_ tt:tt)*) => {
                compile_error!("The alternative invocation style requires `errcode` argument to be specified after `loglevel`.");
            };

            (loglevel = $_ loglevel:expr; hint = $_ ($_ tt:tt)*) => {
                compile_error!("The alternative invocation style requires `errcode` argument to be specified after `loglevel`.");
            };

            (loglevel = $_ loglevel:expr; errcode = $_ errcode:expr; detail = $_ detail:expr; $_ ($_ tt:tt)*) => {
                compile_error!("The alternative invocation style requires `message` argument to be specified after `errcode`.");
            };

            (loglevel = $_ loglevel:expr; errcode = $_ errcode:expr; hint = $_ hint:expr; $_ ($_ tt:tt)*) => {
                compile_error!("The alternative invocation style requires `message` argument to be specified after `errcode`.");
            };

            //

            (loglevel = $_ loglevel:expr, errcode = $_ errcode:expr; message = $_ message:expr; $_ ($_ tt:tt)*) => {
                compile_error!("The alternative invocation style requires component sections to be terminated with a semicolon.");
            };

            (loglevel = $_ loglevel:expr; errcode = $_ errcode:expr, message = $_ message:expr; $_ ($_ tt:tt)*) => {
                compile_error!("The alternative invocation style requires component sections to be terminated with a semicolon.");
            };

            (loglevel = $_ loglevel:expr; errcode = $_ errcode:expr; message = $_ message:expr, $_ ($_ tt:tt)*) => {
                compile_error!("The alternative invocation style requires component sections to be terminated with a semicolon.");
            };

            (loglevel = $_ loglevel:expr; errcode = $_ errcode:expr; message = $_ message:expr; detail = $_ detail:expr, $_ ($_ tt:tt)*) => {
                compile_error!("The alternative invocation style requires component sections to be terminated with a semicolon.");
            };

            (loglevel = $_ loglevel:expr; errcode = $_ errcode:expr; message = $_ message:expr; hint = $_ hint:expr, $_ ($_ tt:tt)*) => {
                compile_error!("The alternative invocation style requires component sections to be terminated with a semicolon.");
            };

            (loglevel = $_ loglevel:expr; errcode = $_ errcode:expr; message = $_ message:expr; detail = $_ detail:expr; hint = $_ hint:expr, $_ ($_ tt:tt)*) => {
                compile_error!("The alternative invocation style requires component sections to be terminated with a semicolon.");
            };

            // alternative syntax cases
            $(
                (loglevel = $fail_level; $_ ($_ tt:tt)*) => {
                    $_ crate::$name!(loglevel = $_ crate::elog::PgLogLevel::$fail_level; $_ ($_ tt)*);
                };
            )+

            $(
                (loglevel = $info_level; $_ ($_ tt:tt)*) => {
                    $_ crate::$name!(loglevel = $_ crate::elog::PgLogLevel::$info_level; $_ ($_ tt)*);
                };
            )+

            // alternative syntax - generic cases
            (
                loglevel = $_ loglevel:expr;
                errcode = $_ errcode:expr;
                message = $_ message:expr;
            ) => {
                $_ crate::panic::ErrorReport::new($_ errcode, $_ message, $_ crate::function_name!())
                    .report($_ loglevel);
            };

            (
                loglevel = $_ loglevel:expr;
                errcode = $_ errcode:expr;
                message = $_ message:expr;
                hint = $_ hint:expr;
            ) => {
                $_ crate::panic::ErrorReport::new($_ errcode, $_ message, $_ crate::function_name!())
                    .set_hint($_ hint)
                    .report($_ loglevel);
            };

            (
                loglevel = $_ loglevel:expr;
                errcode = $_ errcode:expr;
                message = $_ message:expr;
                detail = $_ detail:expr;
            ) => {
                $_ crate::panic::ErrorReport::new($_ errcode, $_ message, $_ crate::function_name!())
                    .set_detail($_ detail)
                    .report($_ loglevel);
            };

            (
                loglevel = $_ loglevel:expr;
                errcode = $_ errcode:expr;
                message = $_ message:expr;
                hint = $_ hint:expr;
                detail = $_ detail:expr;
            ) => {
                $_ crate::panic::ErrorReport::new($_ errcode, $_ message, $_ crate::function_name!())
                    .set_detail($_ detail)
                    .set_hint($_ hint)
                    .report($_ loglevel);
            };

            (
                loglevel = $_ loglevel:expr;
                errcode = $_ errcode:expr;
                message = $_ message:expr;
                detail = $_ detail:expr;
                hint = $_ hint:expr;
            ) => {
                $_ crate::panic::ErrorReport::new($_ errcode, $_ message, $_ crate::function_name!())
                    .set_detail($_ detail)
                    .set_hint($_ hint)
                    .report($_ loglevel);
            };

            // simple syntax cases
            $(
                ($fail_level, $_ errcode:expr, $_ message:expr $_ (, $_ ($_ tt:tt)*)?) => {
                    $_ crate::$name!($_ crate::elog::PgLogLevel::$fail_level, $_ errcode, $_ message $_ (, $_ ($_ tt)*)?);
                    unreachable!();
                };
            )+

            $(
                ($info_level, $_ errcode:expr, $_ message:expr $_ (, $_ ($_ tt:tt)*)?) => {
                    $_ crate::$name!($_ crate::elog::PgLogLevel::$info_level, $_ errcode, $_ message $_ (, $_ ($_ tt)*)?);
                };
            )+

            // simple syntax - generic cases
            ($_ loglevel:expr, $_ errcode:expr, $_ message:expr $_ (,)?) => {
                $_ crate::panic::ErrorReport::new($_ errcode, $_ message, $_ crate::function_name!())
                    .report($_ loglevel);
            };

            ($_ loglevel:expr, $_ errcode:expr, $_ message:expr, $_ detail:expr $_ (,)?) => {
                $_ crate::panic::ErrorReport::new($_ errcode, $_ message, $_ crate::function_name!())
                    .set_detail($_ detail)
                    .report($_ loglevel);
            };

            ($_ loglevel:expr, $_ errcode:expr, $_ message:expr, $_ detail:expr, $_ hint:expr $_ (,)?) => {
                $_ crate::panic::ErrorReport::new($_ errcode, $_ message, $_ crate::function_name!())
                    .set_detail($_ detail)
                    .set_hint($_ hint)
                    .report($_ loglevel);
            };

            ($_ ($_ tt:tt)*) => {
                compile_error!(
                    concat!(
                        "Invalid invocation\n",
                        "\n",
                        "## Required arguments:\n",
                        "  - loglevel: [PgLogLevel]\n",
                        "  - errcode: [PgSqlErrorCode]\n",
                        "  - message: impl Into<String>\n",
                        "\n",
                        "## Optional arguments:\n",
                        "  - detail: impl Into<String>\n",
                        "  - hint: impl Into<String>\n",
                        "\n",
                        "## Simple Usage\n",
                        "  ereport!(\n",
                        "    PgLogLevel::ERROR,\n",
                        "    PgSqlErrorCode::ERRCODE_INTERNAL_ERROR,\n",
                        "    \"invalid input\",\n",
                        "    \"extra detail\",\n",
                        "    \"plus a hint\",\n",
                        "  );\n",
                        "\n",
                        "## Alternative Usage\n",
                        "  ereport! {{\n",
                        "    loglevel = PgLogLevel::ERROR;\n",
                        "    errcode = PgSqlErrorCode::ERRCODE_INTERNAL_ERROR;\n",
                        "    message = \"invalid input\";\n",
                        "    detail = \"extra detail\";\n",
                        "    hint = \"plus a hint\";\n",
                        "  }}\n",
                        "\n",
                    )
                )
            };
        }
    };
}

define_ereport! {
    macro_name = ereport
    #[macro_export]
    #[doc = concat!(
        "Sends some kind of message to Postgres, and if it's a [PgLogLevel::ERROR] or greater, Postgres'\n",
        "error handling takes over and, in the case of [PgLogLevel::ERROR], aborts the current transaction.\n",
        "\n",
        "This macro is necessary when one needs to supply a specific SQL error code as part of their\n",
        "error message.\n",
        "\n",
        "## Simple Usage\n",
        "\n",
        "The argument order is:\n",
        "- `loglevel: [PgLogLevel]`\n",
        "- `errcode: [PgSqlErrorCode]`\n",
        "- `message: String`\n",
        "- (optional) `detail: String`\n",
        "- (optional) `hint: String`\n",
        "\n",
        "## Examples\n",
        "\n",
        "```rust,no_run\n",
        "# use pgrx_pg_sys::ereport;\n",
        "# use pgrx_pg_sys::elog::PgLogLevel;\n",
        "# use pgrx_pg_sys::errcodes::PgSqlErrorCode;\n",
        "ereport!(PgLogLevel::ERROR, PgSqlErrorCode::ERRCODE_INTERNAL_ERROR, \"oh noes!\"); // abort the transaction\n",
        "```\n",
        "\n",
        "```rust,no_run\n",
        "# use pgrx_pg_sys::ereport;\n",
        "# use pgrx_pg_sys::elog::PgLogLevel;\n",
        "# use pgrx_pg_sys::errcodes::PgSqlErrorCode;\n",
        "ereport!(PgLogLevel::LOG, PgSqlErrorCode::ERRCODE_SUCCESSFUL_COMPLETION, \"this is just a message\"); // log output only\n",
        "```\n",
        "\n",
        "```rust,no_run\n",
        "# use pgrx_pg_sys::ereport;\n",
        "# use pgrx_pg_sys::elog::PgLogLevel;\n",
        "# use pgrx_pg_sys::errcodes::PgSqlErrorCode;\n",
        "ereport!(PgLogLevel::ERROR, PgSqlErrorCode::ERRCODE_INTERNAL_ERROR, \"invalid input\", \"extra detail\");\n",
        "```\n",
        "\n",
        "```rust,no_run\n",
        "# use pgrx_pg_sys::ereport;\n",
        "# use pgrx_pg_sys::elog::PgLogLevel;\n",
        "# use pgrx_pg_sys::errcodes::PgSqlErrorCode;\n",
        "ereport!(PgLogLevel::ERROR, PgSqlErrorCode::ERRCODE_INTERNAL_ERROR, \"invalid input\", \"extra detail\", \"plus a hint\");\n",
        "```\n",
        "\n",
        "## Alternative Usage\n",
        "\n",
        "Use this invocation style if you need to include a hint but not a detail. This syntax does not prohibit\n",
        "the use of a detail, but because the simple usage takes it's arguments by position, it isn't possible to\n",
        "use that syntax and include a hint without a detail. In general, the other reporting macros are easier to\n",
        "use than this one.\n",
        "\n",
        "## Examples\n",
        "\n",
        "```rust,no_run\n",
        "# use pgrx_pg_sys::ereport;\n",
        "# use pgrx_pg_sys::elog::PgLogLevel;\n",
        "# use pgrx_pg_sys::errcodes::PgSqlErrorCode;\n",
        "// abort the transaction\n",
        "ereport! {\n",
        "    loglevel = PgLogLevel::ERROR;\n",
        "    errcode = PgSqlErrorCode::ERRCODE_INTERNAL_ERROR;\n",
        "    message = \"oh noes!\";\n",
        "    hint = \"check earlier logs for more info\";\n",
        "}\n",
        "```\n",
        "\n",
        "```rust,no_run\n",
        "# use pgrx_pg_sys::ereport;\n",
        "# use pgrx_pg_sys::elog::PgLogLevel;\n",
        "# use pgrx_pg_sys::errcodes::PgSqlErrorCode;\n",
        "ereport! {\n",
        "    loglevel = PgLogLevel::LOG;\n",
        "    errcode = PgSqlErrorCode::ERRCODE_SUCCESSFUL_COMPLETION;\n",
        "    message = \"this is just a message\";\n",
        "    detail = \"but wait, there's more!\";\n",
        "    hint = \"there are easier macros to log simple messages like this...\";\n",
        "}\n",
        "```\n",
        "\n",
        "> _**NOTE**: the message/detail/hint arguments don't actually need to result in an owned `String`.\n",
        "> The trait bounds for the underlying functions are `Into<String>`, so any type that implements\n",
        "`Into<String>` will work too._\n",
    )]
}

define_ereport! {
    macro_name = internal_ereport
}

pub(crate) use internal_ereport;

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
