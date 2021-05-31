use syn::{Token, token::Token, ItemFn, FnArg};
use syn::parse::{Parse, ParseStream, ParseBuffer};
use quote::{ToTokens, quote, TokenStreamExt};
use proc_macro2::{TokenStream as TokenStream2, Span, Ident};
use proc_macro::TokenStream;
use syn::punctuated::Punctuated;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::ops::Deref;

#[derive(Debug)]
pub(crate) struct PgxExternInventory {
    attrs: PgxExternAttributes,
    func: syn::ItemFn,
}

impl Parse for PgxExternInventory {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(Self {
            attrs: input.parse()?,
            func: input.parse()?,
        })
    }
}

#[derive(Debug)]
struct PgxExternAttributes {
    attrs: Punctuated<PgxExternAttribute, Token![,]>,
}


impl Parse for PgxExternAttributes {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(Self {
            attrs: input.parse_terminated(PgxExternAttribute::parse)?,
        })
    }
}

impl ToTokens for PgxExternAttributes {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let attrs = &self.attrs;
        let quoted = quote! {
            vec![#attrs]
        };
        tokens.append_all(quoted);
    }
}

#[derive(Debug)]
enum PgxExternAttribute {
    Immutable,
    Strict,
    Stable,
    Volatile,
    Raw,
    NoGuard,
    ParallelSafe,
    ParallelUnsafe,
    ParallelRestricted,
    Error(syn::LitStr),
    Schema(syn::LitStr),
    Name(syn::LitStr),
}

impl ToTokens for PgxExternAttribute {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let quoted = match self {
            PgxExternAttribute::Immutable => quote! { pgx_utils::ExternArgs::Immutable },
            PgxExternAttribute::Strict => quote! { pgx_utils::ExternArgs::Strict },
            PgxExternAttribute::Stable => quote! { pgx_utils::ExternArgs::Stable },
            PgxExternAttribute::Volatile => quote! { pgx_utils::ExternArgs::Volatile },
            PgxExternAttribute::Raw => quote! { pgx_utils::ExternArgs::Raw },
            PgxExternAttribute::NoGuard => quote! { pgx_utils::ExternArgs::NoGuard },
            PgxExternAttribute::ParallelSafe => quote! { pgx_utils::ExternArgs::ParallelSafe },
            PgxExternAttribute::ParallelUnsafe => quote! { pgx_utils::ExternArgs::ParallelUnsafe },
            PgxExternAttribute::ParallelRestricted => quote! { pgx_utils::ExternArgs::ParallelRestricted },
            PgxExternAttribute::Error(s) => quote! { pgx_utils::ExternArgs::Error(String::from(#s)) },
            PgxExternAttribute::Schema(s) => quote! { pgx_utils::ExternArgs::Schema(String::from(#s)) },
            PgxExternAttribute::Name(s) => quote! { pgx_utils::ExternArgs::Name(String::from(#s)) },
        };
        tokens.append_all(quoted);
    }
}

impl Parse for PgxExternAttribute {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let ident: syn::Ident = input.parse()?;
        let found = match ident.to_string().as_str() {
            "immutable" => PgxExternAttribute::Immutable,
            "strict" => PgxExternAttribute::Strict,
            "stable" => PgxExternAttribute::Stable,
            "volatile" => PgxExternAttribute::Volatile,
            "raw" => PgxExternAttribute::Raw,
            "no_guard" => PgxExternAttribute::NoGuard,
            "parallel_safe" => PgxExternAttribute::ParallelSafe,
            "parallel_unsafe" => PgxExternAttribute::ParallelUnsafe,
            "parallel_restricted" => PgxExternAttribute::ParallelRestricted,
            "error" => {
                let inner;
                let _punc: syn::token::Paren = syn::parenthesized!(inner in input);
                let literal: syn::LitStr = inner.parse()?;
                PgxExternAttribute::Error(literal)
            },
            "schema" => {
                let inner;
                let _punc: syn::token::Paren = syn::parenthesized!(inner in input);
                let literal: syn::LitStr = inner.parse()?;
                PgxExternAttribute::Schema(literal)
            },
            "name" => {
                let inner;
                let _punc: syn::token::Paren = syn::parenthesized!(inner in input);
                let literal: syn::LitStr = inner.parse()?;
                PgxExternAttribute::Name(literal)
            },
            _ => return Err(syn::Error::new(Span::call_site(), "Invalid option")),
        };
        Ok(found)
    }
}

#[derive(Debug)]
struct PgxExternArgument {
    pat: syn::Pat,
    ty: syn::Type,
    default: Option<syn::Lit>,
}

impl TryFrom<syn::FnArg> for PgxExternArgument {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: FnArg) -> Result<Self, Self::Error> {
        match value {
            syn::FnArg::Typed(pat) => {
                let default = match pat.ty.as_ref() {
                    syn::Type::Macro(macro_pat) => {
                        let mac = &macro_pat.mac;
                        let archetype = mac.path.segments.last().expect("No last segment.");
                        match archetype.ident.to_string().as_str() {
                            "default" => {
                                let out: DefaultMacro = mac.parse_body()?;
                                Some(out.expr)
                            },
                            _ => None,
                        }
                    },
                    _ => None,
                };

                Ok(PgxExternArgument {
                    pat: *pat.pat.clone(),
                    ty: *pat.ty.clone(),
                    default,
                })
            },
            _ => Err(Box::new(syn::Error::new(Span::call_site(), "Unable to parse FnArg."))),
        }
    }
}

impl ToTokens for PgxExternArgument {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let pat = &self.pat;
        let ty = &self.ty;
        let default = self.default.iter();

        let quoted = quote! {
            crate::__pgx_internals::PgxExternInputs {
                pattern: stringify!(#pat),
                ty_id: TypeId::of::<#ty>(),
                ty_name: core::any::type_name::<#ty>(),
                default: None#( .unwrap_or(Some(stringify!(#default))) )*,
            }
        };
        tokens.append_all(quoted);
    }
}

#[derive(Debug)]
enum PgxExternReturn {
    None,
    Type(syn::Type),
    Iterated(Vec<(syn::Type, Option<proc_macro2::Ident>)>),
}

impl ToTokens for PgxExternReturn {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let quoted = match self {
            PgxExternReturn::None => quote! {
                crate::__pgx_internals::PgxExternReturn::None
            },
            PgxExternReturn::Type(ty) => quote! {
                crate::__pgx_internals::PgxExternReturn::Type {
                    id: TypeId::of::<#ty>(),
                    name: core::any::type_name::<#ty>(),
                }
            },
            PgxExternReturn::Iterated(items) => {
                let quoted_items = items.iter().map(|(ty, name)| {
                    let name_iter = name.iter();
                    quote! {
                        (
                            TypeId::of::<#ty>(),
                            core::any::type_name::<#ty>(),
                            None#( .unwrap_or(Some(stringify!(#name_iter))) )*,
                        )
                    }
                }).collect::<Vec<_>>();
                quote! {
                    crate::__pgx_internals::PgxExternReturn::Iterated(vec![
                        #(#quoted_items),*
                    ])
                }
            },
        };
        tokens.append_all(quoted);
    }
}


impl PgxExternInventory {
    fn extern_attrs(&self) -> &PgxExternAttributes {
        &self.attrs
    }

    fn search_path(&self) -> Option<SearchPathList> {
        self.func.attrs.iter().find(|f| {
            f.path.segments.first().map(|f| {
                f.ident == Ident::new("search_path", Span::call_site())
            }).unwrap_or_default()
        }).and_then(|attr| {
            Some(attr.parse_args::<SearchPathList>().unwrap())
        })
    }

    fn inputs(&self) -> Vec<PgxExternArgument> {
        self.func.sig.inputs.iter().flat_map(|input| {
            PgxExternArgument::try_from(input.clone()).ok()
        }).collect()
    }

    fn returns(&self) -> PgxExternReturn {
        match &self.func.sig.output {
            syn::ReturnType::Default => PgxExternReturn::None,
            syn::ReturnType::Type(_, ty) => match *ty.clone() {
                // TODO: Handle this!
                syn::Type::ImplTrait(impl_trait) => match impl_trait.bounds.first().unwrap() {
                    syn::TypeParamBound::Trait(trait_bound) => {
                        let last_path_segment = trait_bound.path.segments.last().unwrap();
                        match last_path_segment.ident.to_string().as_str() {
                            "Iterator" => match &last_path_segment.arguments {
                                syn::PathArguments::AngleBracketed(args) => {
                                    match args.args.first().unwrap() {
                                        syn::GenericArgument::Binding(binding) => match &binding.ty {
                                            syn::Type::Tuple(tuple_type) => {
                                                let returns: Vec<(syn::Type, Option<syn::Ident>)> = tuple_type.elems.iter().flat_map(|elem| {
                                                    match elem {
                                                        syn::Type::Macro(macro_pat) => {
                                                            let mac = &macro_pat.mac;
                                                            let archetype = mac.path.segments.last().unwrap();
                                                            match archetype.ident.to_string().as_str() {
                                                                "name" => {
                                                                    let out: NameMacro = mac.parse_body().expect(&*format!("{:?}", mac));
                                                                    Some((out.ty, Some(out.ident)))
                                                                },
                                                                _ => unimplemented!("Don't support anything other than name."),
                                                            }
                                                        },
                                                        ty => Some((ty.clone(), None)),
                                                    }
                                                }).collect();
                                                PgxExternReturn::Iterated(returns)
                                            }
                                            _ => unimplemented!("Only iters with tuples"),
                                        },
                                        _ => unimplemented!(),
                                    }
                                },
                                _ => unimplemented!(),
                            },
                            _ => unimplemented!(),
                        }
                    },
                    _ => PgxExternReturn::None,
                },
                _ => PgxExternReturn::Type(ty.deref().clone()),
            }
        }
    }


    pub(crate) fn new(attr: TokenStream, item: TokenStream) -> Result<Self, syn::Error> {
        let attrs = syn::parse::<PgxExternAttributes>(attr)?;
        let func = syn::parse::<syn::ItemFn>(item)?;
        Ok(Self {
            attrs,
            func,
        })
    }
}

impl ToTokens for PgxExternInventory {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let ident = &self.func.sig.ident;
        let extern_attrs = self.extern_attrs();
        let search_path = self.search_path().into_iter();
        let inputs = self.inputs();
        let returns = self.returns();

        let inv = quote! {
            pgx::inventory::submit! {
                use core::any::TypeId;
                crate::__pgx_internals::PgxExtern {
                    name: stringify!(#ident),
                    module_path: core::module_path!(),
                    extern_attrs: #extern_attrs,
                    search_path: None#( .unwrap_or(Some(vec![#search_path])) )*,
                    fn_args: vec![#(#inputs),*],
                    fn_return: #returns,
                }
            }
        };
        tokens.append_all(inv);
    }
}

#[derive(Debug)]
pub(crate) struct DefaultMacro {
    ty: syn::Type,
    comma: Token![,],
    pub(crate) expr: syn::Lit,
}

impl Parse for DefaultMacro {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(Self {
            ty: input.parse()?,
            comma: input.parse()?,
            expr: input.parse()?,
        })
    }
}

#[derive(Debug)]
pub(crate) struct NameMacro {
    pub(crate) ident: syn::Ident,
    comma: Token![,],
    pub(crate) ty: syn::Type,
}

impl Parse for NameMacro {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(Self {
            ident: input.parse()?,
            comma: input.parse()?,
            ty: input.parse()?,
        })
    }
}

#[derive(Debug)]
pub(crate) struct SearchPath {
    at_start: Option<syn::token::At>,
    dollar: Option<syn::token::Dollar>,
    path: syn::Ident,
    at_end: Option<syn::token::At>,
}

impl Parse for SearchPath {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(Self {
            at_start: input.parse()?,
            dollar: input.parse()?,
            path: input.parse()?,
            at_end: input.parse()?,
        })
    }
}

impl ToTokens for SearchPath {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let at_start = self.at_start;
        let dollar = self.dollar;
        let path = &self.path;
        let at_end = self.at_end;

        let quoted = quote! {
            #at_start#dollar#path#at_end
        };

        quoted.to_string().to_tokens(tokens);
    }
}

#[derive(Debug)]
pub(crate) struct SearchPathList {
    fields: Punctuated<SearchPath, Token![,]>,
}

impl Parse for SearchPathList {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(Self {
            fields: input.parse_terminated(SearchPath::parse).expect(&format!("Got {}", input)),
        })
    }
}

impl ToTokens for SearchPathList {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.fields.to_tokens(tokens)
    }
}
