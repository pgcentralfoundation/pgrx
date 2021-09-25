// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

//! Helper functions for working with custom Rust trigger functions

use crate::{is_a, pg_sys};

#[inline]
pub unsafe fn called_as_trigger(fcinfo: pg_sys::FunctionCallInfo) -> bool {
    let fcinfo = fcinfo.as_ref().expect("fcinfo was null");
    !fcinfo.context.is_null() && is_a(fcinfo.context, pg_sys::NodeTag_T_TriggerData)
}

#[inline]
pub fn trigger_fired_by_insert(event: u32) -> bool {
    event & pg_sys::TRIGGER_EVENT_OPMASK == pg_sys::TRIGGER_EVENT_INSERT
}

#[inline]
pub fn trigger_fired_by_delete(event: u32) -> bool {
    event & pg_sys::TRIGGER_EVENT_OPMASK == pg_sys::TRIGGER_EVENT_DELETE
}

#[inline]
pub fn trigger_fired_by_update(event: u32) -> bool {
    event & pg_sys::TRIGGER_EVENT_OPMASK == pg_sys::TRIGGER_EVENT_UPDATE
}

#[inline]
pub fn trigger_fired_by_truncate(event: u32) -> bool {
    event & pg_sys::TRIGGER_EVENT_OPMASK == pg_sys::TRIGGER_EVENT_TRUNCATE
}

#[inline]
pub fn trigger_fired_for_row(event: u32) -> bool {
    event & pg_sys::TRIGGER_EVENT_ROW != 0
}

#[inline]
pub fn trigger_fired_for_statement(event: u32) -> bool {
    !trigger_fired_for_row(event)
}

#[inline]
pub fn trigger_fired_before(event: u32) -> bool {
    event & pg_sys::TRIGGER_EVENT_TIMINGMASK == pg_sys::TRIGGER_EVENT_BEFORE
}

#[inline]
pub fn trigger_fired_after(event: u32) -> bool {
    event & pg_sys::TRIGGER_EVENT_TIMINGMASK == pg_sys::TRIGGER_EVENT_AFTER
}

#[inline]
pub fn trigger_fired_instead(event: u32) -> bool {
    event & pg_sys::TRIGGER_EVENT_TIMINGMASK == pg_sys::TRIGGER_EVENT_INSTEAD
}
