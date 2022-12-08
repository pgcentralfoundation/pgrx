/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
/*!

`#[derive(PostgresType)]` related macro expansion for Rust to SQL translation

> Like all of the [`sql_entity_graph`][crate::pgx_sql_entity_graph] APIs, this is considered **internal**
to the `pgx` framework and very subject to change between versions. While you may use this, please do it with caution.

*/
pub mod entity;

use crate::enrich::{ToEntityGraphTokens, ToRustCodeTokens};
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{DeriveInput, Generics, ItemStruct};

use crate::{CodeEnrichment, ToSqlConfig};

/// A parsed `#[derive(PostgresType)]` item.
///
/// It should be used with [`syn::parse::Parse`] functions.
///
/// Using [`quote::ToTokens`] will output the declaration for a [`PostgresTypeEntity`][crate::PostgresTypeEntity].
///
/// ```rust
/// use syn::{Macro, parse::Parse, parse_quote, parse};
/// use quote::{quote, ToTokens};
/// use pgx_sql_entity_graph::PostgresType;
///
/// # fn main() -> eyre::Result<()> {
/// use pgx_sql_entity_graph::CodeEnrichment;
/// let parsed: CodeEnrichment<PostgresType> = parse_quote! {
///     #[derive(PostgresType)]
///     struct Example<'a> {
///         demo: &'a str,
///     }
/// };
/// let sql_graph_entity_tokens = parsed.to_token_stream();
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct PostgresType {
    name: Ident,
    generics: Generics,
    in_fn: Ident,
    out_fn: Ident,
    to_sql_config: ToSqlConfig,
}

impl PostgresType {
    pub fn new(
        name: Ident,
        generics: Generics,
        in_fn: Ident,
        out_fn: Ident,
        to_sql_config: ToSqlConfig,
    ) -> Result<CodeEnrichment<Self>, syn::Error> {
        if !to_sql_config.overrides_default() {
            crate::ident_is_acceptable_to_postgres(&name)?;
        }
        Ok(CodeEnrichment(Self { generics, name, in_fn, out_fn, to_sql_config }))
    }

    pub fn from_derive_input(
        derive_input: DeriveInput,
    ) -> Result<CodeEnrichment<Self>, syn::Error> {
        match derive_input.data {
            syn::Data::Struct(_) | syn::Data::Enum(_) => {}
            syn::Data::Union(_) => {
                return Err(syn::Error::new(derive_input.ident.span(), "expected struct or enum"))
            }
        };
        let to_sql_config =
            ToSqlConfig::from_attributes(derive_input.attrs.as_slice())?.unwrap_or_default();
        let funcname_in = Ident::new(
            &format!("{}_in", derive_input.ident).to_lowercase(),
            derive_input.ident.span(),
        );
        let funcname_out = Ident::new(
            &format!("{}_out", derive_input.ident).to_lowercase(),
            derive_input.ident.span(),
        );
        Self::new(
            derive_input.ident,
            derive_input.generics,
            funcname_in,
            funcname_out,
            to_sql_config,
        )
    }
}

impl ToEntityGraphTokens for PostgresType {
    fn to_entity_graph_tokens(&self) -> TokenStream2 {
        let name = &self.name;
        let mut static_generics = self.generics.clone();
        static_generics.params = static_generics
            .params
            .clone()
            .into_iter()
            .flat_map(|param| match param {
                item @ syn::GenericParam::Type(_) | item @ syn::GenericParam::Const(_) => {
                    Some(item)
                }
                syn::GenericParam::Lifetime(mut lifetime) => {
                    lifetime.lifetime.ident = Ident::new("static", Span::call_site());
                    Some(syn::GenericParam::Lifetime(lifetime))
                }
            })
            .collect();
        let mut staticless_generics = self.generics.clone();
        staticless_generics.params = static_generics
            .params
            .clone()
            .into_iter()
            .flat_map(|param| match param {
                item @ syn::GenericParam::Type(_) | item @ syn::GenericParam::Const(_) => {
                    Some(item)
                }
                syn::GenericParam::Lifetime(_) => None,
            })
            .collect();
        let (staticless_impl_generics, _staticless_ty_generics, _staticless_where_clauses) =
            staticless_generics.split_for_impl();
        let (_static_impl_generics, static_ty_generics, static_where_clauses) =
            static_generics.split_for_impl();

        let in_fn = &self.in_fn;
        let out_fn = &self.out_fn;

        let sql_graph_entity_fn_name =
            syn::Ident::new(&format!("__pgx_internals_type_{}", self.name), Span::call_site());

        let to_sql_config = &self.to_sql_config;

        quote! {
            unsafe impl #staticless_impl_generics ::pgx::pgx_sql_entity_graph::metadata::SqlTranslatable for #name #static_ty_generics #static_where_clauses {
                fn argument_sql() -> core::result::Result<::pgx::pgx_sql_entity_graph::metadata::SqlMapping, ::pgx::pgx_sql_entity_graph::metadata::ArgumentError> {
                    Ok(::pgx::pgx_sql_entity_graph::metadata::SqlMapping::As(String::from(stringify!(#name))))
                }

                fn return_sql() -> core::result::Result<::pgx::pgx_sql_entity_graph::metadata::Returns, ::pgx::pgx_sql_entity_graph::metadata::ReturnsError> {
                    Ok(::pgx::pgx_sql_entity_graph::metadata::Returns::One(::pgx::pgx_sql_entity_graph::metadata::SqlMapping::As(String::from(stringify!(#name)))))
                }
            }


            #[no_mangle]
            #[doc(hidden)]
            pub extern "Rust" fn  #sql_graph_entity_fn_name() -> ::pgx::pgx_sql_entity_graph::SqlGraphEntity {
                extern crate alloc;
                use alloc::vec::Vec;
                use alloc::vec;
                use alloc::string::{String, ToString};
                use ::pgx::datum::WithTypeIds;

                let mut mappings = Default::default();
                <#name #static_ty_generics as ::pgx::datum::WithTypeIds>::register_with_refs(
                    &mut mappings,
                    stringify!(#name).to_string()
                );
                ::pgx::datum::WithSizedTypeIds::<#name #static_ty_generics>::register_sized_with_refs(
                    &mut mappings,
                    stringify!(#name).to_string()
                );
                ::pgx::datum::WithArrayTypeIds::<#name #static_ty_generics>::register_array_with_refs(
                    &mut mappings,
                    stringify!(#name).to_string()
                );
                ::pgx::datum::WithVarlenaTypeIds::<#name #static_ty_generics>::register_varlena_with_refs(
                    &mut mappings,
                    stringify!(#name).to_string()
                );
                let submission = ::pgx::pgx_sql_entity_graph::PostgresTypeEntity {
                    name: stringify!(#name),
                    file: file!(),
                    line: line!(),
                    module_path: module_path!(),
                    full_path: core::any::type_name::<#name #static_ty_generics>(),
                    mappings,
                    in_fn: stringify!(#in_fn),
                    in_fn_module_path: {
                        let in_fn = stringify!(#in_fn);
                        let mut path_items: Vec<_> = in_fn.split("::").collect();
                        let _ = path_items.pop(); // Drop the one we don't want.
                        path_items.join("::")
                    },
                    out_fn: stringify!(#out_fn),
                    out_fn_module_path: {
                        let out_fn = stringify!(#out_fn);
                        let mut path_items: Vec<_> = out_fn.split("::").collect();
                        let _ = path_items.pop(); // Drop the one we don't want.
                        path_items.join("::")
                    },
                    to_sql_config: #to_sql_config,
                };
                ::pgx::pgx_sql_entity_graph::SqlGraphEntity::Type(submission)
            }
        }
    }
}

impl ToRustCodeTokens for PostgresType {}

impl Parse for CodeEnrichment<PostgresType> {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let parsed: ItemStruct = input.parse()?;
        let to_sql_config =
            ToSqlConfig::from_attributes(parsed.attrs.as_slice())?.unwrap_or_default();
        let funcname_in =
            Ident::new(&format!("{}_in", parsed.ident).to_lowercase(), parsed.ident.span());
        let funcname_out =
            Ident::new(&format!("{}_out", parsed.ident).to_lowercase(), parsed.ident.span());
        PostgresType::new(parsed.ident, parsed.generics, funcname_in, funcname_out, to_sql_config)
    }
}
