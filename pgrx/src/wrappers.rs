//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
//! Provides safe wrapper functions around some of Postgres' useful functions.
use crate::{IntoDatum, PgSqlErrorCode, direct_function_call, ereport, pg_sys};
use alloc::ffi::CString;
use core::ffi::CStr;

/// A helper function for Postgres' `regtypein` function to lookup a type by a specific name
///
/// Returns the `oid` of the specified type name.  Will panic if Postgres can't find the type
pub fn regtypein(type_name: &str) -> pg_sys::Oid {
    let cstr =
        alloc::ffi::CString::new(type_name).expect("specified type_name has embedded NULL byte");
    unsafe {
        direct_function_call::<pg_sys::Oid>(pg_sys::regtypein, &[cstr.as_c_str().into_datum()])
            .expect("type lookup returned NULL")
    }
}

/// Look up the SQL type that a Rust type's name names
///
/// The SQL name is the last segment of the Rust type's path, folded and truncated as PostgreSQL
/// folds an unquoted identifier, so `my_ext::Complex` names `complex`. Where to look depends on
/// the crate that defines the Rust type, and never on `search_path`:
///
/// - A struct from `pgrx_pg_sys`, such as `pg_sys::Point`, is PostgreSQL's own C representation of
///   a built-in type, so it names the type of that name in `pg_catalog`.
/// - Any other type names a type that belongs to this library's extension, in whatever schema it
///   is. The extension is the one named by the `.control` file next to the crate's `Cargo.toml`,
///   which [`pg_module_magic!`] records.
///
/// Each call looks the type up in the catalogs.
///
/// [`pg_module_magic!`]: crate::pg_module_magic
///
/// # Panics
///
/// Raises an ERROR if there is no such type, if the extension is not installed in the current
/// database, or if the extension has more than one type of that name.
pub fn rust_regtypein<T>() -> pg_sys::Oid {
    let type_name = std::any::type_name::<T>();

    // pluck out the last part of the type name
    let idx = match type_name.rfind("::") {
        Some(idx) => idx + 2,
        None => 0,
    };
    let typname = fold_identifier(&type_name[idx..]);

    if type_name.starts_with("pgrx_pg_sys::") {
        builtin_type_oid(&typname)
    } else {
        extension_type_oid(&typname)
    }
}

/// Fold `ident` as PostgreSQL folds an unquoted identifier
fn fold_identifier(ident: &str) -> CString {
    let len = i32::try_from(ident.len()).expect("type name is too long");
    let ident = CString::new(ident).expect("specified type_name has embedded NULL byte");
    // SAFETY: ident holds len bytes before its NUL. The result is a new palloc'd string, which
    // only this function refers to, so it is freed once copied.
    unsafe {
        let folded = pg_sys::downcase_truncate_identifier(ident.as_ptr(), len, false);
        let typname = CStr::from_ptr(folded).to_owned();
        pg_sys::pfree(folded.cast());
        typname
    }
}

fn builtin_type_oid(typname: &CStr) -> pg_sys::Oid {
    // SAFETY: TYPENAMENSP's keys are a name, which typname, being NUL-terminated, can be, and a
    // namespace OID. The call returns the OID column of the matching row, if any.
    let oid = unsafe {
        pg_sys::GetSysCacheOid(
            pg_sys::SysCacheIdentifier::TYPENAMENSP as _,
            pg_sys::Anum_pg_type_oid as _,
            pg_sys::Datum::from(typname.as_ptr()),
            pg_sys::Datum::from(pg_sys::Oid::from(pg_sys::PG_CATALOG_NAMESPACE)),
            pg_sys::Datum::from(0),
            pg_sys::Datum::from(0),
        )
    };
    if oid == pg_sys::Oid::INVALID {
        ereport!(
            ERROR,
            PgSqlErrorCode::ERRCODE_UNDEFINED_OBJECT,
            format!("type \"pg_catalog.{}\" does not exist", typname.to_string_lossy())
        );
    }
    oid
}

fn extension_type_oid(typname: &CStr) -> pg_sys::Oid {
    let typname_str = typname.to_string_lossy();
    let extname = crate::pg_magic_func_support::extension_name()
        .unwrap_or_else(|reason| panic!("cannot look up type \"{typname_str}\": {reason}"));
    let extname_str = extname.to_string_lossy();
    // SAFETY: extname is NUL-terminated, and the call only reads it.
    let extension = unsafe { pg_sys::get_extension_oid(extname.as_ptr(), true) };
    if extension == pg_sys::Oid::INVALID {
        ereport!(
            ERROR,
            PgSqlErrorCode::ERRCODE_UNDEFINED_OBJECT,
            format!("extension \"{extname_str}\" is not installed in this database")
        );
    }

    let mut owned = types_named(typname).into_iter().filter(|&oid| {
        // SAFETY: the call takes both OIDs by value and only reads pg_depend.
        unsafe { pg_sys::getExtensionOfObject(pg_sys::TypeRelationId, oid) == extension }
    });
    match (owned.next(), owned.next()) {
        (Some(oid), None) => oid,
        (None, _) => {
            ereport!(
                ERROR,
                PgSqlErrorCode::ERRCODE_UNDEFINED_OBJECT,
                format!("type \"{typname_str}\" does not belong to extension \"{extname_str}\"")
            );
        }
        (Some(_), Some(_)) => {
            ereport!(
                ERROR,
                PgSqlErrorCode::ERRCODE_DUPLICATE_OBJECT,
                format!(
                    "more than one type named \"{typname_str}\" belongs to extension \"{extname_str}\""
                )
            );
        }
    }
}

/// The OIDs of the types named `typname`, in every schema
fn types_named(typname: &CStr) -> Vec<pg_sys::Oid> {
    // SAFETY: TYPENAMENSP's keys are a name, which typname, being NUL-terminated, can be, and a
    // namespace, so searching by the first key lists that name's types in every namespace. The
    // list holds a reference that keeps it and its tuples alive until it is released, and nothing
    // between the search and the release can raise an ERROR. Each member's tuple is a pg_type row.
    unsafe {
        let list = pg_sys::SearchSysCacheList(
            pg_sys::SysCacheIdentifier::TYPENAMENSP as _,
            1,
            pg_sys::Datum::from(typname.as_ptr()),
            pg_sys::Datum::from(0),
            pg_sys::Datum::from(0),
        );
        let members = (*list).members.as_slice((*list).n_members as usize);
        let oids = members
            .iter()
            .map(|&member| {
                let tuple = &raw mut (*member).tuple;
                (*pg_sys::GETSTRUCT!(tuple).get().cast::<pg_sys::FormData_pg_type>()).oid
            })
            .collect();
        pg_sys::ReleaseCatCacheList(list);
        oids
    }
}
