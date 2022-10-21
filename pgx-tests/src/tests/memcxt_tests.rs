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
}
