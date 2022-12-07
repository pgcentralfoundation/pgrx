/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

#[cfg(any(test, feature = "pg_test"))]
#[pgx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;

    use pgx::prelude::*;
    use pgx::PgMemoryContexts;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    struct TestObject {
        did_drop: Arc<AtomicBool>,
    }

    impl Drop for TestObject {
        fn drop(&mut self) {
            self.did_drop.store(true, Ordering::SeqCst);
        }
    }

    #[pg_test]
    fn test_leak_and_drop() {
        let did_drop = Arc::new(AtomicBool::new(false));

        PgMemoryContexts::Transient {
            parent: PgMemoryContexts::CurrentMemoryContext.value(),
            name: "test",
            min_context_size: 4096,
            initial_block_size: 4096,
            max_block_size: 4096,
        }
        .switch_to(|context| {
            let test_object = TestObject { did_drop: did_drop.clone() };
            context.leak_and_drop_on_delete(test_object);
        });

        assert!(did_drop.load(Ordering::SeqCst))
    }

    #[pg_test]
    fn parent() {
        assert!(PgMemoryContexts::TopMemoryContext.parent().is_none());
        assert!(PgMemoryContexts::CurrentMemoryContext.parent().is_some());
    }

    #[pg_test]
    fn switch_to_should_switch_back_on_panic() {
        let mut ctx = PgMemoryContexts::new("test");
        let ctx_sys = ctx.value();
        PgTryBuilder::new(move || {
            ctx.switch_to(|_| {
                assert_eq!(unsafe { pg_sys::CurrentMemoryContext }, ctx_sys);
                panic!();
            });
        })
        .catch_others(|_| {})
        .execute();
        assert_ne!(unsafe { pg_sys::CurrentMemoryContext }, ctx_sys);
    }

    #[pg_test]
    fn test_current_owned_memory_context_drop() {
        let mut ctx = PgMemoryContexts::new("test");
        let mut another_ctx = PgMemoryContexts::new("another");
        another_ctx.set_as_current();
        ctx.set_as_current();
        assert_eq!(unsafe { pg_sys::CurrentMemoryContext }, ctx.value());
        drop(ctx);
        assert_eq!(unsafe { pg_sys::CurrentMemoryContext }, another_ctx.value());
    }

    #[pg_test]
    fn test_current_owned_memory_context_drop_handle_children() {
        let mut ctx = PgMemoryContexts::new("test");
        let ctx_parent = ctx.parent().unwrap().value();
        ctx.set_as_current();
        let mut ctx1 = PgMemoryContexts::new("test1");
        ctx1.set_as_current();
        let mut ctx2 = PgMemoryContexts::new("test2");
        ctx2.set_as_current();
        drop(ctx); // drop the parent of ctx1
        assert_eq!(unsafe { pg_sys::CurrentMemoryContext }, ctx_parent);
    }

    #[pg_test]
    fn test_current_owned_memory_context_drop_handle_sibling_children() {
        let mut ctx = PgMemoryContexts::new("test");
        let ctx_parent = ctx.parent().unwrap().value();
        ctx.set_as_current();
        let mut _ctx1 = PgMemoryContexts::new("test1");
        let mut ctx2 = PgMemoryContexts::new("test2");
        ctx2.set_as_current();
        drop(ctx); // drop the parent of _ctx1 and ctx2
        assert_eq!(unsafe { pg_sys::CurrentMemoryContext }, ctx_parent);
    }

    #[pg_test]
    fn test_current_owned_memory_context_drop_when_set_current_twice() {
        let ctx_parent = PgMemoryContexts::CurrentMemoryContext.value();
        let mut ctx = PgMemoryContexts::new("test");
        ctx.set_as_current();
        ctx.set_as_current();
        drop(ctx);
        assert_eq!(unsafe { pg_sys::CurrentMemoryContext }, ctx_parent);
    }
}
