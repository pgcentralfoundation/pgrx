use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};

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
                use ::pgx::datum::{
                    WithTypeIds,
                    WithoutArrayTypeId,
                    WithVarlenaTypeId,
                    WithArrayTypeId,
                    WithoutVarlenaTypeId
                };
                crate::__pgx_internals::PostgresType(pgx_utils::pg_inventory::InventoryPostgresType {
                    name: stringify!(#name),
                    file: file!(),
                    line: line!(),
                    module_path: module_path!(),
                    full_path: core::any::type_name::<#name>(),
                    id: *<#name as WithTypeIds>::ITEM_ID,
                    option_id: *<#name as WithTypeIds>::OPTION_ID,
                    vec_id: *<#name as WithTypeIds>::VEC_ID,
                    array_id: *WithArrayTypeId::<#name>::ARRAY_ID,
                    varlena_id: *WithVarlenaTypeId::<#name>::VARLENA_ID,
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