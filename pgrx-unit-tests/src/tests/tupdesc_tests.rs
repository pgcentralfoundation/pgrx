//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgrx_unit_tests;
    use pgrx::prelude::*;
    use pgrx::{PgRelation, PgTupleDesc};

    /// The typcache shares a table's relcache descriptor as the table's row type descriptor, so
    /// reading a row of that type takes and releases references to the descriptor that
    /// `tupdesc` wraps. Each header read sees the count PostgreSQL has written by then.
    #[pg_test]
    fn test_tuple_desc_header_reads_shared_refcount() -> Result<(), spi::Error> {
        Spi::run("CREATE TABLE header_dog (name text NOT NULL, scritches int4)")?;
        let typoid = Spi::get_one::<pg_sys::Oid>("SELECT 'header_dog'::regtype::oid")?.unwrap();
        let rel = PgRelation::open_with_name_and_share_lock("header_dog").unwrap();
        let tupdesc = rel.tuple_desc();
        assert_eq!(tupdesc.len(), 2);
        assert_eq!(tupdesc.oid(), typoid);
        assert_eq!(tupdesc.typmod(), -1);
        let constraints = tupdesc.constraints();
        assert!(!constraints.is_null());
        // SAFETY: `rel` keeps the descriptor and its constraints live, and the place read
        // copies the field without creating a reference.
        assert!(unsafe { (*constraints).has_not_null });
        let before = tupdesc.refcount().expect("relcache descriptors are reference counted");

        // The returned tuple holds one reference to the row type descriptor until it drops.
        let dog = Spi::get_one::<pgrx::composite_type!("header_dog")>(
            "SELECT ROW('Nami', 3)::header_dog",
        )?
        .unwrap();
        let held = tupdesc.refcount().unwrap();
        assert!(held > before, "held {held}, before {before}");
        drop(dog);
        assert_eq!(tupdesc.refcount(), Some(held - 1));
        Ok(())
    }

    #[pg_test]
    fn test_tuple_desc_copies_are_not_reference_counted() -> Result<(), spi::Error> {
        Spi::run("CREATE TYPE HeaderCat AS (name text, naps int4)")?;
        let typoid = Spi::get_one::<pg_sys::Oid>("SELECT 'HeaderCat'::regtype::oid")?.unwrap();
        let tupdesc = PgTupleDesc::for_composite_type_by_oid(typoid).unwrap();
        let clone = tupdesc.clone();
        assert_ne!(tupdesc.as_ptr(), clone.as_ptr());
        for copy in [&tupdesc, &clone] {
            assert_eq!(copy.len(), 2);
            assert_eq!(copy.oid(), typoid);
            assert_eq!(copy.typmod(), -1);
            assert_eq!(copy.refcount(), None);
            assert!(copy.constraints().is_null());
        }
        Ok(())
    }
}
