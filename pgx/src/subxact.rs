use crate::{pg_sys, PgMemoryContexts, SpiClient};
use std::fmt::Debug;
use std::ops::Deref;

/// Releases a sub-transaction on Drop
pub trait ReleaseOnDrop {}

/// Sub-transaction's contextual information
#[derive(Clone, Copy)]
pub struct Context {
    memory_context: pg_sys::MemoryContext,
    // Resource ownership before the transaction
    //
    // Based on information from src/backend/utils/resowner/README
    // as well as practical use of it in src/pl/plpython/plpy_spi.c
    resource_owner: pg_sys::ResourceOwner,
}

impl Context {
    /// Captures the context
    fn capture() -> Self {
        // Remember the memory context before starting the sub-transaction
        let memory_context = PgMemoryContexts::CurrentMemoryContext.value();
        // Remember resource owner before starting the sub-transaction
        let resource_owner = unsafe { pg_sys::CurrentResourceOwner };
        Self { memory_context, resource_owner }
    }
}

impl From<Context> for CommitOnDrop {
    fn from(context: Context) -> Self {
        CommitOnDrop(context)
    }
}

impl From<Context> for RollbackOnDrop {
    fn from(context: Context) -> Self {
        RollbackOnDrop(context)
    }
}

/// Commits a sub-transaction on Drop
pub struct CommitOnDrop(Context);

impl Drop for CommitOnDrop {
    fn drop(&mut self) {
        unsafe {
            pg_sys::ReleaseCurrentSubTransaction();
            pg_sys::CurrentResourceOwner = self.0.resource_owner;
        }
        PgMemoryContexts::For(self.0.memory_context).set_as_current();
    }
}

impl ReleaseOnDrop for CommitOnDrop {}

/// Rolls back a sub-transaction on Drop
pub struct RollbackOnDrop(Context);

impl Drop for RollbackOnDrop {
    fn drop(&mut self) {
        unsafe {
            pg_sys::RollbackAndReleaseCurrentSubTransaction();
            pg_sys::CurrentResourceOwner = self.0.resource_owner;
        }
        PgMemoryContexts::For(self.0.memory_context).set_as_current();
    }
}

impl ReleaseOnDrop for RollbackOnDrop {}

impl Into<RollbackOnDrop> for CommitOnDrop {
    fn into(self) -> RollbackOnDrop {
        let result = RollbackOnDrop(self.0);
        // IMPORTANT: avoid running Drop (that would commit)
        std::mem::forget(self);
        result
    }
}

impl Into<CommitOnDrop> for RollbackOnDrop {
    fn into(self) -> CommitOnDrop {
        let result = CommitOnDrop(self.0);
        // IMPORTANT: avoid running Drop (that would roll back)
        std::mem::forget(self);
        result
    }
}

struct NoOpOnDrop;

impl ReleaseOnDrop for NoOpOnDrop {}

/// Sub-transaction
///
/// Can be created by calling `SpiClient::sub_transaction`, `SubTransaction<Parent>::sub_transaction`
/// or any other implementation of `SubTransactionExt` and obtaining it as an argument to the provided closure.
///
/// Unless rolled back or committed explicitly, it'll commit if `Release` generic parameter is `CommitOnDrop`
/// (default) or roll back if it is `RollbackOnDrop`.
#[derive(Debug)]
pub struct SubTransaction<Parent: SubTransactionExt, Release: ReleaseOnDrop = CommitOnDrop> {
    // Transaction release mechanism (commit, drop)
    release: Release,
    // Transaction parent
    parent: Parent,
}

impl<Parent: SubTransactionExt, Release: ReleaseOnDrop> SubTransaction<Parent, Release>
where
    Release: From<Context>,
{
    /// Create a new sub-transaction.
    fn new(parent: Parent) -> Self {
        let context = Context::capture();
        let memory_context = context.memory_context;
        let release = context.into();
        unsafe {
            pg_sys::BeginInternalSubTransaction(std::ptr::null() /* [no] transaction name */);
        }
        // Switch to the outer memory context so that all allocations remain
        // there instead of the sub-transaction's context
        PgMemoryContexts::For(memory_context).set_as_current();
        Self { release, parent }
    }
}

impl<Parent: SubTransactionExt> SubTransaction<Parent, CommitOnDrop> {
    /// Commit the transaction, returning its parent
    pub fn commit(self) -> Parent {
        // `Self::do_nothing_on_drop()` will commit as `Release` is `CommitOnDrop`
        self.do_nothing_on_drop().parent
    }
}

impl<Parent: SubTransactionExt> SubTransaction<Parent, RollbackOnDrop> {
    /// Commit the transaction, returning its parent
    pub fn commit(self) -> Parent {
        // Make sub-transaction commit on drop and then use `commit`
        self.commit_on_drop().commit()
    }
}

impl<Parent: SubTransactionExt> SubTransaction<Parent, RollbackOnDrop> {
    /// Rollback the transaction, returning its parent
    pub fn rollback(self) -> Parent {
        // `Self::do_nothing_on_drop()` will roll back as `Release` is `RollbackOnDrop`
        self.do_nothing_on_drop().parent
    }
}

impl<Parent: SubTransactionExt> SubTransaction<Parent, CommitOnDrop> {
    /// Rollback the transaction, returning its parent
    pub fn rollback(self) -> Parent {
        // Make sub-transaction roll back on drop and then use `rollback`
        self.rollback_on_drop().rollback()
    }
}

impl<Parent: SubTransactionExt> SubTransaction<Parent, CommitOnDrop> {
    /// Make this sub-transaction roll back on drop
    pub fn rollback_on_drop(self) -> SubTransaction<Parent, RollbackOnDrop> {
        SubTransaction { parent: self.parent, release: self.release.into() }
    }
}

impl<Parent: SubTransactionExt> SubTransaction<Parent, RollbackOnDrop> {
    /// Make this sub-transaction commit on drop
    pub fn commit_on_drop(self) -> SubTransaction<Parent, CommitOnDrop> {
        SubTransaction { parent: self.parent, release: self.release.into() }
    }
}

impl<Parent: SubTransactionExt, Release: ReleaseOnDrop> SubTransaction<Parent, Release> {
    /// Make this sub-transaction do nothing on drop
    ///
    /// Releases the sub-transaction based on `Release` generic parameter. Further
    /// dropping of the sub-transaction will not do anything.
    fn do_nothing_on_drop(self) -> SubTransaction<Parent, NoOpOnDrop> {
        SubTransaction { parent: self.parent, release: NoOpOnDrop }
    }
}

// This allows SubTransaction to be de-referenced to SpiClient
impl<'conn, Release: ReleaseOnDrop> Deref for SubTransaction<SpiClient<'conn>, Release> {
    type Target = SpiClient<'conn>;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

// This allows a SubTransaction of a SubTransaction to be de-referenced to SpiClient
impl<Parent: SubTransactionExt, Release: ReleaseOnDrop> Deref
    for SubTransaction<SubTransaction<Parent>, Release>
{
    type Target = Parent;

    fn deref(&self) -> &Self::Target {
        &self.parent.parent
    }
}

/// Trait that allows creating a sub-transaction off any type
pub trait SubTransactionExt {
    /// Parent's type
    ///
    /// In most common cases, it'll be equal to `Self`. However, in some cases
    /// it may be desirable to use a different type to achieve certain goals.
    type Parent: SubTransactionExt;

    /// Consume `self` and execute a closure with a sub-transaction
    ///
    /// If further use of the given sub-transaction is necessary, it must
    /// be returned by the closure alongside with its intended result. Otherwise,
    /// the sub-transaction be released when dropped.
    fn sub_transaction<F: FnOnce(SubTransaction<Self::Parent>) -> R, R>(self, f: F) -> R
    where
        Self: Sized;
}

impl<'a> SubTransactionExt for SpiClient<'a> {
    type Parent = Self;
    fn sub_transaction<F: FnOnce(SubTransaction<Self::Parent>) -> R, R>(self, f: F) -> R
    where
        Self: Sized,
    {
        f(SubTransaction::new(self))
    }
}

impl<Parent: SubTransactionExt> SubTransactionExt for SubTransaction<Parent> {
    type Parent = Self;
    fn sub_transaction<F: FnOnce(SubTransaction<Self::Parent>) -> R, R>(self, f: F) -> R
    where
        Self: Sized,
    {
        f(SubTransaction::new(self))
    }
}
