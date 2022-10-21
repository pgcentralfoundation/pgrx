/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use crate::sql_entity_graph::{NameMacro, PositioningRef};
use proc_macro2::{TokenStream, TokenTree};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use std::collections::HashSet;
use syn::{GenericArgument, PathArguments, Type, TypeParamBound};

pub mod rewriter;
pub mod sql_entity_graph;

#[doc(hidden)]
pub mod __reexports {
    pub use eyre;
    // For `#[no_std]` based `pgx` extensions we use `HashSet` for type mappings.
    pub mod std {
        pub mod collections {
            pub use std::collections::HashSet;
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, PartialOrd, Ord)]
pub enum ExternArgs {
    CreateOrReplace,
    Immutable,
    Strict,
    Stable,
    Volatile,
    Raw,
    NoGuard,
    ParallelSafe,
    ParallelUnsafe,
    ParallelRestricted,
    Error(String),
    Schema(String),
    Name(String),
    Cost(String),
    Requires(Vec<PositioningRef>),
}

impl core::fmt::Display for ExternArgs {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ExternArgs::CreateOrReplace => write!(f, "CREATE OR REPLACE"),
            ExternArgs::Immutable => write!(f, "IMMUTABLE"),
            ExternArgs::Strict => write!(f, "STRICT"),
            ExternArgs::Stable => write!(f, "STABLE"),
            ExternArgs::Volatile => write!(f, "VOLATILE"),
            ExternArgs::Raw => Ok(()),
            ExternArgs::ParallelSafe => write!(f, "PARALLEL SAFE"),
            ExternArgs::ParallelUnsafe => write!(f, "PARALLEL UNSAFE"),
            ExternArgs::ParallelRestricted => write!(f, "PARALLEL RESTRICTED"),
            ExternArgs::Error(_) => Ok(()),
            ExternArgs::NoGuard => Ok(()),
            ExternArgs::Schema(_) => Ok(()),
            ExternArgs::Name(_) => Ok(()),
            ExternArgs::Cost(cost) => write!(f, "COST {}", cost),
            ExternArgs::Requires(_) => Ok(()),
        }
    }
}

impl ToTokens for ExternArgs {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            ExternArgs::CreateOrReplace => tokens.append(format_ident!("CreateOrReplace")),
            ExternArgs::Immutable => tokens.append(format_ident!("Immutable")),
            ExternArgs::Strict => tokens.append(format_ident!("Strict")),
            ExternArgs::Stable => tokens.append(format_ident!("Stable")),
            ExternArgs::Volatile => tokens.append(format_ident!("Volatile")),
            ExternArgs::Raw => tokens.append(format_ident!("Raw")),
            ExternArgs::NoGuard => tokens.append(format_ident!("NoGuard")),
            ExternArgs::ParallelSafe => tokens.append(format_ident!("ParallelSafe")),
            ExternArgs::ParallelUnsafe => tokens.append(format_ident!("ParallelUnsafe")),
            ExternArgs::ParallelRestricted => tokens.append(format_ident!("ParallelRestricted")),
            ExternArgs::Error(_s) => {
                tokens.append_all(
                    quote! {
                        Error(String::from("#_s"))
                    }
                    .to_token_stream(),
                );
            }
            ExternArgs::Schema(_s) => {
                tokens.append_all(
                    quote! {
                        Schema(String::from("#_s"))
                    }
                    .to_token_stream(),
                );
            }
            ExternArgs::Name(_s) => {
                tokens.append_all(
                    quote! {
                        Name(String::from("#_s"))
                    }
                    .to_token_stream(),
                );
            }
            ExternArgs::Cost(_s) => {
                tokens.append_all(
                    quote! {
                        Cost(String::from("#_s"))
                    }
                    .to_token_stream(),
                );
            }
            ExternArgs::Requires(items) => {
                tokens.append_all(
                    quote! {
                        Requires(vec![#(#items),*])
                    }
                    .to_token_stream(),
                );
            }
        }
    }
}

#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum FunctionArgs {
    SearchPath(String),
}

#[derive(Debug)]
pub enum CategorizedType {
    Iterator(Vec<String>),
    OptionalIterator(Vec<String>),
    Tuple(Vec<String>),
    Default,
}

pub fn parse_extern_attributes(attr: TokenStream) -> HashSet<ExternArgs> {
    let mut args = HashSet::<ExternArgs>::new();
    let mut itr = attr.into_iter();
    while let Some(t) = itr.next() {
        match t {
            TokenTree::Group(g) => {
                for arg in parse_extern_attributes(g.stream()).into_iter() {
                    args.insert(arg);
                }
            }
            TokenTree::Ident(i) => {
                let name = i.to_string();
                match name.as_str() {
                    "create_or_replace" => args.insert(ExternArgs::CreateOrReplace),
                    "immutable" => args.insert(ExternArgs::Immutable),
                    "strict" => args.insert(ExternArgs::Strict),
                    "stable" => args.insert(ExternArgs::Stable),
                    "volatile" => args.insert(ExternArgs::Volatile),
                    "raw" => args.insert(ExternArgs::Raw),
                    "no_guard" => args.insert(ExternArgs::NoGuard),
                    "parallel_safe" => args.insert(ExternArgs::ParallelSafe),
                    "parallel_unsafe" => args.insert(ExternArgs::ParallelUnsafe),
                    "parallel_restricted" => args.insert(ExternArgs::ParallelRestricted),
                    "error" => {
                        let _punc = itr.next().unwrap();
                        let literal = itr.next().unwrap();
                        let message = literal.to_string();
                        let message = unescape::unescape(&message).expect("failed to unescape");

                        // trim leading/trailing quotes around the literal
                        let message = message[1..message.len() - 1].to_string();
                        args.insert(ExternArgs::Error(message.to_string()))
                    }
                    "schema" => {
                        let _punc = itr.next().unwrap();
                        let literal = itr.next().unwrap();
                        let schema = literal.to_string();
                        let schema = unescape::unescape(&schema).expect("failed to unescape");

                        // trim leading/trailing quotes around the literal
                        let schema = schema[1..schema.len() - 1].to_string();
                        args.insert(ExternArgs::Schema(schema.to_string()))
                    }
                    "name" => {
                        let _punc = itr.next().unwrap();
                        let literal = itr.next().unwrap();
                        let name = literal.to_string();
                        let name = unescape::unescape(&name).expect("failed to unescape");

                        // trim leading/trailing quotes around the literal
                        let name = name[1..name.len() - 1].to_string();
                        args.insert(ExternArgs::Name(name.to_string()))
                    }
                    // Recognized, but not handled as an extern argument
                    "sql" => {
                        let _punc = itr.next().unwrap();
                        let _value = itr.next().unwrap();
                        false
                    }
                    _ => false,
                };
            }
            TokenTree::Punct(_) => {}
            TokenTree::Literal(_) => {}
        }
    }
    args
}

pub fn categorize_type(ty: &Type) -> CategorizedType {
    match ty {
        Type::Path(ty) => {
            let segments = &ty.path.segments;
            for segment in segments {
                let segment_ident = segment.ident.to_string();
                if segment_ident == "Option" {
                    match &segment.arguments {
                        PathArguments::AngleBracketed(a) => match a.args.first().unwrap() {
                            GenericArgument::Type(ty) => {
                                let result = categorize_type(ty);

                                return match result {
                                    CategorizedType::Iterator(i) => {
                                        CategorizedType::OptionalIterator(i)
                                    }

                                    _ => result,
                                };
                            }
                            _ => {
                                break;
                            }
                        },
                        _ => {
                            break;
                        }
                    }
                }
                if segment_ident == "Box" {
                    match &segment.arguments {
                        PathArguments::AngleBracketed(a) => match a.args.first().unwrap() {
                            GenericArgument::Type(ty) => return categorize_type(ty),
                            _ => {
                                break;
                            }
                        },
                        _ => {
                            break;
                        }
                    }
                }
            }
            CategorizedType::Default
        }
        Type::TraitObject(trait_object) => {
            for bound in &trait_object.bounds {
                return categorize_trait_bound(bound);
            }

            panic!("Unsupported trait return type");
        }
        Type::ImplTrait(ty) => {
            for bound in &ty.bounds {
                return categorize_trait_bound(bound);
            }

            panic!("Unsupported trait return type");
        }
        Type::Tuple(tuple) => {
            if tuple.elems.len() == 0 {
                CategorizedType::Default
            } else {
                let mut types = Vec::new();
                for ty in &tuple.elems {
                    types.push(quote! {#ty}.to_string())
                }
                CategorizedType::Tuple(types)
            }
        }
        _ => CategorizedType::Default,
    }
}

pub fn categorize_trait_bound(bound: &TypeParamBound) -> CategorizedType {
    match bound {
        TypeParamBound::Trait(trait_bound) => {
            let segments = &trait_bound.path.segments;

            let mut ident = String::new();
            for segment in segments {
                if !ident.is_empty() {
                    ident.push_str("::")
                }
                ident.push_str(segment.ident.to_string().as_str());
            }

            match ident.as_str() {
                "Iterator" | "std::iter::Iterator" => {
                    let segment = segments.last().unwrap();
                    match &segment.arguments {
                        PathArguments::None => {
                            panic!("Iterator must have at least one generic type")
                        }
                        PathArguments::Parenthesized(_) => {
                            panic!("Unsupported arguments to Iterator")
                        }
                        PathArguments::AngleBracketed(a) => {
                            let args = &a.args;
                            if args.len() > 1 {
                                panic!(
                                    "Only one generic type is supported when returning an Iterator"
                                )
                            }

                            match args.first().unwrap() {
                                GenericArgument::Binding(b) => {
                                    let mut types = Vec::new();
                                    let ty = &b.ty;
                                    match ty {
                                        Type::Tuple(tuple) => {
                                            for e in &tuple.elems {
                                                types.push(quote! {#e}.to_string());
                                            }
                                        },
                                        _ => {
                                            types.push(quote! {#ty}.to_string())
                                        }
                                    }

                                    return CategorizedType::Iterator(types);
                                }
                                _ => panic!("Only binding type arguments are supported when returning an Iterator")
                            }
                        }
                    }
                }
                _ => panic!("Unsupported trait return type"),
            }
        }
        TypeParamBound::Lifetime(_) => {
            panic!("Functions can't return traits with lifetime bounds")
        }
    }
}

pub fn staticize_lifetimes_in_type_path(value: syn::TypePath) -> syn::TypePath {
    let mut ty = syn::Type::Path(value);
    staticize_lifetimes(&mut ty);
    match ty {
        syn::Type::Path(type_path) => type_path,

        // shouldn't happen
        _ => panic!("not a TypePath"),
    }
}

pub fn staticize_lifetimes(value: &mut syn::Type) {
    match value {
        syn::Type::Path(type_path) => {
            for segment in &mut type_path.path.segments {
                match &mut segment.arguments {
                    syn::PathArguments::AngleBracketed(bracketed) => {
                        for arg in &mut bracketed.args {
                            match arg {
                                // rename lifetimes to the static lifetime so the TypeIds match.
                                syn::GenericArgument::Lifetime(lifetime) => {
                                    lifetime.ident =
                                        syn::Ident::new("static", lifetime.ident.span());
                                }

                                // recurse
                                syn::GenericArgument::Type(ty) => staticize_lifetimes(ty),
                                syn::GenericArgument::Binding(binding) => {
                                    staticize_lifetimes(&mut binding.ty)
                                }
                                syn::GenericArgument::Constraint(constraint) => {
                                    for bound in constraint.bounds.iter_mut() {
                                        match bound {
                                            syn::TypeParamBound::Lifetime(lifetime) => {
                                                lifetime.ident =
                                                    syn::Ident::new("static", lifetime.ident.span())
                                            }
                                            _ => {}
                                        }
                                    }
                                }

                                // nothing to do otherwise
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        syn::Type::Reference(type_ref) => match &mut type_ref.lifetime {
            Some(ref mut lifetime) => {
                lifetime.ident = syn::Ident::new("static", lifetime.ident.span());
            }
            this @ None => *this = Some(syn::parse_quote!('static)),
        },

        syn::Type::Tuple(type_tuple) => {
            for elem in &mut type_tuple.elems {
                staticize_lifetimes(elem);
            }
        }

        syn::Type::Macro(type_macro) => {
            let mac = &type_macro.mac;
            if let Some(archetype) = mac.path.segments.last() {
                match archetype.ident.to_string().as_str() {
                    "name" => {
                        if let Ok(out) = mac.parse_body::<NameMacro>() {
                            // We don't particularly care what the identifier is, so we parse a
                            // raw TokenStream.  Specifically, it's okay for the identifier String,
                            // which we end up using as a Postgres column name, to be nearly any
                            // string, which can include Rust reserved words such as "type" or "match"
                            if let Ok(ident) = syn::parse_str::<TokenStream>(&out.ident) {
                                let mut ty = out.used_ty.resolved_ty;

                                // rewrite the name!() macro's type so that it has a static lifetime, if any
                                staticize_lifetimes(&mut ty);
                                type_macro.mac = syn::parse_quote! {name!(#ident, #ty)};
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
}

pub fn anonymize_lifetimes_in_type_path(value: syn::TypePath) -> syn::TypePath {
    let mut ty = syn::Type::Path(value);
    anonymize_lifetimes(&mut ty);
    match ty {
        syn::Type::Path(type_path) => type_path,

        // shouldn't happen
        _ => panic!("not a TypePath"),
    }
}

pub fn anonymize_lifetimes(value: &mut syn::Type) {
    match value {
        syn::Type::Path(type_path) => {
            for segment in &mut type_path.path.segments {
                match &mut segment.arguments {
                    syn::PathArguments::AngleBracketed(bracketed) => {
                        for arg in &mut bracketed.args {
                            match arg {
                                // rename lifetimes to the anonymous lifetime
                                syn::GenericArgument::Lifetime(lifetime) => {
                                    lifetime.ident = syn::Ident::new("_", lifetime.ident.span());
                                }

                                // recurse
                                syn::GenericArgument::Type(ty) => anonymize_lifetimes(ty),
                                syn::GenericArgument::Binding(binding) => {
                                    anonymize_lifetimes(&mut binding.ty)
                                }
                                syn::GenericArgument::Constraint(constraint) => {
                                    for bound in constraint.bounds.iter_mut() {
                                        match bound {
                                            syn::TypeParamBound::Lifetime(lifetime) => {
                                                lifetime.ident =
                                                    syn::Ident::new("_", lifetime.ident.span())
                                            }
                                            _ => {}
                                        }
                                    }
                                }

                                // nothing to do otherwise
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        syn::Type::Reference(type_ref) => {
            if let Some(lifetime) = type_ref.lifetime.as_mut() {
                lifetime.ident = syn::Ident::new("_", lifetime.ident.span());
            }
        }

        syn::Type::Tuple(type_tuple) => {
            for elem in &mut type_tuple.elems {
                anonymize_lifetimes(elem);
            }
        }

        _ => {}
    }
}

/// Roughly `pgx::pg_sys::NAMEDATALEN`
///
/// Technically it **should** be that exactly, however this is `pgx-utils` and a this data is used at macro time.
const POSTGRES_IDENTIFIER_MAX_LEN: usize = 64;

/// Validate that a given ident is acceptable to PostgreSQL
///
/// PostgreSQL places some restrictions on identifiers for things like functions.
///
/// Namely:
///
/// * It must be less than 64 characters
///
// This list is incomplete, you could expand it!
pub fn ident_is_acceptable_to_postgres(ident: &syn::Ident) -> Result<(), syn::Error> {
    let ident_string = ident.to_string();
    if ident_string.len() >= POSTGRES_IDENTIFIER_MAX_LEN {
        return Err(syn::Error::new(
            ident.span(),
            &format!(
                "Identifier `{}` was {} characters long, PostgreSQL will truncate identifiers with less than {POSTGRES_IDENTIFIER_MAX_LEN} characters, opt for an identifier which Postgres won't truncate",
                ident,
                ident_string.len(),
            )
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{parse_extern_attributes, ExternArgs};
    use std::str::FromStr;

    #[test]
    fn parse_args() {
        let s = "error = \"syntax error at or near \\\"THIS\\\"\"";
        let ts = proc_macro2::TokenStream::from_str(s).unwrap();

        let args = parse_extern_attributes(ts);
        assert!(args.contains(&ExternArgs::Error("syntax error at or near \"THIS\"".to_string())));
    }
}
