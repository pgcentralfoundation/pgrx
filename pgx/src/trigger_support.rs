/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

//! Helper functions for working with custom Rust trigger functions

use crate::{is_a, pg_sys};

/// Indicates which trigger tuple to convert into a [PgHeapTuple].
#[derive(Debug, Copy, Clone)]
pub enum TriggerTuple {
    /// The row for which the trigger was fired. This is the row being inserted, updated, or deleted.
    /// If this trigger was fired for an INSERT or DELETE then this is what you should return from the
    /// function if you don't want to replace the row with a different one (in the case of INSERT) or
    /// skip the operation.
    Current,

    /// The new version of the row, if the trigger was fired for an UPDATE. This is what you have to
    /// return from the function if the event is an UPDATE and you don't want to replace this row by
    /// a different one or skip the operation.
    New,
}

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
