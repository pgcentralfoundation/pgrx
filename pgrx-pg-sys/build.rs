//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use bindgen::callbacks::{DeriveTrait, ImplementsTrait, MacroParsingBehavior};
use bindgen::NonCopyUnionStyle;
use eyre::{eyre, WrapErr};
use pgrx_pg_config::{
    is_supported_major_version, prefix_path, PgConfig, PgConfigSelector, Pgrx, SUPPORTED_VERSIONS,
};
use quote::{quote, ToTokens};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs;
use std::path::{self, Path, PathBuf}; // disambiguate path::Path and syn::Type::Path
use std::process::{Command, Output};
use std::sync::OnceLock;
use syn::{ForeignItem, Item, ItemConst};

const BLOCKLISTED_TYPES: [&str; 3] = ["Datum", "NullableDatum", "Oid"];

mod build {
    pub(super) mod clang;
    pub(super) mod sym_blocklist;
}

#[derive(Debug)]
struct PgrxOverrides(HashSet<String>);

impl PgrxOverrides {
    fn default() -> Self {
        // these cause duplicate definition problems on linux
        // see: https://github.com/rust-lang/rust-bindgen/issues/687
        PgrxOverrides(
            vec![
                "FP_INFINITE".into(),
                "FP_NAN".into(),
                "FP_NORMAL".into(),
                "FP_SUBNORMAL".into(),
                "FP_ZERO".into(),
                "IPPORT_RESERVED".into(),
                // These are just annoying due to clippy
                "M_E".into(),
                "M_LOG2E".into(),
                "M_LOG10E".into(),
                "M_LN2".into(),
                "M_LN10".into(),
                "M_PI".into(),
                "M_PI_2".into(),
                "M_PI_4".into(),
                "M_1_PI".into(),
                "M_2_PI".into(),
                "M_SQRT2".into(),
                "M_SQRT1_2".into(),
                "M_2_SQRTPI".into(),
            ]
            .into_iter()
            .collect(),
        )
    }
}

impl bindgen::callbacks::ParseCallbacks for PgrxOverrides {
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
    if env_tracked("DOCS_RS").as_deref() == Some("1") {
        return Ok(());
    }

    // dump the environment for debugging if asked
    if env_tracked("PGRX_BUILD_VERBOSE").as_deref() == Some("true") {
        for (k, v) in std::env::vars() {
            eprintln!("{k}={v}");
        }
    }

    let compile_cshim = env_tracked("CARGO_FEATURE_CSHIM").as_deref() == Some("1");

    let is_for_release =
        env_tracked("PGRX_PG_SYS_GENERATE_BINDINGS_FOR_RELEASE").as_deref() == Some("1");

    let build_paths = BuildPaths::from_env();

    eprintln!("build_paths={build_paths:?}");

    emit_rerun_if_changed();

    let pg_configs: Vec<(u16, PgConfig)> = if is_for_release {
        // This does not cross-check config.toml and Cargo.toml versions, as it is release infra.
        Pgrx::from_config()?.iter(PgConfigSelector::All)
            .map(|r| r.expect("invalid pg_config"))
            .map(|c| (c.major_version().expect("invalid major version"), c))
            .filter_map(|t| {
                if is_supported_major_version(t.0) {
                    Some(t)
                } else {
                    println!(
                        "cargo:warning={} contains a configuration for pg{}, which pgrx does not support.",
                        Pgrx::config_toml()
                            .expect("Could not get PGRX configuration TOML")
                            .to_string_lossy(),
                        t.0
                    );
                    None
                }
            })
            .collect()
    } else {
        let mut found = Vec::new();
        for pgver in SUPPORTED_VERSIONS() {
            if env_tracked(&format!("CARGO_FEATURE_PG{}", pgver.major)).is_some() {
                found.push(pgver);
            }
        }
        let found_ver = match &found[..] {
            [ver] => ver,
            [] => {
                return Err(eyre!(
                    "Did not find `pg$VERSION` feature. `pgrx-pg-sys` requires one of {} to be set",
                    SUPPORTED_VERSIONS()
                        .iter()
                        .map(|pgver| format!("`pg{}`", pgver.major))
                        .collect::<Vec<_>>()
                        .join(", ")
                ))
            }
            versions => {
                return Err(eyre!(
                    "Multiple `pg$VERSION` features found.\n`--no-default-features` may be required.\nFound: {}",
                    versions
                        .iter()
                        .map(|version| format!("pg{}", version.major))
                        .collect::<Vec<String>>()
                        .join(", ")
                ))
            }
        };
        let found_major = found_ver.major;
        if let Ok(pg_config) = PgConfig::from_env() {
            let major_version = pg_config.major_version()?;

            if major_version != found_major {
                panic!("Feature flag `pg{found_major}` does not match version from the environment-described PgConfig (`{major_version}`)")
            }
            vec![(major_version, pg_config)]
        } else {
            let specific = Pgrx::from_config()?.get(&format!("pg{}", found_ver.major))?;
            vec![(found_ver.major, specific)]
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

fn emit_rerun_if_changed() {
    // `pgrx-pg-config` doesn't emit one for this.
    println!("cargo:rerun-if-env-changed=PGRX_PG_CONFIG_PATH");
    println!("cargo:rerun-if-env-changed=PGRX_PG_CONFIG_AS_ENV");
    // Bindgen's behavior depends on these vars, but it doesn't emit them
    // directly because the output would cause issue with `bindgen-cli`. Do it
    // on bindgen's behalf.
    println!("cargo:rerun-if-env-changed=LLVM_CONFIG_PATH");
    println!("cargo:rerun-if-env-changed=LIBCLANG_PATH");
    println!("cargo:rerun-if-env-changed=LIBCLANG_STATIC_PATH");
    // Follows the logic bindgen uses here, more or less.
    // https://github.com/rust-lang/rust-bindgen/blob/e6dd2c636/bindgen/lib.rs#L2918
    println!("cargo:rerun-if-env-changed=BINDGEN_EXTRA_CLANG_ARGS");
    if let Some(target) = env_tracked("TARGET") {
        println!("cargo:rerun-if-env-changed=BINDGEN_EXTRA_CLANG_ARGS_{target}");
        println!(
            "cargo:rerun-if-env-changed=BINDGEN_EXTRA_CLANG_ARGS_{}",
            target.replace('-', "_"),
        );
    }

    // don't want to get stuck always generating bindings
    println!("cargo:rerun-if-env-changed=PGRX_PG_SYS_GENERATE_BINDINGS_FOR_RELEASE");

    println!("cargo:rerun-if-changed=include");
    println!("cargo:rerun-if-changed=cshim");

    if let Ok(pgrx_config) = Pgrx::config_toml() {
        println!("cargo:rerun-if-changed={}", pgrx_config.display());
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
    include_h.push(format!("pg{major_version}.h"));

    let bindgen_output = get_bindings(major_version, pg_config, &include_h)
        .wrap_err_with(|| format!("bindgen failed for pg{major_version}"))?;

    let oids = extract_oids(&bindgen_output);
    let rewritten_items = rewrite_items(&bindgen_output, &oids)
        .wrap_err_with(|| format!("failed to rewrite items for pg{major_version}"))?;
    let oids = format_builtin_oid_impl(oids);

    let dest_dirs = if is_for_release {
        vec![build_paths.out_dir.clone(), build_paths.src_dir.clone()]
    } else {
        vec![build_paths.out_dir.clone()]
    };
    for dest_dir in dest_dirs {
        let mut bindings_file = dest_dir.clone();
        bindings_file.push(&format!("pg{major_version}.rs"));
        write_rs_file(
            rewritten_items.clone(),
            &bindings_file,
            quote! {
                use crate as pg_sys;
                use crate::{Datum, Oid, PgNode};
            },
            is_for_release,
        )
        .wrap_err_with(|| {
            format!(
                "Unable to write bindings file for pg{} to `{}`",
                major_version,
                bindings_file.display()
            )
        })?;

        let mut oids_file = dest_dir.clone();
        oids_file.push(&format!("pg{major_version}_oids.rs"));
        write_rs_file(oids.clone(), &oids_file, quote! {}, is_for_release).wrap_err_with(|| {
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
        let manifest_dir = env_tracked("CARGO_MANIFEST_DIR").map(PathBuf::from).unwrap();
        let out_dir = env_tracked("OUT_DIR").map(PathBuf::from).unwrap();
        Self {
            src_dir: manifest_dir.join("src/include"),
            shim_src: manifest_dir.join("cshim"),
            shim_dst: out_dir.join("cshim"),
            out_dir,
            manifest_dir,
        }
    }
}

fn write_rs_file(
    code: proc_macro2::TokenStream,
    file_path: &Path,
    header: proc_macro2::TokenStream,
    is_for_release: bool,
) -> eyre::Result<()> {
    use std::io::Write;
    let mut contents = header;
    contents.extend(code);
    let mut file = fs::File::create(file_path)?;
    write!(file, "/* Automatically generated by bindgen. Do not hand-edit.")?;
    if is_for_release {
        write!(
            file,
            "\n
        This code is generated for documentation purposes, so that it is
        easy to reference on docs.rs. Bindings are regenerated for your
        build of pgrx, and the values of your Postgres version may differ.
        */"
        )
    } else {
        write!(file, " */")
    }?;
    write!(file, "{contents}")?;
    rust_fmt(file_path)
}

/// Given a token stream representing a file, apply a series of transformations to munge
/// the bindgen generated code with some postgres specific enhancements
fn rewrite_items(
    file: &syn::File,
    oids: &BTreeMap<syn::Ident, Box<syn::Expr>>,
) -> eyre::Result<proc_macro2::TokenStream> {
    let items_vec = rewrite_oid_consts(&file.items, oids);
    let mut items = apply_pg_guard(&items_vec)?;
    let pgnode_impls = impl_pg_node(&items_vec)?;

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
        let Item::Const(ItemConst { ident, ty, expr, .. }) = item else { continue };
        // Retype as strings for easy comparison
        let name = ident.to_string();
        let ty_str = ty.to_token_stream().to_string();

        // This heuristic identifies "OIDs"
        // We're going to warp the const declarations to be our newtype Oid
        if ty_str == "u32" && is_builtin_oid(&name) {
            oids.insert(ident.clone(), expr.clone());
        }
    }
    oids
}

fn is_builtin_oid(name: &str) -> bool {
    name.ends_with("OID") && name != "HEAP_HASOID"
        || name.ends_with("RelationId")
        || name == "TemplateDbOid"
}

fn rewrite_oid_consts(
    items: &[syn::Item],
    oids: &BTreeMap<syn::Ident, Box<syn::Expr>>,
) -> Vec<syn::Item> {
    items
        .iter()
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

fn format_builtin_oid_impl(oids: BTreeMap<syn::Ident, Box<syn::Expr>>) -> proc_macro2::TokenStream {
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

/// Implement our `PgNode` marker trait for `pg_sys::Node` and its "subclasses"
fn impl_pg_node(items: &[syn::Item]) -> eyre::Result<proc_macro2::TokenStream> {
    let mut pgnode_impls = proc_macro2::TokenStream::new();

    // we scope must of the computation so we can borrow `items` and then
    // extend it at the very end.
    let struct_graph: StructGraph = StructGraph::from(items);

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
    let mut node_set = BTreeSet::new();
    // fill in any children of the roots with a recursive DFS
    // (we are not operating on user input, so it is ok to just
    //  use direct recursion rather than an explicit stack).
    for root in root_node_structs.into_iter() {
        dfs_find_nodes(root, &struct_graph, &mut node_set);
    }

    // now we can finally iterate the Nodes and emit out Display impl
    for node_struct in node_set.into_iter() {
        let struct_name = &node_struct.struct_.ident;

        // impl the PgNode trait for all nodes
        pgnode_impls.extend(quote! {
            impl pg_sys::seal::Sealed for #struct_name {}
            impl pg_sys::PgNode for #struct_name {}
        });

        // impl Rust's Display trait for all nodes
        pgnode_impls.extend(quote! {
            impl ::core::fmt::Display for #struct_name {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    self.display_node().fmt(f)
                }
            }
        });
    }

    Ok(pgnode_impls)
}

/// Given a root node, dfs_find_nodes adds all its children nodes to `node_set`.
fn dfs_find_nodes<'graph>(
    node: &'graph StructDescriptor<'graph>,
    graph: &'graph StructGraph<'graph>,
    node_set: &mut BTreeSet<StructDescriptor<'graph>>,
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
                syn::Item::Struct(syn::ItemStruct {
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

impl PartialOrd for StructDescriptor<'_> {
    #[inline]
    fn partial_cmp(&self, other: &StructDescriptor) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for StructDescriptor<'_> {
    #[inline]
    fn cmp(&self, other: &StructDescriptor) -> Ordering {
        self.struct_.ident.cmp(&other.struct_.ident)
    }
}

fn get_bindings(
    major_version: u16,
    pg_config: &PgConfig,
    include_h: &path::Path,
) -> eyre::Result<syn::File> {
    let bindings = if let Some(info_dir) =
        target_env_tracked(&format!("PGRX_TARGET_INFO_PATH_PG{major_version}"))
    {
        let bindings_file = format!("{info_dir}/pg{major_version}_raw_bindings.rs");
        std::fs::read_to_string(&bindings_file)
            .wrap_err_with(|| format!("failed to read raw bindings from {bindings_file}"))?
    } else {
        let bindings = run_bindgen(major_version, pg_config, include_h)?;
        if let Some(path) = env_tracked("PGRX_PG_SYS_EXTRA_OUTPUT_PATH") {
            std::fs::write(path, &bindings)?;
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
    include_h: &path::Path,
) -> eyre::Result<String> {
    eprintln!("Generating bindings for pg{major_version}");
    let configure = pg_config.configure()?;
    let preferred_clang: Option<&std::path::Path> = configure.get("CLANG").map(|s| s.as_ref());
    eprintln!("pg_config --configure CLANG = {preferred_clang:?}");
    let (autodetect, includes) = build::clang::detect_include_paths_for(preferred_clang);
    let mut binder = bindgen::Builder::default();
    binder = add_blocklists(binder);
    binder = add_derives(binder);
    if !autodetect {
        let builtin_includes = includes.iter().filter_map(|p| Some(format!("-I{}", p.to_str()?)));
        binder = binder.clang_args(builtin_includes);
    };
    let bindings = binder
        .header(include_h.display().to_string())
        .clang_args(extra_bindgen_clang_args(pg_config)?)
        .clang_args(pg_target_include_flags(major_version, pg_config)?)
        .detect_include_paths(autodetect)
        .parse_callbacks(Box::new(PgrxOverrides::default()))
        // The NodeTag enum is closed: additions break existing values in the set, so it is not extensible
        .rustified_non_exhaustive_enum("NodeTag")
        .size_t_is_usize(true)
        .merge_extern_blocks(true)
        .formatter(bindgen::Formatter::None)
        .layout_tests(false)
        .default_non_copy_union_style(NonCopyUnionStyle::ManuallyDrop)
        .generate()
        .wrap_err_with(|| format!("Unable to generate bindings for pg{major_version}"))?;

    Ok(bindings.to_string())
}

fn add_blocklists(bind: bindgen::Builder) -> bindgen::Builder {
    bind.blocklist_type("Datum") // manually wrapping datum for correctness
        .blocklist_type("Oid") // "Oid" is not just any u32
        .blocklist_function("varsize_any") // pgrx converts the VARSIZE_ANY macro, so we don't want to also have this function, which is in heaptuple.c
        .blocklist_function("(?:raw_)?(?:expression|query|query_or_expression)_tree_walker")
        .blocklist_function("planstate_tree_walker")
        .blocklist_function("range_table_(?:entry_)?walker")
        .blocklist_function(".*(?:set|long)jmp")
        .blocklist_function("pg_re_throw")
        .blocklist_function("err(start|code|msg|detail|context_msg|hint|finish)")
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
}

fn add_derives(bind: bindgen::Builder) -> bindgen::Builder {
    bind.derive_debug(true)
        .derive_copy(true)
        .derive_default(true)
        .derive_eq(false)
        .derive_partialeq(false)
        .derive_hash(false)
        .derive_ord(false)
        .derive_partialord(false)
}

fn env_tracked(s: &str) -> Option<String> {
    // a **sorted** list of environment variable keys that cargo might set that we don't need to track
    // these were picked out, by hand, from: https://doc.rust-lang.org/cargo/reference/environment-variables.html
    const CARGO_KEYS: &[&str] = &[
        "BROWSER",
        "DEBUG",
        "DOCS_RS",
        "HOST",
        "HTTP_PROXY",
        "HTTP_TIMEOUT",
        "NUM_JOBS",
        "OPT_LEVEL",
        "OUT_DIR",
        "PATH",
        "PROFILE",
        "TARGET",
        "TERM",
    ];

    let is_cargo_key =
        s.starts_with("CARGO") || s.starts_with("RUST") || CARGO_KEYS.binary_search(&s).is_ok();

    if !is_cargo_key {
        // if it's an envar that cargo gives us, we don't want to ask it to rerun build.rs if it changes
        // we'll let cargo figure that out for itself, and doing so, depending on the key, seems to
        // cause cargo to rerun build.rs every time, which is terrible
        println!("cargo:rerun-if-env-changed={s}");
    }
    std::env::var(s).ok()
}

fn target_env_tracked(s: &str) -> Option<String> {
    let target = env_tracked("TARGET").unwrap();
    env_tracked(&format!("{s}_{target}")).or_else(|| env_tracked(s))
}

/// Returns `Err` if `pg_config` errored, `None` if we should
fn pg_target_include_flags(pg_version: u16, pg_config: &PgConfig) -> eyre::Result<Option<String>> {
    let var = "PGRX_INCLUDEDIR_SERVER";
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

fn build_shim(
    shim_src: &path::Path,
    shim_dst: &path::Path,
    pg_config: &PgConfig,
) -> eyre::Result<()> {
    let major_version = pg_config.major_version()?;

    // then build the shim for the version feature currently being built
    build_shim_for_version(shim_src, shim_dst, pg_config)?;

    // no matter what, tell rustc to link to the library that was built for the feature we're currently building
    let envvar_name = format!("CARGO_FEATURE_PG{major_version}");
    if env_tracked(&envvar_name).is_some() {
        println!("cargo:rustc-link-search={}", shim_dst.display());
        println!("cargo:rustc-link-lib=static=pgrx-cshim-{major_version}");
    }

    Ok(())
}

fn build_shim_for_version(
    shim_src: &path::Path,
    shim_dst: &path::Path,
    pg_config: &PgConfig,
) -> eyre::Result<()> {
    let path_env = prefix_path(pg_config.parent_path());
    let major_version = pg_config.major_version()?;

    eprintln!("PATH for build_shim={path_env}");
    eprintln!("shim_src={}", shim_src.display());
    eprintln!("shim_dst={}", shim_dst.display());

    fs::create_dir_all(shim_dst).unwrap();

    let makefile_dst = path::Path::join(shim_dst, "./Makefile");
    if !makefile_dst.try_exists()? {
        fs::copy(path::Path::join(shim_src, "./Makefile"), makefile_dst).unwrap();
    }

    let cshim_dst = path::Path::join(shim_dst, "./pgrx-cshim.c");
    if !cshim_dst.try_exists()? {
        fs::copy(path::Path::join(shim_src, "./pgrx-cshim.c"), cshim_dst).unwrap();
    }

    let make = env_tracked("MAKE").unwrap_or_else(|| "make".to_string());
    let rc = run_command(
        Command::new(make)
            .arg("clean")
            .arg(&format!("libpgrx-cshim-{major_version}.a"))
            .env("PG_TARGET_VERSION", format!("{major_version}"))
            .env("PATH", path_env)
            .env_remove("TARGET")
            .env_remove("HOST")
            .current_dir(shim_dst),
        &format!("shim for PG v{major_version}"),
    )?;

    if rc.status.code().unwrap() != 0 {
        return Err(eyre!("failed to make pgrx-cshim for v{major_version}"));
    }

    Ok(())
}

fn extra_bindgen_clang_args(pg_config: &PgConfig) -> eyre::Result<Vec<String>> {
    let mut out = vec![];
    if env_tracked("CARGO_CFG_TARGET_OS").as_deref() == Some("macos") {
        // On macOS, find the `-isysroot` arg out of the c preprocessor flags,
        // to handle the case where bindgen uses a libclang isn't provided by
        // the system.
        let flags = pg_config.cppflags()?;
        // In practice this will always be valid UTF-8 because of how the
        // `pgrx-pg-config` crate is implemented, but even if it were not, the
        // problem won't be with flags we are interested in.
        let flags = shlex::split(&flags.to_string_lossy()).unwrap_or_default();
        // Just give clang the full flag set, since presumably that's what we're
        // getting when we build the C shim anyway.
        out.extend(flags.iter().cloned());

        // Find the `-isysroot` flags so we can warn about them, so something
        // reasonable shows up if/when the build fails.
        //
        // Eventually we should probably wrangle the sysroot for `cargo pgrx
        // init`-installed PGs a bit more aggressively, but for now, whatever.
        for pair in flags.windows(2) {
            if pair[0] == "-isysroot" {
                if !std::path::Path::new(&pair[1]).exists() {
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
                    // The logic we'd like ideally is for `cargo pgrx init` to
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
                         need to re-run `cargo pgrx init` and/or update your command line tools.",
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
        .env_remove("PROFILE")
        .env_remove("OUT_DIR")
        .env_remove("NUM_JOBS");

    eprintln!("[{version}] {command:?}");
    dbg.push_str(&format!("[{version}] -------- {command:?} -------- \n"));

    let output = command.output()?;
    let rc = output.clone();

    if !output.stdout.is_empty() {
        for line in String::from_utf8(output.stdout).unwrap().lines() {
            if line.starts_with("cargo:") {
                dbg.push_str(&format!("{line}\n"));
            } else {
                dbg.push_str(&format!("[{version}] [stdout] {line}\n"));
            }
        }
    }

    if !output.stderr.is_empty() {
        for line in String::from_utf8(output.stderr).unwrap().lines() {
            dbg.push_str(&format!("[{version}] [stderr] {line}\n"));
        }
    }
    dbg.push_str(&format!("[{version}] /----------------------------------------\n"));

    eprintln!("{dbg}");
    Ok(rc)
}

// Plausibly it would be better to generate a regex to pass to bindgen for this,
// but this is less error-prone for now.
static BLOCKLISTED: OnceLock<BTreeSet<&'static str>> = OnceLock::new();
fn is_blocklisted_item(item: &ForeignItem) -> bool {
    let sym_name = match item {
        ForeignItem::Fn(f) => &f.sig.ident,
        // We don't *need* to filter statics too (only functions), but it
        // doesn't hurt.
        ForeignItem::Static(s) => &s.ident,
        _ => return false,
    };
    BLOCKLISTED
        .get_or_init(|| build::sym_blocklist::SYMBOLS.iter().copied().collect::<BTreeSet<&str>>())
        .contains(sym_name.to_string().as_str())
}

fn apply_pg_guard(items: &Vec<syn::Item>) -> eyre::Result<proc_macro2::TokenStream> {
    let mut out = proc_macro2::TokenStream::new();
    for item in items {
        match item {
            Item::ForeignMod(block) => {
                let abi = &block.abi;
                let (mut extern_funcs, mut others) = (Vec::new(), Vec::new());
                block.items.iter().filter(|&item| !is_blocklisted_item(item)).cloned().for_each(
                    |item| match item {
                        ForeignItem::Fn(func) => extern_funcs.push(func),
                        item => others.push(item),
                    },
                );
                out.extend(quote! {
                    #[pgrx_macros::pg_guard]
                    #abi { #(#extern_funcs)* }
                });
                out.extend(quote! { #abi { #(#others)* } });
            }
            _ => {
                out.extend(item.into_token_stream());
            }
        }
    }

    Ok(out)
}

fn rust_fmt(path: &Path) -> eyre::Result<()> {
    // We shouldn't hit this path in a case where we care about it, but... just
    // in case we probably should respect RUSTFMT.
    let rustfmt = env_tracked("RUSTFMT").unwrap_or_else(|| "rustfmt".into());
    let out = run_command(Command::new(rustfmt).arg(path).current_dir("."), "[bindings_diff]");
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
        Err(e) => Err(e),
    }
}
