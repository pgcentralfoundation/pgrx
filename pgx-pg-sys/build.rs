// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

extern crate build_deps;

use bindgen::callbacks::MacroParsingBehavior;
use quote::quote;
use rayon::prelude::*;
use std::collections::HashSet;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use syn::export::{ToTokens, TokenStream2};
use syn::Item;

#[derive(Debug)]
struct IgnoredMacros(HashSet<String>);

impl IgnoredMacros {
    fn default() -> Self {
        // these cause duplicate definition problems on linux
        // see: https://github.com/rust-lang/rust-bindgen/issues/687
        IgnoredMacros(
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

impl bindgen::callbacks::ParseCallbacks for IgnoredMacros {
    fn will_parse_macro(&self, name: &str) -> MacroParsingBehavior {
        if self.0.contains(name) {
            bindgen::callbacks::MacroParsingBehavior::Ignore
        } else {
            bindgen::callbacks::MacroParsingBehavior::Default
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    // dump our environment
    for (k, v) in std::env::vars() {
        eprintln!("{}={}", k, v);
    }

    build_deps::rerun_if_changed_paths("download-postgres.sh").unwrap();
    build_deps::rerun_if_changed_paths("compile-postgres.sh").unwrap();
    build_deps::rerun_if_changed_paths("include/*").unwrap();
    build_deps::rerun_if_changed_paths("cshim/pgx-cshim.c").unwrap();
    build_deps::rerun_if_changed_paths("cshim/Makefile").unwrap();

    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let target_dir = PathBuf::from(std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| {
        let mut out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
        out_dir.pop();
        out_dir.pop();
        out_dir.pop();
        out_dir.pop();
        out_dir.display().to_string()
    }));
    let shim_dir = PathBuf::from(format!("{}/cshim", manifest_dir.display()));

    eprintln!("manifest_dir={}", manifest_dir.display());
    eprintln!("target_dir={}", target_dir.display());
    eprintln!("shim_dir={}", shim_dir.display());

    let shim_mutex = Mutex::new(());
    let need_common_rs = AtomicBool::new(false);

    let pg_versions = vec![("10.13", 8810), ("11.8", 8811), ("12.3", 8812)];
    pg_versions.par_iter().for_each(|v| {
        let version: &str = v.0;
        let major_version = u16::from_str(version.split_terminator('.').next().unwrap()).unwrap();
        let port_no = v.1;
        let include_path = PathBuf::from(format!(
            "{}/postgresql-{}/pgx-install/include/server",
            target_dir.display(),
            version
        ));
        let bindings_rs = PathBuf::from(format!(
            "{}/src/pg{}_bindings.rs",
            manifest_dir.display(),
            major_version
        ));
        let specific_rs = PathBuf::from(format!(
            "{}/src/pg{}_specific.rs",
            manifest_dir.display(),
            major_version
        ));
        let include_h = PathBuf::from(format!(
            "{}/include/pg{}.h",
            manifest_dir.display(),
            major_version
        ));

        eprintln!("include_path={}", include_path.display());
        eprintln!("bindings_rs={}", bindings_rs.display());
        eprintln!("include_h={}", include_h.display());
        download_postgres(&manifest_dir, version, &target_dir)
            .unwrap_or_else(|_| panic!("failed to download Postgres v{}", version));
        let did_compile = compile_postgres(
            &manifest_dir,
            version,
            &target_dir,
            port_no,
            pg_versions.len(),
        )
        .unwrap_or_else(|_| panic!("failed to compile Postgres v{}", version));
        build_shim(&shim_dir, &shim_mutex, &target_dir, major_version, version);

        if did_compile || !specific_rs.exists() {
            need_common_rs.store(true, Ordering::SeqCst);
            eprintln!(
                "[{}] Running bindgen on {} with {}",
                version,
                bindings_rs.display(),
                include_path.display()
            );
            let bindings = bindgen::Builder::default()
                .header(include_h.to_str().unwrap())
                .clang_arg(&format!("-I{}", include_path.display()))
                .parse_callbacks(Box::new(IgnoredMacros::default()))
                .blacklist_function("varsize_any") // pgx converts the VARSIZE_ANY macro, so we don't want to also have this function, which is in heaptuple.c
                .blacklist_function("query_tree_walker")
                .blacklist_function("expression_tree_walker")
                .blacklist_function("sigsetjmp")
                .blacklist_function("siglongjmp")
                .blacklist_function("pg_re_throw")
                .size_t_is_usize(true)
                .rustfmt_bindings(true)
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
                .unwrap_or_else(|e| panic!("Unable to generate bindings for {}: {:?}", version, e));

            let bindings = apply_pg_guard(bindings.to_string()).unwrap();
            std::fs::write(bindings_rs.clone(), bindings).unwrap_or_else(|e| {
                panic!(
                    "Unable to save bindings for {} to {}: {:?}",
                    version,
                    bindings_rs.display(),
                    e
                )
            });
        }
    });

    if need_common_rs.load(Ordering::SeqCst) {
        generate_common_rs(manifest_dir);
    }

    Ok(())
}

fn build_shim(
    shim_dir: &PathBuf,
    shim_mutex: &Mutex<()>,
    target_dir: &PathBuf,
    major_version: u16,
    version: &str,
) {
    // build the shim under a lock b/c this can't be built concurrently
    let _lock = shim_mutex.lock().expect("couldn't obtain shim_mutex");

    // then build the shim for the version feature currently being built
    build_shim_for_version(&shim_dir, &target_dir, major_version, version)
        .expect("shim build failed");

    // and tell rustc to link to the library that was built for the feature we're currently building
    if std::env::var("CARGO_FEATURE_PG10").is_ok() {
        println!("cargo:rustc-link-search={}", shim_dir.display());
        println!("cargo:rustc-link-lib=static=pgx-cshim-10");
    } else if std::env::var("CARGO_FEATURE_PG11").is_ok() {
        println!("cargo:rustc-link-search={}", shim_dir.display());
        println!("cargo:rustc-link-lib=static=pgx-cshim-11");
    } else if std::env::var("CARGO_FEATURE_PG12").is_ok() {
        println!("cargo:rustc-link-search={}", shim_dir.display());
        println!("cargo:rustc-link-lib=static=pgx-cshim-12");
    }
}

fn build_shim_for_version(
    shim_dir: &PathBuf,
    target_dir: &PathBuf,
    major_version: u16,
    version: &str,
) -> Result<(), std::io::Error> {
    // put the install directory for this version of Postgres at the head of the path
    // so that `pg_config` gets found and we can make the shim static library
    let path_env = std::env::var("PATH").unwrap();
    let path_env = format!(
        "{}:{}",
        format!(
            "{}/postgresql-{}/pgx-install/bin",
            target_dir.display(),
            version
        ),
        path_env
    );

    eprintln!("PATH={}", path_env);
    eprintln!("shim_dir={}", shim_dir.display());
    let rc = run_command(
        Command::new("make")
            .arg("clean")
            .arg(&format!("libpgx-cshim-{}.a", major_version))
            .env("PG_TARGET_VERSION", format!("{}", major_version))
            .env("PATH", path_env)
            .current_dir(shim_dir),
        &format!("shim for PG v{}", major_version),
    )?;

    if rc.status.code().unwrap() != 0 {
        panic!("failed to make pgx-cshim for v{}", major_version);
    }

    Ok(())
}

fn generate_common_rs(working_dir: PathBuf) {
    eprintln!("[all branches] Regenerating common.rs and XX_specific.rs files...");
    let cwd = std::env::current_dir().unwrap();

    std::env::set_current_dir(&working_dir).unwrap();
    let result = bindings_diff::main();
    std::env::set_current_dir(cwd).unwrap();

    if result.is_err() {
        panic!(result.err().unwrap());
    }
}

fn download_postgres(
    manifest_dir: &PathBuf,
    version_number: &str,
    target_dir: &PathBuf,
) -> Result<(), std::io::Error> {
    let rc = run_command(
        Command::new("./download-postgres.sh")
            .arg(version_number)
            .arg(target_dir.display().to_string())
            .current_dir(manifest_dir.display().to_string()),
        version_number,
    )?;

    if rc.status.code().unwrap() != 0 {
        panic!("failed to download Postgres v{}", version_number);
    }
    Ok(())
}

fn compile_postgres(
    manifest_dir: &PathBuf,
    version_number: &str,
    target_dir: &PathBuf,
    port_number: u16,
    num_versions: usize,
) -> Result<bool, std::io::Error> {
    let num_cpus = 1.max(num_cpus::get() / num_versions);
    let rc = run_command(
        Command::new("./compile-postgres.sh")
            .arg(version_number)
            .arg(target_dir.display().to_string())
            .arg(port_number.to_string())
            .env("NUM_CPUS", num_cpus.to_string())
            .current_dir(manifest_dir.display().to_string()),
        version_number,
    )?;

    match rc.status.code().unwrap() {
        0 => Ok(false), // we did NOT compile Postgres
        2 => Ok(true),  // we did compile Postgres
        _ => panic!("failed to compile Postgres v{}", version_number),
    }
}

fn run_command(mut command: &mut Command, version: &str) -> Result<Output, std::io::Error> {
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
    dbg.push_str(&format!(
        "[{}] /----------------------------------------\n",
        version
    ));

    eprintln!("{}", dbg);
    Ok(rc)
}

fn apply_pg_guard(input: String) -> Result<String, std::io::Error> {
    let file = syn::parse_file(input.as_str()).unwrap();

    let mut stream = TokenStream2::new();
    for item in file.items.into_iter() {
        match item {
            Item::ForeignMod(block) => {
                stream.extend(quote! {
                    #[pg_guard]
                    #block
                });
            }
            _ => {
                stream.extend(quote! { #item });
            }
        }
    }

    Ok(format!("{}", stream.into_token_stream()))
}

fn rust_fmt(path: &str) -> Result<(), std::io::Error> {
    run_command(
        Command::new("rustfmt").arg(path).current_dir("."),
        "[bindings_diff]",
    )?;

    Ok(())
}

pub(crate) mod bindings_diff {
    use crate::rust_fmt;
    use quote::quote;
    use std::cmp::Ordering;
    use std::collections::BTreeMap;
    use std::fs::File;
    use std::hash::{Hash, Hasher};
    use std::io::Read;
    use std::path::PathBuf;
    use std::str::FromStr;
    use syn::export::TokenStream2;
    use syn::ForeignItem;
    use syn::{ImplItem, Item};

    #[derive(Eq, Clone)]
    struct SortableItem {
        item: Item,
    }

    impl SortableItem {
        fn new(item: Item) -> Self {
            SortableItem { item }
        }

        fn ident(&self) -> String {
            match &self.item {
                Item::Const(v) => format!("Const: {}", v.ident.to_string()),
                Item::Enum(v) => format!("Enum: {}", v.ident.to_string()),
                Item::ExternCrate(v) => format!("ExternCrate: {}", v.ident.to_string()),
                Item::Fn(v) => format!("Fn: {}", v.sig.ident.to_string()),
                Item::ForeignMod(v) => format!(
                    "ForeignMod: {}",
                    if v.items.is_empty() {
                        format!("{}", quote! {#v})
                    } else {
                        match v.items.first().unwrap() {
                            ForeignItem::Fn(v) => format!("Fn: {}", v.sig.ident.to_string()),
                            ForeignItem::Static(v) => format!("Static: {}", v.ident.to_string()),
                            ForeignItem::Type(v) => format!("Type: {}", v.ident.to_string()),
                            ForeignItem::Macro(v) => format!("Macro: {}", quote! {#v}),
                            ForeignItem::Verbatim(v) => format!("Verbatim: {}", quote! {#v}),
                            ForeignItem::__Nonexhaustive => panic!("ForeignItem __Nonexhausstive"),
                        }
                    }
                ),
                Item::Impl(v) => format!(
                    "Impl: {}",
                    if v.items.is_empty() {
                        format!("{}", quote! {#v})
                    } else {
                        match v.items.first().unwrap() {
                            ImplItem::Const(v) => format!("Const: {}", v.ident.to_string()),
                            ImplItem::Method(v) => format!("Method: {}", v.sig.ident.to_string()),
                            ImplItem::Type(v) => format!("Type: {}", v.ident.to_string()),
                            ImplItem::Macro(v) => format!("Macro: {}", format!("{}", quote! {#v})),
                            ImplItem::Verbatim(v) => {
                                format!("Verbatim: {}", format!("{}", quote! {#v}))
                            }
                            ImplItem::__Nonexhaustive => panic!("ImplItem __Nonexhausstive"),
                        }
                    }
                ),
                Item::Macro(v) => format!("Macro: {}", quote! {#v}),
                Item::Macro2(v) => format!("Macro2: {}", v.ident.to_string()),
                Item::Mod(v) => format!("Mod: {}", v.ident.to_string()),
                Item::Static(v) => format!("Static: {}", v.ident.to_string()),
                Item::Struct(v) => format!("Struct: {}", v.ident.to_string()),
                Item::Trait(v) => format!("Trait: {}", v.ident.to_string()),
                Item::TraitAlias(v) => format!("TraitAlias: {}", v.ident.to_string()),
                Item::Type(v) => format!("Type: {}", v.ident.to_string()),
                Item::Union(v) => format!("Union: {}", v.ident.to_string()),
                Item::Use(v) => format!("Use: {}", format!("{}", quote! {#v})),
                Item::Verbatim(v) => format!("Verbatim: {}", format!("{}", quote! {#v})),
                Item::__Nonexhaustive => panic!("Item __Nonexhaustive"),
            }
        }
    }

    impl Ord for SortableItem {
        fn cmp(&self, other: &Self) -> Ordering {
            self.ident().cmp(&other.ident())
        }
    }

    impl PartialOrd for SortableItem {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.ident().partial_cmp(&other.ident())
        }
    }

    impl PartialEq for SortableItem {
        fn eq(&self, other: &Self) -> bool {
            self.item.eq(&other.item)
        }
    }

    impl Hash for SortableItem {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.item.hash(state)
        }
    }

    pub(crate) fn main() -> Result<(), std::io::Error> {
        let mut v10 = read_source_file("src/pg10_bindings.rs");
        let mut v11 = read_source_file("src/pg11_bindings.rs");
        let mut v12 = read_source_file("src/pg12_bindings.rs");

        let mut versions = vec![&mut v10, &mut v11, &mut v12];
        let common = build_common_set(&mut versions);

        eprintln!(
            "[all branches]: common={}, v10={}, v11={}, v12={}",
            common.len(),
            v10.len(),
            v11.len(),
            v12.len(),
        );

        write_common_file("src/common.rs", common);
        write_source_file("src/pg10_specific.rs", v10);
        write_source_file("src/pg11_specific.rs", v11);
        write_source_file("src/pg12_specific.rs", v12);

        // delete the bindings files when we're done with them
        std::fs::remove_file(PathBuf::from_str("src/pg10_bindings.rs").unwrap())
            .expect("couldn't delete v10 bindings");
        std::fs::remove_file(PathBuf::from_str("src/pg11_bindings.rs").unwrap())
            .expect("couldn't delete v11 bindings");
        std::fs::remove_file(PathBuf::from_str("src/pg12_bindings.rs").unwrap())
            .expect("couldn't delete v12 bindings");

        Ok(())
    }

    fn build_common_set(
        versions: &mut Vec<&mut BTreeMap<String, SortableItem>>,
    ) -> BTreeMap<String, SortableItem> {
        let mut common = BTreeMap::new();

        for map in versions.iter() {
            for (key, value) in map.iter() {
                if !common.contains_key(key) && all_contain(&versions, &key) {
                    common.insert(key.clone(), value.clone());
                }
            }
        }

        for map in versions.iter_mut() {
            for (key, _) in common.iter() {
                map.remove(key);
            }
        }

        common
    }

    #[inline]
    fn all_contain(maps: &[&mut BTreeMap<String, SortableItem>], key: &str) -> bool {
        for map in maps.iter() {
            if !map.contains_key(key) {
                return false;
            }
        }

        true
    }

    fn read_source_file(filename: &str) -> BTreeMap<String, SortableItem> {
        let mut file = File::open(filename).unwrap();
        let mut input = String::new();

        file.read_to_string(&mut input).unwrap();
        let source = syn::parse_file(input.as_str()).unwrap();

        let mut item_map = BTreeMap::new();
        for item in source.items.into_iter() {
            let mut stream = TokenStream2::new();
            stream.extend(quote! {#item});
            item_map.insert(stream.to_string(), SortableItem::new(item));
        }

        item_map
    }

    fn write_source_file(filename: &str, items: BTreeMap<String, SortableItem>) {
        let mut stream = TokenStream2::new();
        stream.extend(quote! {
            #![allow(clippy::all)]

            use crate as pg_sys;
            use pgx_macros::*;
            use crate::common::*;
        });
        for (_, item) in items {
            match &item.item {
                Item::Use(_) => {}
                item => stream.extend(quote! {#item}),
            }
        }
        std::fs::write(filename, stream.to_string())
            .unwrap_or_else(|e| panic!("Unable to save bindings for {}: {:?}", filename, e));
        rust_fmt(filename)
            .unwrap_or_else(|e| panic!("unable to run rustfmt for {}: {:?}", filename, e));
    }

    fn write_common_file(filename: &str, items: BTreeMap<String, SortableItem>) {
        let mut stream = TokenStream2::new();
        stream.extend(quote! {
            #![allow(clippy::all)]

            use crate as pg_sys;
            use pgx_macros::*;

            #[cfg(feature = "pg10")]
            use crate::pg10_specific::*;
            #[cfg(feature = "pg11")]
            use crate::pg11_specific::*;
            #[cfg(feature = "pg12")]
            use crate::pg12_specific::*;
        });
        for (_, item) in items.iter() {
            match &item.item {
                Item::Use(_) => {}
                item => stream.extend(quote! {#item}),
            }
        }
        std::fs::write(filename, stream.to_string())
            .unwrap_or_else(|e| panic!("Unable to save bindings for {}: {:?}", filename, e));
        rust_fmt(filename)
            .unwrap_or_else(|e| panic!("unable to run rustfmt for {}: {:?}", filename, e));
    }
}
