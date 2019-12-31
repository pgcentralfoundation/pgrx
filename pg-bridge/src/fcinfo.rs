use crate::{pg_sys, rust_str_to_text_p, DatumCompatible, PgDatum};

cfg_if::cfg_if! {
    if #[cfg(feature = "pg10")] {
        #[inline]
        pub fn pg_getarg<T>(
            fcinfo: pg_sys::FunctionCallInfo,
            num: usize,
        ) -> PgDatum<T>
        where
            T: DatumCompatible<T>,
        {
            PgDatum::<T>::new(
                unsafe { fcinfo.as_ref() }.unwrap().arg[num],
                unsafe { fcinfo.as_ref() }.unwrap().argnull[num] as bool,
            )
        }

        #[inline]
        pub fn pg_arg_is_null(
            fcinfo: pg_sys::FunctionCallInfo,
            num: usize,
        ) -> bool {
            unsafe { fcinfo.as_ref() }.unwrap().argnull[num] as bool
        }

        #[inline]
        pub fn pg_getarg_datum(
            fcinfo: pg_sys::FunctionCallInfo,
            num: usize,
        ) -> Option<pg_sys::Datum> {
            if pg_arg_is_null(fcinfo, num) {
                None
            } else {
                Some(unsafe { fcinfo.as_ref() }.unwrap().arg[num])
            }
        }

        #[inline]
        pub fn pg_return_null(fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
            unsafe { fcinfo.as_mut() }.unwrap().isnull = true;
            0 as pg_sys::Datum
        }
    } else if #[cfg(feature = "pg11")] {
        #[inline]
        pub fn pg_getarg<T>(
            fcinfo: pg_sys::FunctionCallInfo,
            num: usize,
        ) -> PgDatum<T>
        where
            T: DatumCompatible<T>,
        {
            PgDatum::<T>::new(
                unsafe { fcinfo.as_ref() }.unwrap().arg[num],
                unsafe { fcinfo.as_ref() }.unwrap().argnull[num] as bool,
            )
        }

        #[inline]
        pub fn pg_arg_is_null(
            fcinfo: pg_sys::FunctionCallInfo,
            num: usize,
        ) -> bool {
            unsafe { fcinfo.as_ref() }.unwrap().argnull[num] as bool
        }

        #[inline]
        pub fn pg_getarg_datum(
            fcinfo: pg_sys::FunctionCallInfo,
            num: usize,
        ) -> Option<pg_sys::Datum> {
            if pg_arg_is_null(fcinfo, num) {
                None
            } else {
                Some(unsafe { fcinfo.as_ref() }.unwrap().arg[num])
            }
        }

        #[inline]
        pub fn pg_return_null(fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
            unsafe { fcinfo.as_mut() }.unwrap().isnull = true;
            0 as pg_sys::Datum
        }
    } else if #[cfg(feature = "pg12")] {
        #[inline]
        pub fn pg_getarg<T>(
            fcinfo: pg_sys::FunctionCallInfo,
            num: usize,
        ) -> PgDatum<T>
        where
            T: DatumCompatible<T>,
        {
            let datum = get_nullable_datum(fcinfo, num);
            PgDatum::<T>::new(datum.value, datum.isnull)
        }

        #[inline]
        pub fn pg_arg_is_null(
            fcinfo: pg_sys::FunctionCallInfo,
            num: usize,
        ) -> bool {
            get_nullable_datum(fcinfo, num).isnull
        }

        #[inline]
        pub fn pg_getarg_datum(
            fcinfo: pg_sys::FunctionCallInfo,
            num: usize,
        ) -> Option<pg_sys::Datum> {
            if pg_arg_is_null(fcinfo, num) {
                None
            } else {
                Some(get_nullable_datum(fcinfo, num).value)
            }
        }

        #[inline]
        fn get_nullable_datum(
            fcinfo: pg_sys::FunctionCallInfo,
            num: usize,
        ) -> pg_sys::pg12_specific::NullableDatum {
            let fcinfo = unsafe { fcinfo.as_mut() }.unwrap();
            unsafe {
                let nargs = fcinfo.nargs;
                let len = std::mem::size_of::<pg_sys::pg12_specific::NullableDatum>() * nargs as usize;
                fcinfo.args.as_slice(len)[num]
            }
        }

        #[inline]
        pub fn pg_return_null(
            fcinfo: pg_sys::FunctionCallInfo,
        ) -> pg_sys::Datum {
            let fcinfo = unsafe { fcinfo.as_mut() }.unwrap();
            fcinfo.isnull = true;
            0 as pg_sys::Datum
        }
    }
}

#[inline]
pub fn pg_return_text_p(s: &str) -> pg_sys::Datum {
    rust_str_to_text_p(s) as pg_sys::Datum
}

#[inline]
pub fn pg_return_void() -> pg_sys::Datum {
    0 as pg_sys::Datum
}
