//! Custom GUCs for the pgrust target.
//!
//! C writes a variable's current value through the `valueAddr` pointer an
//! extension passed to `DefineCustomXVariable`; pgrust keeps values in its
//! per-session store. The shims below define the variable in that store and
//! remember `valueAddr → name`, and `GucSetting::get()` (pgrx's reader)
//! asks [`guc_read_int`] & co. for the store's current value.

use core::ffi::{c_char, c_int};
use std::collections::HashMap;
use std::sync::Mutex;

use super::error::{raise, unwrap_pg};
use super::mem::palloc_copy;
use super::types::{
    GucBoolAssignHook, GucBoolCheckHook, GucContext, GucEnumAssignHook, GucEnumCheckHook,
    GucIntAssignHook, GucIntCheckHook, GucRealAssignHook, GucRealCheckHook, GucShowHook,
    GucStringAssignHook, GucStringCheckHook, config_enum_entry,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    Bool,
    Int,
    Real,
    String,
    Enum,
}

/// `valueAddr → (name, kind)` for every custom variable defined by pgrx code.
/// The settings are `static`s in the extension crate, so one process-wide
/// map serves every session.
static SETTINGS: Mutex<Option<HashMap<usize, (&'static str, Kind)>>> = Mutex::new(None);

fn remember(addr: usize, name: &'static str, kind: Kind) {
    let mut m = SETTINGS.lock().unwrap();
    m.get_or_insert_with(HashMap::new)
        .insert(addr, (name, kind));
}

fn lookup(addr: usize) -> Option<(&'static str, Kind)> {
    SETTINGS
        .lock()
        .unwrap()
        .as_ref()
        .and_then(|m| m.get(&addr).copied())
}

unsafe fn leak_cstr(p: *const c_char) -> Option<&'static str> {
    if p.is_null() {
        return None;
    }
    let s = core::ffi::CStr::from_ptr(p).to_string_lossy().into_owned();
    Some(Box::leak(s.into_boxed_str()))
}

fn context(c: GucContext::Type) -> ::pgr_types_guc::GucContext {
    use ::pgr_types_guc::GucContext as G;
    match c {
        GucContext::PGC_INTERNAL => G::PGC_INTERNAL,
        GucContext::PGC_POSTMASTER => G::PGC_POSTMASTER,
        GucContext::PGC_SIGHUP => G::PGC_SIGHUP,
        GucContext::PGC_SU_BACKEND => G::PGC_SU_BACKEND,
        GucContext::PGC_BACKEND => G::PGC_BACKEND,
        GucContext::PGC_SUSET => G::PGC_SUSET,
        _ => G::PGC_USERSET,
    }
}

fn no_hooks(check: bool, assign: bool, show: bool) {
    if check || assign || show {
        raise(Box::new(::pgr_types_error::PgError::error(
            "pgrx: custom GUC check/assign/show hooks are not supported under pgrust yet",
        )));
    }
}

pub unsafe fn DefineCustomBoolVariable(
    name: *const c_char,
    short_desc: *const c_char,
    long_desc: *const c_char,
    value_addr: *mut bool,
    boot_value: bool,
    ctx: GucContext::Type,
    flags: c_int,
    check_hook: GucBoolCheckHook,
    assign_hook: GucBoolAssignHook,
    show_hook: GucShowHook,
) {
    no_hooks(
        check_hook.is_some(),
        assign_hook.is_some(),
        show_hook.is_some(),
    );
    let name = leak_cstr(name).expect("GUC name");
    unwrap_pg(::pgr_guc::DefineCustomBoolVariable(
        name,
        leak_cstr(short_desc),
        leak_cstr(long_desc),
        boot_value,
        context(ctx),
        flags,
    ));
    remember(value_addr as usize, name, Kind::Bool);
}

pub unsafe fn DefineCustomIntVariable(
    name: *const c_char,
    short_desc: *const c_char,
    long_desc: *const c_char,
    value_addr: *mut c_int,
    boot_value: c_int,
    min_value: c_int,
    max_value: c_int,
    ctx: GucContext::Type,
    flags: c_int,
    check_hook: GucIntCheckHook,
    assign_hook: GucIntAssignHook,
    show_hook: GucShowHook,
) {
    no_hooks(
        check_hook.is_some(),
        assign_hook.is_some(),
        show_hook.is_some(),
    );
    let name = leak_cstr(name).expect("GUC name");
    unwrap_pg(::pgr_guc::DefineCustomIntVariable(
        name,
        leak_cstr(short_desc),
        leak_cstr(long_desc),
        boot_value,
        min_value,
        max_value,
        context(ctx),
        flags,
    ));
    remember(value_addr as usize, name, Kind::Int);
}

pub unsafe fn DefineCustomRealVariable(
    name: *const c_char,
    short_desc: *const c_char,
    long_desc: *const c_char,
    value_addr: *mut f64,
    boot_value: f64,
    min_value: f64,
    max_value: f64,
    ctx: GucContext::Type,
    flags: c_int,
    check_hook: GucRealCheckHook,
    assign_hook: GucRealAssignHook,
    show_hook: GucShowHook,
) {
    no_hooks(
        check_hook.is_some(),
        assign_hook.is_some(),
        show_hook.is_some(),
    );
    let name = leak_cstr(name).expect("GUC name");
    unwrap_pg(::pgr_guc::DefineCustomRealVariable(
        name,
        leak_cstr(short_desc),
        leak_cstr(long_desc),
        boot_value,
        min_value,
        max_value,
        context(ctx),
        flags,
    ));
    remember(value_addr as usize, name, Kind::Real);
}

pub unsafe fn DefineCustomStringVariable(
    name: *const c_char,
    short_desc: *const c_char,
    long_desc: *const c_char,
    value_addr: *mut *mut c_char,
    boot_value: *const c_char,
    ctx: GucContext::Type,
    flags: c_int,
    check_hook: GucStringCheckHook,
    assign_hook: GucStringAssignHook,
    show_hook: GucShowHook,
) {
    no_hooks(
        check_hook.is_some(),
        assign_hook.is_some(),
        show_hook.is_some(),
    );
    let name = leak_cstr(name).expect("GUC name");
    let boot = leak_cstr(boot_value);
    unwrap_pg(::pgr_guc::DefineCustomStringVariable(
        name,
        leak_cstr(short_desc),
        leak_cstr(long_desc),
        boot,
        context(ctx),
        flags,
    ));
    remember(value_addr as usize, name, Kind::String);
}

pub unsafe fn DefineCustomEnumVariable(
    _name: *const c_char,
    _short_desc: *const c_char,
    _long_desc: *const c_char,
    _value_addr: *mut c_int,
    _boot_value: c_int,
    _options: *const config_enum_entry,
    _ctx: GucContext::Type,
    _flags: c_int,
    _check_hook: GucEnumCheckHook,
    _assign_hook: GucEnumAssignHook,
    _show_hook: GucShowHook,
) {
    raise(Box::new(::pgr_types_error::PgError::error(
        "pgrx: custom enum GUCs are not supported under pgrust yet",
    )));
}

pub fn guc_read_bool(key: usize) -> Option<bool> {
    let (name, kind) = lookup(key)?;
    (kind == Kind::Bool)
        .then(|| ::pgr_guc::store::get_bool(name))
        .flatten()
}

pub fn guc_read_int(key: usize) -> Option<i32> {
    let (name, kind) = lookup(key)?;
    (kind == Kind::Int)
        .then(|| ::pgr_guc::store::get_int(name))
        .flatten()
}

pub fn guc_read_real(key: usize) -> Option<f64> {
    let (name, kind) = lookup(key)?;
    (kind == Kind::Real)
        .then(|| ::pgr_guc::store::get_real(name))
        .flatten()
}

pub fn guc_read_enum(key: usize) -> Option<i32> {
    let (name, kind) = lookup(key)?;
    (kind == Kind::Enum)
        .then(|| ::pgr_guc::store::get_enum(name))
        .flatten()
}

/// The current string value as a NUL-terminated copy on the current context
/// (null for SQL NULL); `None` when the variable is not defined.
pub fn guc_read_string(key: usize) -> Option<*mut c_char> {
    let (name, kind) = lookup(key)?;
    if kind != Kind::String {
        return None;
    }
    let v = ::pgr_guc::store::get_string(name)?;
    Some(match v {
        Some(s) => {
            let mut bytes = s.into_bytes();
            bytes.push(0);
            palloc_copy(&bytes) as *mut c_char
        }
        None => core::ptr::null_mut(),
    })
}
