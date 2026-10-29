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
use core::ffi::{CStr, c_char};
use core::{ptr, slice};

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
///   which [`pg_module_magic!`] records. The row type of a table, view or other relation that is
///   not a composite type, the array type that PostgreSQL makes for each type, and the
///   multirange type that it makes for each range type do not count. Whether those belong to
///   the extension differs between PostgreSQL versions.
///
/// A type that another extension or plain SQL creates does not belong to this library's
/// extension, so this function does not find it. Name such a type with [`regtypein`] instead,
/// qualified by its schema.
///
/// # Cost
///
/// Nothing is cached, so each call reads the catalogs. For an extension's type, it lists the
/// types of that name in every schema, then finds which of them belong to the extension, either
/// by looking up each one's extension in `pg_depend` or by listing the extension's members there,
/// whichever finishes first. The cost therefore grows with the number of same-named types, such
/// as tables of that name in other schemas, until it reaches the cost of listing the members.
/// pgrx calls `type_oid` for each value whose type it checks, such as each SPI read or `fn_call`
/// argument.
///
/// [`pg_module_magic!`]: crate::pg_module_magic
///
/// # Panics
///
/// Raises an ERROR if there is no such type, if the type is only a shell, if the extension is
/// not installed in the current database, or if the extension has more than one type of that
/// name.
///
/// For a type outside `pgrx_pg_sys`, panics if the library did not record its extension. That
/// happens when the crate does not call [`pg_module_magic!`], or when its directory did not
/// hold exactly one `.control` file when it was compiled. Names that start with `.`, such as
/// editor lock files, do not count.
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
    // namespace OID. The search returns NULL or a pg_type row that the cache keeps alive until
    // ReleaseSysCache, and nothing between the search and the release can raise an ERROR.
    let found = unsafe {
        let tuple = pg_sys::SearchSysCache2(
            pg_sys::SysCacheIdentifier::TYPENAMENSP as _,
            pg_sys::Datum::from(typname.as_ptr()),
            pg_sys::Datum::from(pg_sys::Oid::from(pg_sys::PG_CATALOG_NAMESPACE)),
        );
        (!tuple.is_null()).then(|| {
            let form = pg_sys::GETSTRUCT!(tuple).get().cast::<pg_sys::FormData_pg_type>();
            let found = TypeCandidate::from_form(form);
            pg_sys::ReleaseSysCache(tuple);
            found
        })
    };
    match found {
        Some(TypeCandidate { oid, defined: true, .. }) => oid,
        Some(TypeCandidate { defined: false, .. }) => {
            ereport!(
                ERROR,
                PgSqlErrorCode::ERRCODE_UNDEFINED_OBJECT,
                format!("type \"pg_catalog.{}\" is only a shell", typname.to_string_lossy())
            );
        }
        None => {
            ereport!(
                ERROR,
                PgSqlErrorCode::ERRCODE_UNDEFINED_OBJECT,
                format!("type \"pg_catalog.{}\" does not exist", typname.to_string_lossy())
            );
        }
    }
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

    match owned_types(extension, types_named(typname)).as_slice() {
        [TypeCandidate { oid, defined: true, .. }] => *oid,
        [TypeCandidate { defined: false, .. }] => {
            ereport!(
                ERROR,
                PgSqlErrorCode::ERRCODE_UNDEFINED_OBJECT,
                format!("type \"{typname_str}\" is only a shell")
            );
        }
        [] => {
            ereport!(
                ERROR,
                PgSqlErrorCode::ERRCODE_UNDEFINED_OBJECT,
                format!("type \"{typname_str}\" does not belong to extension \"{extname_str}\"")
            );
        }
        [_, _, ..] => {
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

/// A type with the name being looked up
#[derive(Clone, Copy)]
struct TypeCandidate {
    oid: pg_sys::Oid,
    /// False for a shell type, which `CREATE TYPE name` makes before the type is defined
    defined: bool,
    /// The relation whose row type this is, if it is one
    relation: pg_sys::Oid,
    /// The element type, if this is an array type
    element: pg_sys::Oid,
    /// The kind of type, such as `TYPTYPE_MULTIRANGE`
    kind: c_char,
}

impl TypeCandidate {
    /// Read the candidate's fields from a pg_type row
    ///
    /// # Safety
    ///
    /// `form` must point to a live pg_type row.
    unsafe fn from_form(form: *const pg_sys::FormData_pg_type) -> Self {
        // SAFETY: the caller passes a live pg_type row, and each read copies one field.
        unsafe {
            TypeCandidate {
                oid: (*form).oid,
                defined: (*form).typisdefined,
                relation: (*form).typrelid,
                element: (*form).typelem,
                kind: (*form).typtype,
            }
        }
    }

    /// Whether PostgreSQL made this type for another object: the row type of a relation that is
    /// not a composite type, the array type of its element type, or the multirange type of a
    /// range type. PostgreSQL 17 and later add such a type to the other object's extension, and
    /// remove it, along with that object. Earlier versions do not, though they add a multirange
    /// type to the extension whose script creates it. The lookup skips these types on every
    /// version, so that whether one counts does not depend on the version.
    fn is_dependent(&self) -> bool {
        if self.kind == pg_sys::TYPTYPE_MULTIRANGE as c_char {
            return true;
        }
        // SAFETY: both functions take an OID by value and only read the catalogs. They return
        // zero when there is no such relation or array type.
        unsafe {
            (self.relation != pg_sys::Oid::INVALID
                && pg_sys::get_rel_relkind(self.relation)
                    != pg_sys::RELKIND_COMPOSITE_TYPE as c_char)
                || (self.element != pg_sys::Oid::INVALID
                    && pg_sys::get_array_type(self.element) == self.oid)
        }
    }
}

/// The types named `typname`, in every schema
fn types_named(typname: &CStr) -> Vec<TypeCandidate> {
    // SAFETY: TYPENAMENSP's keys are a name, which typname, being NUL-terminated, can be, and a
    // namespace, so searching by the first key lists that name's types in every namespace. The
    // list holds a reference that keeps it and its tuples alive until it is released, and nothing
    // between the search and the release can raise an ERROR. The list ends in `n_members` tuple
    // pointers, which the raw place projection reaches with the list's provenance; it is
    // non-null and aligned even when the count is zero. Each member's tuple is a pg_type row.
    unsafe {
        let list = pg_sys::SearchSysCacheList(
            pg_sys::SysCacheIdentifier::TYPENAMENSP as _,
            1,
            pg_sys::Datum::from(typname.as_ptr()),
            pg_sys::Datum::from(0),
            pg_sys::Datum::from(0),
        );
        let len = usize::try_from((*list).n_members).expect("n_members must be non-negative");
        let members = slice::from_raw_parts(
            ptr::addr_of!((*list).members).cast::<*mut pg_sys::CatCTup>(),
            len,
        );
        let candidates = members
            .iter()
            .map(|&member| {
                let tuple = &raw mut (*member).tuple;
                let form = pg_sys::GETSTRUCT!(tuple).get().cast::<pg_sys::FormData_pg_type>();
                TypeCandidate::from_form(form)
            })
            .collect();
        pg_sys::ReleaseCatCacheList(list);
        candidates
    }
}

/// How many candidates [`owned_types`] checks before it starts listing the extension's members,
/// whose scan costs about as much to start as checking a candidate.
const CHECKS_BEFORE_LISTING: usize = 4;

/// How many of the extension's members [`owned_types`] lists for each candidate it checks. In a
/// debug build of PostgreSQL 18, checking a candidate took about as long as listing 85 members.
const MEMBERS_PER_CHECK: usize = 64;

/// The `candidates` that belong to `extension`, other than [dependent] ones, or the first two
/// found, which are enough to make the name ambiguous.
///
/// There are two ways to find them, and either can be slow. Looking up each candidate's
/// extension costs a pg_depend index scan per candidate, and the candidates include every type
/// of the name in every schema, such as the row type of every table of that name. Listing the
/// extension's members costs a pg_depend row per member, and a large extension has thousands.
/// Neither count is known in advance. So after the first [`CHECKS_BEFORE_LISTING`] candidates,
/// which are all that a name shared by a few types needs, this alternates between the two, one
/// candidate at a time and [`MEMBERS_PER_CHECK`] members at a time, and stops when either is
/// complete. That takes at most about twice as long as the faster way alone, plus those first
/// checks. Only a candidate found to belong to the extension is checked for being dependent.
///
/// [dependent]: TypeCandidate::is_dependent
fn owned_types(extension: pg_sys::Oid, mut candidates: Vec<TypeCandidate>) -> Vec<TypeCandidate> {
    candidates.sort_unstable_by_key(|candidate| candidate.oid.to_u32());
    let (mut checked, mut listed) = (Vec::new(), Vec::new());
    let mut members = None;
    let owned = 'search: {
        for (i, candidate) in candidates.iter().enumerate() {
            // SAFETY: the call takes both OIDs by value and only reads pg_depend.
            let owner =
                unsafe { pg_sys::getExtensionOfObject(pg_sys::TypeRelationId, candidate.oid) };
            if owner == extension && !candidate.is_dependent() {
                checked.push(*candidate);
            }
            if checked.len() == 2 || i + 1 == candidates.len() {
                break 'search checked;
            }
            if i + 1 < CHECKS_BEFORE_LISTING {
                continue;
            }

            let members = members.get_or_insert_with(|| ExtensionMembers::begin(extension));
            for _ in 0..MEMBERS_PER_CHECK {
                let Some(member) = members.next() else {
                    break 'search listed;
                };
                let Some(member) = member else {
                    continue;
                };
                let member = member.to_u32();
                if let Ok(j) = candidates.binary_search_by_key(&member, |c| c.oid.to_u32())
                    && !candidates[j].is_dependent()
                {
                    listed.push(candidates[j]);
                    if listed.len() == 2 {
                        break 'search listed;
                    }
                }
            }
        }
        Vec::new()
    };
    if let Some(members) = members {
        members.end();
    }
    owned
}

/// A scan of the pg_depend rows that refer to an extension, which include the rows that
/// `getExtensionOfObject` reads from the member's side
///
/// `end` closes the scan. There is no `Drop`, because after an ERROR the transaction abort closes
/// it, and PostgreSQL must not be called while the ERROR unwinds.
struct ExtensionMembers {
    depend: pg_sys::Relation,
    scan: pg_sys::SysScanDesc,
}

impl ExtensionMembers {
    fn begin(extension: pg_sys::Oid) -> Self {
        // SAFETY: table_open returns an open pg_depend or raises an ERROR. ScanKeyInit fills
        // every field of each key. The keys match the three columns of pg_depend's
        // (refclassid, refobjid, refobjsubid) index, as getExtensionType's scan does, and
        // systable_beginscan copies them before returning. A NULL snapshot scans with the
        // catalog snapshot. Transaction abort closes the scan and the relation if an ERROR
        // interrupts the scan before `end`.
        unsafe {
            let depend = pg_sys::table_open(pg_sys::DependRelationId, pg_sys::AccessShareLock as _);
            let mut keys = [pg_sys::ScanKeyData::default(); 3];
            let columns = [
                (
                    pg_sys::Anum_pg_depend_refclassid,
                    pg_sys::F_OIDEQ,
                    pg_sys::ExtensionRelationId.into(),
                ),
                (pg_sys::Anum_pg_depend_refobjid, pg_sys::F_OIDEQ, extension.into()),
                (pg_sys::Anum_pg_depend_refobjsubid, pg_sys::F_INT4EQ, pg_sys::Datum::from(0_i32)),
            ];
            for (key, (column, procedure, argument)) in keys.iter_mut().zip(columns) {
                pg_sys::ScanKeyInit(
                    key,
                    column as _,
                    pg_sys::BTEqualStrategyNumber as _,
                    procedure.into(),
                    argument,
                );
            }
            let scan = pg_sys::systable_beginscan(
                depend,
                pg_sys::DependReferenceIndexId.into(),
                true,
                ptr::null_mut(),
                keys.len() as _,
                keys.as_mut_ptr(),
            );
            ExtensionMembers { depend, scan }
        }
    }

    fn end(self) {
        // SAFETY: begin opened both, and nothing has closed them.
        unsafe {
            pg_sys::systable_endscan(self.scan);
            pg_sys::table_close(self.depend, pg_sys::AccessShareLock as _);
        }
    }
}

impl Iterator for ExtensionMembers {
    /// For each row, the type it makes a member of the extension, if it does
    type Item = Option<pg_sys::Oid>;

    fn next(&mut self) -> Option<Self::Item> {
        // SAFETY: the scan is open until `end`, which takes self. systable_getnext returns NULL
        // after the last row, or a pg_depend tuple that stays valid until the next call. The
        // field reads go through the raw row pointer, which GETSTRUCT aligns for the row.
        unsafe {
            let tuple = pg_sys::systable_getnext(self.scan);
            if tuple.is_null() {
                return None;
            }
            let row = pg_sys::GETSTRUCT!(tuple).get().cast::<pg_sys::FormData_pg_depend>();
            let member_type = (*row).classid == pg_sys::TypeRelationId
                && (*row).deptype == pg_sys::DependencyType::DEPENDENCY_EXTENSION as c_char;
            Some(member_type.then(|| (*row).objid))
        }
    }
}
