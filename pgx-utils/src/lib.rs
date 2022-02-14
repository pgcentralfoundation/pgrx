// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use crate::{pg_config::PgConfig, sql_entity_graph_generators::PositioningRef};
use colored::Colorize;
use eyre::{eyre, WrapErr};
use proc_macro2::{TokenStream, TokenTree};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use serde_json::value::Value as JsonValue;
use std::{
    collections::HashSet,
    path::PathBuf,
    process::{Command, Stdio},
    str::FromStr,
};
use syn::{GenericArgument, ItemFn, PathArguments, ReturnType, Type, TypeParamBound};

pub mod operator_common;
pub mod pg_config;
pub mod sql_entity_graph_generators;
pub mod sql_entity_graph;
pub mod aggregate_options;

pub static BASE_POSTGRES_PORT_NO: u16 = 28800;
pub static BASE_POSTGRES_TESTING_PORT_NO: u16 = 32200;

pub fn get_target_dir() -> eyre::Result<PathBuf> {
    let mut command = Command::new("cargo");
    command
        .arg("metadata")
        .arg("--format-version=1")
        .arg("--no-deps");
    let output = command
        .output()
        .wrap_err("Unable to get target directory from `cargo metadata`")?;
    if !output.status.success() {
        return Err(eyre!(
            "'cargo metadata' failed with exit code: {}",
            output.status
        ));
    }

    let json: JsonValue =
        serde_json::from_slice(&output.stdout).wrap_err("Invalid `cargo metada` response")?;
    let target_dir = json.get("target_directory");
    match target_dir {
        Some(JsonValue::String(target_dir)) => Ok(target_dir.into()),
        v => Err(eyre!(
            "could not read target dir from `cargo metadata` got: {:?}",
            v,
        )),
    }
}

pub fn prefix_path<P: Into<PathBuf>>(dir: P) -> String {
    let mut path = std::env::split_paths(&std::env::var_os("PATH").expect("failed to get $PATH"))
        .collect::<Vec<_>>();

    path.insert(0, dir.into());
    std::env::join_paths(path)
        .expect("failed to join paths")
        .into_string()
        .expect("failed to construct path")
}

pub fn createdb(
    pg_config: &PgConfig,
    dbname: &str,
    is_test: bool,
    if_not_exists: bool,
) -> eyre::Result<bool> {
    if if_not_exists && does_db_exist(pg_config, dbname)? {
        return Ok(false);
    }

    println!("{} database {}", "     Creating".bold().green(), dbname);
    let mut command = Command::new(pg_config.createdb_path()?);
    command
        .env_remove("PGDATABASE")
        .env_remove("PGHOST")
        .env_remove("PGPORT")
        .env_remove("PGUSER")
        .arg("-h")
        .arg(pg_config.host())
        .arg("-p")
        .arg(if is_test {
            pg_config.test_port()?.to_string()
        } else {
            pg_config.port()?.to_string()
        })
        .arg(dbname)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let command_str = format!("{:?}", command);

    let output = command.output()?;

    if !output.status.success() {
        return Err(eyre!(
            "problem running createdb: {}\n\n{}{}",
            command_str,
            String::from_utf8(output.stdout).unwrap(),
            String::from_utf8(output.stderr).unwrap()
        ));
    }

    Ok(true)
}

fn does_db_exist(pg_config: &PgConfig, dbname: &str) -> eyre::Result<bool> {
    let mut command = Command::new(pg_config.psql_path()?);
    command
        .arg("-XqAt")
        .env_remove("PGUSER")
        .arg("-h")
        .arg(pg_config.host())
        .arg("-p")
        .arg(pg_config.port()?.to_string())
        .arg("template1")
        .arg("-c")
        .arg(&format!(
            "select count(*) from pg_database where datname = '{}';",
            dbname.replace("'", "''")
        ))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let command_str = format!("{:?}", command);
    let output = command.output()?;

    if !output.status.success() {
        return Err(eyre!(
            "problem checking if database '{}' exists: {}\n\n{}{}",
            dbname,
            command_str,
            String::from_utf8(output.stdout).unwrap(),
            String::from_utf8(output.stderr).unwrap()
        ));
    } else {
        let count = i32::from_str(&String::from_utf8(output.stdout).unwrap().trim())
            .wrap_err("result is not a number")?;
        Ok(count > 0)
    }
}

pub fn get_named_capture(
    regex: &regex::Regex,
    name: &'static str,
    against: &str,
) -> Option<String> {
    match regex.captures(against) {
        Some(cap) => Some(cap[name].to_string()),
        None => None,
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, PartialOrd, Ord)]
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
    Schema(String),
    Name(String),
    Cost(String),
    Requires(Vec<PositioningRef>),
}

impl core::fmt::Display for ExternArgs {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
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

pub fn anonymonize_lifetimes_in_type_path(value: syn::TypePath) -> syn::TypePath {
    let mut ty = syn::Type::Path(value);
    anonymonize_lifetimes(&mut ty);
    match ty {
        syn::Type::Path(type_path) => type_path,

        // shouldn't happen
        _ => panic!("not a TypePath"),
    }
}

pub fn anonymonize_lifetimes(value: &mut syn::Type) {
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
                                syn::GenericArgument::Type(ty) => anonymonize_lifetimes(ty),
                                syn::GenericArgument::Binding(binding) => {
                                    anonymonize_lifetimes(&mut binding.ty)
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
                anonymonize_lifetimes(elem);
            }
        }

        _ => {}
    }
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
        assert!(args.contains(&ExternArgs::Error(
            "syntax error at or near \"THIS\"".to_string()
        )));
    }
}
