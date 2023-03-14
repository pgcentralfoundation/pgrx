/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use bindgen::callbacks::{DeriveTrait, ImplementsTrait, MacroParsingBehavior};
use eyre::{eyre, WrapErr};
use once_cell::sync::Lazy;
use pgx_pg_config::{prefix_path, PgConfig, PgConfigSelector, Pgx, SUPPORTED_MAJOR_VERSIONS};
use proc_macro2::Span;
use quote::{quote, ToTokens, format_ident};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::PathBuf;
use std::process::{Command, Output};
use syn::{ForeignItem, Item, ItemConst, Type, Ident};

const BLOCKLISTED_TYPES: [&str; 3] = ["Datum", "NullableDatum", "Oid"];

mod build {
    pub(super) mod sym_blocklist;
}

#[derive(Debug)]
struct PgxOverrides(HashSet<String>);

fn is_nightly() -> bool {
    let rustc = std::env::var_os("RUSTC").map(PathBuf::from).unwrap_or_else(|| "rustc".into());
    let output = match std::process::Command::new(rustc).arg("--verbose").output() {
        Ok(out) if out.status.success() => String::from_utf8_lossy(&out.stdout).trim().to_owned(),
        _ => return false,
    };
    // Output looks like:
    // - for nightly: `"rustc 1.66.0-nightly (0ca356586 2022-10-06)"`
    // - for dev (locally built rust toolchain): `"rustc 1.66.0-dev"`
    output.starts_with("rustc ") && (output.contains("-nightly") || output.contains("-dev"))
}

impl PgxOverrides {
    fn default() -> Self {
        // these cause duplicate definition problems on linux
        // see: https://github.com/rust-lang/rust-bindgen/issues/687
        PgxOverrides(
            vec![
                "FP_INFINITE".into(),
                "FP_NAN".into(),
                "FP_NORMAL".into(),
                "FP_SUBNORMAL".into(),
                "FP_ZERO".into(),
                "IPPORT_RESERVED".into(),
            ]
            .into_iter()
            .collect(),
        )
    }
}

impl bindgen::callbacks::ParseCallbacks for PgxOverrides {
    fn will_parse_macro(&self, name: &str) -> MacroParsingBehavior {
        if self.0.contains(name) {
            bindgen::callbacks::MacroParsingBehavior::Ignore
        } else {
            bindgen::callbacks::MacroParsingBehavior::Default
        }
    }

    fn blocklisted_type_implements_trait(
        &self,
        name: &str,
        derive_trait: DeriveTrait,
    ) -> Option<ImplementsTrait> {
        if !BLOCKLISTED_TYPES.contains(&name) {
            return None;
        }

        let implements_trait = match derive_trait {
            DeriveTrait::Copy => ImplementsTrait::Yes,
            DeriveTrait::Debug => ImplementsTrait::Yes,
            _ => ImplementsTrait::No,
        };
        Some(implements_trait)
    }
}

fn main() -> eyre::Result<()> {
    if std::env::var("DOCS_RS").unwrap_or("false".into()) == "1" {
        return Ok(());
    }

    // dump the environment for debugging if asked
    if std::env::var("PGX_BUILD_VERBOSE").unwrap_or("false".to_string()) == "true" {
        for (k, v) in std::env::vars() {
            eprintln!("{}={}", k, v);
        }
    }

    let compile_cshim =
        std::env::var("CARGO_FEATURE_CSHIM").unwrap_or_else(|_| "0".to_string()) == "1";

    let is_for_release =
        std::env::var("PGX_PG_SYS_GENERATE_BINDINGS_FOR_RELEASE").unwrap_or("0".to_string()) == "1";
    println!("cargo:rerun-if-env-changed=PGX_PG_SYS_GENERATE_BINDINGS_FOR_RELEASE");

    // Do nightly detection to suppress silly warnings.
    if is_nightly() {
        println!("cargo:rustc-cfg=nightly")
    };

    let build_paths = BuildPaths::from_env();

    eprintln!("build_paths={build_paths:?}");

    let pgx = Pgx::from_config()?;

    println!("cargo:rerun-if-changed={}", Pgx::config_toml()?.display().to_string(),);
    println!("cargo:rerun-if-changed=include");
    println!("cargo:rerun-if-changed=cshim");
    emit_missing_rerun_if_env_changed();

    let pg_configs: Vec<(u16, PgConfig)> = if std::env::var(
        "PGX_PG_SYS_GENERATE_BINDINGS_FOR_RELEASE",
    )
    .unwrap_or("false".into())
        == "1"
    {
        pgx.iter(PgConfigSelector::All)
            .map(|r| r.expect("invalid pg_config"))
            .map(|c| (c.major_version().expect("invalid major version"), c))
            .filter_map(|t| {
                if SUPPORTED_MAJOR_VERSIONS.contains(&t.0) {
                    Some(t)
                } else {
                    println!(
                        "cargo:warning={} contains a configuration for pg{}, which pgx does not support.",
                        Pgx::config_toml()
                            .expect("Could not get PGX configuration TOML")
                            .to_string_lossy(),
                        t.0
                    );
                    None
                }
            })
            .collect()
    } else {
        let mut found = None;
        for &version in SUPPORTED_MAJOR_VERSIONS {
            if let Err(_) = std::env::var(&format!("CARGO_FEATURE_PG{}", version)) {
                continue;
            }
            if found.is_some() {
                return Err(eyre!("Multiple `pg$VERSION` features found, `--no-default-features` may be required."));
            }
            found = Some((version, format!("pg{}", version)));
        }
        let (found_ver, found_feat) = found.ok_or_else(|| {
            eyre!(
                "Did not find `pg$VERSION` feature. `pgx-pg-sys` requires one of {} to be set",
                SUPPORTED_MAJOR_VERSIONS
                    .iter()
                    .map(|x| format!("`pg{}`", x))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })?;

        if let Ok(pg_config) = PgConfig::from_env() {
            let major_version = pg_config.major_version()?;

            if major_version != found_ver {
                panic!("Feature flag `pg{found_ver}` does not match version from the environment-described PgConfig (`{major_version}`)")
            }
            vec![(major_version, pg_config)]
        } else {
            let specific = pgx.get(&found_feat)?;
            vec![(found_ver, specific)]
        }
    };
    std::thread::scope(|scope| {
        // This is pretty much either always 1 (normally) or 5 (for releases),
        // but in the future if we ever have way more, we should consider
        // chunking `pg_configs` based on `thread::available_parallelism()`.
        let threads = pg_configs
            .iter()
            .map(|(pg_major_ver, pg_config)| {
                scope.spawn(|| {
                    generate_bindings(*pg_major_ver, pg_config, &build_paths, is_for_release)
                })
            })
            .collect::<Vec<_>>();
        // Most of the rest of this is just for better error handling --
        // `thread::scope` already joins the threads for us before it returns.
        let results = threads
            .into_iter()
            .map(|thread| thread.join().expect("thread panicked while generating bindings"))
            .collect::<Vec<eyre::Result<_>>>();
        results.into_iter().try_for_each(|r| r)
    })?;

    if compile_cshim {
        // compile the cshim for each binding
        for (_version, pg_config) in pg_configs {
            build_shim(&build_paths.shim_src, &build_paths.shim_dst, &pg_config)?;
        }
    }

    Ok(())
}

fn emit_missing_rerun_if_env_changed() {
    // `pgx-pg-config` doesn't emit one for this.
    println!("cargo:rerun-if-env-changed=PGX_PG_CONFIG_PATH");
    println!("cargo:rerun-if-env-changed=PGX_PG_CONFIG_AS_ENV");
    // Bindgen's behavior depends on these vars, but it doesn't emit them
    // directly because the output would cause issue with `bindgen-cli`. Do it
    // on bindgen's behalf.
    println!("cargo:rerun-if-env-changed=LLVM_CONFIG_PATH");
    println!("cargo:rerun-if-env-changed=LIBCLANG_PATH");
    println!("cargo:rerun-if-env-changed=LIBCLANG_STATIC_PATH");
    // Follows the logic bindgen uses here, more or less.
    // https://github.com/rust-lang/rust-bindgen/blob/e6dd2c636/bindgen/lib.rs#L2918
    println!("cargo:rerun-if-env-changed=BINDGEN_EXTRA_CLANG_ARGS");
    if let Ok(target) = std::env::var("TARGET") {
        println!("cargo:rerun-if-env-changed=BINDGEN_EXTRA_CLANG_ARGS_{target}");
        println!(
            "cargo:rerun-if-env-changed=BINDGEN_EXTRA_CLANG_ARGS_{}",
            target.replace('-', "_"),
        );
    }
}

fn generate_bindings(
    major_version: u16,
    pg_config: &PgConfig,
    build_paths: &BuildPaths,
    is_for_release: bool,
) -> eyre::Result<()> {
    let mut include_h = build_paths.manifest_dir.clone();
    include_h.push("include");
    include_h.push(format!("pg{}.h", major_version));

    let bindgen_output = get_bindings(major_version, &pg_config, &include_h)
        .wrap_err_with(|| format!("bindgen failed for pg{}", major_version))?;

    let oids = extract_oids(&bindgen_output);
    let rewritten_items = rewrite_items(&bindgen_output, &oids, is_for_release)
        .wrap_err_with(|| format!("failed to rewrite items for pg{}", major_version))?;
    let oids = format_builtin_oid_impl(oids);

    let dest_dirs = if std::env::var("PGX_PG_SYS_GENERATE_BINDINGS_FOR_RELEASE")
        .unwrap_or("false".into())
        == "1"
    {
        vec![build_paths.out_dir.clone(), build_paths.src_dir.clone()]
    } else {
        vec![build_paths.out_dir.clone()]
    };
    for dest_dir in dest_dirs {
        let mut bindings_file = dest_dir.clone();
        bindings_file.push(&format!("pg{}.rs", major_version));
        write_rs_file(
            rewritten_items.clone(),
            &bindings_file,
            quote! {
                use crate as pg_sys;
                #[cfg(any(feature = "pg12", feature = "pg13", feature = "pg14", feature = "pg15"))]
                use crate::NullableDatum;
                use crate::{Datum, Oid, PgNode};
            },
        )
        .wrap_err_with(|| {
            format!(
                "Unable to write bindings file for pg{} to `{}`",
                major_version,
                bindings_file.display()
            )
        })?;

        let mut oids_file = dest_dir.clone();
        oids_file.push(&format!("pg{}_oids.rs", major_version));
        write_rs_file(oids.clone(), &oids_file, quote! {}).wrap_err_with(|| {
            format!(
                "Unable to write oids file for pg{} to `{}`",
                major_version,
                oids_file.display()
            )
        })?;
    }
    Ok(())
}

#[derive(Debug, Clone)]
struct BuildPaths {
    /// CARGO_MANIFEST_DIR
    manifest_dir: PathBuf,
    /// OUT_DIR
    out_dir: PathBuf,
    /// {manifest_dir}/src
    src_dir: PathBuf,
    /// {manifest_dir}/cshim
    shim_src: PathBuf,
    /// {out_dir}/cshim
    shim_dst: PathBuf,
}

impl BuildPaths {
    fn from_env() -> Self {
        // Cargo guarantees these are provided, so unwrap is fine.
        let manifest_dir = std::env::var_os("CARGO_MANIFEST_DIR").map(PathBuf::from).unwrap();
        let out_dir = std::env::var_os("OUT_DIR").map(PathBuf::from).unwrap();
        Self {
            src_dir: manifest_dir.join("src"),
            shim_src: manifest_dir.join("cshim"),
            shim_dst: out_dir.join("cshim"),
            out_dir,
            manifest_dir,
        }
    }
}

fn write_rs_file(
    code: proc_macro2::TokenStream,
    file: &PathBuf,
    header: proc_macro2::TokenStream,
) -> eyre::Result<()> {
    let mut contents = header;
    contents.extend(code);

    std::fs::write(&file, contents.to_string())?;
    rust_fmt(&file)
}

/// Given a token stream representing a file, apply a series of transformations to munge
/// the bindgen generated code with some postgres specific enhancements
fn rewrite_items(
    file: &syn::File,
    oids: &BTreeMap<syn::Ident, Box<syn::Expr>>,
    is_for_release: bool,
) -> eyre::Result<proc_macro2::TokenStream> {
    let items_vec = rewrite_oid_consts(&file.items, oids);
    let mut items = apply_pg_guard(&items_vec)?;
    let pgnode_impls = impl_pg_node(&items_vec, is_for_release)?;

    // append the pgnodes to the set of items
    items.extend(pgnode_impls);

    Ok(items)
}

/// Find all the constants that represent Postgres type OID values.
///
/// These are constants of type `u32` whose name ends in the string "OID"
fn extract_oids(code: &syn::File) -> BTreeMap<syn::Ident, Box<syn::Expr>> {
    let mut oids = BTreeMap::new(); // we would like to have a nice sorted set
    for item in &code.items {
        match item {
            Item::Const(ItemConst { ident, ty, expr, .. }) => {
                // Retype as strings for easy comparison
                let name = ident.to_string();
                let ty_str = ty.to_token_stream().to_string();

                // This heuristic identifies "OIDs"
                // We're going to warp the const declarations to be our newtype Oid
                if ty_str == "u32" && is_builtin_oid(&name) {
                    oids.insert(ident.clone(), expr.clone());
                }
            }
            _ => {}
        }
    }
    oids
}

fn is_builtin_oid(name: &str) -> bool {
    if name.ends_with("OID") && name != "HEAP_HASOID" {
        true
    } else if name.ends_with("RelationId") {
        true
    } else if name == "TemplateDbOid" {
        true
    } else {
        false
    }
}

fn rewrite_oid_consts(
    items: &Vec<syn::Item>,
    oids: &BTreeMap<syn::Ident, Box<syn::Expr>>,
) -> Vec<syn::Item> {
    items
        .into_iter()
        .map(|item| match item {
            Item::Const(ItemConst { ident, ty, expr, .. })
                if ty.to_token_stream().to_string() == "u32" && oids.get(ident) == Some(expr) =>
            {
                syn::parse2(quote! { pub const #ident : Oid = Oid(#expr); }).unwrap()
            }
            item => item.clone(),
        })
        .collect()
}

fn format_builtin_oid_impl<'a>(
    oids: BTreeMap<syn::Ident, Box<syn::Expr>>,
) -> proc_macro2::TokenStream {
    let enum_variants: proc_macro2::TokenStream;
    let from_impl: proc_macro2::TokenStream;
    (enum_variants, from_impl) = oids
        .iter()
        .map(|(ident, expr)| {
            (quote! { #ident = #expr, }, quote! { #expr => Ok(BuiltinOid::#ident), })
        })
        .unzip();

    quote! {
        use crate::{NotBuiltinOid};

        #[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
        pub enum BuiltinOid {
            #enum_variants
        }

        impl BuiltinOid {
            pub const fn from_u32(uint: u32) -> Result<BuiltinOid, NotBuiltinOid> {
                match uint {
                    0 => Err(NotBuiltinOid::Invalid),
                    #from_impl
                    _ => Err(NotBuiltinOid::Ambiguous),
                }
            }
        }
    }
}
fn is_node_struct(name: &str, node_types: &HashSet<String>, nodes: &Vec<StructDescriptor>) -> bool {
    let node = nodes.iter().filter(|n| n.struct_.ident.to_string() == name).next();
    match node {
        None => false,
        Some(n) => {
            let first_field = n.struct_.fields.iter().next();
            match first_field {
                None => false,
                Some(f) => match &f.ty {
                    Type::Path(p) => {
                        let first_field_type_name =
                            p.path.segments.first().unwrap().ident.to_string();
                        if first_field_type_name == "NodeTag" {
                            return true;
                        }
                        is_node_struct(first_field_type_name.as_ref(), node_types, nodes)
                    }
                    _ => false,
                },
            }
        }
    }
}

/// Implement our `PgNode` marker trait for `pg_sys::Node` and its "subclasses"
fn impl_pg_node(
    items: &Vec<syn::Item>,
    is_for_release: bool,
) -> eyre::Result<proc_macro2::TokenStream> {
    let mut pgnode_impls = proc_macro2::TokenStream::new();

    // we scope must of the computation so we can borrow `items` and then
    // extend it at the very end.
    let struct_graph: StructGraph = StructGraph::from(&items[..]);

    // collect all the structs with `NodeTag` as their first member,
    // these will serve as roots in our forest of `Node`s
    let mut root_node_structs = Vec::new();
    for descriptor in struct_graph.descriptors.iter() {
        // grab the first field, if any
        let first_field = match &descriptor.struct_.fields {
            syn::Fields::Named(fields) => {
                if let Some(first_field) = fields.named.first() {
                    first_field
                } else {
                    continue;
                }
            }
            syn::Fields::Unnamed(fields) => {
                if let Some(first_field) = fields.unnamed.first() {
                    first_field
                } else {
                    continue;
                }
            }
            _ => continue,
        };

        // grab the type name of the first field
        let ty_name = if let syn::Type::Path(p) = &first_field.ty {
            if let Some(last_segment) = p.path.segments.last() {
                last_segment.ident.to_string()
            } else {
                continue;
            }
        } else {
            continue;
        };

        if ty_name == "NodeTag" {
            root_node_structs.push(descriptor);
        }
    }

    // the set of types which subclass `Node` according to postgres' object system
    let mut node_set = HashSet::new();
    // fill in any children of the roots with a recursive DFS
    // (we are not operating on user input, so it is ok to just
    //  use direct recursion rather than an explicit stack).
    for root in root_node_structs.into_iter() {
        dfs_find_nodes(root, &struct_graph, &mut node_set);
    }

    let mut nodes = node_set.into_iter().collect::<Vec<_>>();
    if is_for_release {
        // if it's for release we want to sort by struct name to avoid diff churn
        // otherwise we don't care and want to avoid the CPU overhead of sorting
        nodes.sort_by_key(|s| &s.struct_.ident);
    }

    let node_types = items
        .iter()
        .filter_map(|i| match i {
            Item::Const(c) if c.ident.to_string().starts_with("NodeTag_T_") => {
                Some(c.ident.to_string()["NodeTag_T_".len()..].to_string())
            }
            _ => None,
        })
        .filter(|t| match t.as_ref() {
            // `NodeTag_T_`s that don't have a corresponding struct we care about. This is a single
            // list, for convenience, but technically it's different in different versions of
            // Postgres.
            "BitString" | "Invalid" | "Null" | "IntList" | "OidList" | "AllocSetContext"
            | "SlabContext" | "Integer" | "PathKeyInfo" | "GenerationContext" | "Float"
            | "TsmRoutine" | "String" | "WindowObjectData" | "MemoryContext" | "TIDBitmap" => false,
            _ => true,
        })
        .collect::<HashSet<String>>();

    // Provide a special implementation of PgNode for the Node itself: depending on the `type_` field,
    // call the override for that type. That way, when we get a `*mut Node` out of a List object, we
    // can call this function and let it route appropriately.
    let mut node_traverse_body = proc_macro2::TokenStream::new();
    for node_type in &node_types {
        let nodetag = format_ident!("NodeTag_T_{}", node_type.to_string());
        let type_ = format_ident!("{}", node_type);
        node_traverse_body.extend(quote! {
            #nodetag => {
                emit(format!("Calling specific traverse for item of type {}", stringify!(#type_)));
                #type_::traverse(
                    &mut unsafe { std::ptr::read(self as *mut Node as *mut #type_) },
                    walker_fn,
                    context)
            },
        });
    }
    let mut node_display_body = proc_macro2::TokenStream::new();
    for node_type in &node_types {
        let nodetag = format_ident!("NodeTag_T_{}", node_type.to_string());
        let type_ = format_ident!("{}", node_type);
        node_display_body.extend(quote! {
            #nodetag => #type_::fmt(&unsafe { std::ptr::read(self as *const Node as *const #type_) }, f),
        });
    }

    let join_fields_traverse = if std::env::var("CARGO_FEATURE_PG11").is_ok()
        || std::env::var("CARGO_FEATURE_PG12").is_ok()
    {
        quote! {}
    } else if std::env::var("CARGO_FEATURE_PG13").is_ok() {
        quote! {
            if !self.joinleftcols.is_null() {
                walker_fn(self.joinleftcols as *mut Node, context);
                Node::traverse::<T>(unsafe { &mut *(self.joinleftcols as *mut Node) }, walker_fn, context);
            }
            if !self.joinrightcols.is_null() {
                walker_fn(self.joinrightcols as *mut Node, context);
                Node::traverse::<T>(unsafe { &mut *(self.joinrightcols as *mut Node) }, walker_fn, context);
            }
        }
    } else {
        quote! {
            if !self.joinleftcols.is_null() {
                walker_fn(self.joinleftcols as *mut Node, context);
                Node::traverse::<T>(unsafe { &mut *(self.joinleftcols as *mut Node) }, walker_fn, context);
            }
            if !self.joinrightcols.is_null() {
                walker_fn(self.joinrightcols as *mut Node, context);
                Node::traverse::<T>(unsafe { &mut *(self.joinrightcols as *mut Node) }, walker_fn, context);
            }
            if !self.join_using_alias.is_null() {
                walker_fn(self.join_using_alias as *mut Node, context);
                Node::traverse::<T>(unsafe { &mut *(self.join_using_alias as *mut Node) }, walker_fn, context);
            }
        }
    };
    let rtekind_result_handling = if std::env::var("CARGO_FEATURE_PG11").is_ok() {
        quote! {}
    } else {
        quote! {
            RTEKind_RTE_RESULT => {
                /* no extra fields */
            },
        }
    };

    // Older Postgres' List is linked.
    let list_fns = if std::env::var("CARGO_FEATURE_PG11").is_ok()
        || std::env::var("CARGO_FEATURE_PG12").is_ok()
    {
        quote! {
            impl pg_sys::PgNode for List {
                fn traverse<T>(&mut self, walker_fn: fn(*mut Node, &mut T) -> (), context: &mut T) {
                    emit(format!("Preparing to traverse a linked list of {} elements", self.length));

                    if self.length == 0 { return; }
                    let mut cell = unsafe { *self.head };
                    for _index in 0..self.length {
                        let item = unsafe { cell.data.ptr_value as *mut Node };
                        if !item.is_null() {
                            emit(format!("Traversing list[{}]: {:?}", _index, item));

                            walker_fn(item, context);
                            Node::traverse::<T>( unsafe { &mut *(item as *mut Node) }, walker_fn, context);
                        }
                        if !cell.next.is_null() {
                            cell = unsafe { *cell.next };
                        }
                    }
                    emit(format!("Done traversing a list of {} elements", self.length));
                }
            }
            impl std::fmt::Display for List {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "[")?;
                    if self.length != 0 {
                        let mut cell = unsafe { *self.head };
                        for index in 0..self.length {
                            if index != 0 {
                                write!(f, ", ")?;
                            }
                            let item = unsafe { cell.data.ptr_value as *mut Node };
                            if item.is_null() {
                                write!(f, "(null)")?;
                            } else {
                                write!(f, "{}", unsafe { *item })?;
                            }
                            if !cell.next.is_null() {
                                cell = unsafe { *cell.next };
                            }
                        }
                    }
                    write!(f, "]")
                }
            }
        }
    } else {
        quote! {
            impl pg_sys::PgNode for List {
                fn traverse<T>(&mut self, walker_fn: fn(*mut Node, &mut T) -> (), context: &mut T) {
                    emit(format!("Preparing to traverse an arraylist of {} elements", self.length));

                    for index in 0..self.length {
                        let item = unsafe { (*self.elements.offset(index as isize)).ptr_value as *mut Node };
                        if !item.is_null() {
                            emit(format!("Traversing list[{}]: {:?}", index, item));
                            walker_fn(item, context);
                            Node::traverse::<T>( unsafe { &mut *(item as *mut Node) }, walker_fn, context);
                        }
                    }
                    emit(format!("Done traversing a list of {} elements", self.length));
                }
            }
            impl std::fmt::Display for List {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "[")?;
                    for index in 0..self.length {
                        if index != 0 {
                            write!(f, ", ")?;
                        }
                        let item = unsafe { (*self.elements.offset(index as isize)).ptr_value as *mut Node };
                        if item.is_null() {
                            write!(f, "(null)")?;
                        } else {
                            write!(f, "{}", unsafe { *item })?;
                        }
                    }
                    write!(f, "]")
                }
            }
        }
    };

    pgnode_impls.extend(quote! {
        pub fn emit(string: std::string::String) {
            // let mut file = ::std::fs::OpenOptions::new()
            //     .write(true)
            //     .append(true)
            //     .open("/tmp/log")
            //     .unwrap();
            // ::std::writeln!(file, "{}", string).unwrap();
            eprintln!("{}", string);
        }

        unsafe fn list_length(list: *mut List) -> i32 {
            if list.is_null() {
                0
            } else {
                (*list).length
            }
        }
        #list_fns

        impl pg_sys::PgNode for Node {
            fn traverse<T>(&mut self, walker_fn: fn(*mut Node, &mut T) -> (), context: &mut T) {
                match self.type_ {
                    #node_traverse_body
                    _ => {},
                }
            }
        }
        impl std::fmt::Display for Node {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self.type_ {
                    #node_display_body
                    _ => write!(f, "[unknown type {}]", self.type_),
                }
            }
        }
        impl pg_sys::seal::Sealed for Node {}
        impl pg_sys::seal::Sealed for List {}
        impl pg_sys::seal::Sealed for RangeTblEntry {}
        impl pg_sys::PgNode for RangeTblEntry {
            fn traverse<T>(&mut self, walker_fn: fn(*mut Node, &mut T) -> (), context: &mut T) {
                // Derived from the implementation of `_outRangeTblEntry` in outfuncs.c.
                // TODO: consider making the if/walk/traverse stanza into a macro?
                if !self.alias.is_null() {
                    walker_fn(self.alias as *mut Node, context);
                    Node::traverse::<T>(unsafe { &mut *(self.alias as *mut Node) }, walker_fn, context);
                }
                if !self.eref.is_null() {
                    walker_fn(self.eref as *mut Node, context);
                    Node::traverse::<T>(unsafe { &mut *(self.eref as *mut Node) }, walker_fn, context);
                }
                // WRITE_ENUM_FIELD(rtekind, RTEKind);

                match self.rtekind {
                    RTEKind_RTE_RELATION => {
                        // WRITE_OID_FIELD(relid);
                        // WRITE_CHAR_FIELD(relkind);
                        // WRITE_INT_FIELD(rellockmode);
                        if !self.tablesample.is_null() {
                            walker_fn(self.tablesample as *mut Node, context);
                            Node::traverse::<T>(unsafe { &mut *(self.tablesample as *mut Node) }, walker_fn, context);
                        }
                    },
                    RTEKind_RTE_SUBQUERY => {
                        if !self.subquery.is_null() {
                            walker_fn(self.subquery as *mut Node, context);
                            Node::traverse::<T>(unsafe { &mut *(self.subquery as *mut Node) }, walker_fn, context);
                        }
                        // WRITE_BOOL_FIELD(security_barrier);
                    },
                    RTEKind_RTE_JOIN => {
                        // WRITE_ENUM_FIELD(jointype, JoinType);
                        // WRITE_INT_FIELD(joinmergedcols);

                        if !self.joinaliasvars.is_null() {
                            walker_fn(self.joinaliasvars as *mut Node, context);
                            Node::traverse::<T>(unsafe { &mut *(self.joinaliasvars as *mut Node) }, walker_fn, context);
                        }
                        #join_fields_traverse
                    }
                    RTEKind_RTE_FUNCTION => {
                        if !self.functions.is_null() {
                            walker_fn(self.functions as *mut Node, context);
                            Node::traverse::<T>(unsafe { &mut *(self.functions as *mut Node) }, walker_fn, context);
                        }
                        // WRITE_BOOL_FIELD(funcordinality);
                    }
                    RTEKind_RTE_TABLEFUNC => {
                        if !self.tablefunc.is_null() {
                            walker_fn(self.tablefunc as *mut Node, context);
                            Node::traverse::<T>(unsafe { &mut *(self.tablefunc as *mut Node) }, walker_fn, context);
                        }
                    }
                    RTEKind_RTE_VALUES => {
                        if !self.values_lists.is_null() {
                            walker_fn(self.values_lists as *mut Node, context);
                            Node::traverse::<T>(unsafe { &mut *(self.values_lists as *mut Node) }, walker_fn, context);
                        }
                        if !self.coltypes.is_null() {
                            walker_fn(self.coltypes as *mut Node, context);
                            Node::traverse::<T>(unsafe { &mut *(self.coltypes as *mut Node) }, walker_fn, context);
                        }
                        if !self.coltypmods.is_null() {
                            walker_fn(self.coltypmods as *mut Node, context);
                            Node::traverse::<T>(unsafe { &mut *(self.coltypmods as *mut Node)}, walker_fn, context);
                        }
                        if !self.colcollations.is_null() {
                            walker_fn(self.colcollations as *mut Node, context);
                            Node::traverse::<T>(unsafe { &mut *(self.colcollations as *mut Node) }, walker_fn, context);
                        }
                    }
                    RTEKind_RTE_CTE => {
                        // WRITE_STRING_FIELD(ctename);
                        // WRITE_UINT_FIELD(ctelevelsup);
                        // WRITE_BOOL_FIELD(self_reference);
                        if !self.coltypes.is_null() {
                            walker_fn(self.coltypes as *mut Node, context);
                            Node::traverse::<T>(unsafe { &mut *(self.coltypes as *mut Node) }, walker_fn, context);
                        }
                        if !self.coltypmods.is_null() {
                            walker_fn(self.coltypmods as *mut Node, context);
                            Node::traverse::<T>(unsafe { &mut *(self.coltypmods as *mut Node) }, walker_fn, context);
                        }
                        if !self.colcollations.is_null() {
                            walker_fn(self.colcollations as *mut Node, context);
                            Node::traverse::<T>(unsafe { &mut *(self.colcollations as *mut Node) }, walker_fn, context);
                        }
                    }
                    RTEKind_RTE_NAMEDTUPLESTORE => {
                        // WRITE_STRING_FIELD(enrname);
                        // WRITE_FLOAT_FIELD(enrtuples, "%.0f");
                        // WRITE_OID_FIELD(relid);
                        if !self.coltypes.is_null() {
                            walker_fn(self.coltypes as *mut Node, context);
                            Node::traverse::<T>(unsafe { &mut *(self.coltypes as *mut Node) }, walker_fn, context);
                        }
                        if !self.coltypmods.is_null() {
                            walker_fn(self.coltypmods as *mut Node, context);
                            Node::traverse::<T>(unsafe { &mut *(self.coltypmods as *mut Node) }, walker_fn, context);
                        }
                        if !self.colcollations.is_null() {
                            walker_fn(self.colcollations as *mut Node, context);
                            Node::traverse::<T>(unsafe { &mut *(self.colcollations as *mut Node) }, walker_fn, context);
                        }
                    }
                    #rtekind_result_handling
                    _ => {
                        unsafe { pre_format_elog_string(ERROR as i32, format!("unrecognized RTE kind: {}", self.rtekind).as_ptr() as *const i8); }
                    }
                }

                // WRITE_BOOL_FIELD(lateral);
                // WRITE_BOOL_FIELD(inh);
                // WRITE_BOOL_FIELD(inFromCl);
                // WRITE_UINT_FIELD(requiredPerms);
                // WRITE_OID_FIELD(checkAsUser);
                // WRITE_BITMAPSET_FIELD(selectedCols);
                // WRITE_BITMAPSET_FIELD(insertedCols);
                // WRITE_BITMAPSET_FIELD(updatedCols);
                // WRITE_BITMAPSET_FIELD(extraUpdatedCols);
                if !self.securityQuals.is_null() {
                    walker_fn(self.securityQuals as *mut Node, context);
                    Node::traverse::<T>(unsafe { &mut *(self.securityQuals as *mut Node) }, walker_fn, context);
                }
            }
        }
        impl std::fmt::Display for RangeTblEntry {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.display_node() )
            }
        }
    });

    struct ArrayBoundsInfo {
        n: Option<proc_macro2::TokenStream>,
    }
    let mut array_fields: HashMap<(&'static str, &'static str), ArrayBoundsInfo> = HashMap::new();
    let mut in_versions = |versions: &[u8],
                           matchable: (&'static str, &'static str),
                           n: Option<proc_macro2::TokenStream>| {
        if versions.iter().any(|v| std::env::var(format!("CARGO_FEATURE_PG{}", v)).is_ok()) {
            array_fields.insert(matchable, ArrayBoundsInfo { n: n });
        }
    };
    in_versions(&[11, 12, 13, 14, 15], ("AggState", "aggcontexts"), Some(quote! { self.numaggs }));
    in_versions(
        &[11, 12, 13, 14, 15],
        ("AppendState", "appendplans"),
        Some(quote! { self.as_nplans }),
    );
    in_versions(&[14, 15], ("AppendState", "as_asyncplans"), Some(quote! { self.as_nasyncplans }));
    in_versions(
        &[14, 15],
        ("AppendState", "as_asyncrequests"),
        Some(quote! { unsafe { bms_num_members(self.as_valid_asyncplans) } }),
    );
    in_versions(
        &[14, 15],
        ("AppendState", "as_asyncresults"),
        Some(quote! { self.as_nasyncresults }),
    );
    in_versions(
        &[11, 12, 13, 14, 15],
        ("BitmapAndState", "bitmapplans"),
        Some(quote! { self.nplans }),
    );
    in_versions(
        &[11, 12, 13, 14, 15],
        ("BitmapOrState", "bitmapplans"),
        Some(quote! { self.nplans }),
    );
    in_versions(
        &[11, 12, 13],
        ("EState", "es_result_relations"),
        Some(quote! { self.es_num_result_relations }),
    );
    in_versions(
        &[14, 15],
        ("EState", "es_result_relations"),
        Some(quote! { self.es_range_table_size }),
    );
    in_versions(
        &[12],
        ("EState", "es_range_table_array"),
        Some(quote! { self.es_range_table_size }),
    );
    in_versions(
        &[12, 13, 14, 15],
        ("EState", "es_rowmarks"),
        Some(quote! { self.es_range_table_size }),
    );
    in_versions(
        &[11, 12, 13, 14, 15],
        ("GatherMergeState", "gm_slots"),
        Some(quote! { self.nreaders + 1 }),
    );
    in_versions(
        &[11, 12, 13, 14, 15],
        ("GatherMergeState", "reader"),
        Some(quote! { self.nreaders }),
    );
    in_versions(&[11, 12, 13, 14, 15], ("GatherState", "reader"), Some(quote! { self.nreaders }));
    in_versions(&[13, 14, 15], ("IndexOptInfo", "opclassoptions"), None); // ignored, just a byte array.
    in_versions(&[14, 15], ("MemoizeState", "param_exprs"), Some(quote! { self.nkeys }));
    in_versions(
        &[11, 12, 13, 14, 15],
        ("MergeAppendState", "mergeplans"),
        Some(quote! { self.ms_nplans }),
    );
    in_versions(
        &[11, 12, 13, 14, 15],
        ("MergeAppendState", "ms_slots"),
        Some(quote! { self.ms_nplans }),
    );
    in_versions(&[11, 12, 13], ("ModifyTableState", "mt_plans"), Some(quote! { self.mt_nplans }));
    in_versions(&[12, 13], ("ModifyTableState", "mt_scans"), Some(quote! { self.mt_nplans }));
    in_versions(
        &[11, 12, 13, 14, 15],
        ("PlannerInfo", "append_rel_array"),
        Some(quote! { self.simple_rel_array_size }),
    );
    // I couldn't figure out how to traverse this list, I'm not sure how long it is.
    in_versions(&[11, 12, 13, 14, 15], ("PlannerInfo", "join_rel_level"), None);
    in_versions(
        &[11, 12, 13, 14, 15],
        ("PlannerInfo", "simple_rel_array"),
        Some(quote! { self.simple_rel_array_size }),
    );
    // in_versions(&[11], ("PlannerInfo", "append_rte_array"), Some(quote! { self.simple_rel_array_size }));
    in_versions(
        &[11, 12, 13, 14, 15],
        ("PlannerInfo", "simple_rte_array"),
        Some(quote! { self.simple_rel_array_size }),
    );
    in_versions(&[11, 12, 13, 14, 15], ("ProjectSetState", "elems"), Some(quote! { self.nelems }));
    in_versions(&[11, 12, 13, 14, 15], ("RelOptInfo", "part_rels"), Some(quote! { self.nparts }));
    in_versions(&[11, 12, 13, 14, 15], ("RelOptInfo", "partexprs"), Some(quote! { self.nparts }));
    in_versions(
        &[11, 12, 13, 14, 15],
        ("RelOptInfo", "nullable_partexprs"),
        Some(quote! { self.nparts }),
    );
    in_versions(
        &[11, 12, 13, 14, 15],
        ("ResultRelInfo", "ri_ConstraintExprs"),
        Some(quote! { unsafe { (*(*(*self.ri_RelationDesc).rd_att).constr).num_check } }),
    );
    in_versions(&[12, 13, 14, 15], ("ResultRelInfo", "ri_GeneratedExprs"), None);
    in_versions(
        &[11, 12, 13, 14, 15],
        ("ResultRelInfo", "ri_IndexRelationInfo"),
        Some(quote! { self.ri_NumIndices }),
    );
    in_versions(
        &[11, 12, 13, 14, 15],
        ("ResultRelInfo", "ri_TrigWhenExprs"),
        Some(quote! { unsafe { *self.ri_TrigDesc }.numtriggers  }),
    );
    in_versions(
        &[14, 15],
        ("ResultRelInfo", "ri_Slots"),
        Some(quote! { self.ri_NumSlotsInitialized }),
    );
    in_versions(
        &[14, 15],
        ("ResultRelInfo", "ri_PlanSlots"),
        Some(quote! { self.ri_NumSlotsInitialized }),
    );
    in_versions(
        &[11, 12, 13, 14, 15],
        ("ValuesScanState", "exprlists"),
        Some(quote! { self.array_len }),
    );
    in_versions(
        &[11, 12, 13, 14, 15],
        ("ValuesScanState", "exprstatelists"),
        Some(quote! { self.array_len }),
    );

    fn handle_length_bounded_array(
        ts: &mut Vec<proc_macro2::TokenStream>,
        n: &proc_macro2::TokenStream,
        array: &str,
    ) {
        let array = Ident::new(array, Span::call_site());
        ts.push(quote! {
            for index in 0..(#n) {
                let item = unsafe { self.#array.offset(index as isize) };
                if !item.is_null() {
                    walker_fn(item as *mut Node, context);
                    Node::traverse::<T>(unsafe { &mut *(item as *mut Node) }, walker_fn, context);
                }
            }
        });
    }

    let mut ptr_problems: Vec<String> = Vec::new();

    // now we can finally iterate the Nodes and emit various trait impls
    for node_struct in &nodes {
        let mut does_walking = false;
        let struct_name = &node_struct.struct_.ident;
        match struct_name.to_string().as_ref() {
            "Node" | "RangeTblEntry" | "List" => {
                // use the special implementation defined above.
                continue;
            }
            _ => {}
        }
        let mut traverse_elements: Vec<proc_macro2::TokenStream> = Vec::new();
        if struct_name != "A_Const" && struct_name != "Value" {
            traverse_elements.push(quote!{
                emit(format!("Preparing to process struct {} {:?} at {}", stringify!(#struct_name), self, &self));
            });
        }

        for field in node_struct.struct_.fields.iter() {
            let field_name = field.ident.as_ref().unwrap();
            println!("Handling {} -> {} of type {:?}", struct_name, field_name, &field.ty);
            // Some structures have array fields in them, rather than Lists. Handle those specially.
            match array_fields
                .get(&(struct_name.to_string().as_ref(), field_name.to_string().as_ref()))
            {
                Some(abi) => {
                    match &abi.n {
                        Some(n) => handle_length_bounded_array(
                            &mut traverse_elements,
                            n,
                            field_name.to_string().as_ref(),
                        ),
                        _ => {}
                    }
                    continue;
                }
                _ => {}
            }
            // Some structures have fields that we need to handle specially because they're not populated, or are Lists
            // that don't contain pointers.
            match (struct_name.to_string().as_ref(), field_name.to_string().as_ref()) {
                ("ModifyTableState", "mt_arowmarks")
                | ("ModifyTableState", "mt_per_subplan_tupconv_maps")
                | (_, "xpr") => continue,
                _ => {}
            }
            match &field.ty {
                Type::Array(a) => {
                    if let Type::Ptr(ptr) = a.elem.as_ref() {
                        if let Type::Path(p) = ptr.elem.as_ref() {
                            if is_node_struct(
                                &p.path.segments.first().unwrap().ident.to_string(),
                                &node_types,
                                &nodes,
                            ) {
                                does_walking = true;
                                traverse_elements.push(quote!{
                                    emit(format!("Traversing array {}::{} with value {:?}",
                                        stringify!(#struct_name), stringify!(#field_name), self.#field_name));
                                    for (index, item) in self.#field_name.iter().enumerate() {
                                        if !item.is_null() {
                                            emit(format!("Traversing array:item {}::{}[{}] with value {:?}",
                                                stringify!(#struct_name), stringify!(#field_name), index, item));
                                            walker_fn(item.clone() as *mut Node, context);
                                            Node::traverse::<T>( unsafe { &mut *(*item as *mut Node) }, walker_fn, context);
                                        }
                                    }
                                });
                            }
                        } else {
                            ptr_problems.push(format!(
                                "Unexpected type inside array:ptr {} -> field {}: {:?}",
                                struct_name, field_name, ptr.elem
                            ));
                        }
                    }
                }
                Type::Ptr(t) => {
                    if let Type::Path(p) = t.elem.as_ref() {
                        if is_node_struct(
                            &p.path.segments.first().unwrap().ident.to_string(),
                            &node_types,
                            &nodes,
                        ) {
                            let debug_log = if field_name.to_string() == "extname" {
                                quote! {}
                            } else {
                                quote! {
                                    emit(format!("Traversing ptr:item {}::{} at {:?}",
                                        stringify!(#struct_name), stringify!(#field_name), self.#field_name));
                                }
                            };
                            does_walking = true;
                            traverse_elements.push(quote!{
                                if !self.#field_name.is_null() {
                                    #debug_log
                                    walker_fn(self.#field_name as *mut Node, context);
                                    Node::traverse::<T>(unsafe { &mut *(self.#field_name as *mut Node) }, walker_fn, context);
                                }
                            });
                        }
                    } else {
                        ptr_problems.push(format!(
                            "Unexpected type inside ptr {} -> field {}: {:?}",
                            struct_name, field_name, t.elem
                        ));
                    }
                }
                Type::Path(p) => {
                    if is_node_struct(
                        &p.path.segments.first().unwrap().ident.to_string(),
                        &node_types,
                        &nodes,
                    ) {
                        let type_ = &p.path;
                        does_walking = true;
                        if struct_name != "A_Const" {
                            traverse_elements.push(quote!{
                                emit(format!("Traversing item {}::{} with value {:?}",
                                    stringify!(#struct_name), stringify!(#field_name), self.#field_name));
                            });
                        }
                        traverse_elements.push(quote! {
                            walker_fn((&mut self.#field_name) as *mut #type_ as *mut Node, context);
                            // Explicitly don't look at _type here - for example, a Result has a concrete Plan as its
                            // first member, but has _type == NodeTag_T_Result so if we delegated to `Node::traverse`
                            // we'd be recursing forever.
                            self.#field_name.traverse(walker_fn, context);
                        });
                    }
                }
                _ => panic!("In {}: Don't know how to handle {:?}", struct_name, field.ty),
            }
        }
        // impl the PgNode trait for all nodes
        let mut traversal = proc_macro2::TokenStream::new();
        for item in traverse_elements {
            traversal.extend(item);
        }
        pgnode_impls.extend(if !does_walking {
            quote! {
                impl pg_sys::seal::Sealed for #struct_name {}
                impl pg_sys::PgNode for #struct_name {
                    fn traverse<T>(&mut self, _walker_fn: fn(*mut Node, &mut T) -> (), _context: &mut T) {
                        emit(format!("Empty traverse function for {}", stringify!(#struct_name)));
                        #traversal
                    }
                }
            }
        } else {
            quote! {
                impl pg_sys::seal::Sealed for #struct_name {}
                impl pg_sys::PgNode for #struct_name {
                    fn traverse<T>(&mut self, walker_fn: fn(*mut Node, &mut T) -> (), context: &mut T) {
                        emit(format!("Begin traverse function for {}", stringify!(#struct_name)));
                        #traversal
                        emit(format!("Finish traverse function for {}", stringify!(#struct_name)));
                    }
                }
            }
        });

        // impl Rust's Display trait for all nodes
        pgnode_impls.extend(quote! {
            impl std::fmt::Display for #struct_name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", self.display_node() )
                }
            }
        });
    }
    if !ptr_problems.is_empty() {
        panic!("{}", ptr_problems.join("\n"));
    }

    Ok(pgnode_impls)
}

/// Given a root node, dfs_find_nodes adds all its children nodes to `node_set`.
fn dfs_find_nodes<'graph>(
    node: &'graph StructDescriptor<'graph>,
    graph: &'graph StructGraph<'graph>,
    node_set: &mut HashSet<StructDescriptor<'graph>>,
) {
    node_set.insert(node.clone());

    for child in node.children(graph) {
        if node_set.contains(child) {
            continue;
        }
        dfs_find_nodes(child, graph, node_set);
    }
}

/// A graph describing the inheritance relationships between different nodes
/// according to postgres' object system.
///
/// NOTE: the borrowed lifetime on a StructGraph should also ensure that the offsets
///       it stores into the underlying items struct are always correct.
#[derive(Clone, Debug)]
struct StructGraph<'a> {
    #[allow(dead_code)]
    /// A table mapping struct names to their offset in the descriptor table
    name_tab: HashMap<String, usize>,
    #[allow(dead_code)]
    /// A table mapping offsets into the underlying items table to offsets in the descriptor table
    item_offset_tab: Vec<Option<usize>>,
    /// A table of struct descriptors
    descriptors: Vec<StructDescriptor<'a>>,
}

impl<'a> From<&'a [syn::Item]> for StructGraph<'a> {
    fn from(items: &'a [syn::Item]) -> StructGraph<'a> {
        let mut descriptors = Vec::new();

        // a table mapping struct names to their offset in `descriptors`
        let mut name_tab: HashMap<String, usize> = HashMap::new();
        let mut item_offset_tab: Vec<Option<usize>> = vec![None; items.len()];
        for (i, item) in items.iter().enumerate() {
            if let &syn::Item::Struct(struct_) = &item {
                let next_offset = descriptors.len();
                descriptors.push(StructDescriptor {
                    struct_,
                    items_offset: i,
                    parent: None,
                    children: Vec::new(),
                });
                name_tab.insert(struct_.ident.to_string(), next_offset);
                item_offset_tab[i] = Some(next_offset);
            }
        }

        for item in items.iter() {
            // grab the first field if it is struct
            let (id, first_field) = match &item {
                &syn::Item::Struct(syn::ItemStruct {
                    ident: id,
                    fields: syn::Fields::Named(fields),
                    ..
                }) => {
                    if let Some(first_field) = fields.named.first() {
                        (id.to_string(), first_field)
                    } else {
                        continue;
                    }
                }
                &syn::Item::Struct(syn::ItemStruct {
                    ident: id,
                    fields: syn::Fields::Unnamed(fields),
                    ..
                }) => {
                    if let Some(first_field) = fields.unnamed.first() {
                        (id.to_string(), first_field)
                    } else {
                        continue;
                    }
                }
                _ => continue,
            };

            if let syn::Type::Path(p) = &first_field.ty {
                // We should be guaranteed that just extracting the last path
                // segment is ok because these structs are all from the same module.
                // (also, they are all generated from C code, so collisions should be
                //  impossible anyway thanks to C's single shared namespace).
                if let Some(last_segment) = p.path.segments.last() {
                    if let Some(parent_offset) = name_tab.get(&last_segment.ident.to_string()) {
                        // establish the 2-way link
                        let child_offset = name_tab[&id];
                        descriptors[child_offset].parent = Some(*parent_offset);
                        descriptors[*parent_offset].children.push(child_offset);
                    }
                }
            }
        }

        StructGraph { name_tab, item_offset_tab, descriptors }
    }
}

impl<'a> StructDescriptor<'a> {
    /// children returns an iterator over the children of this node in the graph
    fn children(&'a self, graph: &'a StructGraph) -> StructDescriptorChildren {
        StructDescriptorChildren { offset: 0, descriptor: self, graph }
    }
}

/// An iterator over a StructDescriptor's children
struct StructDescriptorChildren<'a> {
    offset: usize,
    descriptor: &'a StructDescriptor<'a>,
    graph: &'a StructGraph<'a>,
}

impl<'a> std::iter::Iterator for StructDescriptorChildren<'a> {
    type Item = &'a StructDescriptor<'a>;
    fn next(&mut self) -> Option<&'a StructDescriptor<'a>> {
        if self.offset >= self.descriptor.children.len() {
            None
        } else {
            let ret = Some(&self.graph.descriptors[self.descriptor.children[self.offset]]);
            self.offset += 1;
            ret
        }
    }
}

/// A node a StructGraph
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct StructDescriptor<'a> {
    /// A reference to the underlying struct syntax node
    struct_: &'a syn::ItemStruct,
    /// An offset into the items slice that was used to construct the struct graph that
    /// this StructDescriptor is a part of
    items_offset: usize,
    /// The offset of the "parent" (first member) struct (if any).
    parent: Option<usize>,
    /// The offsets of the "children" structs (if any).
    children: Vec<usize>,
}

fn get_bindings(
    major_version: u16,
    pg_config: &PgConfig,
    include_h: &PathBuf,
) -> eyre::Result<syn::File> {
    let bindings = if let Some(info_dir) =
        target_env_tracked(&format!("PGX_TARGET_INFO_PATH_PG{major_version}"))
    {
        let bindings_file = format!("{info_dir}/pg{major_version}_raw_bindings.rs");
        std::fs::read_to_string(&bindings_file)
            .wrap_err_with(|| format!("failed to read raw bindings from {bindings_file}"))?
    } else {
        let bindings = run_bindgen(major_version, pg_config, include_h)?;
        if let Some(path) = env_tracked("PGX_PG_SYS_EXTRA_OUTPUT_PATH") {
            std::fs::write(&path, &bindings)?;
        }
        bindings
    };
    syn::parse_file(bindings.as_str()).wrap_err_with(|| "failed to parse generated bindings")
}

/// Given a specific postgres version, `run_bindgen` generates bindings for the given
/// postgres version and returns them as a token stream.
fn run_bindgen(
    major_version: u16,
    pg_config: &PgConfig,
    include_h: &PathBuf,
) -> eyre::Result<String> {
    eprintln!("Generating bindings for pg{major_version}");
    let bindings = bindgen::Builder::default()
        .header(include_h.display().to_string())
        .clang_args(&extra_bindgen_clang_args(pg_config)?)
        .clang_args(pg_target_include_flags(major_version, pg_config)?)
        .detect_include_paths(target_env_tracked("PGX_BINDGEN_NO_DETECT_INCLUDES").is_none())
        .parse_callbacks(Box::new(PgxOverrides::default()))
        .blocklist_type("(Nullable)?Datum") // manually wrapping datum types for correctness
        .blocklist_type("Oid") // "Oid" is not just any u32
        .blocklist_function("varsize_any") // pgx converts the VARSIZE_ANY macro, so we don't want to also have this function, which is in heaptuple.c
        .blocklist_function("(?:query|expression)_tree_walker")
        .blocklist_function(".*(?:set|long)jmp")
        .blocklist_function("pg_re_throw")
        .blocklist_function("errstart")
        .blocklist_function("errcode")
        .blocklist_function("errmsg")
        .blocklist_function("errdetail")
        .blocklist_function("errcontext_msg")
        .blocklist_function("errhint")
        .blocklist_function("errfinish")
        .blocklist_item("CONFIGURE_ARGS") // configuration during build is hopefully irrelevant
        .blocklist_item("_*(?:HAVE|have)_.*") // header tracking metadata
        .blocklist_item("_[A-Z_]+_H") // more header metadata
        .blocklist_item("__[A-Z].*") // these are reserved and unused by Postgres
        .blocklist_item("__darwin.*") // this should always be Apple's names
        .blocklist_function("pq(?:Strerror|Get.*)") // wrappers around platform functions: user can call those themselves
        .blocklist_function("log")
        .blocklist_item(".*pthread.*)") // shims for pthreads on non-pthread systems, just use std::thread
        .blocklist_item(".*(?i:va)_(?i:list|start|end|copy).*") // do not need va_list anything!
        .blocklist_function("(?:pg_|p)v(?:sn?|f)?printf")
        .blocklist_function("appendStringInfoVA")
        .blocklist_file("stdarg.h")
        // these cause cause warnings, errors, or deprecations on some systems,
        // and are not useful for us.
        .blocklist_function("(?:sigstack|sigreturn|siggetmask|gets|vfork|te?mpnam(?:_r)?|mktemp)")
        // Missing on some systems, despite being in their headers.
        .blocklist_function("inet_net_pton.*")
        .size_t_is_usize(true)
        .rustfmt_bindings(false)
        .derive_debug(true)
        .derive_copy(true) // necessary to avoid __BindgenUnionField usages -- I don't understand why?
        .derive_default(true)
        .derive_eq(false)
        .derive_partialeq(false)
        .derive_hash(false)
        .derive_ord(false)
        .derive_partialord(false)
        .layout_tests(false)
        .generate()
        .wrap_err_with(|| format!("Unable to generate bindings for pg{}", major_version))?;

    Ok(bindings.to_string())
}

fn env_tracked(s: &str) -> Option<String> {
    println!("cargo:rerun-if-env-changed={s}");
    std::env::var(s).ok()
}

fn target_env_tracked(s: &str) -> Option<String> {
    let target = std::env::var("TARGET").unwrap();
    env_tracked(&format!("{s}_{target}")).or_else(|| env_tracked(s))
}

/// Returns `Err` if `pg_config` errored, `None` if we should
fn pg_target_include_flags(pg_version: u16, pg_config: &PgConfig) -> eyre::Result<Option<String>> {
    let var = "PGX_INCLUDEDIR_SERVER";
    let value =
        target_env_tracked(&format!("{var}_PG{pg_version}")).or_else(|| target_env_tracked(var));
    match value {
        // No configured value: ask `pg_config`.
        None => Ok(Some(format!("-I{}", pg_config.includedir_server()?.display()))),
        // Configured to empty string: assume bindgen is getting it some other
        // way, pass nothing.
        Some(overridden) if overridden.is_empty() => Ok(None),
        // Configured to non-empty string: pass to bindgen
        Some(overridden) => Ok(Some(format!("-I{overridden}"))),
    }
}

fn build_shim(shim_src: &PathBuf, shim_dst: &PathBuf, pg_config: &PgConfig) -> eyre::Result<()> {
    let major_version = pg_config.major_version()?;
    let mut libpgx_cshim: PathBuf = shim_dst.clone();

    libpgx_cshim.push(format!("libpgx-cshim-{}.a", major_version));

    eprintln!("libpgx_cshim={}", libpgx_cshim.display());
    // then build the shim for the version feature currently being built
    build_shim_for_version(&shim_src, &shim_dst, pg_config)?;

    // no matter what, tell rustc to link to the library that was built for the feature we're currently building
    let envvar_name = format!("CARGO_FEATURE_PG{}", major_version);
    if std::env::var(envvar_name).is_ok() {
        println!("cargo:rustc-link-search={}", shim_dst.display());
        println!("cargo:rustc-link-lib=static=pgx-cshim-{}", major_version);
    }

    Ok(())
}

fn build_shim_for_version(
    shim_src: &PathBuf,
    shim_dst: &PathBuf,
    pg_config: &PgConfig,
) -> eyre::Result<()> {
    let path_env = prefix_path(pg_config.parent_path());
    let major_version = pg_config.major_version()?;

    eprintln!("PATH for build_shim={}", path_env);
    eprintln!("shim_src={}", shim_src.display());
    eprintln!("shim_dst={}", shim_dst.display());

    std::fs::create_dir_all(shim_dst).unwrap();

    if !std::path::Path::new(&format!("{}/Makefile", shim_dst.display())).exists() {
        std::fs::copy(
            format!("{}/Makefile", shim_src.display()),
            format!("{}/Makefile", shim_dst.display()),
        )
        .unwrap();
    }

    if !std::path::Path::new(&format!("{}/pgx-cshim.c", shim_dst.display())).exists() {
        std::fs::copy(
            format!("{}/pgx-cshim.c", shim_src.display()),
            format!("{}/pgx-cshim.c", shim_dst.display()),
        )
        .unwrap();
    }

    let make = option_env!("MAKE").unwrap_or("make").to_string();
    let rc = run_command(
        Command::new(make)
            .arg("clean")
            .arg(&format!("libpgx-cshim-{}.a", major_version))
            .env("PG_TARGET_VERSION", format!("{}", major_version))
            .env("PATH", path_env)
            .current_dir(shim_dst),
        &format!("shim for PG v{}", major_version),
    )?;

    if rc.status.code().unwrap() != 0 {
        return Err(eyre!("failed to make pgx-cshim for v{}", major_version));
    }

    Ok(())
}

fn extra_bindgen_clang_args(pg_config: &PgConfig) -> eyre::Result<Vec<String>> {
    let mut out = vec![];
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() == "macos" {
        // On macOS, find the `-isysroot` arg out of the c preprocessor flags,
        // to handle the case where bindgen uses a libclang isn't provided by
        // the system.
        let flags = pg_config.cppflags()?;
        // In practice this will always be valid UTF-8 because of how the
        // `pgx-pg-config` crate is implemented, but even if it were not, the
        // problem won't be with flags we are interested in.
        let flags = shlex::split(&flags.to_string_lossy()).unwrap_or_default();
        // Find the `-isysroot` flags -- The rest are `-I` flags that don't seem
        // to be needed inside the code (and feel likely to cause bindgen to
        // emit bindings for unrelated libraries)
        for pair in flags.windows(2) {
            if pair[0] == "-isysroot" {
                if std::path::Path::new(&pair[1]).exists() {
                    out.extend(pair.into_iter().cloned());
                } else {
                    // The SDK path doesn't exist. Emit a warning, which they'll
                    // see if the build ends up failing (it may not fail in all
                    // cases, so we don't panic here).
                    //
                    // There's a bunch of smarter things we can try here, but
                    // most of them either break things that currently work, or
                    // are very difficult to get right. If you try to fix this,
                    // be sure to consider cases like:
                    //
                    // - User may have CommandLineTools and not Xcode, vice
                    //   versa, or both installed.
                    // - User may using a newer SDK than their OS, or vice
                    //   versa.
                    // - User may be using a newer SDK than their XCode (updated
                    //   Command line tools, not OS), or vice versa.
                    // - And so on.
                    //
                    // These are all actually fairly common. Note that the code
                    // as-is is *not* broken in these cases (except on OS/SDK
                    // updates), so care should be taken to avoid changing that
                    // if possible.
                    //
                    // The logic we'd like ideally is for `cargo pgx init` to
                    // choose a good SDK in the first place, and force postgres
                    // to use it. Then, the logic in this build script would
                    // Just Work without changes (since we are using its
                    // sysroot verbatim).
                    //
                    // The value of "Good" here is tricky, but the logic should
                    // probably:
                    //
                    // - prefer SDKs from the CLI tools to ones from XCode
                    //   (since they're guaranteed compatible with the user's OS
                    //   version)
                    //
                    // - prefer SDKs that specify only the major SDK version
                    //   (e.g. MacOSX12.sdk and not MacOSX12.4.sdk or
                    //   MacOSX.sdk), to avoid breaking too frequently (if we
                    //   have a minor version) or being totally unable to detect
                    //   what version of the SDK was used to build postgres (if
                    //   we have neither).
                    //
                    // - Avoid choosing an SDK newer than the user's OS version,
                    //   since postgres fails to detect that they are missing if
                    //   you do.
                    //
                    // This is surprisingly hard to implement, as the
                    // information is scattered across a dozen ini files.
                    // Presumably Apple assumes you'll use
                    // `MACOSX_DEPLOYMENT_TARGET`, rather than basing it off the
                    // SDK version, but it's not an option for postgres.
                    let major_version = pg_config.major_version()?;
                    println!(
                        "cargo:warning=postgres v{major_version} was compiled against an \
                         SDK Root which does not seem to exist on this machine ({}). You may \
                         need to re-run `cargo pgx init` and/or update your command line tools.",
                        pair[1],
                    );
                };
                // Either way, we stop here.
                break;
            }
        }
    }
    Ok(out)
}

fn run_command(mut command: &mut Command, version: &str) -> eyre::Result<Output> {
    let mut dbg = String::new();

    command = command
        .env_remove("DEBUG")
        .env_remove("MAKEFLAGS")
        .env_remove("MAKELEVEL")
        .env_remove("MFLAGS")
        .env_remove("DYLD_FALLBACK_LIBRARY_PATH")
        .env_remove("OPT_LEVEL")
        .env_remove("TARGET")
        .env_remove("PROFILE")
        .env_remove("OUT_DIR")
        .env_remove("HOST")
        .env_remove("NUM_JOBS");

    eprintln!("[{}] {:?}", version, command);
    dbg.push_str(&format!("[{}] -------- {:?} -------- \n", version, command));

    let output = command.output()?;
    let rc = output.clone();

    if !output.stdout.is_empty() {
        for line in String::from_utf8(output.stdout).unwrap().lines() {
            if line.starts_with("cargo:") {
                dbg.push_str(&format!("{}\n", line));
            } else {
                dbg.push_str(&format!("[{}] [stdout] {}\n", version, line));
            }
        }
    }

    if !output.stderr.is_empty() {
        for line in String::from_utf8(output.stderr).unwrap().lines() {
            dbg.push_str(&format!("[{}] [stderr] {}\n", version, line));
        }
    }
    dbg.push_str(&format!("[{}] /----------------------------------------\n", version));

    eprintln!("{}", dbg);
    Ok(rc)
}

// Plausibly it would be better to generate a regex to pass to bindgen for this,
// but this is less error-prone for now.
static BLOCKLISTED: Lazy<BTreeSet<&'static str>> =
    Lazy::new(|| build::sym_blocklist::SYMBOLS.iter().copied().collect::<BTreeSet<&str>>());
fn is_blocklisted_item(item: &ForeignItem) -> bool {
    let sym_name = match item {
        ForeignItem::Fn(f) => &f.sig.ident,
        // We don't *need* to filter statics too (only functions), but it
        // doesn't hurt.
        ForeignItem::Static(s) => &s.ident,
        _ => return false,
    };
    BLOCKLISTED.contains(sym_name.to_string().as_str())
}

fn apply_pg_guard(items: &Vec<syn::Item>) -> eyre::Result<proc_macro2::TokenStream> {
    let mut out = proc_macro2::TokenStream::new();
    for item in items {
        match item {
            Item::ForeignMod(block) => {
                let abi = &block.abi;
                for item in &block.items {
                    if is_blocklisted_item(item) {
                        continue;
                    }
                    match item {
                        ForeignItem::Fn(func) => {
                            out.extend(quote! {
                                #[pgx_macros::pg_guard]
                                #abi { #func }
                            });
                        }
                        other => out.extend(quote! { #abi { #other } }),
                    }
                }
            }
            _ => {
                out.extend(item.into_token_stream());
            }
        }
    }

    Ok(out)
}

fn rust_fmt(path: &PathBuf) -> eyre::Result<()> {
    let out = run_command(Command::new("rustfmt").arg(path).current_dir("."), "[bindings_diff]");
    match out {
        Ok(_) => Ok(()),
        Err(e)
            if e.downcast_ref::<std::io::Error>()
                .ok_or(eyre!("Couldn't downcast error ref"))?
                .kind()
                == std::io::ErrorKind::NotFound =>
        {
            Err(e).wrap_err("Failed to run `rustfmt`, is it installed?")
        }
        Err(e) => Err(e.into()),
    }
}
