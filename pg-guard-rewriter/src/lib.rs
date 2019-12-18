use quote::quote;
use std::ops::Deref;
use std::str::FromStr;
use syn::export::{ToTokens, TokenStream2};
use syn::{FnArg, ForeignItem, ForeignItemFn, ItemFn, ItemForeignMod, Pat, Signature, Visibility};

pub enum RewriteMode {
    ApplyPgGuardMacro,
    RewriteFunctionWithWrapper,
}

pub struct PgGuardRewriter(RewriteMode);

impl PgGuardRewriter {
    pub fn new(mode: RewriteMode) -> Self {
        PgGuardRewriter(mode)
    }

    pub fn extern_block(&self, block: ItemForeignMod) -> proc_macro2::TokenStream {
        let mut stream = TokenStream2::new();

        match self.0 {
            RewriteMode::ApplyPgGuardMacro => {
                stream.extend(quote! {
                    #[pg_guard::pg_guard]
                    #block
                });
            }
            RewriteMode::RewriteFunctionWithWrapper => {
                for item in block.items.into_iter() {
                    stream.extend(self.foreign_item(item));
                }
            }
        }

        stream
    }

    pub fn item_fn(&self, func: ItemFn) -> proc_macro2::TokenStream {
        let mut orig_func: ItemFn = func.clone();
        let mut sig = func.sig;
        let arg_list = PgGuardRewriter::build_arg_list(&sig);
        let func_name = PgGuardRewriter::build_func_name(&sig);

        orig_func.vis = Visibility::Inherited;
        sig.abi = Some(syn::parse_str("extern \"C\"").unwrap());
        let sig = sig.into_token_stream();

        proc_macro2::TokenStream::from(quote! {
            #[no_mangle]
            pub #sig {
                #orig_func

                pg_guard::guard( || unsafe { # func_name( # arg_list) })
            }
        })
    }

    pub fn foreign_item(&self, item: ForeignItem) -> proc_macro2::TokenStream {
        match item {
            ForeignItem::Fn(func) => {
                if func.sig.variadic.is_some() {
                    return quote! { extern "C" { #func } };
                }

                self.foreign_item_fn(func)
            }
            _ => quote! { extern "C" { #item } },
        }
    }

    pub fn foreign_item_fn(&self, func: ForeignItemFn) -> proc_macro2::TokenStream {
        let func_name = PgGuardRewriter::build_func_name(&func.sig);
        let arg_list = PgGuardRewriter::rename_arg_list(&func.sig);
        let arg_list_with_types = PgGuardRewriter::rename_arg_list_with_types(&func.sig);
        let return_type = PgGuardRewriter::get_return_type(&func.sig);

        let body = quote! {
                pub unsafe fn #func_name ( #arg_list_with_types ) #return_type {
                    extern "C" {
                        pub fn #func_name( #arg_list_with_types ) #return_type ;
                    }

                    pg_guard::guard(|| unsafe { #func_name( #arg_list) })
                }
        };

        body
    }

    pub fn build_func_name(sig: &Signature) -> proc_macro2::TokenStream {
        sig.ident.to_token_stream()
    }

    pub fn build_arg_list(sig: &Signature) -> proc_macro2::TokenStream {
        let mut arg_list = proc_macro2::TokenStream::new();

        for arg in &sig.inputs {
            match arg {
                FnArg::Typed(ty) => {
                    if let Pat::Ident(ident) = ty.pat.deref() {
                        let name = ident.ident.to_token_stream();
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

    pub fn rename_arg_list(sig: &Signature) -> proc_macro2::TokenStream {
        let mut arg_list = proc_macro2::TokenStream::new();

        for arg in &sig.inputs {
            match arg {
                FnArg::Typed(ty) => {
                    if let Pat::Ident(ident) = ty.pat.deref() {
                        let name = ident.ident.to_token_stream();

                        // prefix argument name with an underscore
                        let name =
                            proc_macro2::TokenStream::from_str(&format!("arg_{}", name)).unwrap();

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
                        // prefix argument name with an underscore
                        let arg =
                            proc_macro2::TokenStream::from_str(&format!("arg_{}", quote! { #ty}))
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

    pub fn get_return_type(sig: &Signature) -> proc_macro2::TokenStream {
        let rc = &sig.output;
        quote! { #rc }
    }
}
