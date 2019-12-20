use crate::pg_sys;

#[inline]
pub fn item_pointer_get_block_number(ctid: *const pg_sys::ItemPointerData) -> pg_sys::BlockNumber {
    assert!(item_pointer_is_valid(ctid));
    item_pointer_get_block_number_no_check(ctid)
}

#[inline]
pub fn item_pointer_get_offset_number(
    ctid: *const pg_sys::ItemPointerData,
) -> pg_sys::OffsetNumber {
    assert!(item_pointer_is_valid(ctid));
    item_pointer_get_offset_number_no_check(ctid)
}

#[inline]
pub fn item_pointer_get_block_number_no_check(
    ctid: *const pg_sys::ItemPointerData,
) -> pg_sys::BlockNumber {
    let block_id = (unsafe { *ctid }).ip_blkid;
    (((block_id.bi_hi as u32) << 16) | (block_id.bi_lo as u32)) as pg_sys::BlockNumber
}

#[inline]
pub fn item_pointer_get_offset_number_no_check(
    ctid: *const pg_sys::ItemPointerData,
) -> pg_sys::OffsetNumber {
    (unsafe { *ctid }).ip_posid
}

#[inline]
pub fn item_pointer_is_valid(ctid: *const pg_sys::ItemPointerData) -> bool {
    (unsafe { *ctid }).ip_posid != pg_sys::InvalidOffsetNumber
}
