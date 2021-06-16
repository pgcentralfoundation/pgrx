use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{punctuated::Punctuated, Ident, Token};

#[derive(Debug, Clone)]
pub struct PostgresEnum {
    pub name: Ident,
    pub variants: Punctuated<syn::Variant, Token![,]>,
}

impl PostgresEnum {
    pub fn new(name: Ident, variants: Punctuated<syn::Variant, Token![,]>) -> Self {
        Self { name, variants }
    }
}

impl ToTokens for PostgresEnum {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = &self.name;
        let variants = self.variants.iter();
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
                crate::__pgx_internals::PostgresEnum(pgx_utils::pg_inventory::InventoryPostgresEnum {
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
                    variants: vec![ #(  stringify!(#variants)  ),* ],
                })
            }
        };
        tokens.append_all(inv);
    }
}


#[derive(Debug, Clone)]
pub struct InventoryPostgresEnum {
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
    pub variants: Vec<&'static str>,
}