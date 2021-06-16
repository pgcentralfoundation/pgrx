use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};
use core::{any::Any, marker::PhantomData};
use std::mem::MaybeUninit;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct PostgresType {
    name: Ident,
    in_fn: Ident,
    out_fn: Ident,
}

impl PostgresType {
    pub fn new(name: Ident, in_fn: Ident, out_fn: Ident) -> Self {
        Self {
            name,
            in_fn,
            out_fn,
        }
    }
}

impl ToTokens for PostgresType {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = &self.name;
        let in_fn = &self.in_fn;
        let out_fn = &self.out_fn;
        let inv = quote! {
            pgx::inventory::submit! {
                use core::{mem::MaybeUninit, any::{TypeId, Any}, marker::PhantomData};
                use crate::__pgx_internals::{WithoutTypeIds, WithoutArrayTypeId, WithoutVarlenaTypeId};
                println!("WithBasicTypeIds {:?} base {:?} opt {:?} vec {:?} arr {:?} varl {:?}",
                    stringify!(#name),
                    *crate::__pgx_internals::WithBasicTypeIds::<#name>::ITEM_ID,
                    *crate::__pgx_internals::WithBasicTypeIds::<#name>::OPTION_ID,
                    *crate::__pgx_internals::WithBasicTypeIds::<#name>::VEC_ID,
                    *crate::__pgx_internals::WithArrayTypeId::<#name>::ARRAY_ID,
                    *crate::__pgx_internals::WithVarlenaTypeId::<#name>::VARLENA_ID,
                );
                crate::__pgx_internals::PostgresType(pgx_utils::pg_inventory::InventoryPostgresType {
                    name: stringify!(#name),
                    file: file!(),
                    line: line!(),
                    module_path: module_path!(),
                    full_path: core::any::type_name::<#name>(),
                    id: TypeId::of::<#name>(),
                    option_id: TypeId::of::<Option<#name>>(),
                    vec_id: TypeId::of::<Vec<#name>>(),
                    array_id: *crate::__pgx_internals::WithArrayTypeId::<#name>::ARRAY_ID,
                    varlena_id: *crate::__pgx_internals::WithVarlenaTypeId::<#name>::VARLENA_ID,
                    in_fn: stringify!(#in_fn),
                    out_fn: stringify!(#out_fn),
                })
            }
        };
        tokens.append_all(inv);
    }
}

#[derive(Debug, Clone)]
pub struct InventoryPostgresType {
    pub name: &'static str,
    pub file: &'static str,
    pub line: u32,
    pub full_path: &'static str,
    pub module_path: &'static str,
    pub id: core::any::TypeId,
    pub option_id: core::any::TypeId,
    pub vec_id: core::any::TypeId,
    pub array_id: Option<core::any::TypeId>,
    pub varlena_id: Option<core::any::TypeId>,
    pub in_fn: &'static str,
    pub out_fn: &'static str,
}