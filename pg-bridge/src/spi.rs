use crate::pg_sys;
use crate::pg_sys::{SPI_execute, SPI_execute_with_args, SPI_finish};
use num_traits::FromPrimitive;
use pg_guard::*;

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

pub struct SpiClient {
    #[allow(dead_code)]
    spi: Spi,
}

impl Spi {
    pub fn connect<R, F: FnOnce(SpiClient) -> std::result::Result<R, SpiError>>(
        f: F,
    ) -> std::result::Result<R, SpiError> {
        Spi::check_status(unsafe { pg_sys::SPI_connect() })?;

        f(SpiClient { spi: Spi() })
    }

    pub fn check_status(status_code: i32) -> std::result::Result<SpiOk, SpiError> {
        if status_code > 0 {
            let status_enum = SpiOk::from_i32(status_code);
            match status_enum {
                Some(status) => Ok(status),
                None => panic!("unrecognized SPI status code {}", status_code),
            }
        } else {
            let status_enum = SpiError::from_i32(-status_code);
            match status_enum {
                Some(status) => Err(status),
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
        args: Option<Vec<(pg_sys::Oid, pg_sys::Datum)>>,
    ) -> std::result::Result<(SpiOk, u64), SpiError> {
        SpiClient::execute(query, true, limit, args)
    }

    /// perform any query (including utility statements) that modify the database in some way
    pub fn update(
        &mut self,
        query: &str,
        limit: Option<i64>,
        args: Option<Vec<(pg_sys::Oid, pg_sys::Datum)>>,
    ) -> std::result::Result<(SpiOk, u64), SpiError> {
        SpiClient::execute(query, false, limit, args)
    }

    fn execute(
        query: &str,
        read_only: bool,
        limit: Option<i64>,
        args: Option<Vec<(u32, usize)>>,
    ) -> Result<(SpiOk, u64), SpiError> {
        let src = std::ffi::CString::new(query).unwrap();
        let status_code = match args {
            Some(args) => {
                let nargs = args.len();
                let mut argtypes = vec![];
                let mut values = vec![];
                let mut nulls = vec![];

                for (argtype, value) in args {
                    let ptr = value as *mut std::os::raw::c_void;

                    nulls.push(ptr.is_null() as std::os::raw::c_char);
                    argtypes.push(argtype);
                    values.push(value);
                }

                unsafe {
                    SPI_execute_with_args(
                        src.as_ptr(),
                        nargs as i32,
                        argtypes.as_mut_ptr(),
                        values.as_mut_ptr(),
                        nulls.as_mut_ptr(),
                        read_only,
                        if limit.is_some() { limit.unwrap() } else { 0 },
                    )
                }
            }
            None => {
                let rc;
                unsafe {
                    rc = SPI_execute(
                        src.as_ptr(),
                        read_only,
                        if limit.is_some() { limit.unwrap() } else { 0 },
                    );
                }
                rc
            }
        };
        let status = Spi::check_status(status_code);
        match status {
            Ok(status) => Ok((status, unsafe { pg_sys::SPI_processed })),
            Err(e) => Err(e),
        }
    }
}

impl Drop for Spi {
    fn drop(&mut self) {
        match Spi::check_status(unsafe { SPI_finish() }) {
            Ok(_) => { /* we're good */ }
            Err(e) => panic!("problem calling SPI_finish: code={:?}", e),
        }
    }
}
