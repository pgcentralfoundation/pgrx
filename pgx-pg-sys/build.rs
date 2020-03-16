extern crate build_deps;

use bindgen::callbacks::MacroParsingBehavior;
use quote::quote;
use rayon::prelude::*;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::str::FromStr;
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

fn make_git_repo_path(branch_name: &str) -> PathBuf {
    PathBuf::from(format!("/tmp/pgx-build/{}", branch_name))
}

fn make_include_path(git_repo_path: &PathBuf) -> PathBuf {
    let mut include_path = git_repo_path.clone();
    include_path.push("install/include/postgresql/server");
    include_path
}

fn make_shim_path(manifest_dir: &str) -> PathBuf {
    let mut shim_dir = PathBuf::from(manifest_dir);

    // backup a directory
    shim_dir.pop();

    // and a new dir named "pgx-cshim"
    shim_dir.push("pgx-cshim");

    shim_dir
}

fn main() -> Result<(), std::io::Error> {
    build_deps::rerun_if_changed_paths("include/*").unwrap();
    build_deps::rerun_if_changed_paths("../pgx-macros/src/*").unwrap();
    build_deps::rerun_if_changed_paths("../pgx-cshim/pgx-cshim.c").unwrap();
    build_deps::rerun_if_changed_paths("../pgx-cshim/Makefile").unwrap();
    build_deps::rerun_if_changed_paths(
        "/tmp/pgx-build/REL_10_STABLE/.git/refs/heads/REL_10_STABLE",
    )
    .unwrap();
    build_deps::rerun_if_changed_paths(
        "/tmp/pgx-build/REL_11_STABLE/.git/refs/heads/REL_11_STABLE",
    )
    .unwrap();
    build_deps::rerun_if_changed_paths(
        "/tmp/pgx-build/REL_12_STABLE/.git/refs/heads/REL_12_STABLE",
    )
    .unwrap();

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let cwd = PathBuf::from(&manifest_dir);
    let pg_git_repo_url = "git://git.postgresql.org/git/postgresql.git";
    let shim_dir = make_shim_path(&manifest_dir);

    let shim_mutex = Mutex::new(());
    vec![
        ("pg10", "REL_10_STABLE", "8810"),
        ("pg11", "REL_11_STABLE", "8811"),
        ("pg12", "REL_12_STABLE", "8812"),
    ]
    .par_iter()
    .for_each(|v| {
        let version = v.0;
        let branch_name = v.1;
        let port_no = u16::from_str(v.2).unwrap();
        let pg_git_path = make_git_repo_path(branch_name);
        let include_path = make_include_path(&pg_git_path);
        let bindings_rs = PathBuf::from(format!("src/{}_bindings.rs", version));
        let include_h = PathBuf::from(format!("include/{}.h", version));
        let config_status = PathBuf::from(format!("{}/config.status", pg_git_path.display()));
        let need_configure_and_make =
            git_clone_postgres(&pg_git_path, pg_git_repo_url, branch_name)
                .unwrap_or_else(|e| panic!("Unable to git clone {}: {:?}", pg_git_repo_url, e));

        if need_configure_and_make || !config_status.is_file() {
            eprintln!("[{}] cleaning and building", branch_name);

            git_clean(&pg_git_path, &branch_name)
                .unwrap_or_else(|e| panic!("Unable to switch to branch {}: {:?}", branch_name, e));

            configure_and_make(&pg_git_path, &branch_name, port_no).unwrap_or_else(|e| {
                panic!(
                    "Unable to make clean and configure postgres branch {}: {:?}",
                    branch_name, e
                )
            });
        }

        build_shim(&shim_dir, &shim_mutex, &pg_git_path, version);

        eprintln!(
            "[{}] Running bindgen on {} with {}",
            branch_name,
            bindings_rs.display(),
            include_path.display()
        );
        let bindings = bindgen::Builder::default()
            .header(include_h.to_str().unwrap())
            .clang_arg(&format!("-I{}", include_path.display()))
            .parse_callbacks(Box::new(IgnoredMacros::default()))
            .blacklist_function("varsize_any") // pgx converts the VARSIZE_ANY macro, so we don't want to also have this function, which is in heaptuple.c
            .rustfmt_bindings(true)
            .derive_debug(false)
            .derive_copy(true) // necessary to avoid __BindgenUnionField usages -- I don't understand why?
            .derive_default(false)
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
    });

    generate_common_rs(cwd);

    Ok(())
}

fn build_shim(shim_dir: &PathBuf, shim_mutex: &Mutex<()>, pg_git_path: &PathBuf, version: &str) {
    // build the shim under a lock b/c this can't be built concurrently
    let _lock = shim_mutex.lock().expect("couldn't obtain shim_mutex");

    // then build the shim for the version feature currently being built
    if version.eq("pg10") {
        build_shim_for_version(&shim_dir, &pg_git_path, 10).expect("shim build for pg10 failed");
    } else if version.eq("pg11") {
        build_shim_for_version(&shim_dir, &pg_git_path, 11).expect("shim build for pg11 failed");
    } else if version.eq("pg12") {
        build_shim_for_version(&shim_dir, &pg_git_path, 12).expect("shim build for pg12 failed");
    } else {
        panic!("can't determine which c-shim to build");
    }

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
    git_repo_path: &PathBuf,
    version_no: i32,
) -> Result<(), std::io::Error> {
    // put the install directory fir this version of Postgres at the head of the path
    // so that `pg_config` gets found and we can make the shim static library
    let path_env = std::env::var("PATH").unwrap();
    let path_env = format!(
        "{}:{}",
        format!("{}/install/bin", git_repo_path.display()),
        path_env
    );

    eprintln!("PATH={}", path_env);
    eprintln!("shim_dir={}", shim_dir.display());
    let rc = run_command(
        Command::new("make")
            .arg("clean")
            .arg(&format!("libpgx-cshim-{}.a", version_no))
            .env("PG_TARGET_VERSION", format!("{}", version_no))
            .env("PATH", path_env)
            .current_dir(shim_dir),
        &format!("shim for PG v{}", version_no),
    )?;

    if rc.status.code().unwrap() != 0 {
        panic!("failed to make pgx-cshim for v{}", version_no);
    }

    Ok(())
}

fn generate_common_rs(mut working_dir: PathBuf) {
    eprintln!("[all branches] Regenerating common.rs and XX_specific.rs files...");
    let cwd = std::env::current_dir().unwrap();

    working_dir.pop();
    std::env::set_current_dir(&working_dir).unwrap();
    let result = bindings_diff::main();
    std::env::set_current_dir(cwd).unwrap();

    if result.is_err() {
        panic!(result.err().unwrap());
    }
}

fn git_clone_postgres(
    path: &Path,
    repo_url: &str,
    branch_name: &str,
) -> Result<bool, std::io::Error> {
    if path.exists() {
        let mut gitdir = path.to_path_buf();
        gitdir.push(Path::new(".git/config"));

        if gitdir.exists() && gitdir.is_file() {
            // we already have git cloned
            // do a pull instead
            let output = run_command(
                Command::new("git").arg("pull").current_dir(path),
                branch_name,
            )?;

            // a status code of zero and more than 1 line on stdout means we fetched new stuff
            return Ok(output.status.code().unwrap() == 0
                && String::from_utf8(output.stdout).unwrap().lines().count() > 1);
        }
    }

    let output = run_command(
        Command::new("git").arg("clone").arg(repo_url).arg(path),
        branch_name,
    )?;

    // if the output status is zero, that means we cloned the repo
    if output.status.code().unwrap() != 0 {
        return Ok(false);
    }

    let output = run_command(
        Command::new("git")
            .arg("checkout")
            .arg(branch_name)
            .current_dir(path),
        branch_name,
    )?;

    // if the output status is zero, that means we switched to the right branch
    Ok(output.status.code().unwrap() == 0)
}

fn git_clean(path: &Path, branch_name: &str) -> Result<(), std::io::Error> {
    run_command(
        Command::new("git")
            .arg("clean")
            .arg("-f")
            .arg("-d")
            .arg("-x")
            .current_dir(path),
        branch_name,
    )?;

    run_command(
        Command::new("git").arg("pull").current_dir(path),
        branch_name,
    )?;

    Ok(())
}

fn configure_and_make(path: &Path, branch_name: &str, port_no: u16) -> Result<(), std::io::Error> {
    // configure
    let rc = run_command(
        Command::new(format!("{}/configure", path.display()))
            .arg("--without-readline") // don't need readline to build PG extensions -- one less system dep to install
            .arg(format!("--prefix={}/install", path.display()))
            .arg(format!("--with-pgport={}", port_no))
            .current_dir(path),
        branch_name,
    )?;

    if rc.status.code().unwrap() != 0 {
        panic!("configure failed for {}", branch_name)
    }

    let num_jobs = u32::from_str(std::env::var("NUM_JOBS").unwrap().as_str()).unwrap();
    let num_jobs = std::cmp::max(1, num_jobs / 3);

    // make install
    let rc = run_command(
        Command::new("make")
            .arg("-j")
            .arg(format!("{}", num_jobs))
            .arg("install")
            .current_dir(path),
        branch_name,
    )?;

    if rc.status.code().unwrap() != 0 {
        panic!("make install failed for {}", branch_name)
    }

    // make clean
    let rc = run_command(
        Command::new("make")
            .arg("-j")
            .arg(format!("{}", num_jobs))
            .arg("clean")
            .current_dir(path),
        branch_name,
    )?;

    if rc.status.code().unwrap() != 0 {
        panic!("make clean failed for {}", branch_name)
    }

    Ok(())
}

fn run_command(mut command: &mut Command, branch_name: &str) -> Result<Output, std::io::Error> {
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

    eprintln!("[{}] {:?}", branch_name, command);
    dbg.push_str(&format!(
        "[{}] -------- {:?} -------- \n",
        branch_name, command
    ));

    let output = command.output()?;
    let rc = output.clone();

    if !output.stdout.is_empty() {
        for line in String::from_utf8(output.stdout).unwrap().lines() {
            if line.starts_with("cargo:") {
                dbg.push_str(&format!("{}\n", line));
            } else {
                dbg.push_str(&format!("[{}] [stdout] {}\n", branch_name, line));
            }
        }
    }

    if !output.stderr.is_empty() {
        for line in String::from_utf8(output.stderr).unwrap().lines() {
            dbg.push_str(&format!("[{}] [stderr] {}\n", branch_name, line));
        }
    }
    dbg.push_str(&format!(
        "[{}] /----------------------------------------\n",
        branch_name
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
        let mut v10 = read_source_file("pgx-pg-sys/src/pg10_bindings.rs");
        let mut v11 = read_source_file("pgx-pg-sys/src/pg11_bindings.rs");
        let mut v12 = read_source_file("pgx-pg-sys/src/pg12_bindings.rs");

        let mut versions = vec![&mut v10, &mut v11, &mut v12];
        let common = build_common_set(&mut versions);

        eprintln!(
            "[all branches]: common={}, v10={}, v11={}, v12={}",
            common.len(),
            v10.len(),
            v11.len(),
            v12.len(),
        );

        write_common_file("pgx-pg-sys/src/common.rs", common);
        write_source_file("pgx-pg-sys/src/pg10_specific.rs", v10);
        write_source_file("pgx-pg-sys/src/pg11_specific.rs", v11);
        write_source_file("pgx-pg-sys/src/pg12_specific.rs", v12);

        // delete the bindings files when we're done with them
        std::fs::remove_file(PathBuf::from_str("pgx-pg-sys/src/pg10_bindings.rs").unwrap())
            .expect("couldn't delete v10 bindings");
        std::fs::remove_file(PathBuf::from_str("pgx-pg-sys/src/pg11_bindings.rs").unwrap())
            .expect("couldn't delete v11 bindings");
        std::fs::remove_file(PathBuf::from_str("pgx-pg-sys/src/pg12_bindings.rs").unwrap())
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
