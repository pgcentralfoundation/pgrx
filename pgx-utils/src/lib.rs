// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use colored::Colorize;
use proc_macro2::TokenTree;
use quote::quote;
use std::collections::HashSet;
use std::process::Command;
use syn::export::TokenStream2;
use syn::{GenericArgument, ItemFn, PathArguments, ReturnType, Type, TypeParamBound};

pub static BASE_POSTGRES_PORT_NO: u16 = 28800;

pub fn get_target_dir() -> String {
    std::env::var("CARGO_TARGET_DIR")
        .unwrap_or_else(|_| format!("{}/target", std::env::var("PWD").unwrap()))
}

pub fn get_pg_download_dir() -> String {
    std::env::var("PG_DOWNLOAD_TARGET_DIR").unwrap_or_else(|_| get_target_dir())
}

pub fn run_pg_config(pg_config: &Option<String>, arg: &str) -> String {
    let pg_config = pg_config
        .clone()
        .unwrap_or_else(|| std::env::var("PG_CONFIG").unwrap_or_else(|_| "pg_config".to_string()));
    let output = Command::new(&pg_config).arg(arg).output();

    match output {
        Ok(output) => String::from_utf8(output.stdout).unwrap().trim().to_string(),

        Err(e) => {
            eprintln!(
                "{}: Problem running {}: {}",
                "error".bold().red(),
                pg_config,
                e
            );
            std::process::exit(1);
        }
    }
}

#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum ExternArgs {
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
}

#[derive(Debug)]
pub enum CategorizedType {
    Iterator(Vec<String>),
    OptionalIterator(Vec<String>),
    Default,
}

pub fn parse_extern_attributes(attr: TokenStream2) -> HashSet<ExternArgs> {
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
                    _ => false,
                };
            }
            TokenTree::Punct(_) => {}
            TokenTree::Literal(_) => {}
        }
    }
    args
}

pub fn categorize_return_type(func: &ItemFn) -> CategorizedType {
    let rt = &func.sig.output;

    match rt {
        ReturnType::Default => CategorizedType::Default,
        ReturnType::Type(_, ty) => categorize_type(ty),
    }
}

pub fn categorize_type(ty: &Type) -> CategorizedType {
    match ty {
        Type::Path(ty) => {
            let segments = &ty.path.segments;
            for segment in segments {
                if segment.ident.to_string() == "Option" {
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
            }
            CategorizedType::Default
        }

        Type::ImplTrait(ty) => {
            for bound in &ty.bounds {
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
                                            panic!("Only one generic type is supported when returning an Iterator")
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

            panic!("Unsupported trait return type");
        }
        _ => CategorizedType::Default,
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_extern_attributes, ExternArgs};
    use std::str::FromStr;
    use syn::export::TokenStream2;

    #[test]
    fn parse_args() {
        let s = "error = \"syntax error at or near \\\"THIS\\\"\"";
        let ts = TokenStream2::from_str(s).unwrap();

        let args = parse_extern_attributes(ts);
        assert!(args.contains(&ExternArgs::Error(
            "syntax error at or near \"THIS\"".to_string()
        )));
    }
}
