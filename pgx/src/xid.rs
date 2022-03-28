/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use crate::pg_sys;

#[cfg(any(feature = "pg10", feature = "pg11"))]
#[inline]
pub fn xid_to_64bit(xid: pg_sys::TransactionId) -> u64 {
    let mut last_xid = pg_sys::InvalidTransactionId;
    let mut epoch = 0u32;

    unsafe {
        pg_sys::GetNextXidAndEpoch(&mut last_xid, &mut epoch);
    }

    convert_xid_common(xid, last_xid, epoch)
}

#[cfg(any(feature = "pg12", feature = "pg13", feature = "pg14"))]
#[inline]
pub fn xid_to_64bit(xid: pg_sys::TransactionId) -> u64 {
    let full_xid = unsafe { pg_sys::ReadNextFullTransactionId() };

    let last_xid = full_xid.value as u32;
    let epoch = (full_xid.value >> 32) as u32;

    convert_xid_common(xid, last_xid, epoch)
}

#[inline]
fn convert_xid_common(xid: pg_sys::TransactionId, last_xid: u32, epoch: u32) -> u64 {
    /* return special xid's as-is */
    if !pg_sys::TransactionIdIsNormal(xid) {
        return xid as u64;
    }

    /* xid can be on either side when near wrap-around */
    let mut epoch = epoch as u64;
    if xid > last_xid && unsafe { pg_sys::TransactionIdPrecedes(xid, last_xid) } {
        epoch -= 1;
    } else if xid < last_xid && unsafe { pg_sys::TransactionIdFollows(xid, last_xid) } {
        epoch += 1;
    }

    (epoch << 32) | xid as u64
}
