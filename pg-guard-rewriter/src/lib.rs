extern crate proc_macro;

use proc_macro2::Ident;
use quote::{quote, quote_spanned};
use std::ops::Deref;
use std::str::FromStr;
use syn::export::TokenStream2;
use syn::spanned::Spanned;
use syn::{
    FnArg, ForeignItem, ForeignItemFn, ItemFn, ItemForeignMod, ItemStruct, Pat, ReturnType,
    Signature, Type, Visibility,
};

pub enum RewriteMode {
    ApplyPgGuardMacro,
    RewriteFunctionWithWrapper,
}

pub struct PgGuardRewriter(RewriteMode);

impl PgGuardRewriter {
    pub fn new(mode: RewriteMode) -> Self {
        PgGuardRewriter(mode)
    }

    pub fn item_struct(&self, item_struct: ItemStruct) -> proc_macro2::TokenStream {
        let mut stream = TokenStream2::new();
        stream.extend(quote! {
            #[derive(DatumCompatible)]
            #item_struct
        });

        stream
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

    pub fn item_fn(&self, mut func: ItemFn, rewrite_args: bool) -> proc_macro2::TokenStream {
        // remember the original visibility and signature classifications as we want
        // to use those for the outer function
        let vis = func.vis.clone();

        // but for the inner function (the one we're wrapping) we don't need any kind of
        // abi classification
        func.sig.abi = None;

        // nor do we need a visibility beyond "private"
        func.vis = Visibility::Inherited;

        let arg_list = PgGuardRewriter::build_arg_list(&func.sig);
        let func_name = PgGuardRewriter::build_func_name(&func.sig);
        let func_span = func.span().clone();
        let rewritten_args = if rewrite_args {
            self.rewrite_args(func.clone())
        } else {
            quote_spanned! {func_span=>#func}
        };
        let rewritten_return_type = if rewrite_args {
            self.rewrite_return_type(func.clone())
        } else {
            quote! { result }
        };

        quote_spanned! {func_span=>
            #vis fn #func_name(fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
                #func

                let result = pg_guard::guard( || {
                    #rewritten_args

                    #func_name(#arg_list)
                } );

                #rewritten_return_type
            }
        }
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

        quote! {
            pub unsafe fn #func_name ( #arg_list_with_types ) #return_type {
                extern "C" {
                    pub fn #func_name( #arg_list_with_types ) #return_type ;
                }

                pg_guard::guard(|| unsafe { #func_name( #arg_list) })
            }
        }
    }

    pub fn build_func_name(sig: &Signature) -> Ident {
        sig.ident.clone()
    }

    pub fn build_arg_list(sig: &Signature) -> proc_macro2::TokenStream {
        let mut arg_list = proc_macro2::TokenStream::new();

        for arg in &sig.inputs {
            match arg {
                FnArg::Typed(ty) => {
                    if let Pat::Ident(ident) = ty.pat.deref() {
                        arg_list.extend(quote! { #ident, });
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

    pub fn rewrite_args(&self, func: ItemFn) -> proc_macro2::TokenStream {
        let fsr = FunctionSignatureRewriter::new(func);
        let args = fsr.args();

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
                    pg_bridge::pg_return_void()
                });
            }
            ReturnType::Type(_, type_) => {
                stream.extend(quote! {
                    pg_bridge::PgDatum::<#type_>::from(result).into()
                });
            }
        }

        stream
    }

    fn args(&self) -> proc_macro2::TokenStream {
        eprintln!("len={}", self.func.sig.inputs.len());
        if self.func.sig.inputs.len() == 1 && self.return_type_is_datum() {
            match self.func.sig.inputs.first().unwrap() {
                FnArg::Typed(ty) => {
                    if FunctionSignatureRewriter::type_matches(&ty.ty, "pg_sys :: FunctionCallInfo")
                    {
                        return proc_macro2::TokenStream::new();
                    }
                }
                _ => {}
            }
        }

        let mut stream = proc_macro2::TokenStream::new();
        let mut i = 0usize;
        for arg in &self.func.sig.inputs {
            match arg {
                FnArg::Receiver(_) => panic!("Functions that take self are not supported"),
                FnArg::Typed(ty) => match ty.pat.deref() {
                    Pat::Ident(ident) => {
                        let name = &ident.ident;
                        let type_ = &ty.ty;
                        stream.extend(quote! {
                            let #name: #type_ = pg_bridge::pg_getarg::<#type_>(fcinfo, #i).try_into().expect(&format!("argument '{}'", stringify! { #name }));
                        });

                        i += 1;
                    }
                    _ => panic!("Unrecognized function arg type"),
                },
            }
        }

        stream
    }

    fn return_type_is_datum(&self) -> bool {
        match &self.func.sig.output {
            ReturnType::Default => false,
            ReturnType::Type(_, ty) => {
                FunctionSignatureRewriter::type_matches(ty, "pg_sys :: Datum")
            }
        }
    }

    fn type_matches(ty: &Box<Type>, pattern: &str) -> bool {
        match ty.deref() {
            Type::Path(path) => {
                let path = format!("{}", quote! {#path});
                eprintln!("path for {} = {}", pattern, path);
                path.ends_with(pattern)
            }
            _ => false,
        }
    }
}
