/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

//! Provides a safe interface into Postgres' Configuration System (GUC)
use crate::{pg_sys, PgMemoryContexts};
use core::ffi::CStr;
pub use pgx_macros::PostgresGucEnum;
use std::cell::Cell;

/// Defines at what level this GUC can be set
pub enum GucContext {
    /// cannot be set by the user at all, but only through
    /// internal processes ("server_version" is an example).  These are GUC
    /// variables only so they can be shown by SHOW, etc.
    Internal = pg_sys::GucContext_PGC_INTERNAL as isize,

    /// can only be set when the postmaster starts,
    /// either from the configuration file or the command line.
    Postmaster = pg_sys::GucContext_PGC_POSTMASTER as isize,

    /// can only be set at postmaster startup or by changing
    /// the configuration file and sending the HUP signal to the postmaster
    /// or a backend process. (Notice that the signal receipt will not be
    /// evaluated immediately. The postmaster and the backend check it at a
    /// certain point in their main loop. It's safer to wait than to read a
    /// file asynchronously.)
    Sighup = pg_sys::GucContext_PGC_SIGHUP as isize,

    /// can only be set at postmaster startup, from the configuration file, or by
    /// client request in the connection startup packet (e.g., from libpq's PGOPTIONS
    /// variable).  
    SuBackend = pg_sys::GucContext_PGC_SU_BACKEND as isize,

    /// can be set from the startup packet only when the user is a
    /// superuser.  Furthermore, an already-started backend will ignore changes
    /// to such an option in the configuration file.  The idea is that these
    /// options are fixed for a given backend once it's started, but they can
    /// vary across backends.
    Backend = pg_sys::GucContext_PGC_BACKEND as isize,

    /// can be set at postmaster startup, with the SIGHUP
    /// mechanism, or from the startup packet or SQL if you're a superuser.
    Suset = pg_sys::GucContext_PGC_SUSET as isize,

    /// can be set by anyone any time.
    Userset = pg_sys::GucContext_PGC_USERSET as isize,
}

bitflags! {
    #[derive(Default)]
    /// Flags to control special behaviour for the GUC that these are set on. See their
    /// descriptions below for their behaviour.
    pub struct GucFlags: i32 {
        /// Exclude from SHOW ALL
        const NO_SHOW_ALL = pg_sys::GUC_NO_SHOW_ALL as i32;
        /// Exclude from RESET ALL
        const NO_RESET_ALL = pg_sys::GUC_NO_RESET_ALL as i32;
        /// Auto-report changes to client
        const REPORT = pg_sys::GUC_REPORT as i32;
        /// Can't set in postgresql.conf
        const DISALLOW_IN_FILE = pg_sys::GUC_DISALLOW_IN_FILE as i32;
        /// Placeholder for custom variable
        const CUSTOM_PLACEHOLDER = pg_sys::GUC_CUSTOM_PLACEHOLDER as i32;
        /// Show only to superuser
        const SUPERUSER_ONLY = pg_sys::GUC_SUPERUSER_ONLY as i32;
        /// Limit string to `NAMEDATALEN-1`
        const IS_NAME = pg_sys::GUC_IS_NAME as i32;
        /// Can't set if security restricted
        const NOT_WHILE_SEC_REST = pg_sys::GUC_NOT_WHILE_SEC_REST as i32;
        /// Can't set in `PG_AUTOCONF_FILENAME`
        const DISALLOW_IN_AUTO_FILE = pg_sys::GUC_DISALLOW_IN_AUTO_FILE as i32;
        /// Value is in kilobytes
        const UNIT_KB = pg_sys::GUC_UNIT_KB as i32;
        /// Value is in blocks
        const UNIT_BLOCKS = pg_sys::GUC_UNIT_BLOCKS as i32;
        /// Value is in xlog blocks
        const UNIT_XBLOCKS = pg_sys::GUC_UNIT_XBLOCKS as i32;
        /// Value is in megabytes
        const UNIT_MB = pg_sys::GUC_UNIT_MB as i32;
        /// Value is in bytes
        const UNIT_BYTE = pg_sys::GUC_UNIT_BYTE as i32;
        /// Value is in milliseconds
        const UNIT_MS = pg_sys::GUC_UNIT_MS as i32;
        /// Value is in seconds
        const UNIT_S = pg_sys::GUC_UNIT_S as i32;
        /// Value is in minutes
        const UNIT_MIN = pg_sys::GUC_UNIT_MIN as i32;
        /// Include in `EXPLAIN` output
        #[cfg(not(feature = "pg11"))]
        const EXPLAIN = pg_sys::GUC_EXPLAIN as i32;
        #[cfg(feature = "pg15")]
        /// `RUNTIME_COMPUTED` is intended for runtime-computed GUCs that are only available via
        /// `postgres -C` if the server is not running
        const RUNTIME_COMPUTED = pg_sys::GUC_RUNTIME_COMPUTED as i32;
    }
}

/// A trait that can be derived using [`PostgresGucEnum`] on enums, such that they can be
/// usad as a GUC.
pub trait GucEnum<T>
where
    T: Copy,
{
    fn from_ordinal(ordinal: i32) -> T;
    fn to_ordinal(&self) -> i32;
    unsafe fn config_matrix(&self) -> *const pg_sys::config_enum_entry;
}

/// A safe wrapper around a global variable that can be edited through a GUC
pub struct GucSetting<T> {
    value: Cell<T>,
    char_p: Cell<*mut std::os::raw::c_char>,
    enum_o: Cell<i32>,
}

impl<T> GucSetting<T> {
    pub const fn new(value: T) -> Self {
        GucSetting {
            value: Cell::new(value),
            char_p: Cell::new(std::ptr::null_mut()),
            enum_o: Cell::new(0),
        }
    }
}

unsafe impl Sync for GucSetting<bool> {}
impl GucSetting<bool> {
    pub fn get(&self) -> bool {
        self.value.get()
    }

    unsafe fn as_ptr(&self) -> *mut bool {
        self.value.as_ptr()
    }
}

unsafe impl Sync for GucSetting<i32> {}
impl GucSetting<i32> {
    pub fn get(&self) -> i32 {
        self.value.get()
    }

    unsafe fn as_ptr(&self) -> *mut i32 {
        self.value.as_ptr()
    }
}

unsafe impl Sync for GucSetting<f64> {}
impl GucSetting<f64> {
    pub fn get(&self) -> f64 {
        self.value.get()
    }

    unsafe fn as_ptr(&self) -> *mut f64 {
        self.value.as_ptr()
    }
}

unsafe impl Sync for GucSetting<Option<&'static str>> {}
impl GucSetting<Option<&'static str>> {
    pub fn get(&self) -> Option<String> {
        let ptr = self.get_char_ptr();
        if ptr.is_null() {
            None
        } else {
            let cstr = unsafe { CStr::from_ptr(ptr) };
            Some(cstr.to_str().unwrap().to_owned())
        }
    }

    pub fn get_char_ptr(&self) -> *mut std::os::raw::c_char {
        unsafe { *self.char_p.as_ptr() }
    }

    unsafe fn as_ptr(&self) -> *mut *mut std::os::raw::c_char {
        self.char_p.as_ptr()
    }
}

unsafe impl<T> Sync for GucSetting<T> where T: GucEnum<T> + Copy {}
impl<T> GucSetting<T>
where
    T: GucEnum<T> + Copy,
{
    pub fn get(&self) -> T {
        T::from_ordinal(self.enum_o.get())
    }

    pub fn as_ptr(&self) -> *mut i32 {
        self.enum_o.as_ptr()
    }
}

/// A struct that has associated functions to register new GUCs
pub struct GucRegistry {}
impl GucRegistry {
    pub fn define_bool_guc(
        name: &str,
        short_description: &str,
        long_description: &str,
        setting: &GucSetting<bool>,
        context: GucContext,
        flags: GucFlags,
    ) {
        unsafe {
            pg_sys::DefineCustomBoolVariable(
                PgMemoryContexts::TopMemoryContext.pstrdup(name),
                PgMemoryContexts::TopMemoryContext.pstrdup(short_description),
                PgMemoryContexts::TopMemoryContext.pstrdup(long_description),
                setting.as_ptr(),
                setting.get(),
                context as isize as u32,
                flags.bits(),
                None,
                None,
                None,
            )
        }
    }

    pub fn define_int_guc(
        name: &str,
        short_description: &str,
        long_description: &str,
        setting: &GucSetting<i32>,
        min_value: i32,
        max_value: i32,
        context: GucContext,
        flags: GucFlags,
    ) {
        unsafe {
            pg_sys::DefineCustomIntVariable(
                PgMemoryContexts::TopMemoryContext.pstrdup(name),
                PgMemoryContexts::TopMemoryContext.pstrdup(short_description),
                PgMemoryContexts::TopMemoryContext.pstrdup(long_description),
                setting.as_ptr(),
                setting.get(),
                min_value,
                max_value,
                context as isize as u32,
                flags.bits(),
                None,
                None,
                None,
            )
        }
    }

    pub fn define_string_guc(
        name: &str,
        short_description: &str,
        long_description: &str,
        setting: &GucSetting<Option<&'static str>>,
        context: GucContext,
        flags: GucFlags,
    ) {
        unsafe {
            let boot_value = match setting.value.get() {
                Some(s) => PgMemoryContexts::TopMemoryContext.pstrdup(s),
                None => std::ptr::null_mut(),
            };

            pg_sys::DefineCustomStringVariable(
                PgMemoryContexts::TopMemoryContext.pstrdup(name),
                PgMemoryContexts::TopMemoryContext.pstrdup(short_description),
                PgMemoryContexts::TopMemoryContext.pstrdup(long_description),
                setting.as_ptr(),
                boot_value,
                context as isize as u32,
                flags.bits(),
                None,
                None,
                None,
            )
        }
    }

    pub fn define_float_guc(
        name: &str,
        short_description: &str,
        long_description: &str,
        setting: &GucSetting<f64>,
        min_value: f64,
        max_value: f64,
        context: GucContext,
        flags: GucFlags,
    ) {
        unsafe {
            pg_sys::DefineCustomRealVariable(
                PgMemoryContexts::TopMemoryContext.pstrdup(name),
                PgMemoryContexts::TopMemoryContext.pstrdup(short_description),
                PgMemoryContexts::TopMemoryContext.pstrdup(long_description),
                setting.as_ptr(),
                setting.get(),
                min_value,
                max_value,
                context as isize as u32,
                flags.bits(),
                None,
                None,
                None,
            )
        }
    }

    pub fn define_enum_guc<T>(
        name: &str,
        short_description: &str,
        long_description: &str,
        setting: &GucSetting<T>,
        context: GucContext,
        flags: GucFlags,
    ) where
        T: GucEnum<T> + Copy,
    {
        unsafe {
            pg_sys::DefineCustomEnumVariable(
                PgMemoryContexts::TopMemoryContext.pstrdup(name),
                PgMemoryContexts::TopMemoryContext.pstrdup(short_description),
                PgMemoryContexts::TopMemoryContext.pstrdup(long_description),
                setting.as_ptr(),
                setting.value.get().to_ordinal(),
                setting.value.get().config_matrix(),
                context as isize as u32,
                flags.bits(),
                None,
                None,
                None,
            )
        }
    }
}
