/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

extern crate proc_macro;

use proc_macro2::Ident;
use quote::{quote, quote_spanned};
use std::ops::Deref;
use std::str::FromStr;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    FnArg, ForeignItem, ForeignItemFn, GenericParam, ItemFn, ItemForeignMod, Pat, ReturnType,
    Signature, Token, Visibility,
};

pub struct PgGuardRewriter();

impl PgGuardRewriter {
    pub fn new() -> Self {
        PgGuardRewriter()
    }

    pub fn extern_block(&self, block: ItemForeignMod) -> proc_macro2::TokenStream {
        let mut stream = proc_macro2::TokenStream::new();

        for item in block.items.into_iter() {
            stream.extend(self.foreign_item(item, &block.abi));
        }

        stream
    }

    pub fn item_fn_without_rewrite(&self, mut func: ItemFn) -> proc_macro2::TokenStream {
        // remember the original visibility and signature classifications as we want
        // to use those for the outer function
        let input_func_name = func.sig.ident.to_string();
        let sig = func.sig.clone();
        let vis = func.vis.clone();
        let attrs = func.attrs.clone();

        let generics = func.sig.generics.clone();

        if attrs.iter().any(|attr| attr.path.is_ident("no_mangle"))
            && generics.params.iter().any(|p| match p {
                GenericParam::Type(_) => true,
                GenericParam::Lifetime(_) => false,
                GenericParam::Const(_) => true,
            })
        {
            panic!("#[pg_guard] for function with generic parameters must not be combined with #[no_mangle]");
        }

        // but for the inner function (the one we're wrapping) we don't need any kind of
        // abi classification
        func.sig.abi = None;
        func.attrs.clear();

        // nor do we need a visibility beyond "private"
        func.vis = Visibility::Inherited;

        func.sig.ident =
            Ident::new(&format!("{}_inner", func.sig.ident.to_string()), func.sig.ident.span());

        let arg_list = PgGuardRewriter::build_arg_list(&sig, false);
        let func_name = PgGuardRewriter::build_func_name(&func.sig);

        let prolog = if input_func_name == "__pgx_private_shmem_hook"
            || input_func_name == "__pgx_private_shmem_request_hook"
        {
            // we do not want "no_mangle" on these functions
            quote! {}
        } else if input_func_name == "_PG_init" || input_func_name == "_PG_fini" {
            quote! {
                #[allow(non_snake_case)]
                #[no_mangle]
            }
        } else {
            quote! {}
        };

        let body = if generics.params.is_empty() {
            quote! { #func_name(#arg_list) }
        } else {
            let ty = generics
                .params
                .into_iter()
                .filter_map(|p| match p {
                    GenericParam::Type(ty) => Some(ty.ident),
                    GenericParam::Const(c) => Some(c.ident),
                    GenericParam::Lifetime(_) => None,
                })
                .collect::<Punctuated<_, Token![,]>>();
            quote! { #func_name::<#ty>(#arg_list) }
        };

        quote_spanned! {func.span()=>
            #prolog
            #(#attrs)*
            #vis #sig {
                #[allow(non_snake_case)]
                #func

                #[allow(unused_unsafe)]
                unsafe {
                    pg_sys::panic::pgx_extern_c_guard( || #body )
                }
            }
        }
    }

    pub fn foreign_item(&self, item: ForeignItem, abi: &syn::Abi) -> proc_macro2::TokenStream {
        match item {
            ForeignItem::Fn(func) => {
                if func.sig.variadic.is_some() {
                    return quote! { #abi { #func } };
                }

                self.foreign_item_fn(&func, abi)
            }
            _ => quote! { #abi { #item } },
        }
    }

    pub fn foreign_item_fn(
        &self,
        func: &ForeignItemFn,
        abi: &syn::Abi,
    ) -> proc_macro2::TokenStream {
        let func_name = PgGuardRewriter::build_func_name(&func.sig);
        let arg_list = PgGuardRewriter::rename_arg_list(&func.sig);
        let arg_list_with_types = PgGuardRewriter::rename_arg_list_with_types(&func.sig);
        let return_type = PgGuardRewriter::get_return_type(&func.sig);

        quote! {
            #[track_caller]
            pub unsafe fn #func_name ( #arg_list_with_types ) #return_type {
                crate::ffi::pg_guard_ffi_boundary(move || {
                    #abi { #func }
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
                    } else {
                        panic!(
                            "Unknown argument pattern in `#[pg_guard]` function: `{:?}`",
                            ty.pat
                        );
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
                    } else {
                        panic!(
                            "Unknown argument pattern in `#[pg_guard]` function: `{:?}`",
                            ty.pat,
                        );
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
                    } else {
                        panic!(
                            "Unknown argument pattern in `#[pg_guard]` function: `{:?}`",
                            ty.pat,
                        );
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
}
