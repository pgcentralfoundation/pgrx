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

`#[pg_extern]` related macro expansion for Rust to SQL translation

> Like all of the [`sql_entity_graph`][crate] APIs, this is considered **internal**
to the `pgrx` framework and very subject to change between versions. While you may use this, please do it with caution.

*/
mod argument;
mod attribute;
pub mod entity;
mod operator;
mod returning;
mod search_path;

pub use argument::PgExternArgument;
pub use operator::PgOperator;
pub use returning::NameMacro;

use crate::ToSqlConfig;
use attribute::Attribute;
use operator::{PgrxOperatorAttributeWithIdent, PgrxOperatorOpName};
use search_path::SearchPathList;

use crate::enrich::CodeEnrichment;
use crate::enrich::ToEntityGraphTokens;
use crate::enrich::ToRustCodeTokens;
use crate::lifetimes::staticize_lifetimes;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{quote, quote_spanned, ToTokens};
use syn::parse::{Parse, ParseStream, Parser};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Meta, Token};

use self::returning::Returning;

use super::UsedType;

/// A parsed `#[pg_extern]` item.
///
/// It should be used with [`syn::parse::Parse`] functions.
///
/// Using [`quote::ToTokens`] will output the declaration for a [`PgExternEntity`][crate::PgExternEntity].
///
/// ```rust
/// use syn::{Macro, parse::Parse, parse_quote, parse};
/// use quote::{quote, ToTokens};
/// use pgrx_sql_entity_graph::PgExtern;
///
/// # fn main() -> eyre::Result<()> {
/// use pgrx_sql_entity_graph::CodeEnrichment;
/// let parsed: CodeEnrichment<PgExtern> = parse_quote! {
///     fn example(x: Option<str>) -> Option<&'a str> {
///         unimplemented!()
///     }
/// };
/// let sql_graph_entity_tokens = parsed.to_token_stream();
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct PgExtern {
    attrs: Vec<Attribute>,
    func: syn::ItemFn,
    to_sql_config: ToSqlConfig,
    operator: Option<PgOperator>,
    search_path: Option<SearchPathList>,
    inputs: Vec<PgExternArgument>,
    input_types: Vec<syn::Type>,
    returns: Returning,
}

impl PgExtern {
    pub fn new(attr: TokenStream2, item: TokenStream2) -> Result<CodeEnrichment<Self>, syn::Error> {
        let mut attrs = Vec::new();
        let mut to_sql_config: Option<ToSqlConfig> = None;

        let parser = Punctuated::<Attribute, Token![,]>::parse_terminated;
        let punctuated_attrs = parser.parse2(attr)?;
        for pair in punctuated_attrs.into_pairs() {
            match pair.into_value() {
                Attribute::Sql(config) => {
                    to_sql_config.get_or_insert(config);
                }
                attr => {
                    attrs.push(attr);
                }
            }
        }

        let mut to_sql_config = to_sql_config.unwrap_or_default();

        let func = syn::parse2::<syn::ItemFn>(item)?;

        if let Some(ref mut content) = to_sql_config.content {
            let value = content.value();
            let updated_value =
                value.replace("@FUNCTION_NAME@", &(func.sig.ident.to_string() + "_wrapper")) + "\n";
            *content = syn::LitStr::new(&updated_value, Span::call_site());
        }

        if !to_sql_config.overrides_default() {
            crate::ident_is_acceptable_to_postgres(&func.sig.ident)?;
        }
        let operator = Self::operator(&func)?;
        let search_path = Self::search_path(&func)?;
        let inputs = Self::inputs(&func)?;
        let input_types = Self::input_types(&func)?;
        let returns = Returning::try_from(&func.sig.output)?;
        Ok(CodeEnrichment(Self {
            attrs,
            func,
            to_sql_config,
            operator,
            search_path,
            inputs,
            input_types,
            returns,
        }))
    }

    fn input_types(func: &syn::ItemFn) -> syn::Result<Vec<syn::Type>> {
        func.sig
            .inputs
            .iter()
            .filter_map(|v| -> Option<syn::Result<syn::Type>> {
                match v {
                    syn::FnArg::Receiver(_) => None,
                    syn::FnArg::Typed(pat_ty) => {
                        let static_ty = pat_ty.ty.clone();
                        let mut static_ty = match UsedType::new(*static_ty) {
                            Ok(v) => v.resolved_ty,
                            Err(e) => return Some(Err(e)),
                        };
                        staticize_lifetimes(&mut static_ty);
                        Some(Ok(static_ty))
                    }
                }
            })
            .collect()
    }

    fn name(&self) -> String {
        self.attrs
            .iter()
            .find_map(|a| match a {
                Attribute::Name(name) => Some(name.value()),
                _ => None,
            })
            .unwrap_or_else(|| self.func.sig.ident.to_string())
    }

    fn schema(&self) -> Option<String> {
        self.attrs.iter().find_map(|a| match a {
            Attribute::Schema(name) => Some(name.value()),
            _ => None,
        })
    }

    pub fn extern_attrs(&self) -> &[Attribute] {
        self.attrs.as_slice()
    }

    fn overridden(&self) -> Option<syn::LitStr> {
        let mut span = None;
        let mut retval = None;
        let mut in_commented_sql_block = false;
        for attr in &self.func.attrs {
            let meta = attr.parse_meta().ok();
            if let Some(meta) = meta {
                if meta.path().is_ident("doc") {
                    let content = match meta {
                        Meta::Path(_) | Meta::List(_) => continue,
                        Meta::NameValue(mnv) => mnv,
                    };
                    if let syn::Lit::Str(ref inner) = content.lit {
                        span.get_or_insert(content.lit.span());
                        if !in_commented_sql_block && inner.value().trim() == "```pgrxsql" {
                            in_commented_sql_block = true;
                        } else if in_commented_sql_block && inner.value().trim() == "```" {
                            in_commented_sql_block = false;
                        } else if in_commented_sql_block {
                            let line = inner.value().trim_start().replace(
                                "@FUNCTION_NAME@",
                                &(self.func.sig.ident.to_string() + "_wrapper"),
                            ) + "\n";
                            retval.get_or_insert_with(String::default).push_str(&line);
                        }
                    }
                }
            }
        }
        retval.map(|s| syn::LitStr::new(s.as_ref(), span.unwrap()))
    }

    fn operator(func: &syn::ItemFn) -> syn::Result<Option<PgOperator>> {
        let mut skel = Option::<PgOperator>::default();
        for attr in &func.attrs {
            let last_segment = attr.path.segments.last().unwrap();
            match last_segment.ident.to_string().as_str() {
                "opname" => {
                    let attr: PgrxOperatorOpName = syn::parse2(attr.tokens.clone())?;
                    skel.get_or_insert_with(Default::default).opname.get_or_insert(attr);
                }
                "commutator" => {
                    let attr: PgrxOperatorAttributeWithIdent = syn::parse2(attr.tokens.clone())?;
                    skel.get_or_insert_with(Default::default).commutator.get_or_insert(attr);
                }
                "negator" => {
                    let attr: PgrxOperatorAttributeWithIdent = syn::parse2(attr.tokens.clone())?;
                    skel.get_or_insert_with(Default::default).negator.get_or_insert(attr);
                }
                "join" => {
                    let attr: PgrxOperatorAttributeWithIdent = syn::parse2(attr.tokens.clone())?;
                    skel.get_or_insert_with(Default::default).join.get_or_insert(attr);
                }
                "restrict" => {
                    let attr: PgrxOperatorAttributeWithIdent = syn::parse2(attr.tokens.clone())?;
                    skel.get_or_insert_with(Default::default).restrict.get_or_insert(attr);
                }
                "hashes" => {
                    skel.get_or_insert_with(Default::default).hashes = true;
                }
                "merges" => {
                    skel.get_or_insert_with(Default::default).merges = true;
                }
                _ => (),
            }
        }
        Ok(skel)
    }

    fn search_path(func: &syn::ItemFn) -> syn::Result<Option<SearchPathList>> {
        func.attrs
            .iter()
            .find(|f| {
                f.path
                    .segments
                    .first()
                    .map(|f| f.ident == Ident::new("search_path", Span::call_site()))
                    .unwrap_or_default()
            })
            .map(|attr| attr.parse_args::<SearchPathList>())
            .transpose()
    }

    fn inputs(func: &syn::ItemFn) -> syn::Result<Vec<PgExternArgument>> {
        let mut args = Vec::default();
        for input in &func.sig.inputs {
            let arg = PgExternArgument::build(input.clone())?;
            args.push(arg);
        }
        Ok(args)
    }

    fn entity_tokens(&self) -> TokenStream2 {
        let ident = &self.func.sig.ident;
        let name = self.name();
        let unsafety = &self.func.sig.unsafety;
        let schema = self.schema();
        let schema_iter = schema.iter();
        let extern_attrs = self
            .attrs
            .iter()
            .map(|attr| attr.to_sql_entity_graph_tokens())
            .collect::<Punctuated<_, Token![,]>>();
        let search_path = self.search_path.clone().into_iter();
        let inputs = &self.inputs;
        let inputs_iter = inputs.iter().map(|v| v.entity_tokens());

        let input_types = self.input_types.iter().cloned();

        let returns = &self.returns;

        let return_type = match &self.func.sig.output {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(arrow, ty) => {
                let mut static_ty = ty.clone();
                staticize_lifetimes(&mut static_ty);
                Some(syn::ReturnType::Type(*arrow, static_ty))
            }
        };

        let operator = self.operator.clone().into_iter();
        let to_sql_config = match self.overridden() {
            None => self.to_sql_config.clone(),
            Some(content) => {
                let mut config = self.to_sql_config.clone();
                config.content = Some(content);
                config
            }
        };

        let sql_graph_entity_fn_name =
            syn::Ident::new(&format!("__pgrx_internals_fn_{}", ident), Span::call_site());
        quote_spanned! { self.func.sig.span() =>
            #[no_mangle]
            #[doc(hidden)]
            #[allow(unknown_lints, clippy::no_mangle_with_rust_abi)]
            pub extern "Rust" fn  #sql_graph_entity_fn_name() -> ::pgrx::pgrx_sql_entity_graph::SqlGraphEntity {
                extern crate alloc;
                #[allow(unused_imports)]
                use alloc::{vec, vec::Vec};
                type FunctionPointer = #unsafety fn(#( #input_types ),*) #return_type;
                let metadata: FunctionPointer = #ident;
                let submission = ::pgrx::pgrx_sql_entity_graph::PgExternEntity {
                    name: #name,
                    unaliased_name: stringify!(#ident),
                    module_path: core::module_path!(),
                    full_path: concat!(core::module_path!(), "::", stringify!(#ident)),
                    metadata: ::pgrx::pgrx_sql_entity_graph::metadata::FunctionMetadata::entity(&metadata),
                    fn_args: vec![#(#inputs_iter),*],
                    fn_return: #returns,
                    #[allow(clippy::or_fun_call)]
                    schema: None #( .unwrap_or_else(|| Some(#schema_iter)) )*,
                    file: file!(),
                    line: line!(),
                    extern_attrs: vec![#extern_attrs],
                    #[allow(clippy::or_fun_call)]
                    search_path: None #( .unwrap_or_else(|| Some(vec![#search_path])) )*,
                    #[allow(clippy::or_fun_call)]
                    operator: None #( .unwrap_or_else(|| Some(#operator)) )*,
                    to_sql_config: #to_sql_config,
                };
                ::pgrx::pgrx_sql_entity_graph::SqlGraphEntity::Function(submission)
            }
        }
    }

    fn finfo_tokens(&self) -> TokenStream2 {
        let finfo_name = syn::Ident::new(
            &format!("pg_finfo_{}_wrapper", self.func.sig.ident),
            Span::call_site(),
        );
        quote_spanned! { self.func.sig.span() =>
            #[no_mangle]
            #[doc(hidden)]
            pub extern "C" fn #finfo_name() -> &'static ::pgrx::pg_sys::Pg_finfo_record {
                const V1_API: ::pgrx::pg_sys::Pg_finfo_record = ::pgrx::pg_sys::Pg_finfo_record { api_version: 1 };
                &V1_API
            }
        }
    }

    pub fn wrapper_func(&self) -> TokenStream2 {
        let func_name = &self.func.sig.ident;
        let func_name_wrapper = Ident::new(
            &format!("{}_wrapper", &self.func.sig.ident.to_string()),
            self.func.sig.ident.span(),
        );
        let func_generics = &self.func.sig.generics;
        let is_raw = self.extern_attrs().contains(&Attribute::Raw);
        // We use a `_` prefix to make functions with no args more satisfied during linting.
        let fcinfo_ident = syn::Ident::new("_fcinfo", self.func.sig.ident.span());

        let args = &self.inputs;
        let arg_pats = args
            .iter()
            .map(|v| syn::Ident::new(&format!("{}_", &v.pat), self.func.sig.span()))
            .collect::<Vec<_>>();
        let arg_fetches = args.iter().enumerate().map(|(idx, arg)| {
            let pat = &arg_pats[idx];
            let resolved_ty = &arg.used_ty.resolved_ty;
            if arg.used_ty.resolved_ty.to_token_stream().to_string() == quote!(pgrx::pg_sys::FunctionCallInfo).to_token_stream().to_string()
                || arg.used_ty.resolved_ty.to_token_stream().to_string() == quote!(pg_sys::FunctionCallInfo).to_token_stream().to_string()
                || arg.used_ty.resolved_ty.to_token_stream().to_string() == quote!(::pgrx::pg_sys::FunctionCallInfo).to_token_stream().to_string()
            {
                quote_spanned! {pat.span()=>
                    let #pat = #fcinfo_ident;
                }
            } else if arg.used_ty.resolved_ty.to_token_stream().to_string() == quote!(()).to_token_stream().to_string() {
                quote_spanned! {pat.span()=>
                    debug_assert!(unsafe { ::pgrx::fcinfo::pg_getarg::<()>(#fcinfo_ident, #idx).is_none() }, "A `()` argument should always receive `NULL`");
                    let #pat = ();
                }
            } else {
                match (is_raw, &arg.used_ty.optional) {
                    (true, None) | (true, Some(_)) => quote_spanned! { pat.span() =>
                        let #pat = unsafe { ::pgrx::fcinfo::pg_getarg_datum_raw(#fcinfo_ident, #idx) as #resolved_ty };
                    },
                    (false, None) => quote_spanned! { pat.span() =>
                        let #pat = unsafe { ::pgrx::fcinfo::pg_getarg::<#resolved_ty>(#fcinfo_ident, #idx).unwrap_or_else(|| panic!("{} is null", stringify!{#pat})) };
                    },
                    (false, Some(inner)) => quote_spanned! { pat.span() =>
                        let #pat = unsafe { ::pgrx::fcinfo::pg_getarg::<#inner>(#fcinfo_ident, #idx) };
                    },
                }
            }
        });

        match &self.returns {
            Returning::None => quote_spanned! { self.func.sig.span() =>
                  #[no_mangle]
                  #[doc(hidden)]
                  #[::pgrx::pgrx_macros::pg_guard]
                  pub unsafe extern "C" fn #func_name_wrapper #func_generics(#fcinfo_ident: ::pgrx::pg_sys::FunctionCallInfo) {
                      #(
                          #arg_fetches
                      )*

                    #[allow(unused_unsafe)] // unwrapped fn might be unsafe
                    unsafe { #func_name(#(#arg_pats),*) }
                }
            },
            Returning::Type(retval_ty) => {
                let result_ident = syn::Ident::new("result", self.func.sig.span());
                let retval_transform = if retval_ty.resolved_ty == syn::parse_quote!(()) {
                    quote_spanned! { self.func.sig.output.span() =>
                       unsafe { ::pgrx::fcinfo::pg_return_void() }
                    }
                } else if retval_ty.result {
                    if retval_ty.optional.is_some() {
                        // returning `Result<Option<T>>`
                        quote_spanned! {
                            self.func.sig.output.span() =>
                                match ::pgrx::datum::IntoDatum::into_datum(#result_ident) {
                                    Some(datum) => datum,
                                    None => unsafe { ::pgrx::fcinfo::pg_return_null(#fcinfo_ident) },
                                }
                        }
                    } else {
                        // returning Result<T>
                        quote_spanned! {
                            self.func.sig.output.span() =>
                                ::pgrx::datum::IntoDatum::into_datum(#result_ident).unwrap_or_else(|| panic!("returned Datum was NULL"))
                        }
                    }
                } else if retval_ty.resolved_ty == syn::parse_quote!(pg_sys::Datum)
                    || retval_ty.resolved_ty == syn::parse_quote!(pgrx::pg_sys::Datum)
                    || retval_ty.resolved_ty == syn::parse_quote!(::pgrx::pg_sys::Datum)
                {
                    quote_spanned! { self.func.sig.output.span() =>
                       #result_ident
                    }
                } else if retval_ty.optional.is_some() {
                    quote_spanned! { self.func.sig.output.span() =>
                        match #result_ident {
                            Some(result) => {
                                ::pgrx::datum::IntoDatum::into_datum(result).unwrap_or_else(|| panic!("returned Option<T> was NULL"))
                            },
                            None => unsafe { ::pgrx::fcinfo::pg_return_null(#fcinfo_ident) }
                        }
                    }
                } else {
                    quote_spanned! { self.func.sig.output.span() =>
                        ::pgrx::datum::IntoDatum::into_datum(#result_ident).unwrap_or_else(|| panic!("returned Datum was NULL"))
                    }
                };

                quote_spanned! { self.func.sig.span() =>
                    #[no_mangle]
                    #[doc(hidden)]
                    #[::pgrx::pgrx_macros::pg_guard]
                    pub unsafe extern "C" fn #func_name_wrapper #func_generics(#fcinfo_ident: ::pgrx::pg_sys::FunctionCallInfo) -> ::pgrx::pg_sys::Datum {
                        #(
                            #arg_fetches
                        )*

                        #[allow(unused_unsafe)] // unwrapped fn might be unsafe
                        let #result_ident = unsafe { #func_name(#(#arg_pats),*) };

                        #retval_transform
                    }
                }
            }
            Returning::SetOf { ty: _retval_ty, optional, result } => {
                let result_handler = if *optional && !*result {
                    // don't need unsafe annotations because of the larger unsafe block coming up
                    quote_spanned! { self.func.sig.span() =>
                        #func_name(#(#arg_pats),*)
                    }
                } else if *result {
                    if *optional {
                        quote_spanned! { self.func.sig.span() =>
                            use ::pgrx::pg_sys::panic::ErrorReportable;
                            #func_name(#(#arg_pats),*).report()
                        }
                    } else {
                        quote_spanned! { self.func.sig.span() =>
                            use ::pgrx::pg_sys::panic::ErrorReportable;
                            Some(#func_name(#(#arg_pats),*).report())
                        }
                    }
                } else {
                    quote_spanned! { self.func.sig.span() =>
                        Some(#func_name(#(#arg_pats),*))
                    }
                };

                quote_spanned! { self.func.sig.span() =>
                    #[no_mangle]
                    #[doc(hidden)]
                    #[::pgrx::pgrx_macros::pg_guard]
                    pub unsafe extern "C" fn #func_name_wrapper #func_generics(#fcinfo_ident: ::pgrx::pg_sys::FunctionCallInfo) -> ::pgrx::pg_sys::Datum {
                        #[allow(unused_unsafe)]
                        unsafe {
                            // SAFETY: the caller has asserted that `fcinfo` is a valid FunctionCallInfo pointer, allocated by Postgres
                            // with all its fields properly setup.  Unless the user is calling this wrapper function directly, this
                            // will always be the case
                            ::pgrx::iter::SetOfIterator::srf_next(#fcinfo_ident, || {
                                #( #arg_fetches )*
                                #result_handler
                            })
                        }
                    }
                }
            }
            Returning::Iterated { tys: retval_tys, optional, result } => {
                let result_handler = if *optional && *result {
                    // don't need unsafe annotations because of the larger unsafe block coming up
                    quote_spanned! { self.func.sig.span() =>
                            use ::pgrx::pg_sys::panic::ErrorReportable;
                            let unwrapped = #func_name(#(#arg_pats),*).report();
                            unwrapped
                    }
                } else if *optional {
                    // don't need unsafe annotations because of the larger unsafe block coming up
                    quote_spanned! { self.func.sig.span() =>
                        #func_name(#(#arg_pats),*)
                    }
                } else if *result {
                    quote_spanned! { self.func.sig.span() =>
                        {
                            use ::pgrx::pg_sys::panic::ErrorReportable;
                            Some(#func_name(#(#arg_pats),*).report())
                        }
                    }
                } else {
                    quote_spanned! { self.func.sig.span() =>
                        Some(#func_name(#(#arg_pats),*))
                    }
                };

                if retval_tys.len() == 1 {
                    // Postgres considers functions returning a 1-field table (`RETURNS TABLE (T)`) to be
                    // a function that `RETRUNS SETOF T`.  So we write a different wrapper implementation
                    // that transparently transforms the `TableIterator` returned by the user into a `SetOfIterator`
                    quote_spanned! { self.func.sig.span() =>
                        #[no_mangle]
                        #[doc(hidden)]
                        #[::pgrx::pgrx_macros::pg_guard]
                        pub unsafe extern "C" fn #func_name_wrapper #func_generics(#fcinfo_ident: ::pgrx::pg_sys::FunctionCallInfo) -> ::pgrx::pg_sys::Datum {
                            #[allow(unused_unsafe)]
                            unsafe {
                                // SAFETY: the caller has asserted that `fcinfo` is a valid FunctionCallInfo pointer, allocated by Postgres
                                // with all its fields properly setup.  Unless the user is calling this wrapper function directly, this
                                // will always be the case
                                ::pgrx::iter::SetOfIterator::srf_next(#fcinfo_ident, || {
                                    #( #arg_fetches )*
                                    let table_iterator = { #result_handler };

                                    // we need to convert the 1-field `TableIterator` provided by the user
                                    // into a SetOfIterator in order to properly handle the case of `RETURNS TABLE (T)`,
                                    // which is a table that returns only 1 field.
                                    table_iterator.map(|i| ::pgrx::iter::SetOfIterator::new(i.into_iter().map(|(v,)| v)))
                                })
                            }
                        }
                    }
                } else {
                    quote_spanned! { self.func.sig.span() =>
                        #[no_mangle]
                        #[doc(hidden)]
                        #[::pgrx::pgrx_macros::pg_guard]
                        pub unsafe extern "C" fn #func_name_wrapper #func_generics(#fcinfo_ident: ::pgrx::pg_sys::FunctionCallInfo) -> ::pgrx::pg_sys::Datum {
                            #[allow(unused_unsafe)]
                            unsafe {
                                // SAFETY: the caller has asserted that `fcinfo` is a valid FunctionCallInfo pointer, allocated by Postgres
                                // with all its fields properly setup.  Unless the user is calling this wrapper function directly, this
                                // will always be the case
                                ::pgrx::iter::TableIterator::srf_next(#fcinfo_ident, || {
                                    #( #arg_fetches )*
                                    #result_handler
                                })
                            }
                        }
                    }
                }
            }
        }
    }
}

impl ToEntityGraphTokens for PgExtern {
    fn to_entity_graph_tokens(&self) -> TokenStream2 {
        self.entity_tokens()
    }
}

impl ToRustCodeTokens for PgExtern {
    fn to_rust_code_tokens(&self) -> TokenStream2 {
        let original_func = &self.func;
        let wrapper_func = self.wrapper_func();
        let finfo_tokens = self.finfo_tokens();

        quote_spanned! { self.func.sig.span() =>
            #original_func
            #wrapper_func
            #finfo_tokens
        }
    }
}

impl Parse for CodeEnrichment<PgExtern> {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let mut attrs = Vec::new();

        let parser = Punctuated::<Attribute, Token![,]>::parse_terminated;
        let punctuated_attrs = input.call(parser).ok().unwrap_or_default();
        for pair in punctuated_attrs.into_pairs() {
            attrs.push(pair.into_value())
        }
        PgExtern::new(quote! {#(#attrs)*}, input.parse()?)
    }
}
