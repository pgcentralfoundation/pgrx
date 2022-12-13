use crate::{pg_sys, PgMemoryContexts, SpiClient};
use std::fmt::Debug;
use std::ops::Deref;

/// Sub-transaction
///
/// Can be created by calling `SpiClient::sub_transaction`, `SubTransaction<Parent>::sub_transaction`
/// or any other implementation of `SubTransactionExt` and obtaining it as an argument to the provided closure.
///
/// Unless rolled back or committed explicitly, it'll commit if `COMMIT` generic parameter is `true`
/// (default) or roll back if it is `false`.
#[derive(Debug)]
pub struct SubTransaction<Parent: SubTransactionExt, const COMMIT: bool = true> {
    memory_context: pg_sys::MemoryContext,
    resource_owner: pg_sys::ResourceOwner,
    // Should the transaction be released, or was it already committed or rolled back?
    //
    // The reason we are not calling this `released` as we're also using this flag when
    // we convert between commit_on_drop and rollback_on_drop to ensure it doesn't get released
    // on the drop of the original value.
    should_release: bool,
    parent: Option<Parent>,
}

impl<Parent: SubTransactionExt, const COMMIT: bool> SubTransaction<Parent, COMMIT> {
    /// Create a new sub-transaction.
    fn new(parent: Parent) -> Self {
        // Remember the memory context before starting the sub-transaction
        let ctx = PgMemoryContexts::CurrentMemoryContext.value();
        // Remember resource owner before starting the sub-transaction
        let resource_owner = unsafe { pg_sys::CurrentResourceOwner };
        unsafe {
            pg_sys::BeginInternalSubTransaction(std::ptr::null());
        }
        // Switch to the outer memory context so that all allocations remain
        // there instead of the sub-transaction's context
        PgMemoryContexts::For(ctx).set_as_current();
        Self { memory_context: ctx, should_release: true, resource_owner, parent: Some(parent) }
    }

    /// Commit the transaction, returning its parent
    pub fn commit(mut self) -> Parent {
        self.internal_commit();
        self.should_release = false;
        self.parent.take().unwrap()
    }

    /// Rollback the transaction, returning its parent
    pub fn rollback(mut self) -> Parent {
        self.internal_rollback();
        self.should_release = false;
        self.parent.take().unwrap()
    }

    /// Returns the memory context this transaction is in
    pub fn memory_context(&self) -> PgMemoryContexts {
        PgMemoryContexts::For(self.memory_context)
    }

    fn internal_rollback(&self) {
        unsafe {
            pg_sys::RollbackAndReleaseCurrentSubTransaction();
            pg_sys::CurrentResourceOwner = self.resource_owner;
        }
        PgMemoryContexts::For(self.memory_context).set_as_current();
    }

    fn internal_commit(&self) {
        unsafe {
            pg_sys::ReleaseCurrentSubTransaction();
            pg_sys::CurrentResourceOwner = self.resource_owner;
        }
        PgMemoryContexts::For(self.memory_context).set_as_current();
    }
}

impl<Parent: SubTransactionExt> SubTransaction<Parent, true> {
    /// Make this sub-transaction roll back on drop
    pub fn rollback_on_drop(self) -> SubTransaction<Parent, false> {
        self.into()
    }
}

impl<Parent: SubTransactionExt> SubTransaction<Parent, false> {
    /// Make this sub-transaction commit on drop
    pub fn commit_on_drop(self) -> SubTransaction<Parent, true> {
        self.into()
    }
}

impl<Parent: SubTransactionExt> Into<SubTransaction<Parent, false>>
    for SubTransaction<Parent, true>
{
    fn into(mut self) -> SubTransaction<Parent, false> {
        let result = SubTransaction {
            memory_context: self.memory_context,
            resource_owner: self.resource_owner,
            should_release: self.should_release,
            parent: self.parent.take(),
        };
        // Make sure original sub-transaction won't commit
        self.should_release = false;
        result
    }
}

impl<Parent: SubTransactionExt> Into<SubTransaction<Parent, true>>
    for SubTransaction<Parent, false>
{
    fn into(mut self) -> SubTransaction<Parent, true> {
        let result = SubTransaction {
            memory_context: self.memory_context,
            resource_owner: self.resource_owner,
            should_release: self.should_release,
            parent: self.parent.take(),
        };
        // Make sure original sub-transaction won't roll back
        self.should_release = false;
        result
    }
}

impl<Parent: SubTransactionExt, const COMMIT: bool> Drop for SubTransaction<Parent, COMMIT> {
    fn drop(&mut self) {
        if self.should_release {
            if COMMIT {
                self.internal_commit();
            } else {
                self.internal_rollback();
            }
        }
    }
}

// This allows SubTransaction to be de-referenced to SpiClient
impl<'conn, const COMMIT: bool> Deref for SubTransaction<SpiClient<'conn>, COMMIT> {
    type Target = SpiClient<'conn>;

    fn deref(&self) -> &Self::Target {
        self.parent.as_ref().unwrap()
    }
}

// This allows a SubTransaction of a SubTransaction to be de-referenced to SpiClient
impl<Parent: SubTransactionExt, const COMMIT: bool> Deref
    for SubTransaction<SubTransaction<Parent>, COMMIT>
{
    type Target = Parent;

    fn deref(&self) -> &Self::Target {
        self.parent.as_ref().and_then(|p| p.parent.as_ref()).unwrap()
    }
}

/// Trait that allows creating a sub-transaction off any type
pub trait SubTransactionExt {
    /// Parent's type
    ///
    /// In most common cases, it'll be equal to `Self`. However, in some cases
    /// it may be desirable to use a different type to achieve certain goals.
    type T: SubTransactionExt;

    /// Consume `self` and execute a closure with a sub-transaction
    ///
    /// If further use of the given sub-transaction is necessary, it must
    /// be returned by the closure alongside with its intended result. Otherwise,
    /// the sub-transaction be released when dropped.
    fn sub_transaction<F: FnOnce(SubTransaction<Self::T>) -> R, R>(self, f: F) -> R
    where
        Self: Sized;
}

impl<'a> SubTransactionExt for SpiClient<'a> {
    type T = Self;
    fn sub_transaction<F: FnOnce(SubTransaction<Self::T>) -> R, R>(self, f: F) -> R
    where
        Self: Sized,
    {
        f(SubTransaction::new(self))
    }
}

impl<Parent: SubTransactionExt> SubTransactionExt for SubTransaction<Parent> {
    type T = Self;
    fn sub_transaction<F: FnOnce(SubTransaction<Self::T>) -> R, R>(self, f: F) -> R
    where
        Self: Sized,
    {
        f(SubTransaction::new(self))
    }
}
