use crate::{
    direct_function_call, ereport, pg_sys, rust_str_to_text_p, PgLogLevel, PgSqlErrorCode,
};

extern "C" {
    fn pgx_GETSTRUCT(tuple: pg_sys::HeapTuple) -> *mut std::os::raw::c_char;
}

pub fn lookup_enum_by_oid(enumval: pg_sys::Oid) -> (String, pg_sys::Oid, f32) {
    let tup = unsafe {
        pg_sys::SearchSysCache(
            pg_sys::SysCacheIdentifier_ENUMOID as i32,
            enumval as pg_sys::Datum,
            0,
            0,
            0,
        )
    };
    if tup.is_null() {
        ereport(
            PgLogLevel::ERROR,
            PgSqlErrorCode::ERRCODE_INVALID_BINARY_REPRESENTATION,
            &format!("invalid internal value for enum: {}", enumval),
            file!(),
            line!(),
            column!(),
        );
    }

    let en = unsafe { pgx_GETSTRUCT(tup) } as pg_sys::Form_pg_enum;
    let en = unsafe { en.as_ref() }.unwrap();
    let result = (
        unsafe { std::ffi::CStr::from_ptr(en.enumlabel.data.as_ptr() as *const i8) }
            .to_str()
            .unwrap()
            .to_string()
            .to_owned(),
        en.enumtypid,
        en.enumsortorder as f32,
    );

    eprintln!("result={}", result.0);
    unsafe {
        pg_sys::ReleaseSysCache(tup);
    }

    result
}

pub fn lookup_enum_by_label(typname: &str, label: &str) -> pg_sys::Datum {
    let typname_as_text = rust_str_to_text_p(typname);
    let enumtypoid = unsafe {
        direct_function_call::<pg_sys::Oid>(
            pg_sys::to_regtype,
            vec![Some(typname_as_text.as_ptr() as pg_sys::Datum)],
        )
    }
    .expect("could not convert enum type to oid") as pg_sys::Oid;

    if enumtypoid == pg_sys::InvalidOid {
        panic!("could not locate type oid for type: {}", typname);
    }

    let tup = unsafe {
        pg_sys::SearchSysCache(
            pg_sys::SysCacheIdentifier_ENUMTYPOIDNAME as i32,
            enumtypoid as pg_sys::Datum,
            std::ffi::CString::new(label)
                .expect("failed to convert enum typname to a CString")
                .as_ptr() as pg_sys::Datum,
            0,
            0,
        )
    };

    if tup.is_null() {
        panic!(
            "could not find heap tuple for enum: {}.{}, typoid={}",
            typname, label, enumtypoid
        );
    }

    let oid = extract_enum_oid(tup);

    unsafe {
        pg_sys::ReleaseSysCache(tup);
    }

    oid as pg_sys::Datum
}

#[cfg(any(feature = "pg10", feature = "pg11"))]
fn extract_enum_oid(tup: *mut pg_sys::HeapTupleData) -> pg_sys::Oid {
    extern "C" {
        fn pgx_HeapTupleHeaderGetOid(htup_header: pg_sys::HeapTupleHeader) -> pg_sys::Oid;
    }

    unsafe { pgx_HeapTupleHeaderGetOid(tup.as_ref().unwrap().t_data) }
}

#[cfg(feature = "pg12")]
fn extract_enum_oid(tup: *mut pg_sys::HeapTupleData) -> pg_sys::Oid {
    let en = unsafe { pgx_GETSTRUCT(tup) } as pg_sys::Form_pg_enum;
    let en = unsafe { en.as_ref() }.unwrap();
    en.oid
}
