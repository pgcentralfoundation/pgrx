//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use core::ffi::CStr;
use pgrx::datum::PgVarlena;
use pgrx::prelude::*;
use pgrx::{PgVarlenaInOutFuncs, StringInfo};

/// Too large for a one-byte varlena header, so its Datums have the four-byte header that
/// detoasting leaves in place.
#[derive(Copy, Clone, PostgresType)]
#[bikeshed_postgres_type_manually_impl_from_into_datum]
#[pgvarlena_inoutfuncs]
pub struct PgVarlenaCounter {
    count: i64,
    #[allow(dead_code, reason = "only sizes the type past a one-byte header")]
    filler: [i64; 25],
}

impl PgVarlenaInOutFuncs for PgVarlenaCounter {
    fn input(input: &CStr) -> PgVarlena<Self> {
        let mut counter = PgVarlena::<Self>::new();
        counter.count = input.to_str().expect("utf-8").parse().expect("an i64");
        counter
    }

    fn output(&self, buffer: &mut StringInfo) {
        buffer.push_str(&self.count.to_string());
    }
}

/// Change an argument, which copies it first when its Datum was borrowed.
#[pg_extern]
fn pgvarlena_counter_increment(mut counter: PgVarlena<PgVarlenaCounter>) -> i64 {
    counter.count += 1;
    counter.count
}

#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgrx_unit_tests;

    use pgrx::prelude::*;

    /// A table row stores the counter after an int4, at the type's four-byte alignment, so a
    /// scan passes a pointer into a shared buffer that is not MAXALIGNed. Changing that
    /// borrowed value must not read a memory chunk header before it.
    #[pg_test(error = "cannot find the memory context of a varlena that is not MAXALIGNed")]
    fn test_pgvarlena_change_rejects_a_misaligned_borrowed_value() -> Result<(), spi::Error> {
        Spi::run("CREATE TABLE pgvarlena_counter_rows (id int4, counter PgVarlenaCounter)")?;
        Spi::run("INSERT INTO pgvarlena_counter_rows VALUES (1, '41')")?;
        Spi::get_one::<i64>(
            "SELECT pgvarlena_counter_increment(counter) FROM pgvarlena_counter_rows",
        )?;
        Ok(())
    }
}
