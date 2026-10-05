use pgrx::datum::Timestamp;
use pgrx::iter::TableIterator;
use pgrx::pg_sys;
use pgrx::prelude::*;

::pgrx::pg_module_magic!(name, version);

type TransactionId = pg_sys::TransactionId;

/// RAII wrapper for PostgreSQL LWLock to ensure proper release
///
/// This guard acquires a shared lock and releases that acquisition on ordinary
/// Rust scope exit. PostgreSQL error recovery has its own lock cleanup.
struct PgLwLockGuard {
    lock: *mut pg_sys::LWLock,
}

impl PgLwLockGuard {
    /// Acquire a PostgreSQL LWLock in shared mode with RAII cleanup
    ///
    /// # Safety
    /// The caller must ensure that:
    /// - `lock` is a valid, non-null pointer to an initialized LWLock
    /// - the call runs on PostgreSQL's initialized backend thread
    /// - The lock will not be destroyed before this guard is dropped
    unsafe fn new_shared(lock: *mut pg_sys::LWLock) -> Self {
        if lock.is_null() {
            panic!("Attempted to acquire null LWLock");
        }

        // SAFETY: the caller supplies a permanent initialized lock and runs on
        // the backend thread; this guard owns the resulting shared acquisition.
        unsafe { pg_sys::LWLockAcquire(lock, pg_sys::LWLockMode::LW_SHARED) };
        Self { lock }
    }
}

impl Drop for PgLwLockGuard {
    fn drop(&mut self) {
        // SAFETY: new_shared acquired this live lock for this guard, which
        // releases that acquisition once on ordinary scope exit.
        unsafe {
            pg_sys::LWLockRelease(self.lock);
        }
    }
}

// Transaction status constants to avoid duplication
const STATUS_IN_PROGRESS: &str = "in progress";
const STATUS_COMMITTED: &str = "committed";
const STATUS_ABORTED: &str = "aborted";

/// Preserve a transaction ID's bits in the SQL int4 output representation.
#[inline(always)]
fn transaction_id_to_i32(xid: TransactionId) -> i32 {
    u32::from(xid) as i32
}

/// Get the top-level parent transaction ID and subtransaction level
/// This is based on the get_top_parent function from the original C implementation
///
/// # Safety
/// The caller runs on the initialized backend thread and holds
/// XactTruncationLock in shared mode while reading the subtransaction state.
unsafe fn get_top_parent_xid(xid: TransactionId) -> (Option<TransactionId>, Option<i32>) {
    // SAFETY: the caller runs on the backend thread. TransactionXmin is its
    // initialized process-local horizon and comparison functions only read IDs.
    let xmin = unsafe { pg_sys::TransactionXmin };
    // Additional safety check - this should have been verified by caller but double-check
    // SAFETY: both values are initialized transaction IDs on the backend thread.
    if !unsafe { pg_sys::TransactionIdFollowsOrEquals(xid, xmin) } {
        // Cannot safely traverse subtrans for transactions older than TransactionXmin
        return (None, None);
    }

    let mut parent_xid = xid;
    let mut previous_xid = xid;
    let mut sub_level: i32 = -1;

    // Traverse the subtransaction hierarchy
    while pg_sys::TransactionIdIsValid!(parent_xid).get() != 0 {
        previous_xid = parent_xid;

        // Safety check: don't call SubTransGetParent on transactions older than TransactionXmin
        // SAFETY: both values are initialized IDs on the backend thread.
        if unsafe { pg_sys::TransactionIdPrecedes(parent_xid, xmin) } {
            break;
        }

        // SAFETY: the checks establish a valid ID at or after TransactionXmin;
        // the caller holds XactTruncationLock against concurrent truncation.
        parent_xid = unsafe { pg_sys::SubTransGetParent(parent_xid) };
        sub_level += 1;

        if pg_sys::TransactionIdIsValid!(parent_xid).get() == 0 {
            break;
        }

        // Safety check: parent xid should always precede child xid to avoid infinite loops
        // SAFETY: both values are initialized IDs on the backend thread.
        if !unsafe { pg_sys::TransactionIdPrecedes(parent_xid, previous_xid) } {
            error!(
                "pg_subtrans contains invalid entry: xid {} points to parent xid {}",
                previous_xid, parent_xid
            );
        }
    }

    // Return top parent and sublevel, or None if this is a top-level transaction
    if sub_level > 0 { (Some(previous_xid), Some(sub_level)) } else { (None, None) }
}

/// Check if transaction ID is in recent past and accessible
/// This implements the complete logic from the original C implementation with proper locking
/// Returns (extracted_xid, is_accessible) where is_accessible indicates if the transaction
/// data is still available in the CLOG/SLRU cache
///
/// This is a direct Rust translation of the PostgreSQL C function `TransactionIdInRecentPast`
/// from the subtrans_infos extension, adapted for PostgreSQL 14+ with FullTransactionId support.
///
/// # Safety
/// The caller runs on the initialized backend thread and holds
/// XactTruncationLock in shared mode while reading transaction metadata.
unsafe fn transaction_id_in_recent_past(
    xid_with_epoch: u64,
) -> Result<TransactionId, &'static str> {
    // For PostgreSQL 14+, use FullTransactionId APIs for proper epoch handling
    // SAFETY: the caller establishes backend access; the native constructor
    // stores the supplied bits in a FullTransactionId without dereferencing them.
    let full_xid = unsafe { pg_sys::FullTransactionIdFromU64(xid_with_epoch) };
    // SAFETY: the constructor initialized full_xid's value field; the macros
    // read only this owned local's initialized storage.
    let (xid_epoch, xid) = unsafe {
        (
            pg_sys::EpochFromFullTransactionId!(full_xid).get(),
            TransactionId::from(pg_sys::XidFromFullTransactionId!(full_xid).get()),
        )
    };

    // Basic validation - invalid transaction IDs are not accessible
    if pg_sys::TransactionIdIsValid!(xid).get() == 0 {
        return Err("invalid transaction ID");
    }

    // Special transaction IDs (bootstrap, frozen) are always accessible but don't need CLOG
    if pg_sys::TransactionIdIsNormal!(xid).get() == 0 {
        return Ok(xid);
    }

    // Get current full transaction ID for comparison
    // SAFETY: the caller runs on the initialized backend thread.
    let now_fullxid = unsafe { pg_sys::ReadNextFullTransactionId() };
    // SAFETY: ReadNextFullTransactionId initialized the owned record, and these
    // macros read only its value field.
    let (now_epoch_next_xid, now_epoch) = unsafe {
        (
            TransactionId::from(pg_sys::XidFromFullTransactionId!(now_fullxid).get()),
            pg_sys::EpochFromFullTransactionId!(now_fullxid).get(),
        )
    };
    let oldest_clog_xid = {
        #[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
        {
            TransactionId::FIRST_NORMAL
        }
        #[cfg(not(any(feature = "pg17", feature = "pg18", feature = "pg19")))]
        {
            // SAFETY: PostgreSQL initializes this permanent shared record before
            // backend calls; the caller's lock protects the truncation horizon.
            unsafe { (*pg_sys::ShmemVariableCache).oldestClogXid }
        }
    };

    // Check if the transaction ID is in the future - this is an error
    // For PostgreSQL 14+, we can compare FullTransactionId values directly
    // SAFETY: both owned records contain initialized value fields.
    if unsafe { pg_sys::FullTransactionIdFollowsOrEquals!(full_xid, now_fullxid).get() } != 0 {
        return Err("transaction ID is in the future");
    }

    // Check if the transaction has wrapped around too far - older than we can determine
    // A transaction that's more than one full epoch older is definitely too old
    // This implements the wraparound detection logic from the original C code
    if (xid_epoch + 1) < now_epoch
        // SAFETY: these comparisons receive initialized IDs on the backend thread.
        || ((xid_epoch + 1) == now_epoch && unsafe {
            pg_sys::TransactionIdPrecedes(xid, now_epoch_next_xid)
        })
        // If the XID is older than what's available in CLOG, it's not accessible
        || unsafe { pg_sys::TransactionIdPrecedes(xid, oldest_clog_xid) }
    {
        return Err("transaction ID is too old and CLOG data is unavailable");
    }

    // The transaction ID is recent enough and CLOG data is still available
    Ok(xid)
}

/// Determine transaction status and optionally get commit timestamp
/// This centralizes the status determination logic to avoid duplication
/// Returns (status, commit_timestamp)
///
/// # Safety
/// The caller runs on the initialized backend thread with an active transaction
/// and has established that xid's status data is still accessible.
unsafe fn get_transaction_status(xid: TransactionId) -> (String, Option<Timestamp>) {
    let mut commit_timestamp = None::<Timestamp>;
    // SAFETY: the caller establishes an active backend transaction. This owned
    // call does not change the lifetime of the returned transaction snapshot.
    let snapshot = unsafe { pg_sys::GetActiveSnapshot() };

    // SAFETY: xid is accessible under the caller's lock and all calls run on the
    // backend thread. A non-null active snapshot remains live for this call;
    // timestamp output points to initialized, exclusively borrowed local storage.
    let status = unsafe {
        if pg_sys::TransactionIdIsCurrentTransactionId(xid) {
            STATUS_IN_PROGRESS
        } else if pg_sys::TransactionIdDidCommit(xid) {
            // Try to get commit timestamp for committed transactions
            if pg_sys::track_commit_timestamp {
                let mut ts: pg_sys::TimestampTz = 0;
                if pg_sys::TransactionIdGetCommitTsData(xid, &mut ts, std::ptr::null_mut()) {
                    commit_timestamp = Some(Timestamp::saturating_from_raw(ts));
                }
            }
            STATUS_COMMITTED
        } else if pg_sys::TransactionIdDidAbort(xid)
            || (!snapshot.is_null() && pg_sys::TransactionIdPrecedes(xid, (*snapshot).xmin))
        {
            STATUS_ABORTED
        } else {
            STATUS_IN_PROGRESS
        }
    };

    (status.to_string(), commit_timestamp)
}

/// Main function that provides subtransaction information
///
/// # Safety
/// Call on PostgreSQL's initialized backend thread within an active transaction.
#[pg_extern]
unsafe fn subtrans_infos(
    xid_input: i64,
    _fcinfo: pg_sys::FunctionCallInfo,
) -> TableIterator<
    'static,
    (
        name!(xid, i32),
        name!(status, String),
        name!(parent_xid, Option<i32>),
        name!(top_parent_xid, Option<i32>),
        name!(sub_level, Option<i32>),
        name!(commit_timestamp, Option<Timestamp>),
    ),
> {
    let xid_input = xid_input as u64;

    // CRITICAL: Acquire the XactTruncationLock FIRST, just like in the original C implementation, This protects against concurrent SLRU cache truncation operations
    // SAFETY: PostgreSQL initializes the backend's MainLWLockArray before
    // pg_extern calls. The generated object macro provides the lock's address
    // from this installation's headers without forming a mutable reference.
    let lock = unsafe { pg_sys::XactTruncationLock!().get() };
    // SAFETY: the generated pointer names a permanent initialized lock, and this
    // backend call keeps the guard alive through all metadata reads below.
    let _lock_guard = unsafe { PgLwLockGuard::new_shared(lock) };

    // Check if the transaction is accessible and get the extracted XID
    // SAFETY: this backend transaction holds XactTruncationLock in shared mode.
    let xid = match unsafe { transaction_id_in_recent_past(xid_input) } {
        Ok(xid) => xid,
        Err(_err_msg) => {
            error!("Invalid transaction ID {}: {}", xid_input, _err_msg);
        }
    };

    // SAFETY CHECK: Before calling any SubTrans functions, we MUST verify that xid >= TransactionXmin to avoid violating PostgreSQL's assertion, This is the critical safety requirement that prevents crashes
    // SAFETY: the process-local horizon is initialized and xid is a validated
    // accessible ID; the comparison runs on the backend thread.
    if !unsafe { pg_sys::TransactionIdFollowsOrEquals(xid, pg_sys::TransactionXmin) } {
        // Transaction is too old and subtrans data is not available
        // SAFETY: validation established accessible status data under our lock.
        let (status, commit_timestamp) = unsafe { get_transaction_status(xid) };

        return TableIterator::once((
            transaction_id_to_i32(xid),
            status,
            None,             // parent_xid is null - subtrans data not available
            None,             // top_parent_xid is null - subtrans data not available
            None,             // sub_level is null - subtrans data not available
            commit_timestamp, // commit_timestamp may be null for old transactions
        ));
    }

    // Safe to access subtrans data - transaction is recent enough
    // SAFETY: xid is valid and at or after TransactionXmin; our shared lock
    // prevents truncation during these subtransaction reads.
    let (parent_xid, (top_parent_xid, sub_level)) =
        unsafe { (pg_sys::SubTransGetParent(xid), get_top_parent_xid(xid)) };

    // Determine transaction status using the centralized helper function
    // SAFETY: validation established accessible status data under our lock.
    let (status, commit_timestamp) = unsafe { get_transaction_status(xid) };

    let parent_xid_result = if pg_sys::TransactionIdIsValid!(parent_xid).get() != 0 {
        Some(transaction_id_to_i32(parent_xid))
    } else {
        None
    };

    let top_parent_xid_result = top_parent_xid.map(|x| transaction_id_to_i32(x));

    TableIterator::once((
        transaction_id_to_i32(xid),
        status,
        parent_xid_result,
        top_parent_xid_result,
        sub_level,
        commit_timestamp,
    ))
}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    #[must_use]
    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}

/// Comprehensive unit tests (run outside PostgreSQL context)
#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_transaction_id_to_i32() {
        // Test normal transaction ID conversion
        let xid = pg_sys::TransactionId::from(12345u32);
        assert_eq!(transaction_id_to_i32(xid), 12345i32);

        // Test maximum safe positive value
        let max_safe_xid = pg_sys::TransactionId::from(i32::MAX as u32);
        assert_eq!(transaction_id_to_i32(max_safe_xid), i32::MAX);

        // Test wrap-around case (values above i32::MAX)
        let wrap_xid = pg_sys::TransactionId::from(i32::MAX as u32 + 1);
        assert_eq!(transaction_id_to_i32(wrap_xid), i32::MIN);
    }

    /// Validate the generated predicate across the special and normal ID ranges.
    #[test]
    fn generated_transaction_ids_reject_only_the_invalid_id() {
        for (bits, valid) in [(0_u32, false), (1, true), (2, true), (3, true), (u32::MAX, true)] {
            let xid = pg_sys::TransactionId::from(bits);
            assert_eq!(pg_sys::TransactionIdIsValid!(xid).get() != 0, valid);
        }
    }

    #[test]
    fn test_status_constants() {
        // Ensure status constants are correctly defined
        assert_eq!(STATUS_IN_PROGRESS, "in progress");
        assert_eq!(STATUS_COMMITTED, "committed");
        assert_eq!(STATUS_ABORTED, "aborted");
    }
}

#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    use pgrx::Spi;
    use pgrx::prelude::*;

    /// Exercise the lock address selected by this installation's generated macro.
    #[pg_test]
    fn generated_truncation_lock_can_be_acquired() {
        // SAFETY: pg_test runs in an initialized backend. The generated pointer
        // names the permanent XactTruncationLock; this guard alone owns this
        // test's shared acquisition and releases it before returning.
        unsafe {
            let lock = pg_sys::XactTruncationLock!().get();
            assert!(!lock.is_null());
            let _guard = super::PgLwLockGuard::new_shared(lock);
        }
    }

    /// Test basic functionality with current transaction ID
    #[pg_test]
    fn test_basic_function() {
        Spi::connect(|client| {
            let current_xid = unsafe { pg_sys::GetCurrentTransactionId() };
            let query = format!("SELECT * FROM subtrans_infos({})", current_xid.into_inner());

            let mut count = 0;
            for _row in client.select(&query, None, &[])? {
                count += 1;
            }

            assert!(count > 0, "Function should return at least one row");
            Ok(Some(())) as Result<Option<()>, pgrx::spi::SpiError>
        })
        .unwrap();
    }

    /// Test with Bootstrap transaction ID
    #[pg_test]
    fn test_bootstrap_xid() {
        Spi::connect(|client| {
            let mut count = 0;
            for _row in client.select("SELECT * FROM subtrans_infos(1)", None, &[])? {
                count += 1;
            }
            assert_eq!(count, 1, "Bootstrap XID should return exactly one row");
            Ok(Some(())) as Result<Option<()>, pgrx::spi::SpiError>
        })
        .unwrap();
    }

    /// Test with Frozen transaction ID
    #[pg_test]
    fn test_frozen_xid() {
        Spi::connect(|client| {
            let mut count = 0;
            for _row in client.select("SELECT * FROM subtrans_infos(2)", None, &[])? {
                count += 1;
            }
            assert_eq!(count, 1, "Frozen XID should return exactly one row");
            Ok(Some(())) as Result<Option<()>, pgrx::spi::SpiError>
        })
        .unwrap();
    }

    /// Test multiple calls for consistency
    #[pg_test]
    fn test_consistency() {
        let current_xid = unsafe { pg_sys::GetCurrentTransactionId() };

        for _i in 0..3 {
            Spi::connect(|client| {
                let query = format!("SELECT * FROM subtrans_infos({})", current_xid.into_inner());
                let mut count = 0;
                for _row in client.select(&query, None, &[])? {
                    count += 1;
                }
                assert!(count > 0, "Iteration should return at least one row");
                Ok(Some(())) as Result<Option<()>, pgrx::spi::SpiError>
            })
            .unwrap();
        }
    }

    /// Test PostgreSQL version compatibility
    #[pg_test]
    fn test_version_compatibility() {
        Spi::connect(|client| {
            let current_xid = unsafe { pg_sys::GetCurrentTransactionId() };
            let xid_with_epoch = current_xid.into_inner() as u64;
            let query = format!("SELECT * FROM subtrans_infos({})", xid_with_epoch);

            let mut count = 0;
            for _row in client.select(&query, None, &[])? {
                count += 1;
            }

            assert!(count > 0, "Function should handle transaction IDs correctly");
            Ok(Some(())) as Result<Option<()>, pgrx::spi::SpiError>
        })
        .unwrap();
    }

    /// Test error recovery
    #[pg_test]
    fn test_error_handling() {
        // After any potential error, function should still work
        Spi::connect(|client| {
            let current_xid = unsafe { pg_sys::GetCurrentTransactionId() };
            let query = format!("SELECT * FROM subtrans_infos({})", current_xid.into_inner());
            let mut count = 0;
            for _row in client.select(&query, None, &[])? {
                count += 1;
            }
            assert!(count > 0, "Function should work reliably");
            Ok(Some(())) as Result<Option<()>, pgrx::spi::SpiError>
        })
        .unwrap();
    }
}
