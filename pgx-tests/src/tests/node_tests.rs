// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.


#[cfg(any(test, feature = "pg_test"))]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;

    use pgx::*;

    #[pg_test]
    fn test_alloc_node_by_type() {
        let node = PgNodeFactory::makeIndexAmRoutine();
        assert_eq!(PgNode::IndexAmRoutine as u32, node.type_)
    }
}
