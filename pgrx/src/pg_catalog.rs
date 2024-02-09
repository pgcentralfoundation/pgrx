//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use crate as pgrx;
use pgrx::datum::Array;
use pgrx::pg_sys::Oid;
use pgrx::pg_sys::Oid as Regproc;
use std::ffi::{c_char, CStr};

unsafe trait GetStruct<T> {
    unsafe fn get_struct(raw: *const T) -> Self;
}

unsafe impl<T> GetStruct<T> for T {
    unsafe fn get_struct(raw: *const T) -> Self {
        unsafe { raw.read() }
    }
}

unsafe impl GetStruct<pgrx::pg_sys::nameData> for &CStr {
    unsafe fn get_struct(raw: *const pgrx::pg_sys::nameData) -> Self {
        unsafe { CStr::from_ptr(raw.cast::<c_char>()) }
    }
}

unsafe impl GetStruct<pgrx::pg_sys::int2vector> for &[i16] {
    unsafe fn get_struct(raw: *const pgrx::pg_sys::int2vector) -> Self {
        unsafe { (*raw).values.as_slice((*raw).dim1 as usize) }
    }
}

unsafe impl GetStruct<pgrx::pg_sys::oidvector> for &[Oid] {
    unsafe fn get_struct(raw: *const pgrx::pg_sys::oidvector) -> Self {
        unsafe { (*raw).values.as_slice((*raw).dim1 as usize) }
    }
}

macro_rules! _macro_1 {
    {
        $table:ident, ($column:ident, character { $(($variant:ident, $value:literal))* } $($x:tt)*)
    } => {
        paste::paste! {
            #[non_exhaustive]
            #[repr(u8)]
            #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
            pub enum [<$table:camel $column:camel>] {
                $($variant = $value),*
            }

            impl [<$table:camel $column:camel>] {
                fn from_c_char(value: c_char) -> Self {
                    match value as u8 {
                        $($value => Self::$variant,)*

                        _ => panic!("unrecognized value: `{}`", value as u8 as char),
                    }
                }
            }

            impl pgrx::datum::FromDatum for [<$table:camel $column:camel>] {
                unsafe fn from_polymorphic_datum(
                    datum: pgrx::pg_sys::Datum,
                    is_null: bool,
                    _: Oid,
                ) -> Option<Self>
                where
                    Self: Sized,
                {
                    if is_null {
                        None
                    } else {
                        Some(Self::from_c_char(datum.value() as _))
                    }
                }
            }

            impl pgrx::datum::IntoDatum for [<$table:camel $column:camel>] {
                fn into_datum(self) -> std::option::Option<pgrx::pg_sys::Datum> {
                    Some(pgrx::pg_sys::Datum::from(self as i8))
                }
                fn type_oid() -> pgrx_pg_sys::Oid {
                    pgrx::pg_sys::CHAROID
                }
            }

            unsafe impl pgrx::datum::UnboxDatum for [<$table:camel $column:camel>] {
                type As<'src> = Self;
                #[inline]
                unsafe fn unbox<'src>(datum: pgrx::datum::Datum<'src>) -> Self::As<'src>
                where
                    Self: 'src,
                {
                    Self::from_c_char(datum.sans_lifetime().value() as c_char)
                }
            }

            unsafe impl GetStruct<c_char> for [<$table:camel $column:camel>] {
                unsafe fn get_struct(raw: *const c_char) -> Self {
                    unsafe { Self::from_c_char(raw.read()) }
                }
            }
        }
    };
}

macro_rules! _macro_0 {
    {
        $table:ident, $(#[$m_column:meta])* ($column:ident, c_char, character { $(($variant:ident, $value:literal))* } $($x:tt)*)
    } => {
        paste::paste! {
            _macro_1! { $table, ($column, character { $(($variant, $value))* }) }
            _macro_0! { $table, $(#[$m_column])* ($column, [<$table:camel $column:camel>] $($x)*) }
        }
    };
    {
        $table:ident, $(#[$m_column:meta])* ($column:ident, Array<c_char>, character { $(($variant:ident, $value:literal))* } $($x:tt)*)
    } => {
        paste::paste! {
            _macro_1! { $table, ($column, character { $(($variant, $value))* }) }
            _macro_0! { $table, $(#[$m_column])* ($column, Array<[<$table:camel $column:camel>]> $($x)*) }
        }
    };
    {
        $table:ident, $(#[$m_column:meta])* ($column:ident, $type:ty, get_struct)
    } => {
        paste::paste! {
            impl [<$table:camel>]<'_> {
                $(#[$m_column])*
                pub fn $column(&self) -> $type {
                    unsafe {
                        let start = self.inner.t_data.cast::<u8>();
                        let offset = (*self.inner.t_data).t_hoff as usize;
                        let p = start.add(offset).cast::<pgrx::pg_sys::[<FormData_ $table>]>();
                        GetStruct::get_struct(std::ptr::addr_of!((*p).$column))
                    }
                }
            }
        }
    };
    {
        $table:ident, $(#[$m_column:meta])* ($column:ident, $type:ty, get_attr)
    } => {
        paste::paste! {
            impl [<$table:camel>]<'_> {
                $(#[$m_column])*
                pub fn $column(&self) -> Option<$type> {
                    self.get_attr::<$type>(pgrx::pg_sys::[<Anum_ $table _ $column>])
                }
            }
        }
    };
    {
        $table:ident, $(#[$m_column:meta])* ($column:ident, $type:ty, get_attr, notnull)
    } => {
        paste::paste! {
            impl [<$table:camel>]<'_> {
                $(#[$m_column])*
                pub fn $column(&self) -> $type {
                    self.get_attr::<$type>(pgrx::pg_sys::[<Anum_ $table _ $column>]).unwrap()
                }
            }
        }
    };
}

macro_rules! define {
    {
        catalog ($table:ident) {
            $($(#[$m_column:meta])* ($($x:tt)*))*
        }
    } => {
        paste::paste!{
            pub struct [<$table:camel>]<'a> {
                inner: &'a pgrx::pg_sys::HeapTupleData,
                cache_id: i32,
            }

            impl<'a> [<$table:camel>]<'a> {
                #[inline]
                #[allow(dead_code)]
                fn get_attr<T: pgrx::datum::FromDatum>(&self, attribute: u32) -> Option<T> {
                    unsafe {
                        let mut is_null = false;
                        let datum = pgrx::pg_sys::SysCacheGetAttr(
                            self.cache_id,
                            std::ptr::addr_of!(*self.inner).cast_mut(),
                            attribute as _,
                            &mut is_null,
                        );
                        T::from_datum(datum, is_null)
                    }
                }
            }

            pub struct [<$table:camel Search>] {
                inner: Option<std::ptr::NonNull<pgrx::pg_sys::HeapTupleData>>,
                cache_id: i32,
            }

            impl [<$table:camel Search>] {
                pub fn is_empty(&self) -> bool {
                    self.inner.is_none()
                }
                pub fn get(&self) -> Option<[<$table:camel>]> {
                    unsafe {
                        Some([<$table:camel>] {
                            inner: self.inner?.as_ref(),
                            cache_id: self.cache_id,
                        })
                    }
                }
            }

            impl Drop for [<$table:camel Search>] {
                fn drop(&mut self) {
                    unsafe {
                        if let Some(inner) = self.inner {
                            pgrx::pg_sys::ReleaseSysCache(inner.as_ptr());
                        }
                    }
                }
            }

            pub struct [<$table:camel SearchList>] {
                inner: std::ptr::NonNull<pgrx::pg_sys::CatCList>,
                cache_id: i32,
            }

            impl [<$table:camel SearchList>] {
                pub fn len(&self) -> usize {
                    unsafe {
                        let inner = self.inner.as_ref();
                        inner.n_members as usize
                    }
                }
                pub fn is_empty(&self) -> bool {
                    self.len() == 0
                }
                pub fn get(&self, i: usize) -> Option<[<$table:camel>]> {
                    unsafe {
                        let inner = self.inner.as_ref();
                        let slice = inner.members.as_slice(inner.n_members as usize);
                        let member = *slice.get(i)?;
                        let tuple = &(*member).tuple;
                        Some([<$table:camel>] {
                            inner: tuple,
                            cache_id: self.cache_id
                        })
                    }
                }
            }

            impl Drop for [<$table:camel SearchList>] {
                fn drop(&mut self) {
                    unsafe { pgrx::pg_sys::ReleaseCatCacheList(self.inner.as_ptr()) }
                }
            }

            $(_macro_0! { $table, $(#[$m_column])* ($($x)*) })*
        }
    };
    {
        cache ($cache:ident, $catalog:ident) {
            ($p0_name:ident, $p0_type:ty)
        }
    } => {
        paste::paste!{
            impl<'a> [<$catalog:camel>]<'a> {
                pub fn [<search _ $cache>]($p0_name: $p0_type) -> Option<[<$catalog:camel Search>]> {
                    unsafe {
                        use pgrx::datum::IntoDatum;
                        let cache_id = pgrx::pg_sys::[<SysCacheIdentifier_ $cache:upper>] as i32;
                        let entry = pgrx::pg_sys::SearchSysCache1(cache_id, $p0_name.into_datum()?);
                        let inner = std::ptr::NonNull::new(entry);
                        Some([<$catalog:camel Search>] { inner, cache_id })
                    }
                }
            }
        }
    };
    {
        cache ($cache:ident, $catalog:ident) {
            ($p0_name:ident, $p0_type:ty)
            ($p1_name:ident, $p1_type:ty)
        }
    } => {
        paste::paste!{
            impl<'a> [<$catalog:camel>]<'a> {
                pub fn [<search _ $cache>]($p0_name: $p0_type, $p1_name: $p1_type) -> Option<[<$catalog:camel Search>]> {
                    unsafe {
                        use pgrx::datum::IntoDatum;
                        let cache_id = pgrx::pg_sys::[<SysCacheIdentifier_ $cache:upper>] as i32;
                        let entry = pgrx::pg_sys::SearchSysCache2(
                            cache_id,
                            $p0_name.into_datum()?,
                            $p1_name.into_datum()?,
                        );
                        let inner = std::ptr::NonNull::new(entry);
                        Some([<$catalog:camel Search>] { inner, cache_id })
                    }
                }
                pub fn [<search _list_ $cache _1>]($p0_name: $p0_type) -> Option<[<$catalog:camel SearchList>]> {
                    unsafe {
                        use pgrx::datum::IntoDatum;
                        let cache_id = pgrx::pg_sys::[<SysCacheIdentifier_ $cache:upper>] as i32;
                        let entry = pgrx::pg_sys::SearchSysCacheList(
                            cache_id,
                            1,
                            $p0_name.into_datum()?,
                            0.into(),
                            0.into(),
                        );
                        let inner = std::ptr::NonNull::new(entry).unwrap();
                        Some([<$catalog:camel SearchList>] { inner, cache_id })
                    }
                }
            }
        }
    };
    {
        cache ($cache:ident, $catalog:ident) {
            ($p0_name:ident, $p0_type:ty)
            ($p1_name:ident, $p1_type:ty)
            ($p2_name:ident, $p2_type:ty)
        }
    } => {
        paste::paste!{
            impl<'a> [<$catalog:camel>]<'a> {
                pub fn [<search _ $cache>]($p0_name: $p0_type, $p1_name: $p1_type, $p2_name: $p2_type) -> Option<[<$catalog:camel Search>]> {
                    unsafe {
                        use pgrx::datum::IntoDatum;
                        let cache_id = pgrx::pg_sys::[<SysCacheIdentifier_ $cache:upper>] as i32;
                        let entry = pgrx::pg_sys::SearchSysCache3(
                            cache_id,
                            $p0_name.into_datum()?,
                            $p1_name.into_datum()?,
                            $p2_name.into_datum()?,
                        );
                        let inner = std::ptr::NonNull::new(entry);
                        Some([<$catalog:camel Search>] { inner, cache_id })
                    }
                }
                pub fn [<search _list_ $cache _1>]($p0_name: $p0_type) -> Option<[<$catalog:camel SearchList>]> {
                    unsafe {
                        use pgrx::datum::IntoDatum;
                        let cache_id = pgrx::pg_sys::[<SysCacheIdentifier_ $cache:upper>] as i32;
                        let entry = pgrx::pg_sys::SearchSysCacheList(
                            cache_id,
                            1,
                            $p0_name.into_datum()?,
                            0.into(),
                            0.into(),
                        );
                        let inner = std::ptr::NonNull::new(entry).unwrap();
                        Some([<$catalog:camel SearchList>] { inner, cache_id })
                    }
                }
                pub fn [<search _list_ $cache _2>]($p0_name: $p0_type, $p1_name: $p1_type) -> Option<[<$catalog:camel SearchList>]> {
                    unsafe {
                        use pgrx::datum::IntoDatum;
                        let cache_id = pgrx::pg_sys::[<SysCacheIdentifier_ $cache:upper>] as i32;
                        let entry = pgrx::pg_sys::SearchSysCacheList(
                            cache_id,
                            2,
                            $p0_name.into_datum()?,
                            $p1_name.into_datum()?,
                            0.into(),
                        );
                        let inner = std::ptr::NonNull::new(entry).unwrap();
                        Some([<$catalog:camel SearchList>] { inner, cache_id })
                    }
                }
            }
        }
    };
    {
        cache ($cache:ident, $catalog:ident) {
            ($p0_name:ident, $p0_type:ty)
            ($p1_name:ident, $p1_type:ty)
            ($p2_name:ident, $p2_type:ty)
            ($p3_name:ident, $p3_type:ty)
        }
    } => {
        paste::paste!{
            impl<'a> [<$catalog:camel>]<'a> {
                pub fn [<search _ $cache>]($p0_name: $p0_type, $p1_name: $p1_type, $p2_name: $p2_type, $p3_name: $p3_type) -> Option<[<$catalog:camel Search>]> {
                    unsafe {
                        use pgrx::datum::IntoDatum;
                        let cache_id = pgrx::pg_sys::[<SysCacheIdentifier_ $cache:upper>] as i32;
                        let entry = pgrx::pg_sys::SearchSysCache4(
                            cache_id,
                            $p0_name.into_datum()?,
                            $p1_name.into_datum()?,
                            $p2_name.into_datum()?,
                            $p3_name.into_datum()?,
                        );
                        let inner = std::ptr::NonNull::new(entry);
                        Some([<$catalog:camel Search>] { inner, cache_id })
                    }
                }
                pub fn [<search _list_ $cache _1>]($p0_name: $p0_type) -> Option<[<$catalog:camel SearchList>]> {
                    unsafe {
                        use pgrx::datum::IntoDatum;
                        let cache_id = pgrx::pg_sys::[<SysCacheIdentifier_ $cache:upper>] as i32;
                        let entry = pgrx::pg_sys::SearchSysCacheList(
                            cache_id,
                            1,
                            $p0_name.into_datum()?,
                            0.into(),
                            0.into(),
                        );
                        let inner = std::ptr::NonNull::new(entry).unwrap();
                        Some([<$catalog:camel SearchList>] { inner, cache_id })
                    }
                }
                pub fn [<search _list_ $cache _2>]($p0_name: $p0_type, $p1_name: $p1_type) -> Option<[<$catalog:camel SearchList>]> {
                    unsafe {
                        use pgrx::datum::IntoDatum;
                        let cache_id = pgrx::pg_sys::[<SysCacheIdentifier_ $cache:upper>] as i32;
                        let entry = pgrx::pg_sys::SearchSysCacheList(
                            cache_id,
                            2,
                            $p0_name.into_datum()?,
                            $p1_name.into_datum()?,
                            0.into(),
                        );
                        let inner = std::ptr::NonNull::new(entry).unwrap();
                        Some([<$catalog:camel SearchList>] { inner, cache_id })
                    }
                }
                pub fn [<search _list_ $cache _3>]($p0_name: $p0_type, $p1_name: $p1_type, $p2_name: $p2_type) -> Option<[<$catalog:camel SearchList>]> {
                    unsafe {
                        use pgrx::datum::IntoDatum;
                        let cache_id = pgrx::pg_sys::[<SysCacheIdentifier_ $cache:upper>] as i32;
                        let entry = pgrx::pg_sys::SearchSysCacheList(
                            cache_id,
                            3,
                            $p0_name.into_datum()?,
                            $p1_name.into_datum()?,
                            $p2_name.into_datum()?,
                        );
                        let inner = std::ptr::NonNull::new(entry).unwrap();
                        Some([<$catalog:camel SearchList>] { inner, cache_id })
                    }
                }
            }
        }
    };
}

macro_rules! defines {
    {
        $($x:ident $y:tt $z:tt)*
    } => {
        $(define! { $x $y $z })*
    }
}

defines! {
    catalog (pg_am) {
        (oid, Oid, get_struct)
        (amname, &CStr, get_struct)
        (amhandler, Regproc, get_struct)
        (amtype, c_char, character {
            (Table, b't')
            (Index, b'i')
        }, get_struct)
    }
    catalog (pg_amop) {
        (oid, Oid, get_struct)
        (amopfamily, Oid, get_struct)
        (amoplefttype, Oid, get_struct)
        (amoprighttype, Oid, get_struct)
        (amopstrategy, i16, get_struct)
        (amoppurpose, c_char, character {
            (Search, b's')
            (Order, b'o')
        }, get_struct)
        (amopopr, Oid, get_struct)
        (amopmethod, Oid, get_struct)
        (amopsortfamily, Oid, get_struct)
    }
    catalog (pg_amproc) {
        (oid, Oid, get_struct)
        (amprocfamily, Oid, get_struct)
        (amproclefttype, Oid, get_struct)
        (amprocrighttype, Oid, get_struct)
        (amprocnum, i16, get_struct)
        (amproc, Regproc, get_struct)
    }
    catalog (pg_class) {
        (oid, Oid, get_struct)
        (relname, &CStr, get_struct)
        (relnamespace, Oid, get_struct)
        (reltype, Oid, get_struct)
        (reloftype, Oid, get_struct)
        (relowner, Oid, get_struct)
        (relam, Oid, get_struct)
        (relfilenode, Oid, get_struct)
        (reltablespace, Oid, get_struct)
        (relpages, i32, get_struct)
        (reltuples, f32, get_struct)
        (relallvisible, i32, get_struct)
        (reltoastrelid, Oid, get_struct)
        (relhasindex, bool, get_struct)
        (relisshared, bool, get_struct)
        (relpersistence, c_char, character {
            (Permanent, b'p')
            (Unlogged, b'u')
            (Temp, b't')
        }, get_struct)
        (relkind, c_char, character {
            (Relation, b'r')
            (Index, b'i')
            (Sequence, b'S')
            (Toastvalue, b't')
            (View, b'v')
            (Matview, b'm')
            (CompositeType, b'c')
            (ForeignTable, b'f')
            (PartitionedTable, b'p')
            (PartitionedIndex, b'I')
        }, get_struct)
        (relnatts, i16, get_struct)
        (relchecks, i16, get_struct)
        (relhasrules, bool, get_struct)
        (relhastriggers, bool, get_struct)
        (relhassubclass, bool, get_struct)
        (relrowsecurity, bool, get_struct)
        (relforcerowsecurity, bool, get_struct)
        (relispopulated, bool, get_struct)
        (relreplident, c_char, character {
            (DEFAULT, b'd')
            (NOTHING, b'n')
            (FULL, b'f')
            (INDEX, b'i')
        }, get_struct)
        (relispartition, bool, get_struct)
        (relrewrite, Oid, get_struct)
        (relfrozenxid, u32, get_struct)
        (relminmxid, u32, get_struct)
        // (relacl, aclitem[], get_attr)
        (reloptions, Array<String>, get_attr)
        // (relpartbound, pg_node_tree, get_attr)
    }
    catalog (pg_enum) {
        (oid, Oid, get_struct)
        (enumtypid, Oid, get_struct)
        (enumsortorder, f32, get_struct)
        (enumlabel, &CStr, get_struct)
    }
    catalog (pg_index) {
        (indexrelid, Oid, get_struct)
        (indrelid, Oid, get_struct)
        (indnatts, i16, get_struct)
        (indnkeyatts, i16, get_struct)
        (indisunique, bool, get_struct)
        #[cfg(not(any(feature = "pg12", feature = "pg13", feature = "pg14")))]
        (indnullsnotdistinct, bool, get_struct)
        (indisprimary, bool, get_struct)
        (indisexclusion, bool, get_struct)
        (indimmediate, bool, get_struct)
        (indisclustered, bool, get_struct)
        (indisvalid, bool, get_struct)
        (indcheckxmin, bool, get_struct)
        (indisready, bool, get_struct)
        (indislive, bool, get_struct)
        (indisreplident, bool, get_struct)
        (indkey, &[i16], get_struct)
        (indcollation, Array<Oid>, get_attr, notnull)
        (indclass, Array<Oid>, get_attr, notnull)
        (indoption, Array<i16>, get_attr, notnull)
        // (indexprs, pg_node_tree, get_attr)
        // (indpred, pg_node_tree, get_attr)
    }
    catalog (pg_namespace) {
        (oid, Oid, get_struct)
        (nspname, &CStr, get_struct)
        (nspowner, Oid, get_struct)
        // (nspacl, aclitem[], get_attr)
    }
    catalog (pg_opclass) {
        (oid, Oid, get_struct)
        (opcmethod, Oid, get_struct)
        (opcname, &CStr, get_struct)
        (opcnamespace, Oid, get_struct)
        (opcowner, Oid, get_struct)
        (opcfamily, Oid, get_struct)
        (opcintype, Oid, get_struct)
        (opcdefault, bool, get_struct)
        (opckeytype, Oid, get_struct)
    }
    catalog (pg_operator) {
        (oid, Oid, get_struct)
        (oprname, &CStr, get_struct)
        (oprnamespace, Oid, get_struct)
        (oprowner, Oid, get_struct)
        (oprkind, c_char, character {
            (Prefix, b'l')
            (Infix, b'b')
        }, get_struct)
        (oprcanmerge, bool, get_struct)
        (oprcanhash, bool, get_struct)
        (oprleft, Oid, get_struct)
        (oprright, Oid, get_struct)
        (oprresult, Oid, get_struct)
        (oprcom, Oid, get_struct)
        (oprnegate, Oid, get_struct)
        (oprcode, Regproc, get_struct)
        (oprrest, Regproc, get_struct)
        (oprjoin, Regproc, get_struct)
    }
    catalog (pg_opfamily) {
        (oid, Oid, get_struct)
        (opfmethod, Oid, get_struct)
        (opfname, &CStr, get_struct)
        (opfnamespace, Oid, get_struct)
        (opfowner, Oid, get_struct)
    }
    catalog (pg_proc) {
        (oid, Oid, get_struct)
        (proname, &CStr, get_struct)
        (pronamespace, Oid, get_struct)
        (proowner, Oid, get_struct)
        (prolang, Oid, get_struct)
        (procost, f32, get_struct)
        (prorows, f32, get_struct)
        (provariadic, Oid, get_struct)
        (prosupport, Regproc, get_struct)
        (prokind, c_char, character {
            (Function, b'f')
            (Procedure, b'p')
            (Aggregate, b'a')
            (Window, b'w')
        }, get_struct)
        (prosecdef, bool, get_struct)
        (proleakproof, bool, get_struct)
        (proisstrict, bool, get_struct)
        (proretset, bool, get_struct)
        (provolatile, c_char, character {
            (Immutable, b'i')
            (Stable, b's')
            (Volatile, b'v')
        }, get_struct)
        (proparallel, c_char, character {
            (Safe, b's')
            (Restricted, b'r')
            (Unsafe, b'u')
        }, get_struct)
        (pronargs, i16, get_struct)
        (pronargdefaults, i16, get_struct)
        (prorettype, Oid, get_struct)
        (proargtypes, &[Oid], get_struct)
        (proallargtypes, Array<Oid>, get_attr)
        (proargmodes, Array<c_char>, character {
            (In, b'i')
            (Out, b'o')
            (Inout, b'b')
            (Variadic, b'v')
            (Table, b't')
        }, get_attr)
        (proargnames, Array<String>, get_attr)
        // (proargdefaults, pg_node_tree, get_attr)
        (protrftypes, Array<Oid>, get_attr)
        (prosrc, &str, get_attr, notnull)
        (probin, &str, get_attr)
        // (prosqlbody, pg_node_tree, get_attr)
        (proconfig, Array<String>, get_attr)
        // (proacl, aclitem[], get_attr)
    }
    catalog (pg_type) {
        (oid, Oid, get_struct)
        (typname, &CStr, get_struct)
        (typnamespace, Oid, get_struct)
        (typowner, Oid, get_struct)
        (typlen, i16, get_struct)
        (typbyval, bool, get_struct)
        (typtype, c_char, character {
            (Base, b'b')
            (Composite, b'c')
            (Domain, b'd')
            (Enum, b'e')
            (Multirange, b'm')
            (Pseudo, b'p')
            (Range, b'r')
        }, get_struct)
        (typcategory, c_char, character {
            (Array, b'A')
            (Boolean, b'B')
            (Composite, b'C')
            (DateTime, b'D')
            (Enum, b'E')
            (Geometric, b'G')
            (Network, b'I')
            (Numeric, b'N')
            (PseudoType, b'P')
            (Range, b'R')
            (String, b'S')
            (TimeSpan, b'T')
            (User, b'U')
            (BitString, b'V')
            (Unknown, b'X')
            (Internal, b'Z')
        }, get_struct)
        (typispreferred, bool, get_struct)
        (typisdefined, bool, get_struct)
        (typdelim, c_char, get_struct)
        (typrelid, Oid, get_struct)
        #[cfg(not(any(feature = "pg12", feature = "pg13")))]
        (typsubscript, Regproc, get_struct)
        (typelem, Oid, get_struct)
        (typarray, Oid, get_struct)
        (typinput, Regproc, get_struct)
        (typoutput, Regproc, get_struct)
        (typreceive, Regproc, get_struct)
        (typsend, Regproc, get_struct)
        (typmodin, Regproc, get_struct)
        (typmodout, Regproc, get_struct)
        (typanalyze, Regproc, get_struct)
        (typalign, c_char, character {
            (Char, b'c')
            (Short, b's')
            (Int, b'i')
            (Double, b'd')
        }, get_struct)
        (typstorage, c_char, character {
            (Plain, b'p')
            (External, b'e')
            (Extended, b'x')
            (Main, b'm')
        }, get_struct)
        (typnotnull, bool, get_struct)
        (typbasetype, Oid, get_struct)
        (typtypmod, i32, get_struct)
        (typndims, i32, get_struct)
        (typcollation, Oid, get_struct)
        // (typdefaultbin, pg_node_tree, get_attr)
        (typdefault, String, get_attr)
        // (typacl, aclitem[], get_attr)
    }
    cache (amname, pg_am) {
        (amname, &CStr)
    }
    cache (amoid, pg_am) {
        (oid, Oid)
    }
    cache (amopopid, pg_amop) {
        (amopopr, Oid)
        (amoppurpose, PgAmopAmoppurpose)
        (amopfamily, Oid)
    }
    cache (amopstrategy, pg_amop) {
        (amopfamily, Oid)
        (amoplefttype, Oid)
        (amoprighttype, Oid)
        (amopstrategy, i16)
    }
    cache (amprocnum, pg_amproc) {
        (amprocfamily, Oid)
        (amproclefttype, Oid)
        (amprocrighttype, Oid)
        (amprocnum, i16)
    }
    cache (claamnamensp, pg_opclass) {
        (opcmethod, Oid)
        (opcname, &CStr)
        (opcnamespace, Oid)
    }
    cache (claoid, pg_opclass) {
        (oid, Oid)
    }
    cache (enumoid, pg_enum) {
        (oid, Oid)
    }
    cache (enumtypoidname, pg_enum) {
        (enumtypid, Oid)
        (enumlabel, &CStr)
    }
    cache (indexrelid, pg_index) {
        (indexrelid, Oid)
    }
    cache (namespacename, pg_namespace) {
        (nspname, &CStr)
    }
    cache (namespaceoid, pg_namespace) {
        (oid, Oid)
    }
    cache (opernamensp, pg_operator) {
        (oprname, &CStr)
        (oprleft, Oid)
        (oprright, Oid)
        (oprnamespace, Oid)
    }
    cache (operoid, pg_operator) {
        (oid, Oid)
    }
    cache (opfamilyamnamensp, pg_opfamily) {
        (opfmethod, Oid)
        (opfname, &CStr)
        (opfnamespace, Oid)
    }
    cache (opfamilyoid, pg_opfamily) {
        (oid, Oid)
    }
    cache (procnameargsnsp, pg_proc) {
        (proname, &CStr)
        (proargtypes, &[Oid])
        (pronamespace, Oid)
    }
    cache (procoid, pg_proc) {
        (oid, Oid)
    }
    cache (relnamensp, pg_class) {
        (relname, &CStr)
        (relnamespace, Oid)
    }
    cache (reloid, pg_class) {
        (oid, Oid)
    }
    cache (typenamensp, pg_type) {
        (typname, &CStr)
        (typnamespace, Oid)
    }
    cache (typeoid, pg_type) {
        (oid, Oid)
    }
}

use pgrx::list::List;

impl PgProc<'_> {
    /// Expression trees for default values. This is a [`List`] with `pronargdefaults` elements,
    /// corresponding to the last N input arguments (i.e., the last N proargtypes positions).
    ///
    /// If none of the arguments have defaults, this function returns [`Option::None`].
    pub fn proargdefaults<'cx>(
        &self,
        mcx: &'cx pgrx::memcx::MemCx<'_>,
    ) -> Option<pgrx::list::List<'cx, *mut std::ffi::c_void>> {
        unsafe {
            use crate::{pg_sys, FromDatum};
            use pgrx_pg_sys::AsPgCStr;

            let mut is_null = false;
            let proargdefaults = pg_sys::SysCacheGetAttr(
                pg_sys::SysCacheIdentifier_PROCOID as _,
                std::ptr::addr_of!(*self.inner).cast_mut(),
                pg_sys::Anum_pg_proc_proargdefaults as _,
                &mut is_null,
            );
            let proargdefaults = <&str>::from_datum(proargdefaults, is_null)?;

            let str = proargdefaults.as_pg_cstr();
            let argdefaults = mcx.exec_in(|| pg_sys::stringToNode(str)).cast::<pg_sys::List>();
            pg_sys::pfree(str.cast());
            List::downcast_ptr_in_memcx(argdefaults, mcx)
        }
    }
}
