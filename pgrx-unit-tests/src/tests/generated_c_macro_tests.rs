//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Exercise generated C macros in their PostgreSQL backend execution environment.
//!
//! A macro can reference guarded PostgreSQL functions even when a particular
//! invocation short-circuits before calling them. Its complete expansion still
//! needs the backend's symbols at link/load time, so these integration checks
//! belong in the extension test harness rather than a standalone test binary.
//! The availability classifier follows the selected build's actual emitted API;
//! Windows profiles currently do not generate C macro definitions.

#![cfg(not(target_os = "windows"))]
#![deny(unsafe_op_in_unsafe_fn)]

// Assertion-enabled C profiles skip this macro's stringification dependency.
// Test its generated API only when that API exists in the inspected profile.
pgrx::pg_sys::__pgrx_c_classify! { @if_available PageSetPrunable {
/// Register generated page-macro integration checks in the harness's tests schema.
#[pgrx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgrx_unit_tests;
    use core::mem::MaybeUninit;
    use core::ptr::{addr_of, addr_of_mut};
    use pgrx::pg_sys as pg;
    use pgrx::prelude::*;

    /// Check that the generated page macro short-circuits before a backend comparison.
    #[pg_test]
    fn generated_prunable_page_short_circuits_before_the_backend_comparison() {
        for input in [3_u32, u32::MAX] {
            let mut storage = MaybeUninit::<pg::PageHeaderData>::uninit();
            let header = addr_of_mut!(storage).cast::<pg::PageHeaderData>();
            let page = header as pg::Page;
            let xid = pg::TransactionId::from_inner(input);
            // SAFETY: header points into owned aligned live PageHeaderData storage
            // with exclusive access. Only pd_prune_xid is initialized and accessed;
            // no reference or value of the incomplete whole page is formed. Its
            // invalid initial xid makes the OR's first operand true, so the guarded
            // TransactionIdPrecedes backend function is never called. The input xid
            // is normal, so assertion-enabled builds need no failure callback either.
            unsafe {
                addr_of_mut!((*header).pd_prune_xid).write(pg::TransactionId::INVALID);
                pg::PageSetPrunable!(page, xid);
                assert_eq!(addr_of!((*header).pd_prune_xid).read().into_inner(), input);
            }
        }
    }

    /// Keep the general form typechecked without executing its guarded backend call.
    ///
    /// # Safety
    /// page points to live aligned page storage with initialized pd_prune_xid and
    /// exclusive write access. xid is normal. The caller runs in the permitted
    /// PostgreSQL backend thread and obeys the page's locking and pinning contract.
    #[allow(dead_code)]
    unsafe fn set_prunable_in_backend(page: pg::Page, xid: pg::TransactionId) {
        // SAFETY: the caller establishes page access and backend-call preconditions.
        unsafe { pg::PageSetPrunable!(page, xid) }
    }
}
} }
