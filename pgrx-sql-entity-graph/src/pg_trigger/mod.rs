//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
/*!

`#[pg_trigger]` related macro expansion for Rust to SQL translation

> Like all of the [`sql_entity_graph`][crate::pgrx_sql_entity_graph] APIs, this is considered **internal**
to the `pgrx` framework and very subject to change between versions. While you may use this, please do it with caution.

*/
pub mod attribute;
pub mod entity;

use crate::enrich::{ToEntityGraphTokens, ToRustCodeTokens};
use crate::{CodeEnrichment, ToSqlConfig};
use attribute::PgTriggerAttribute;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{ItemFn, Token};

#[derive(Debug, Clone)]
pub struct PgTrigger {
    func: syn::ItemFn,
    to_sql_config: ToSqlConfig,
}

impl PgTrigger {
    pub fn new(
        func: ItemFn,
        attributes: syn::punctuated::Punctuated<PgTriggerAttribute, Token![,]>,
    ) -> Result<CodeEnrichment<Self>, syn::Error> {
        if attributes.len() > 1 {
            return Err(syn::Error::new(
                Span::call_site(),
                "Multiple `sql` arguments found, it must be unique",
            ));
        };
        let to_sql_config = attributes
            .first()
            .cloned()
            .map(|PgTriggerAttribute::Sql(mut config)| {
                if let Some(ref mut content) = config.content {
                    let value = content.value();
                    let updated_value = value
                        .replace("@FUNCTION_NAME@", &*(func.sig.ident.to_string() + "_wrapper"))
                        + "\n";
                    *content = syn::LitStr::new(&updated_value, Span::call_site());
                };
                config
            })
            .unwrap_or_default();

        if !to_sql_config.overrides_default() {
            crate::ident_is_acceptable_to_postgres(&func.sig.ident)?;
        }

        Ok(CodeEnrichment(PgTrigger { func, to_sql_config }))
    }

    pub fn wrapper_tokens(&self) -> Result<ItemFn, syn::Error> {
        let function_ident = &self.func.sig.ident;
        let extern_func_ident = syn::Ident::new(
            &format!("{}_wrapper", self.func.sig.ident.to_string()),
            self.func.sig.ident.span(),
        );
        let tokens = quote! {
            #[no_mangle]
            #[::pgrx::pgrx_macros::pg_guard]
            unsafe extern "C" fn #extern_func_ident(fcinfo: ::pgrx::pg_sys::FunctionCallInfo) -> ::pgrx::pg_sys::Datum {
                let fcinfo_ref = unsafe {
                    // SAFETY:  The caller should be Postgres in this case and it will give us a valid "fcinfo" pointer
                    fcinfo.as_ref().expect("fcinfo was NULL from Postgres")
                };
                let maybe_pg_trigger = unsafe { ::pgrx::trigger_support::PgTrigger::from_fcinfo(fcinfo_ref) };
                let pg_trigger = maybe_pg_trigger.expect("PgTrigger::from_fcinfo failed");
                let trigger_fn_result: Result<
                    Option<::pgrx::heap_tuple::PgHeapTuple<'_, _>>,
                    _,
                > = #function_ident(&pg_trigger);


                // The trigger "protocol" allows a function to return the null pointer, but NOT to
                // set the isnull result flag.  This is why we return `Datum::from(0)` in the None cases
                let trigger_retval = trigger_fn_result.expect("Trigger function panic");
                match trigger_retval {
                    None => unsafe { ::pgrx::pg_sys::Datum::from(0) },
                    Some(trigger_retval) => match trigger_retval.into_trigger_datum() {
                        None => unsafe { ::pgrx::pg_sys::Datum::from(0) },
                        Some(datum) => datum,
                    }
                }
            }

        };
        syn::parse2(tokens)
    }

    pub fn finfo_tokens(&self) -> Result<ItemFn, syn::Error> {
        let finfo_name = syn::Ident::new(
            &format!("pg_finfo_{}_wrapper", self.func.sig.ident),
            proc_macro2::Span::call_site(),
        );
        let tokens = quote! {
            #[no_mangle]
            #[doc(hidden)]
            pub extern "C" fn #finfo_name() -> &'static ::pgrx::pg_sys::Pg_finfo_record {
                const V1_API: ::pgrx::pg_sys::Pg_finfo_record = ::pgrx::pg_sys::Pg_finfo_record { api_version: 1 };
                &V1_API
            }
        };
        syn::parse2(tokens)
    }
}

impl ToEntityGraphTokens for PgTrigger {
    fn to_entity_graph_tokens(&self) -> TokenStream2 {
        let sql_graph_entity_fn_name = syn::Ident::new(
            &format!("__pgrx_internals_trigger_{}", self.func.sig.ident.to_string()),
            self.func.sig.ident.span(),
        );
        let func_sig_ident = &self.func.sig.ident;
        let function_name = func_sig_ident.to_string();
        let to_sql_config = &self.to_sql_config;

        quote! {
            #[no_mangle]
            #[doc(hidden)]
            #[allow(unknown_lints, clippy::no_mangle_with_rust_abi)]
            pub extern "Rust" fn #sql_graph_entity_fn_name() -> ::pgrx::pgrx_sql_entity_graph::SqlGraphEntity {
                use core::any::TypeId;
                extern crate alloc;
                use alloc::vec::Vec;
                use alloc::vec;
                let submission = ::pgrx::pgrx_sql_entity_graph::PgTriggerEntity {
                    function_name: #function_name,
                    file: file!(),
                    line: line!(),
                    full_path: concat!(module_path!(), "::", stringify!(#func_sig_ident)),
                    module_path: module_path!(),
                    to_sql_config: #to_sql_config,
                };
                ::pgrx::pgrx_sql_entity_graph::SqlGraphEntity::Trigger(submission)
            }
        }
    }
}

impl ToRustCodeTokens for PgTrigger {
    fn to_rust_code_tokens(&self) -> TokenStream2 {
        let wrapper_func = self.wrapper_tokens().expect("Generating wrappper function for trigger");
        let finfo_func = self.finfo_tokens().expect("Generating finfo function for trigger");
        let func = &self.func;

        quote! {
            #func
            #wrapper_func
            #finfo_func
        }
    }
}
