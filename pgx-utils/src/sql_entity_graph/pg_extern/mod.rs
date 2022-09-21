/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

/*!

`#[pg_extern]` related macro expansion for Rust to SQL translation

> Like all of the [`sql_entity_graph`][crate::sql_entity_graph] APIs, this is considered **internal**
to the `pgx` framework and very subject to change between versions. While you may use this, please do it with caution.

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

use crate::{sql_entity_graph::ToSqlConfig, staticize_lifetimes};
use attribute::Attribute;
use operator::{PgxOperatorAttributeWithIdent, PgxOperatorOpName};
use search_path::SearchPathList;

use eyre::WrapErr;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{quote, quote_spanned, ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream, Parser},
    punctuated::Punctuated,
    spanned::Spanned,
    Meta, Token,
};

use self::returning::Returning;

use super::UsedType;

/// A parsed `#[pg_extern]` item.
///
/// It should be used with [`syn::parse::Parse`] functions.
///
/// Using [`quote::ToTokens`] will output the declaration for a [`PgExternEntity`][crate::sql_entity_graph::PgExternEntity].
///
/// ```rust
/// use syn::{Macro, parse::Parse, parse_quote, parse};
/// use quote::{quote, ToTokens};
/// use pgx_utils::sql_entity_graph::PgExtern;
///
/// # fn main() -> eyre::Result<()> {
/// let parsed: PgExtern = parse_quote! {
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
}

impl PgExtern {
    pub fn new(attr: TokenStream2, item: TokenStream2) -> Result<Self, syn::Error> {
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
            let updated_value = value.replace(
                "@FUNCTION_NAME@",
                &*(func.sig.ident.to_string() + "_wrapper"),
            ) + "\n";
            *content = syn::LitStr::new(&updated_value, Span::call_site());
        }

        if !to_sql_config.overrides_default() {
            crate::ident_is_acceptable_to_postgres(&func.sig.ident)?;
        }

        Ok(Self {
            attrs,
            func,
            to_sql_config,
        })
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
                        if !in_commented_sql_block && inner.value().trim() == "```pgxsql" {
                            in_commented_sql_block = true;
                        } else if in_commented_sql_block && inner.value().trim() == "```" {
                            in_commented_sql_block = false;
                        } else if in_commented_sql_block {
                            let sql = retval.get_or_insert_with(String::default);
                            let line = inner.value().trim_start().replace(
                                "@FUNCTION_NAME@",
                                &*(self.func.sig.ident.to_string() + "_wrapper"),
                            ) + "\n";
                            sql.push_str(&*line);
                        }
                    }
                }
            }
        }
        retval.map(|s| syn::LitStr::new(s.as_ref(), span.unwrap()))
    }

    fn operator(&self) -> Option<PgOperator> {
        let mut skel = Option::<PgOperator>::default();
        for attr in &self.func.attrs {
            let last_segment = attr.path.segments.last().unwrap();
            match last_segment.ident.to_string().as_str() {
                "opname" => {
                    let attr: PgxOperatorOpName = syn::parse2(attr.tokens.clone())
                        .expect(&format!("Unable to parse {:?}", &attr.tokens));
                    skel.get_or_insert_with(Default::default)
                        .opname
                        .get_or_insert(attr);
                }
                "commutator" => {
                    let attr: PgxOperatorAttributeWithIdent = syn::parse2(attr.tokens.clone())
                        .expect(&format!("Unable to parse {:?}", &attr.tokens));
                    skel.get_or_insert_with(Default::default)
                        .commutator
                        .get_or_insert(attr);
                }
                "negator" => {
                    let attr: PgxOperatorAttributeWithIdent = syn::parse2(attr.tokens.clone())
                        .expect(&format!("Unable to parse {:?}", &attr.tokens));
                    skel.get_or_insert_with(Default::default)
                        .negator
                        .get_or_insert(attr);
                }
                "join" => {
                    let attr: PgxOperatorAttributeWithIdent = syn::parse2(attr.tokens.clone())
                        .expect(&format!("Unable to parse {:?}", &attr.tokens));
                    skel.get_or_insert_with(Default::default)
                        .join
                        .get_or_insert(attr);
                }
                "restrict" => {
                    let attr: PgxOperatorAttributeWithIdent = syn::parse2(attr.tokens.clone())
                        .expect(&format!("Unable to parse {:?}", &attr.tokens));
                    skel.get_or_insert_with(Default::default)
                        .restrict
                        .get_or_insert(attr);
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
        skel
    }

    fn search_path(&self) -> Option<SearchPathList> {
        self.func
            .attrs
            .iter()
            .find(|f| {
                f.path
                    .segments
                    .first()
                    .map(|f| f.ident == Ident::new("search_path", Span::call_site()))
                    .unwrap_or_default()
            })
            .and_then(|attr| Some(attr.parse_args::<SearchPathList>().unwrap()))
    }

    fn inputs(&self) -> eyre::Result<Vec<PgExternArgument>> {
        let mut args = Vec::default();
        for input in &self.func.sig.inputs {
            let arg = PgExternArgument::build(input.clone())
                .wrap_err_with(|| format!("Could not map {:?}", input))?;
            args.push(arg);
        }
        Ok(args)
    }

    fn returns(&self) -> Result<Returning, syn::Error> {
        Returning::try_from(&self.func.sig.output)
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
        let search_path = self.search_path().into_iter();
        let inputs = self.inputs().unwrap();
        let inputs_iter = inputs.iter().map(|v| v.entity_tokens());

        let input_types = self.func.sig.inputs.iter().filter_map(|v| match v {
            syn::FnArg::Receiver(_) => None,
            syn::FnArg::Typed(pat_ty) => {
                let static_ty = pat_ty.ty.clone();
                let mut static_ty = UsedType::new(*static_ty).unwrap().resolved_ty;
                staticize_lifetimes(&mut static_ty);
                Some(static_ty)
            }
        });

        let returns = match self.returns() {
            Ok(returns) => returns,
            Err(e) => {
                let msg = e.to_string();
                return quote! {
                    std::compile_error!(#msg);
                };
            }
        };

        let return_type = match &self.func.sig.output {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(arrow, ty) => {
                let mut static_ty = ty.clone();
                staticize_lifetimes(&mut static_ty);
                Some(syn::ReturnType::Type(*arrow, static_ty))
            }
        };

        let operator = self.operator().into_iter();
        let to_sql_config = match self.overridden() {
            None => self.to_sql_config.clone(),
            Some(content) => {
                let mut config = self.to_sql_config.clone();
                config.content = Some(content);
                config
            }
        };

        let sql_graph_entity_fn_name =
            syn::Ident::new(&format!("__pgx_internals_fn_{}", ident), Span::call_site());
        quote_spanned! { self.func.sig.span() =>
            #[no_mangle]
            #[doc(hidden)]
            pub extern "Rust" fn  #sql_graph_entity_fn_name() -> ::pgx::utils::sql_entity_graph::SqlGraphEntity {
                extern crate alloc;
                #[allow(unused_imports)]
                use alloc::{vec, vec::Vec};
                type FunctionPointer = #unsafety fn(#( #input_types ),*) #return_type;
                let metadata: FunctionPointer = #ident;
                let submission = ::pgx::utils::sql_entity_graph::PgExternEntity {
                    name: #name,
                    unaliased_name: stringify!(#ident),
                    module_path: core::module_path!(),
                    full_path: concat!(core::module_path!(), "::", stringify!(#ident)),
                    metadata: pgx::utils::sql_entity_graph::metadata::FunctionMetadata::entity(&metadata),
                    fn_args: vec![#(#inputs_iter),*],
                    fn_return: #returns,
                    schema: None #( .unwrap_or(Some(#schema_iter)) )*,
                    file: file!(),
                    line: line!(),
                    extern_attrs: vec![#extern_attrs],
                    search_path: None #( .unwrap_or(Some(vec![#search_path])) )*,
                    operator: None #( .unwrap_or(Some(#operator)) )*,
                    to_sql_config: #to_sql_config,
                };
                ::pgx::utils::sql_entity_graph::SqlGraphEntity::Function(submission)
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
            pub extern "C" fn #finfo_name() -> &'static pg_sys::Pg_finfo_record {
                const V1_API: pg_sys::Pg_finfo_record = pg_sys::Pg_finfo_record { api_version: 1 };
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

        let args = self.inputs().unwrap();
        let arg_pats = args
            .iter()
            .map(|v| syn::Ident::new(&format!("{}_", &v.pat), self.func.sig.span()))
            .collect::<Vec<_>>();
        let arg_fetches = args.iter().enumerate().map(|(idx, arg)| {
            let pat = &arg_pats[idx];
            let resolved_ty = &arg.used_ty.resolved_ty;
            if arg.used_ty.resolved_ty.to_token_stream().to_string() == quote!(pgx::pg_sys::FunctionCallInfo).to_token_stream().to_string()
            || arg.used_ty.resolved_ty.to_token_stream().to_string() == quote!(pg_sys::FunctionCallInfo).to_token_stream().to_string() {
                quote_spanned! {pat.span()=>
                    let #pat = #fcinfo_ident;
                }
            } else if arg.used_ty.resolved_ty.to_token_stream().to_string() == quote!(()).to_token_stream().to_string() {
                quote_spanned! {pat.span()=>
                    debug_assert!(pgx::pg_getarg::<()>(#fcinfo_ident, #idx).is_none(), "A `()` argument should always recieve `NULL`");
                    let #pat = ();
                }
            } else {
                match (is_raw, &arg.used_ty.optional) {
                    (true, None) | (true, Some(_)) => quote_spanned! { pat.span() =>
                        let #pat = pgx::pg_getarg_datum_raw(#fcinfo_ident, #idx) as #resolved_ty;
                    },
                    (false, None) => quote_spanned! { pat.span() =>
                        let #pat = pgx::pg_getarg::<#resolved_ty>(#fcinfo_ident, #idx).unwrap_or_else(|| panic!("{} is null", stringify!{#pat}));
                    },
                    (false, Some(inner)) => quote_spanned! { pat.span() =>
                        let #pat = pgx::pg_getarg::<#inner>(#fcinfo_ident, #idx);
                    },
                }
            }
        });

        match self.returns().unwrap() {
            Returning::None => quote_spanned! { self.func.sig.span() =>
                    #[no_mangle]
                    #[doc(hidden)]
                    #[pg_guard]
                    pub unsafe extern "C" fn #func_name_wrapper #func_generics(#fcinfo_ident: ::pgx::pg_sys::FunctionCallInfo) {
                        #(
                            #arg_fetches
                        )*

                        #func_name(#(#arg_pats),*)
                    }
            },
            Returning::Type(retval_ty) => {
                let result_ident = syn::Ident::new("result", self.func.sig.span());
                let retval_transform = if retval_ty.resolved_ty == syn::parse_quote!(()) {
                    quote_spanned! { self.func.sig.output.span() =>
                       pgx::pg_return_void()
                    }
                } else if retval_ty.resolved_ty == syn::parse_quote!(pg_sys::Datum)
                    || retval_ty.resolved_ty == syn::parse_quote!(pgx::pg_sys::Datum)
                {
                    quote_spanned! { self.func.sig.output.span() =>
                       #result_ident
                    }
                } else if retval_ty.optional.is_some() {
                    quote_spanned! { self.func.sig.output.span() =>
                        match #result_ident {
                            Some(result) => {
                                pgx::datum::IntoDatum::into_datum(result).unwrap_or_else(|| panic!("returned Option<T> was NULL"))
                            },
                            None => pgx::pg_return_null(#fcinfo_ident)
                        }
                    }
                } else {
                    quote_spanned! { self.func.sig.output.span() =>
                        pgx::datum::IntoDatum::into_datum(#result_ident).unwrap_or_else(|| panic!("returned Datum was NULL"))
                    }
                };

                quote_spanned! { self.func.sig.span() =>
                    #[no_mangle]
                    #[doc(hidden)]
                    #[pg_guard]
                    pub unsafe extern "C" fn #func_name_wrapper #func_generics(#fcinfo_ident: ::pgx::pg_sys::FunctionCallInfo) -> ::pgx::pg_sys::Datum {
                        #(
                            #arg_fetches
                        )*

                        let #result_ident = #func_name(#(#arg_pats),*);

                        #retval_transform
                    }
                }
            }
            Returning::SetOf {
                ty: retval_ty,
                optional,
            } => {
                let result_ident = syn::Ident::new("result", self.func.sig.span());
                let funcctx_ident = syn::Ident::new("funcctx", self.func.sig.span());
                let retval_ty_resolved = retval_ty.original_ty;
                let result_handler = if optional {
                    quote_spanned! { self.func.sig.span() =>
                        let #result_ident = match pgx::PgMemoryContexts::For(funcctx.multi_call_memory_ctx).switch_to(|_| { #func_name(#(#arg_pats),*) }) {
                            Some(result) => result,
                            None => {
                                pgx::srf_return_done(#fcinfo_ident, &mut funcctx);
                                return pgx::pg_return_null(#fcinfo_ident)
                            }
                        };
                    }
                } else {
                    quote_spanned! { self.func.sig.span() =>
                        let #result_ident = pgx::PgMemoryContexts::For(#funcctx_ident.multi_call_memory_ctx).switch_to(|_| { #func_name(#(#arg_pats),*) });
                    }
                };

                quote_spanned! { self.func.sig.span() =>
                    #[no_mangle]
                    #[doc(hidden)]
                    #[pg_guard]
                    pub unsafe extern "C" fn #func_name_wrapper #func_generics(#fcinfo_ident: ::pgx::pg_sys::FunctionCallInfo) -> ::pgx::pg_sys::Datum {
                        struct IteratorHolder<'__pgx_internal_lifetime, T: std::panic::UnwindSafe + std::panic::RefUnwindSafe> {
                            iter: *mut SetOfIterator<'__pgx_internal_lifetime, T>,
                        }

                        let mut funcctx: ::pgx::PgBox<pg_sys::FuncCallContext>;
                        let mut iterator_holder: ::pgx::PgBox<IteratorHolder<#retval_ty_resolved>>;

                        if srf_is_first_call(#fcinfo_ident) {
                            funcctx = pgx::srf_first_call_init(#fcinfo_ident);
                            funcctx.user_fctx = pgx::PgMemoryContexts::For(funcctx.multi_call_memory_ctx).palloc_struct::<IteratorHolder<#retval_ty_resolved>>() as void_mut_ptr;

                            iterator_holder = pgx::PgBox::from_pg(funcctx.user_fctx as *mut IteratorHolder<#retval_ty_resolved>);

                            #( #arg_fetches )*
                            #result_handler

                            iterator_holder.iter = pgx::PgMemoryContexts::For(funcctx.multi_call_memory_ctx).leak_trivial_alloc(result);
                        }

                        funcctx = pgx::srf_per_call_setup(#fcinfo_ident);
                        iterator_holder = pgx::PgBox::from_pg(funcctx.user_fctx as *mut IteratorHolder<#retval_ty_resolved>);

                        let mut iter = Box::from_raw(iterator_holder.iter);
                        match iter.next() {
                            Some(result) => {
                                // we need to leak the boxed iterator so that it's not freed by rust and we can
                                // continue to use it
                                Box::leak(iter);

                                pgx::srf_return_next(#fcinfo_ident, &mut funcctx);
                                match pgx::datum::IntoDatum::into_datum(result) {
                                    Some(datum) => datum,
                                    None => pgx::pg_return_null(#fcinfo_ident),
                                }
                            },
                            None => {
                                // leak the iterator here too, even tho we're done, b/c our MemoryContextCallback
                                // function is going to properly drop it for us
                                Box::leak(iter);

                                pgx::srf_return_done(#fcinfo_ident, &mut funcctx);
                                pgx::pg_return_null(#fcinfo_ident)
                            },
                        }
                    }
                }
            }
            Returning::Iterated {
                tys: retval_tys,
                optional,
            } => {
                let result_ident = syn::Ident::new("result", self.func.sig.span());
                let funcctx_ident = syn::Ident::new("funcctx", self.func.sig.span());
                let retval_tys_resolved = retval_tys.iter().map(|v| &v.used_ty.resolved_ty);
                let retval_tys_tuple = quote! { (#(#retval_tys_resolved,)*) };

                let retval_tuple_indexes = (0..retval_tys.len()).map(syn::Index::from);
                let retval_tuple_len = retval_tuple_indexes.len();
                let create_heap_tuple = quote! {
                    let mut datums: [Datum; #retval_tuple_len] = [Datum::from(0); #retval_tuple_len];
                    let mut nulls: [bool; #retval_tuple_len] = [false; #retval_tuple_len];

                    #(
                        let datum = pgx::datum::IntoDatum::into_datum(result.#retval_tuple_indexes);
                        match datum {
                            Some(datum) => { datums[#retval_tuple_indexes] = datum.into(); },
                            None => { nulls[#retval_tuple_indexes] = true; }
                        }
                    )*

                    let heap_tuple = pgx::pg_sys::heap_form_tuple(#funcctx_ident.tuple_desc, datums.as_mut_ptr(), nulls.as_mut_ptr());
                };

                let result_handler = if optional {
                    quote_spanned! { self.func.sig.span() =>
                        let #result_ident = match pgx::PgMemoryContexts::For(funcctx.multi_call_memory_ctx).switch_to(|_| { #func_name(#(#arg_pats),*) }) {
                            Some(result) => result,
                            None => {
                                pgx::srf_return_done(#fcinfo_ident, &mut funcctx);
                                return pgx::pg_return_null(#fcinfo_ident)
                            }
                        };
                    }
                } else {
                    quote_spanned! { self.func.sig.span() =>
                        let #result_ident = pgx::PgMemoryContexts::For(#funcctx_ident.multi_call_memory_ctx).switch_to(|_| { #func_name(#(#arg_pats),*) });
                    }
                };

                quote_spanned! { self.func.sig.span() =>
                    #[no_mangle]
                    #[doc(hidden)]
                    #[pg_guard]
                    pub unsafe extern "C" fn #func_name_wrapper #func_generics(#fcinfo_ident: ::pgx::pg_sys::FunctionCallInfo) -> ::pgx::pg_sys::Datum {
                        struct IteratorHolder<'__pgx_internal_lifetime, T: std::panic::UnwindSafe + std::panic::RefUnwindSafe> {
                            iter: *mut ::pgx::iter::TableIterator<'__pgx_internal_lifetime, T>,
                        }

                        let mut funcctx: pgx::PgBox<pg_sys::FuncCallContext>;
                        let mut iterator_holder: pgx::PgBox<IteratorHolder<#retval_tys_tuple>>;

                        if srf_is_first_call(#fcinfo_ident) {
                            funcctx = pgx::srf_first_call_init(#fcinfo_ident);
                            funcctx.user_fctx = pgx::PgMemoryContexts::For(funcctx.multi_call_memory_ctx).palloc_struct::<IteratorHolder<#retval_tys_tuple>>() as void_mut_ptr;
                            funcctx.tuple_desc = pgx::PgMemoryContexts::For(funcctx.multi_call_memory_ctx).switch_to(|_| {
                                let mut tupdesc: *mut pgx::pg_sys::TupleDescData = std::ptr::null_mut();

                                /* Build a tuple descriptor for our result type */
                                if pgx::pg_sys::get_call_result_type(#fcinfo_ident, std::ptr::null_mut(), &mut tupdesc) != pgx::pg_sys::TypeFuncClass_TYPEFUNC_COMPOSITE {
                                    pgx::error!("return type must be a row type");
                                }

                                pgx::pg_sys::BlessTupleDesc(tupdesc)
                            });
                            iterator_holder = pgx::PgBox::from_pg(funcctx.user_fctx as *mut IteratorHolder<#retval_tys_tuple>);

                            #( #arg_fetches )*
                            #result_handler

                            iterator_holder.iter = pgx::PgMemoryContexts::For(funcctx.multi_call_memory_ctx).leak_and_drop_on_delete(result);
                        }

                        funcctx = pgx::srf_per_call_setup(#fcinfo_ident);
                        iterator_holder = pgx::PgBox::from_pg(funcctx.user_fctx as *mut IteratorHolder<#retval_tys_tuple>);

                        let mut iter = Box::from_raw(iterator_holder.iter);
                        match iter.next() {
                            Some(result) => {
                                // we need to leak the boxed iterator so that it's not freed by rust and we can
                                // continue to use it
                                Box::leak(iter);

                                #create_heap_tuple

                                let datum = pgx::heap_tuple_get_datum(heap_tuple);
                                pgx::srf_return_next(#fcinfo_ident, &mut funcctx);
                                pgx::pg_sys::Datum::from(datum)
                            },
                            None => {
                                // leak the iterator here too, even tho we're done, b/c our MemoryContextCallback
                                // function is going to properly drop it for us
                                Box::leak(iter);

                                pgx::srf_return_done(#fcinfo_ident, &mut funcctx);
                                pgx::pg_return_null(#fcinfo_ident)
                            },
                        }
                    }
                }
            }
            // /// We don't actually do this since triggers have their own cool kids zone
            // Returning::Trigger => {
            //     quote_spanned! { self.func.sig.span() =>
            //         #[no_mangle]
            //         #[doc(hidden)]
            //         #[pg_guard]
            //         pub unsafe extern "C" fn #func_name_wrapper #func_generics(#fcinfo_ident: ::pgx::pg_sys::FunctionCallInfo) -> ::pgx::pg_sys::Datum {
            //             let maybe_pg_trigger = unsafe { ::pgx::trigger_support::PgTrigger::from_fcinfo(#fcinfo_ident) };
            //             let pg_trigger = maybe_pg_trigger.expect("PgTrigger::from_fcinfo failed");
            //             let trigger_fn_result: Result<
            //                 ::pgx::heap_tuple::PgHeapTuple<'_, _>,
            //                 _,
            //             > = #func_name(&pg_trigger);

            //             let trigger_retval = trigger_fn_result.expect("Trigger function panic");
            //             match trigger_retval.into_trigger_datum() {
            //                 None => ::pgx::pg_return_null(fcinfo),
            //                 Some(datum) => datum,
            //             }
            //         }
            //     }
            // },
        }
    }
}

impl ToTokens for PgExtern {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let original_func = &self.func;
        let wrapper_func = self.wrapper_func();
        let entity_func = self.entity_tokens();
        let finfo_tokens = self.finfo_tokens();

        let expansion = quote_spanned! { self.func.sig.span() =>
            #original_func

            #wrapper_func

            #entity_func

            #finfo_tokens
        };
        tokens.append_all(expansion);
    }
}

impl Parse for PgExtern {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let mut attrs = Vec::new();
        let mut to_sql_config: Option<ToSqlConfig> = None;

        let parser = Punctuated::<Attribute, Token![,]>::parse_terminated;
        let punctuated_attrs = input.call(parser).ok().unwrap_or_default();
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

        let to_sql_config = to_sql_config.unwrap_or_default();

        let func: syn::ItemFn = input.parse()?;

        if !to_sql_config.overrides_default() {
            crate::ident_is_acceptable_to_postgres(&func.sig.ident)?;
        }

        Ok(Self {
            attrs,
            func,
            to_sql_config,
        })
    }
}
