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
use syn::spanned::Spanned;
use syn::{
    FnArg, ForeignItem, ForeignItemFn, ItemFn, ItemForeignMod, Pat, ReturnType, Signature,
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
            stream.extend(self.foreign_item(item, &block.abi));
        }

        stream
    }

    pub fn item_fn_without_rewrite(
        &self,
        mut func: ItemFn,
    ) -> eyre::Result<proc_macro2::TokenStream> {
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

        func.sig.ident =
            Ident::new(&format!("{}_inner", func.sig.ident.to_string()), func.sig.ident.span());

        let arg_list = PgGuardRewriter::build_arg_list(&sig, false)?;
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
            quote! {
                #[no_mangle]
            }
        };

        Ok(quote_spanned! {func.span()=>
            #prolog
            #[doc(hidden)]
            #vis #sig {
                #[allow(non_snake_case)]
                #func
                pg_sys::guard::guard( #[allow(unused_unsafe)] || unsafe { #func_name(#arg_list) } )
            }
        })
    }

    pub fn foreign_item(
        &self,
        item: ForeignItem,
        abi: &syn::Abi,
    ) -> eyre::Result<proc_macro2::TokenStream> {
        match item {
            ForeignItem::Fn(func) => {
                if func.sig.variadic.is_some() {
                    return Ok(quote! { #abi { #func } });
                }

                self.foreign_item_fn(&func, abi)
            }
            _ => Ok(quote! { #abi { #item } }),
        }
    }

    pub fn foreign_item_fn(
        &self,
        func: &ForeignItemFn,
        abi: &syn::Abi,
    ) -> eyre::Result<proc_macro2::TokenStream> {
        let func_name = PgGuardRewriter::build_func_name(&func.sig);
        let arg_list = PgGuardRewriter::rename_arg_list(&func.sig)?;
        let arg_list_with_types = PgGuardRewriter::rename_arg_list_with_types(&func.sig)?;
        let return_type = PgGuardRewriter::get_return_type(&func.sig);

        Ok(quote! {
            pub unsafe fn #func_name ( #arg_list_with_types ) #return_type {
                crate::submodules::setjmp::pg_guard_ffi_boundary(move || {
                    #abi { #func }
                    #func_name(#arg_list)
                })
            }
        })
    }

    pub fn build_func_name(sig: &Signature) -> Ident {
        sig.ident.clone()
    }

    #[allow(clippy::cmp_owned)]
    pub fn build_arg_list(
        sig: &Signature,
        suffix_arg_name: bool,
    ) -> eyre::Result<proc_macro2::TokenStream> {
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
                        eyre::bail!(
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

        Ok(arg_list)
    }

    pub fn rename_arg_list(sig: &Signature) -> eyre::Result<proc_macro2::TokenStream> {
        let mut arg_list = proc_macro2::TokenStream::new();

        for arg in &sig.inputs {
            match arg {
                FnArg::Typed(ty) => {
                    if let Pat::Ident(ident) = ty.pat.deref() {
                        // prefix argument name with "arg_""
                        let name = Ident::new(&format!("arg_{}", ident.ident), ident.ident.span());
                        arg_list.extend(quote! { #name, });
                    } else {
                        eyre::bail!(
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

        Ok(arg_list)
    }

    pub fn rename_arg_list_with_types(sig: &Signature) -> eyre::Result<proc_macro2::TokenStream> {
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
                        eyre::bail!(
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

        Ok(arg_list)
    }

    pub fn get_return_type(sig: &Signature) -> ReturnType {
        sig.output.clone()
    }
}
