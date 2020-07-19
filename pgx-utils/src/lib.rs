// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use colored::Colorize;
use proc_macro2::TokenTree;
use quote::quote;
use serde_derive::Deserialize;
use std::collections::HashSet;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::str::FromStr;
use syn::export::TokenStream2;
use syn::{GenericArgument, ItemFn, PathArguments, ReturnType, Type, TypeParamBound};

pub static BASE_POSTGRES_PORT_NO: u16 = 28800;
pub static BASE_POSTGRES_TESTING_PORT_NO: u16 = 32200;

#[macro_export]
macro_rules! exit_with_error {
    () => ({ exit_with_error!("explicit panic") });
    ($msg:expr) => ({ exit_with_error!("{}", $msg) });
    ($msg:expr,) => ({ exit_with_error!($msg) });
    ($fmt:expr, $($arg:tt)+) => ({
        use colored::Colorize;
        eprint!("{} ", "     [error]".bold().red());
        eprintln!($fmt, $($arg)+);
        std::process::exit(1);
    });
}

#[macro_export]
macro_rules! exit {
    () => ({ exit!("explicit panic") });
    ($msg:expr) => ({ exit!("{}", $msg) });
    ($msg:expr,) => ({ exit!($msg) });
    ($fmt:expr, $($arg:tt)+) => ({
        eprintln!($fmt, $($arg)+);
        std::process::exit(1);
    });
}

#[macro_export]
macro_rules! handle_result {
    ($message:expr, $expr:expr) => {{
        match $expr {
            Ok(result) => result,
            Err(e) => crate::exit_with_error!("{}: {}", $message, e),
        }
    }};
}

#[derive(Debug, Deserialize)]
pub struct PgConfigPaths {
    pub pg10: String,
    pub pg11: String,
    pub pg12: String,
}

#[derive(Debug, Deserialize)]
struct Configs {
    configs: PgConfigPaths,
}

pub fn load_pgx_config() -> PgConfigPaths {
    let path = get_pgx_config_path();

    if !path.exists() {
        // TODO:  do this automatically if an environment variable is set?
        //        I think we want/need that ability
        exit_with_error!(
            "{} not found.  Have you run `{}` yet?",
            path.display(),
            "cargo pgx init".bold().yellow()
        )
    }

    handle_result!(
        "config.toml invalid",
        toml::from_str::<Configs>(handle_result!(
            "Unable to read config.toml",
            &std::fs::read_to_string(path)
        ))
    )
    .configs
}

pub fn get_pgbin_dir(major_version: u16) -> PathBuf {
    run_pg_config(&get_pg_config(major_version), "--bindir").into()
}

pub fn get_postmaster_path(major_version: u16) -> PathBuf {
    let mut path = get_pgbin_dir(major_version);
    path.push("postmaster");
    path
}

pub fn get_initdb_path(major_version: u16) -> PathBuf {
    let mut path = get_pgbin_dir(major_version);
    path.push("initdb");
    path
}

pub fn get_createdb_path(major_version: u16) -> PathBuf {
    let mut path = get_pgbin_dir(major_version);
    path.push("createdb");
    path
}

pub fn get_dropdb_path(major_version: u16) -> PathBuf {
    let mut path = get_pgbin_dir(major_version);
    path.push("dropdb");
    path
}

pub fn get_pgdata_dir(major_version: u16) -> PathBuf {
    let mut path = get_pgx_home();
    path.push(format!("data-{}", major_version));
    path
}

pub fn get_pglog_file(major_version: u16) -> PathBuf {
    let mut path = get_pgx_home();
    path.push(format!("{}.log", major_version));
    path
}

pub fn get_pgx_home() -> PathBuf {
    std::env::var("PGX_HOME").map_or_else(
        |_| {
            let mut dir = match dirs::home_dir() {
                Some(dir) => dir,
                None => exit_with_error!("You don't seem to have a home directory"),
            };
            dir.push(".pgx");
            if !dir.exists() {
                handle_result!(
                    format!("creating {}", dir.display()),
                    std::fs::create_dir_all(&dir)
                );
            }

            dir
        },
        |v| v.into(),
    )
}

pub fn get_pgx_config_path() -> PathBuf {
    let mut path = get_pgx_home();
    path.push("config.toml");
    path
}

pub fn get_target_dir() -> PathBuf {
    std::env::var("CARGO_TARGET_DIR").map_or_else(
        |_| {
            let mut cwd = std::env::current_dir().unwrap();
            cwd.push("target");
            cwd
        },
        |v| v.into(),
    )
}

pub fn get_pg_config(major_version: u16) -> Option<String> {
    let paths = load_pgx_config();
    match major_version {
        10 => Some(paths.pg10),
        11 => Some(paths.pg11),
        12 => Some(paths.pg12),
        _ => None,
    }
}

pub fn get_pg_config_major_version(pg_config: &Option<String>) -> u16 {
    let version_string = run_pg_config(&pg_config, "--version");
    let version_parts = version_string.split_whitespace().collect::<Vec<&str>>();
    let version = version_parts.get(1);
    let version = f64::from_str(&version.unwrap()).expect("not a valid version number");
    version.floor() as u16
}

pub fn get_pg_download_dir() -> PathBuf {
    std::env::var("PG_DOWNLOAD_TARGET_DIR").map_or_else(|_| get_target_dir(), |v| v.into())
}

pub fn get_psql_path(major_version: u16) -> PathBuf {
    let mut bindir = get_pgbin_dir(major_version);
    bindir.push("psql");
    bindir
}

pub fn run_pg_config(pg_config: &Option<String>, arg: &str) -> String {
    let pg_config = pg_config
        .clone()
        .unwrap_or_else(|| std::env::var("PG_CONFIG").unwrap_or_else(|_| "pg_config".to_string()));
    let output = handle_result!(
        format!("{}", pg_config),
        Command::new(&pg_config).arg(arg).output()
    );

    String::from_utf8(output.stdout).unwrap().trim().to_string()
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
    major_version: u16,
    host: &str,
    port: u16,
    dbname: &str,
    if_not_exists: bool,
) -> bool {
    if if_not_exists && does_db_exist(major_version, host, port, dbname) {
        return false;
    }

    println!("{} database {}", "    Creating".bold().green(), dbname);
    let mut command = Command::new(get_createdb_path(major_version));
    command
        .arg("-h")
        .arg(host)
        .arg("-p")
        .arg(port.to_string())
        .arg(dbname)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let command_str = format!("{:?}", command);

    let output = handle_result!(
        format!("Failed to create database {}", dbname),
        command.output()
    );

    if !output.status.success() {
        exit_with_error!(
            "problem running createdb: {}\n\n{}{}",
            command_str,
            String::from_utf8(output.stdout).unwrap(),
            String::from_utf8(output.stderr).unwrap()
        )
    }

    true
}

fn does_db_exist(major_version: u16, host: &str, port: u16, dbname: &str) -> bool {
    let mut command = Command::new(get_psql_path(major_version));
    command
        .arg("-XqAt")
        .arg("-h")
        .arg(host)
        .arg("-p")
        .arg(port.to_string())
        .arg("template1")
        .arg("-c")
        .arg(&format!(
            "select count(*) from pg_database where datname = '{}';",
            dbname.replace("'", "''")
        ))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let command_str = format!("{:?}", command);

    let output = handle_result!(
        format!("Failed to create database {}", dbname),
        command.output()
    );

    if !output.status.success() {
        exit_with_error!(
            "problem checking if database '{}' exists: {}\n\n{}{}",
            dbname,
            command_str,
            String::from_utf8(output.stdout).unwrap(),
            String::from_utf8(output.stderr).unwrap()
        )
    } else {
        let count = i32::from_str(&String::from_utf8(output.stdout).unwrap().trim())
            .expect("result is not a number");
        count > 0
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
