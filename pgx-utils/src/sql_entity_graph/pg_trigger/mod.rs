pub mod entity;

use quote::{quote, TokenStreamExt};
use syn::{ItemFn, parse::{ParseStream, Parse}};
use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use crate::sql_entity_graph::ToSqlConfig;

#[derive(Debug, Clone)]
pub struct PgTrigger {
    func: syn::ItemFn,
    to_sql_config: ToSqlConfig,
}

impl PgTrigger {
    pub fn new(func: ItemFn) -> Result<Self, syn::Error> {
        let to_sql_config =
            ToSqlConfig::from_attributes(func.attrs.as_slice())?.unwrap_or_default();
        
        Ok(Self {
            func,
            to_sql_config
        })
    }

    pub fn entity_tokens(&self) -> Result<ItemFn, syn::Error> {
        let sql_graph_entity_fn_name = syn::Ident::new(
            &format!("__pgx_internals_trigger_{}", self.func.sig.ident.to_string()),
            self.func.sig.ident.span(),
        );
        let func_sig_ident = &self.func.sig.ident;
        let function_name = func_sig_ident.to_string();
        let to_sql_config = &self.to_sql_config;

        let tokens = quote! {
            #[no_mangle]
            #[doc(hidden)]
            pub extern "C" fn #sql_graph_entity_fn_name() -> ::pgx::utils::sql_entity_graph::SqlGraphEntity {
                use core::any::TypeId;
                extern crate alloc;
                use alloc::vec::Vec;
                use alloc::vec;
                let submission = ::pgx::utils::sql_entity_graph::PgTriggerEntity {
                    function_name: #function_name,
                    file: file!(),
                    line: line!(),
                    full_path: concat!(module_path!(), "::", stringify!(#func_sig_ident)),
                    module_path: module_path!(),
                    to_sql_config: #to_sql_config,
                };
                ::pgx::utils::sql_entity_graph::SqlGraphEntity::Trigger(submission)
            }
        };
        syn::parse2(tokens)
    }

    pub fn wrapper_tokens(&self) -> Result<ItemFn, syn::Error> {
        let function_ident = &self.func.sig.ident;
        let extern_func_ident = syn::Ident::new(
            &format!("{}_wrapper", self.func.sig.ident.to_string()),
            self.func.sig.ident.span(),
        );
        let tokens = quote! {
            #[no_mangle]
            #[pg_guard]
            extern "C" fn #extern_func_ident(fcinfo: ::pgx::pg_sys::FunctionCallInfo) -> ::pgx::pg_sys::Datum {
                let maybe_pg_trigger = unsafe { ::pgx::trigger_support::PgTrigger::from_fcinfo(fcinfo) };
                let pg_trigger = maybe_pg_trigger.expect("PgTrigger::from_fcinfo failed");
                let trigger_fn_result: Result<
                    Option<::pgx::PgHeapTuple<'_, _>>,
                    _,
                > = #function_ident(&pg_trigger);
            
                let trigger_retval = trigger_fn_result.expect("Trigger function panic");
                let retval_datum = trigger_retval.into_datum();

                retval_datum.expect("Failed to turn trigger function return value into Datum")
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
            pub extern "C" fn #finfo_name() -> &'static pg_sys::Pg_finfo_record {
                const V1_API: pg_sys::Pg_finfo_record = pg_sys::Pg_finfo_record { api_version: 1 };
                &V1_API
            }
        };
        syn::parse2(tokens)
    }
}

impl Parse for PgTrigger {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Self::new(input.parse()?)
    }
}

impl ToTokens for PgTrigger {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let entity_func = self.entity_tokens().expect("Generating entity function for trigger");
        let wrapper_func = self.wrapper_tokens().expect("Generating wrappper function for trigger");
        let finfo_func = self.finfo_tokens().expect("Generating finfo function for trigger");
        let func = &self.func;
        
        let items = quote! {
            #func

            #wrapper_func

            #finfo_func

            #entity_func
        };
        tokens.append_all(items);
    }
}