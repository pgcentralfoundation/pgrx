// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.


use crate::{pg_sys, FromDatum, IntoDatum, Json, PgMemoryContexts, PgOid};
use enum_primitive_derive::*;
use num_traits::FromPrimitive;
use std::fmt::Debug;

#[derive(Debug, Primitive)]
pub enum SpiOk {
    Connect = 1,
    Finish = 2,
    Fetch = 3,
    Utility = 4,
    Select = 5,
    Selinto = 6,
    Insert = 7,
    Delete = 8,
    Update = 9,
    Cursor = 10,
    InsertReturning = 11,
    DeleteReturning = 12,
    UpdateReturning = 13,
    Rewritten = 14,
    RelRegister = 15,
    RelUnregister = 16,
    TdRegister = 17,
}

#[derive(Debug, Primitive)]
pub enum SpiError {
    // NB:  These are #define'd as negative, but we redefine them as positive so that
    // #[derive(Primitive)] will work.  We just need to negate result codes from the
    // various SPI_xxx functions when looking for errors
    Connect = 1,
    Copy = 2,
    Opunknown = 3,
    Unconnected = 4,
    #[allow(dead_code)]
    Cursor = 5, /* not used anymore */
    Argument = 6,
    Param = 7,
    Transaction = 8,
    Noattribute = 9,
    Nooutfunc = 10,
    Typunknown = 11,
    RelDuplicate = 12,
    RelNotFound = 13,
}

pub struct Spi();

pub struct SpiClient();

#[derive(Debug)]
pub struct SpiTupleTable {
    status_code: SpiOk,
    table: *mut pg_sys::SPITupleTable,
    size: usize,
    tupdesc: Option<pg_sys::TupleDesc>,
    current: isize,
}

pub struct SpiHeapTupleData {
    data: *mut pg_sys::HeapTupleData,
    tupdesc: pg_sys::TupleDesc,
}

impl Spi {
    pub fn get_one<A: FromDatum + IntoDatum>(query: &str) -> Option<A> {
        Spi::connect(|client| {
            let result = client.select(query, Some(1), None).first().get_one();
            Ok(result)
        })
    }

    pub fn get_two<A: FromDatum + IntoDatum, B: FromDatum + IntoDatum>(
        query: &str,
    ) -> (Option<A>, Option<B>) {
        Spi::connect(|client| {
            let (a, b) = client
                .select(query, Some(1), None)
                .first()
                .get_two::<A, B>();
            Ok(Some((a, b)))
        })
        .unwrap()
    }

    pub fn get_three<
        A: FromDatum + IntoDatum,
        B: FromDatum + IntoDatum,
        C: FromDatum + IntoDatum,
    >(
        query: &str,
    ) -> (Option<A>, Option<B>, Option<C>) {
        Spi::connect(|client| {
            let (a, b, c) = client
                .select(query, Some(1), None)
                .first()
                .get_three::<A, B, C>();
            Ok(Some((a, b, c)))
        })
        .unwrap()
    }

    pub fn get_one_with_args<A: FromDatum + IntoDatum>(
        query: &str,
        args: Vec<(PgOid, Option<pg_sys::Datum>)>,
    ) -> Option<A> {
        Spi::connect(|client| Ok(client.select(query, Some(1), Some(args)).first().get_one()))
    }

    pub fn get_two_with_args<A: FromDatum + IntoDatum, B: FromDatum + IntoDatum>(
        query: &str,
        args: Vec<(PgOid, Option<pg_sys::Datum>)>,
    ) -> (Option<A>, Option<B>) {
        Spi::connect(|client| {
            let (a, b) = client
                .select(query, Some(1), Some(args))
                .first()
                .get_two::<A, B>();
            Ok(Some((a, b)))
        })
        .unwrap()
    }

    pub fn get_three_with_args<
        A: FromDatum + IntoDatum,
        B: FromDatum + IntoDatum,
        C: FromDatum + IntoDatum,
    >(
        query: &str,
        args: Vec<(PgOid, Option<pg_sys::Datum>)>,
    ) -> (Option<A>, Option<B>, Option<C>) {
        Spi::connect(|client| {
            let (a, b, c) = client
                .select(query, Some(1), Some(args))
                .first()
                .get_three::<A, B, C>();
            Ok(Some((a, b, c)))
        })
        .unwrap()
    }

    /// just run an arbitrary SQL statement.
    ///
    /// ## Safety
    ///
    /// The statement runs in read/write mode
    pub fn run(query: &str) {
        Spi::execute(|mut client| {
            client.update(query, None, None);
        })
    }

    /// explain a query, returning its result in json form
    pub fn explain(query: &str) -> Json {
        Spi::connect(|mut client| {
            let table = client
                .update(&format!("EXPLAIN (format json) {}", query), None, None)
                .first();
            Ok(Some(
                table
                    .get_one::<Json>()
                    .expect("failed to get json EXPLAIN result"),
            ))
        })
        .unwrap()
    }

    /// execute SPI commands via the provided `SpiClient`
    pub fn execute<F: FnOnce(SpiClient) + std::panic::UnwindSafe>(f: F) {
        Spi::connect(|client| {
            f(client);
            Ok(Some(()))
        });
    }

    /// execute SPI commands via the provided `SpiClient` and return a value from SPI which is
    /// automatically copied into the `CurrentMemoryContext` at the time of this function call
    pub fn connect<
        R: FromDatum + IntoDatum,
        F: FnOnce(SpiClient) -> std::result::Result<Option<R>, SpiError>,
    >(
        f: F,
    ) -> Option<R> {
        let mut outer_memory_context =
            PgMemoryContexts::For(PgMemoryContexts::CurrentMemoryContext.value());

        // connect to SPI
        Spi::check_status(unsafe { pg_sys::SPI_connect() });

        // run the provided closure within the memory context that SPI_connect()
        // just put us un.  We'll disconnect from SPI when the closure is finished.
        // If there's a panic or elog(ERROR), we don't care about also disconnecting from
        // SPI b/c Postgres will do that for us automatically
        match f(SpiClient()) {
            // copy the result to the outer memory context we saved above
            Ok(result) => {
                // disconnect from SPI
                Spi::check_status(unsafe { pg_sys::SPI_finish() });

                match result {
                    Some(result) => {
                        let as_datum = result.into_datum();
                        if as_datum.is_none() {
                            // SPI function returned Some(()), which means we just want to return None
                            None
                        } else {
                            // transfer the returned datum to the outer memory context
                            outer_memory_context.switch_to(|_| {
                                // TODO:  can we get the type oid from somewhere?
                                //        - do we even need it?
                                unsafe {
                                    R::from_datum(
                                        as_datum.expect("SPI result datum was NULL"),
                                        false,
                                        pg_sys::InvalidOid,
                                    )
                                }
                            })
                        }
                    }
                    None => None,
                }
            }

            // closure returned an error
            Err(e) => {
                // disconnect from SPI
                Spi::check_status(unsafe { pg_sys::SPI_finish() });
                panic!(e)
            }
        }
    }

    pub fn check_status(status_code: i32) -> SpiOk {
        if status_code > 0 {
            let status_enum = SpiOk::from_i32(status_code);
            match status_enum {
                Some(ok) => ok,
                None => panic!("unrecognized SPI status code {}", status_code),
            }
        } else {
            let status_enum = SpiError::from_i32(-status_code);
            match status_enum {
                Some(e) => panic!(e),
                None => panic!("unrecognized SPI status code {}", status_code),
            }
        }
    }
}

impl SpiClient {
    /// perform a SELECT statement
    pub fn select(
        &self,
        query: &str,
        limit: Option<i64>,
        args: Option<Vec<(PgOid, Option<pg_sys::Datum>)>>,
    ) -> SpiTupleTable {
        // Postgres docs say:
        //
        //    It is generally unwise to mix read-only and read-write commands within a single function
        //    using SPI; that could result in very confusing behavior, since the read-only queries
        //    would not see the results of any database updates done by the read-write queries.
        //
        // As such, we don't actually set read-only to true here

        // TODO:  can we detect if the command counter (or something?) has incremented and if yes
        //        then we set read_only=false, else we can set it to true?
        //        Is this even a good idea?
        SpiClient::execute(query, false, limit, args)
    }

    /// perform any query (including utility statements) that modify the database in some way
    pub fn update(
        &mut self,
        query: &str,
        limit: Option<i64>,
        args: Option<Vec<(PgOid, Option<pg_sys::Datum>)>>,
    ) -> SpiTupleTable {
        SpiClient::execute(query, false, limit, args)
    }

    fn execute(
        query: &str,
        read_only: bool,
        limit: Option<i64>,
        args: Option<Vec<(PgOid, Option<pg_sys::Datum>)>>,
    ) -> SpiTupleTable {
        unsafe {
            pg_sys::SPI_tuptable = std::ptr::null_mut();
        }

        let src = std::ffi::CString::new(query).expect("query contained a null byte");
        let status_code = match args {
            Some(args) => {
                let nargs = args.len();
                let mut argtypes = vec![];
                let mut datums = vec![];
                let mut nulls = vec![];

                for (argtype, datum) in args {
                    argtypes.push(argtype.value());

                    match datum {
                        Some(datum) => {
                            datums.push(datum);
                            nulls.push(0 as std::os::raw::c_char);
                        }

                        None => {
                            datums.push(0);
                            nulls.push(1 as std::os::raw::c_char);
                        }
                    }
                }

                unsafe {
                    pg_sys::SPI_execute_with_args(
                        src.as_ptr(),
                        nargs as i32,
                        argtypes.as_mut_ptr(),
                        datums.as_mut_ptr(),
                        nulls.as_mut_ptr(),
                        read_only,
                        limit.unwrap_or(0),
                    )
                }
            }
            None => unsafe { pg_sys::SPI_execute(src.as_ptr(), read_only, limit.unwrap_or(0)) },
        };

        SpiTupleTable {
            status_code: Spi::check_status(status_code),
            table: unsafe { pg_sys::SPI_tuptable },
            size: unsafe { pg_sys::SPI_processed as usize },
            tupdesc: if unsafe { pg_sys::SPI_tuptable }.is_null() {
                None
            } else {
                Some(unsafe { (*pg_sys::SPI_tuptable).tupdesc })
            },
            current: -1,
        }
    }
}

impl SpiTupleTable {
    /// `SpiTupleTable`s are positioned before the start, for iteration purposes.
    ///
    /// This method moves the position to the first row.  If there are no rows, this
    /// method will silently return.
    pub fn first(mut self) -> Self {
        self.current = 0;
        self
    }

    /// How many rows were processed?
    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get_one<A: FromDatum>(&self) -> Option<A> {
        self.get_datum(1)
    }

    pub fn get_two<A: FromDatum, B: FromDatum>(&self) -> (Option<A>, Option<B>) {
        let a = self.get_datum::<A>(1);
        let b = self.get_datum::<B>(2);
        (a, b)
    }

    pub fn get_three<A: FromDatum, B: FromDatum, C: FromDatum>(
        &self,
    ) -> (Option<A>, Option<B>, Option<C>) {
        let a = self.get_datum::<A>(1);
        let b = self.get_datum::<B>(2);
        let c = self.get_datum::<C>(3);
        (a, b, c)
    }

    pub fn get_heap_tuple(&self) -> Option<SpiHeapTupleData> {
        if self.current < 0 {
            panic!("SpiTupleTable positioned before start")
        }
        if self.current as u64 >= unsafe { pg_sys::SPI_processed } {
            None
        } else {
            match self.tupdesc {
                Some(tupdesc) => unsafe {
                    let heap_tuple = std::slice::from_raw_parts((*self.table).vals, self.size)
                        [self.current as usize];
                    Some(SpiHeapTupleData {
                        data: heap_tuple,
                        tupdesc: tupdesc,
                    })
                },
                None => panic!("TupDesc is NULL"),
            }
        }
    }

    pub fn get_datum<T: FromDatum>(&self, ordinal: i32) -> Option<T> {
        if self.current < 0 {
            panic!("SpiTupleTable positioned before start")
        }
        if self.current as u64 >= unsafe { pg_sys::SPI_processed } {
            None
        } else {
            match self.tupdesc {
                Some(tupdesc) => unsafe {
                    let natts = (*tupdesc).natts;

                    if ordinal < 1 || ordinal > natts {
                        None
                    } else {
                        let heap_tuple = std::slice::from_raw_parts((*self.table).vals, self.size)
                            [self.current as usize];
                        let mut is_null = false;
                        let datum =
                            pg_sys::SPI_getbinval(heap_tuple, tupdesc, ordinal, &mut is_null);

                        T::from_datum(datum, is_null, pg_sys::SPI_gettypeid(tupdesc, ordinal))
                    }
                },
                None => panic!("TupDesc is NULL"),
            }
        }
    }
}

impl SpiHeapTupleData {
    pub fn get_datum<T: FromDatum>(&self, ordinal: i32) -> Option<T> {
        unsafe {
            let natts = (*self.tupdesc).natts;

            if ordinal < 1 || ordinal > natts {
                None
            } else {
                let mut is_null = false;
                let datum = pg_sys::SPI_getbinval(self.data, self.tupdesc, ordinal, &mut is_null);

                T::from_datum(datum, is_null, pg_sys::SPI_gettypeid(self.tupdesc, ordinal))
            }
        }
    }
}

impl Iterator for SpiTupleTable {
    type Item = SpiHeapTupleData;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.size as isize {
            self.current = -1;
            None
        } else {
            self.current += 1;
            assert!(self.current >= 0);
            self.get_heap_tuple()
        }
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.size))
    }

    #[inline]
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.size
    }

    // Removed this function as it comes with an iterator
    //fn nth(&mut self, mut n: usize) -> Option<Self::Item> {
}
