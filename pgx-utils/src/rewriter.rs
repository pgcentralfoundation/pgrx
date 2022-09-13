/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

extern crate proc_macro;

use proc_macro2::Ident;
use quote::{quote, quote_spanned, ToTokens};
use std::ops::Deref;
use std::str::FromStr;
use syn::spanned::Spanned;
use syn::{
    FnArg, ForeignItem, ForeignItemFn, ItemFn, ItemForeignMod, Pat, ReturnType, Signature, Type,
    Visibility,
};

pub struct PgGuardRewriter();

impl PgGuardRewriter {
    pub fn new() -> Self {
        PgGuardRewriter()
    }

    pub fn extern_block(&self, block: ItemForeignMod) -> proc_macro2::TokenStream {
        let mut stream = proc_macro2::TokenStream::new();

        for item in block.items.into_iter() {
            stream.extend(self.foreign_item(item));
        }

        stream
    }

    pub fn item_fn_without_rewrite(&self, mut func: ItemFn) -> proc_macro2::TokenStream {
        // remember the original visibility and signature classifications as we want
        // to use those for the outer function
        let input_func_name = func.sig.ident.to_string();
        let sig = func.sig.clone();
        let vis = func.vis.clone();

        // but for the inner function (the one we're wrapping) we don't need any kind of
        // abi classification
        func.sig.abi = None;

        // nor do we need a visibility beyond "private"
        func.vis = Visibility::Inherited;

        func.sig.ident = Ident::new(
            &format!("{}_inner", func.sig.ident.to_string()),
            func.sig.ident.span(),
        );

        let arg_list = PgGuardRewriter::build_arg_list(&sig, false);
        let func_name = PgGuardRewriter::build_func_name(&func.sig);

        let prolog = if input_func_name == "__pgx_private_shmem_hook" {
            // we do not want "no_mangle" on this function
            quote! {}
        } else if input_func_name == "_PG_init" || input_func_name == "_PG_fini" {
            quote! {
                #[allow(non_snake_case)]
                #[no_mangle]
            }
        } else {
            quote! {
                #[no_mangle]
            }
        };

        quote_spanned! {func.span()=>
            #prolog
            #[doc(hidden)]
            #vis #sig {
                #[allow(non_snake_case)]
                #func
                pg_sys::guard::guard( || #func_name(#arg_list) )
            }
        }
    }

    pub fn foreign_item(&self, item: ForeignItem) -> proc_macro2::TokenStream {
        match item {
            ForeignItem::Fn(func) => {
                if func.sig.variadic.is_some() {
                    return quote! { extern "C" { #func } };
                }

                self.foreign_item_fn(&func)
            }
            _ => quote! { extern "C" { #item } },
        }
    }

    pub fn foreign_item_fn(&self, func: &ForeignItemFn) -> proc_macro2::TokenStream {
        let func_name = PgGuardRewriter::build_func_name(&func.sig);
        let arg_list = PgGuardRewriter::rename_arg_list(&func.sig);
        let arg_list_with_types = PgGuardRewriter::rename_arg_list_with_types(&func.sig);
        let return_type = PgGuardRewriter::get_return_type(&func.sig);

        quote! {
            pub unsafe fn #func_name ( #arg_list_with_types ) #return_type {
                crate::submodules::setjmp::pg_guard_ffi_boundary(move || {
                    extern "C" {
                        fn #func_name( #arg_list_with_types ) #return_type ;
                    }
                    #func_name(#arg_list)
                })
            }
        }
    }

    pub fn build_func_name(sig: &Signature) -> Ident {
        sig.ident.clone()
    }

    #[allow(clippy::cmp_owned)]
    pub fn build_arg_list(sig: &Signature, suffix_arg_name: bool) -> proc_macro2::TokenStream {
        let mut arg_list = proc_macro2::TokenStream::new();

        for arg in &sig.inputs {
            match arg {
                FnArg::Typed(ty) => {
                    if let Pat::Ident(ident) = ty.pat.deref() {
                        if suffix_arg_name && ident.ident.to_string() != "fcinfo" {
                            let ident = Ident::new(&format!("{}_", ident.ident), ident.span());
                            arg_list.extend(quote! { #ident, });
                        } else {
                            arg_list.extend(quote! { #ident, });
                        }
                    }
                }
                FnArg::Receiver(_) => panic!(
                    "#[pg_guard] doesn't support external functions with 'self' as the argument"
                ),
            }
        }

        arg_list
    }

    pub fn rename_arg_list(sig: &Signature) -> proc_macro2::TokenStream {
        let mut arg_list = proc_macro2::TokenStream::new();

        for arg in &sig.inputs {
            match arg {
                FnArg::Typed(ty) => {
                    if let Pat::Ident(ident) = ty.pat.deref() {
                        // prefix argument name with "arg_""
                        let name = Ident::new(&format!("arg_{}", ident.ident), ident.ident.span());
                        arg_list.extend(quote! { #name, });
                    }
                }
                FnArg::Receiver(_) => panic!(
                    "#[pg_guard] doesn't support external functions with 'self' as the argument"
                ),
            }
        }

        arg_list
    }

    pub fn rename_arg_list_with_types(sig: &Signature) -> proc_macro2::TokenStream {
        let mut arg_list = proc_macro2::TokenStream::new();

        for arg in &sig.inputs {
            match arg {
                FnArg::Typed(ty) => {
                    if let Pat::Ident(_) = ty.pat.deref() {
                        // prefix argument name with a "arg_"
                        let arg =
                            proc_macro2::TokenStream::from_str(&format!("arg_{}", quote! {#ty}))
                                .unwrap();
                        arg_list.extend(quote! { #arg, });
                    }
                }
                FnArg::Receiver(_) => panic!(
                    "#[pg_guard] doesn't support external functions with 'self' as the argument"
                ),
            }
        }

        arg_list
    }

    pub fn get_return_type(sig: &Signature) -> ReturnType {
        sig.output.clone()
    }

    pub fn rewrite_args(&self, func: ItemFn, is_raw: bool) -> proc_macro2::TokenStream {
        let fsr = FunctionSignatureRewriter::new(func);
        let args = fsr.args(is_raw);

        quote! {
            #args
        }
    }

    pub fn rewrite_return_type(&self, func: ItemFn) -> proc_macro2::TokenStream {
        let fsr = FunctionSignatureRewriter::new(func);
        let result = fsr.return_type();

        quote! {
            #result
        }
    }
}

struct FunctionSignatureRewriter {
    func: ItemFn,
}

impl FunctionSignatureRewriter {
    fn new(func: ItemFn) -> Self {
        FunctionSignatureRewriter { func }
    }

    fn return_type(&self) -> proc_macro2::TokenStream {
        let mut stream = proc_macro2::TokenStream::new();
        match &self.func.sig.output {
            ReturnType::Default => {
                stream.extend(quote! {
                    pgx::pg_return_void()
                });
            }
            ReturnType::Type(_, type_) => {
                if type_matches(type_, "Option") {
                    stream.extend(quote! {
                        match result {
                            Some(result) => {
                                result.into_datum().unwrap_or_else(|| panic!("returned Option<T> was NULL"))
                            },
                            None => pgx::pg_return_null(fcinfo)
                        }
                    });
                } else if type_matches(type_, "pg_sys :: Datum") {
                    stream.extend(quote! {
                        result
                    });
                } else if type_matches(type_, "()") {
                    stream.extend(quote! {
                       pgx::pg_return_void()
                    });
                } else {
                    stream.extend(quote! {
                        result.into_datum().unwrap_or_else(|| panic!("returned Datum was NULL"))
                    });
                }
            }
        }

        stream
    }

    fn args(&self, is_raw: bool) -> proc_macro2::TokenStream {
        if self.func.sig.inputs.len() == 1 && self.return_type_is_datum() {
            if let FnArg::Typed(ty) = self.func.sig.inputs.first().unwrap() {
                if type_matches(&ty.ty, "pg_sys :: FunctionCallInfo")
                    || type_matches(&ty.ty, "pgx :: pg_sys :: FunctionCallInfo")
                {
                    return proc_macro2::TokenStream::new();
                }
            }
        }

        let mut stream = proc_macro2::TokenStream::new();
        let mut i = 0usize;
        let fcinfo_ident: syn::Ident = syn::parse_quote! { fcinfo };

        for arg in &self.func.sig.inputs {
            match arg {
                FnArg::Receiver(_) => panic!("Functions that take self are not supported"),
                FnArg::Typed(ty) => match ty.pat.deref() {
                    Pat::Ident(ident) => {
                        let name = Ident::new(&format!("{}_", ident.ident), ident.span());
                        let type_ = ty.ty.clone();
                        let mut type_ = crate::sql_entity_graph::UsedType::new(*type_)
                            .unwrap()
                            .resolved_ty;
                        let is_option = type_matches(&type_, "Option");

                        let ts = if is_option {
                            let option_type = extract_option_type(&type_);
                            let mut option_type = syn::parse2::<syn::Type>(option_type).unwrap();
                            crate::anonymize_lifetimes(&mut option_type);

                            quote_spanned! {ident.span()=>
                                let #name = pgx::pg_getarg::<#option_type>(#fcinfo_ident, #i);
                            }
                        } else if type_matches(&type_, "pg_sys :: FunctionCallInfo")
                            || type_matches(&type_, "pgx :: pg_sys :: FunctionCallInfo")
                        {
                            quote_spanned! {ident.span()=>
                                let #name = #fcinfo_ident;
                            }
                        } else if type_matches(&type_, "()") {
                            quote_spanned! {ident.span()=>
                                debug_assert!(pgx::pg_getarg::<()>(#fcinfo_ident, #i).is_none(), "A `()` argument should always recieve `NULL`");
                                let #name: () = ();
                            }
                        } else if is_raw {
                            quote_spanned! {ident.span()=>
                                let #name = pgx::pg_getarg_datum_raw(#fcinfo_ident, #i) as #type_;
                            }
                        } else {
                            crate::anonymize_lifetimes(&mut type_);
                            quote_spanned! {ident.span()=>
                                let #name = pgx::pg_getarg::<#type_>(#fcinfo_ident, #i).unwrap_or_else(|| panic!("{} is null", stringify!{#ident}));
                            }
                        };

                        stream.extend(ts);

                        i += 1;
                    }
                    _ => panic!(
                        "Unrecognized function arg type: {}",
                        arg.to_token_stream().to_string()
                    ),
                },
            }
        }

        stream
    }

    fn return_type_is_datum(&self) -> bool {
        match &self.func.sig.output {
            ReturnType::Default => false,
            ReturnType::Type(_, ty) => type_matches(ty, "pg_sys :: Datum"),
        }
    }
}

fn type_matches(ty: &Type, pattern: &str) -> bool {
    let type_string = format!("{}", quote! {#ty});
    type_string.starts_with(pattern)
}

fn extract_option_type(ty: &Type) -> proc_macro2::TokenStream {
    match ty {
        Type::Path(path) => {
            let mut stream = proc_macro2::TokenStream::new();
            for segment in &path.path.segments {
                let arguments = &segment.arguments;

                stream.extend(quote! { #arguments });
            }

            let string = stream.to_string();
            let string = string.trim().trim_start_matches('<');
            let string = string.trim().trim_end_matches('>');

            proc_macro2::TokenStream::from_str(string.trim()).unwrap()
        }
        _ => panic!("No type found inside Option"),
    }
}
