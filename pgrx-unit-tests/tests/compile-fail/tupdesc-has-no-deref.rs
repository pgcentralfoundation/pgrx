use pgrx::{PgTupleDesc, pg_sys};

// `PgTupleDesc` does not dereference to `pg_sys::TupleDescData`. PostgreSQL changes a shared
// descriptor's reference count while a `PgTupleDesc` for it exists, so a reference held across
// a call into PostgreSQL would be undefined behavior. The header accessors, such as `len` and
// `refcount`, read one field at a time.
fn main() {
    let tupdesc = PgTupleDesc::for_composite_type("Dog").unwrap();
    let header: &pg_sys::TupleDescData = &tupdesc;
    let natts = tupdesc.natts;
}
