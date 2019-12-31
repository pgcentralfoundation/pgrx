use crate::{pg_sys, DatumCompatible, PgDatum, PgMemoryContexts, PgOid};
use num_traits::FromPrimitive;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt::Debug;

pub trait SpiSend {}

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
    current: usize,
}

impl Spi {
    pub fn connect<R, F: FnOnce(SpiClient) -> std::result::Result<PgDatum<R>, SpiError>>(
        f: F,
    ) -> PgDatum<R>
    where
        R: DatumCompatible<R>,
    {
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
                let result = result.copy_into(&mut outer_memory_context);

                // disconnect from SPI
                Spi::check_status(unsafe { pg_sys::SPI_finish() });
                result
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
    /// perform a read-only SELECT statement
    pub fn select(
        &self,
        query: &str,
        limit: Option<i64>,
        args: Option<Vec<(PgOid, PgDatum<pg_sys::Datum>)>>,
    ) -> SpiTupleTable {
        SpiClient::execute(query, true, limit, args)
    }

    /// perform any query (including utility statements) that modify the database in some way
    pub fn update(
        &mut self,
        query: &str,
        limit: Option<i64>,
        args: Option<Vec<(PgOid, PgDatum<pg_sys::Datum>)>>,
    ) -> SpiTupleTable {
        SpiClient::execute(query, false, limit, args)
    }

    fn execute(
        query: &str,
        read_only: bool,
        limit: Option<i64>,
        args: Option<Vec<(PgOid, PgDatum<pg_sys::Datum>)>>,
    ) -> SpiTupleTable {
        unsafe {
            pg_sys::SPI_tuptable = std::ptr::null_mut();
        }

        let src = std::ffi::CString::new(query).unwrap();
        let status_code = match args {
            Some(args) => {
                let nargs = args.len();
                let mut argtypes = vec![];
                let mut values = vec![];
                let mut nulls = vec![];

                for (argtype, value) in args {
                    argtypes.push(argtype.value());
                    values.push(value.as_raw_pg_datum());
                    nulls.push(value.is_null() as std::os::raw::c_char);
                }

                unsafe {
                    pg_sys::SPI_execute_with_args(
                        src.as_ptr(),
                        nargs as i32,
                        argtypes.as_mut_ptr(),
                        values.as_mut_ptr(),
                        nulls.as_mut_ptr(),
                        read_only,
                        limit.unwrap_or(0),
                    )
                }
            }
            None => {
                let rc;
                unsafe {
                    rc = pg_sys::SPI_execute(src.as_ptr(), read_only, limit.unwrap_or(0));
                }
                rc
            }
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
            current: 0,
        }
    }
}

impl SpiTupleTable {
    pub fn get_one<A>(self) -> A
    where
        A: DatumCompatible<A> + SpiSend + TryFrom<PgDatum<A>>,
        <A as std::convert::TryFrom<PgDatum<A>>>::Error: std::fmt::Debug,
    {
        self.get_datum(1).try_into().unwrap()
    }

    pub fn get_two<A, B>(self) -> (A, B)
    where
        A: DatumCompatible<A> + SpiSend + TryFrom<PgDatum<A>>,
        <A as std::convert::TryFrom<PgDatum<A>>>::Error: std::fmt::Debug,

        B: DatumCompatible<B> + SpiSend + TryFrom<PgDatum<B>>,
        <B as std::convert::TryFrom<PgDatum<B>>>::Error: std::fmt::Debug,
    {
        let a = self.get_datum::<A>(1).try_into().unwrap();
        let b = self.get_datum::<B>(2).try_into().unwrap();
        (a, b)
    }

    pub fn get_three<A, B, C>(self) -> (A, B, C)
    where
        A: DatumCompatible<A> + SpiSend + TryFrom<PgDatum<A>>,
        <A as std::convert::TryFrom<PgDatum<A>>>::Error: std::fmt::Debug,

        B: DatumCompatible<B> + SpiSend + TryFrom<PgDatum<B>>,
        <B as std::convert::TryFrom<PgDatum<B>>>::Error: std::fmt::Debug,

        C: DatumCompatible<C> + SpiSend + TryFrom<PgDatum<C>>,
        <C as std::convert::TryFrom<PgDatum<C>>>::Error: std::fmt::Debug,
    {
        let a = self.get_datum::<A>(1).try_into().unwrap();
        let b = self.get_datum::<B>(2).try_into().unwrap();
        let c = self.get_datum::<C>(3).try_into().unwrap();
        (a, b, c)
    }

    pub fn get_datum<T>(&self, ordinal: i32) -> PgDatum<T>
    where
        T: DatumCompatible<T> + SpiSend,
    {
        match self.tupdesc {
            Some(tupdesc) => unsafe {
                let natts = (*tupdesc).natts;

                if ordinal < 1 || ordinal > natts {
                    panic!("invalid column ordinal {}", ordinal)
                } else {
                    let heap_tuple =
                        std::slice::from_raw_parts((*self.table).vals, self.size)[self.current];
                    let mut is_null = false;
                    let datum = pg_sys::SPI_getbinval(heap_tuple, tupdesc, ordinal, &mut is_null);

                    PgDatum::new(datum, is_null)
                }
            },
            None => panic!("TupDesc is NULL"),
        }
    }
    fn make_vec(&self) -> Option<Vec<PgDatum<pg_sys::Datum>>> {
        match self.tupdesc {
            Some(tupdesc) => {
                let natts = unsafe { (*tupdesc).natts };
                let mut row = Vec::with_capacity(natts as usize);
                let heap_tuple = unsafe {
                    std::slice::from_raw_parts((*self.table).vals, self.size)[self.current]
                };

                for i in 1..=natts {
                    let mut is_null = false;
                    let datum =
                        unsafe { pg_sys::SPI_getbinval(heap_tuple, tupdesc, i, &mut is_null) };
                    row.push(PgDatum::new(datum, is_null));
                }

                Some(row)
            }

            None => None,
        }
    }
}

impl Iterator for SpiTupleTable {
    type Item = Vec<PgDatum<pg_sys::Datum>>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.size {
            // reset the iterator back to the start
            self.current = 0;

            // and indicate that we're done
            None
        } else {
            let rc = self.nth(self.current);
            self.current += 1;
            rc
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

    #[allow(unused_mut)]
    #[inline]
    fn nth(&mut self, mut n: usize) -> Option<Self::Item> {
        if n >= self.size {
            None
        } else {
            self.make_vec()
        }
    }
}

impl<'a> SpiSend for &'a str {}
impl<'a> SpiSend for &'a pg_sys::varlena {}

impl SpiSend for i8 {}
impl SpiSend for i16 {}
impl SpiSend for i32 {}
impl SpiSend for i64 {}

impl SpiSend for u8 {}
impl SpiSend for u16 {}
impl SpiSend for u32 {}
impl SpiSend for u64 {}

impl SpiSend for f32 {}
impl SpiSend for f64 {}

impl SpiSend for bool {}
impl SpiSend for char {}
