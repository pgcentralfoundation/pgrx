// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

extern crate build_deps;

use bindgen::callbacks::MacroParsingBehavior;
use pgx_utils::{get_pg_config, get_pgx_config_path, prefix_path, run_pg_config};
use quote::quote;
use std::collections::HashSet;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::sync::Mutex;
use std::error::Error;
use syn::export::TokenStream2;
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

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // dump the environment for debugging if asked
    if std::env::var("PGX_BUILD_VERBOSE").unwrap_or("false".to_string()) == "true" {
        for (k, v) in std::env::vars() {
            eprintln!("{}={}", k, v);
        }
    }

    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(format!(
        "{}/generated-bindings/",
        std::env::var("CARGO_MANIFEST_DIR").unwrap()
    ));
    let shim_dir = PathBuf::from(format!("{}/cshim", manifest_dir.display()));
    let common_rs = PathBuf::from(format!("{}/common.rs", out_dir.display(),));

    eprintln!("manifest_dir={}", manifest_dir.display());
    eprintln!("shim_dir={}", shim_dir.display());
    eprintln!("common_rs={}", common_rs.display());

    let major_versions = vec![10, 11, 12];

    if std::env::var("DOCS_RS").unwrap_or("false".into()) == "1" {
        return Ok(());
    }

    build_deps::rerun_if_changed_paths(&get_pgx_config_path().display().to_string()).unwrap();
    build_deps::rerun_if_changed_paths("include/*").unwrap();
    build_deps::rerun_if_changed_paths("cshim/pgx-cshim.c").unwrap();
    build_deps::rerun_if_changed_paths("cshim/Makefile").unwrap();

    let shim_mutex = Mutex::new(());

    let mut shims: Vec<PgShim> = Vec::with_capacity(major_versions.len());
    for major_version in major_versions.iter() {
        let major_version = *major_version;

        let pg_config = get_pg_config(major_version);
        let include_h = PathBuf::from(format!(
            "{}/include/pg{}.h",
            manifest_dir.display(),
            major_version
        ));
        let specific_rs = PathBuf::from(format!(
            "{}/pg{}_specific.rs",
            out_dir.display(),
            major_version
        ));

        eprintln!("specific_rs={}", specific_rs.display());

        let bindings = run_bindgen(&pg_config, major_version, &include_h)?;
        let bindings = rewrite_items(bindings)?;
        shims.push(PgShim { major_version, bindings, pg_config });
    }

    // consolidate common items, this step also lands the transformed token streams to disk
    generate_common_rs(manifest_dir, &out_dir, shims.as_slice());

    // compile .a files from the generated shim code
    for shim in shims.iter() {
        build_shim(&shim_dir, &shim_mutex, shim.major_version, &shim.pg_config);
    }

    Ok(())
}

/// PgShim describes the bindings for a specific postgres version
struct PgShim {
    major_version: u16,
    bindings: TokenStream2,
    pg_config: Option<String>,
}

/// Given a token stream representing a file, apply a series of transformations to munge
/// the bindgen generated code with some postgres specific enhacements
fn rewrite_items(
    file: syn::File,
) -> Result<TokenStream2, Box<dyn Error + Send + Sync>> {
    let items = apply_pg_guard(file.items)?;

    let mut stream = TokenStream2::new();
    for item in items.into_iter() {
        stream.extend(quote! { #item });
    }
    Ok(stream)
}

/// Given a specific postgres version, `run_bindgen` generates bindings for the given
/// postgres version and returns them as a token stream.
fn run_bindgen(
    pg_config: &Option<String>,
    major_version: u16,
    include_h: &PathBuf,
) -> Result<syn::File, Box<dyn Error + Send + Sync>> {
    eprintln!(
        "Generating bindings for pg{}",
        major_version,
    );
    let includedir_server = run_pg_config(pg_config, "--includedir-server");
    let bindings = bindgen::Builder::default()
        .header(include_h.display().to_string())
        .clang_arg(&format!("-I{}", includedir_server))
        .parse_callbacks(Box::new(IgnoredMacros::default()))
        .blacklist_function("varsize_any") // pgx converts the VARSIZE_ANY macro, so we don't want to also have this function, which is in heaptuple.c
        .blacklist_function("query_tree_walker")
        .blacklist_function("expression_tree_walker")
        .blacklist_function("sigsetjmp")
        .blacklist_function("siglongjmp")
        .blacklist_function("pg_re_throw")
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
        .unwrap_or_else(|e| {
            panic!(
                "Unable to generate bindings for pg{}: {:?}",
                major_version, e
            )
        });

    syn::parse_file(bindings.to_string().as_str()).map_err(|e| From::from(e))
}

fn build_shim(
    shim_dir: &PathBuf,
    shim_mutex: &Mutex<()>,
    major_version: u16,
    pg_config: &Option<String>,
) {
    let libpgx_cshim: PathBuf =
        format!("{}/libpgx-cshim-{}.a", shim_dir.display(), major_version).into();

    eprintln!("libpgx_cshim={}", libpgx_cshim.display());
    if !libpgx_cshim.exists() {
        // build the shim under a lock b/c this can't be built concurrently
        let _lock = shim_mutex.lock().expect("couldn't obtain shim_mutex");

        // then build the shim for the version feature currently being built
        build_shim_for_version(&shim_dir, major_version, pg_config).expect("shim build failed");
    }

    // no matter what, tell rustc to link to the library that was built for the feature we're currently building
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
    major_version: u16,
    pg_config: &Option<String>,
) -> Result<(), std::io::Error> {
    let pg_config: PathBuf = pg_config.as_ref().unwrap().into();
    let path_env = prefix_path(pg_config.parent().unwrap());

    eprintln!("PATH for build_shim={}", path_env);
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

fn generate_common_rs(working_dir: PathBuf, out_dir: &PathBuf, shims: &[PgShim]) {
    eprintln!("[all branches] Regenerating common.rs and XX_specific.rs files...");
    let cwd = std::env::current_dir().unwrap();

    std::env::set_current_dir(&working_dir).unwrap();
    let result = bindings_diff::main(out_dir, shims);
    std::env::set_current_dir(cwd).unwrap();

    if result.is_err() {
        panic!(result.err().unwrap());
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

fn apply_pg_guard(items: Vec<syn::Item>) -> Result<Vec<syn::Item>, Box<dyn Error + Send + Sync>> {
    let mut out = Vec::with_capacity(items.len());
    for item in items.into_iter() {
        match item {
            Item::ForeignMod(block) => {
                out.push(syn::parse2(quote! {
                    #[pg_guard]
                    #block
                })?);
            }
            _ => {
                out.push(item);
            }
        }
    }

    Ok(out)
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
    use quote::{quote, ToTokens};
    use std::collections::BTreeSet;
    use std::io::{Write, BufWriter};
    use std::path::PathBuf;
    use super::PgShim;
    use syn::export::TokenStream2;

    pub(crate) fn main(out_dir: &PathBuf, shims: &[PgShim]) -> Result<(), std::io::Error> {
        // NOTE: this gross assertion can be ripped out once the code is made generic
        //       over different postgres versions.
        assert!(
            shims[0].major_version == 10 &&
            shims[1].major_version == 11 &&
            shims[2].major_version == 12
        );
        let mut v10 = parse_file_stream(&shims[0].bindings);
        let mut v11 = parse_file_stream(&shims[1].bindings);
        let mut v12 = parse_file_stream(&shims[2].bindings);

        let mut versions = vec![&mut v10, &mut v11, &mut v12];
        let common = build_common_set(&mut versions);

        eprintln!(
            "[all branches]: common={}, v10={}, v11={}, v12={}",
            common.len(),
            v10.len(),
            v11.len(),
            v12.len(),
        );

        write_common_file(&format!("{}common.rs", out_dir.display()), &common);
        write_source_file(&format!("{}pg10_specific.rs", out_dir.display()), &v10);
        write_source_file(&format!("{}pg11_specific.rs", out_dir.display()), &v11);
        write_source_file(&format!("{}pg12_specific.rs", out_dir.display()), &v12);

        Ok(())
    }

    fn build_common_set(versions: &mut Vec<&mut BTreeSet<String>>) -> BTreeSet<String> {
        let mut common = BTreeSet::new();

        for map in versions.iter() {
            for key in map.iter() {
                if !common.contains(key) && all_contain(&versions, &key) {
                    common.insert(key.clone());
                }
            }
        }

        for map in versions.iter_mut() {
            for key in common.iter() {
                map.remove(key);
            }
        }

        common
    }

    #[inline]
    fn all_contain(maps: &[&mut BTreeSet<String>], key: &String) -> bool {
        for map in maps.iter() {
            if !map.contains(key) {
                return false;
            }
        }

        true
    }

    /// Given a token stream, parse the stream as a file and return
    /// a set of the items found in the file.
    fn parse_file_stream(stream: &TokenStream2) -> BTreeSet<String> {
        let source: syn::File = syn::parse2(stream.clone()).unwrap();

        let mut item_map = BTreeSet::new();
        for item in source.items.into_iter() {
            item_map.insert(item.to_token_stream().to_string());
        }

        item_map
    }

    fn write_source_file(filename: &str, items: &BTreeSet<String>) {
        let file = std::fs::File::create(filename).expect(&format!("failed to create {}", filename));
        let mut writer = BufWriter::new(file);
        writer.write_all(
            quote! {
                use crate as pg_sys;
                use pgx_macros::*;
                use crate::common::*;
            }
            .to_string()
            .as_bytes(),
        )
        .expect(&format!("failed to write to {}", filename));
        for item in items {
            writer.write_all(item.as_bytes())
                .expect(&format!("failed to write to {}", filename));
        }
        writer.flush().expect(&format!("failed to flush {}", filename));
        rust_fmt(filename)
            .unwrap_or_else(|e| panic!("unable to run rustfmt for {}: {:?}", filename, e));
    }

    fn write_common_file(filename: &str, items: &BTreeSet<String>) {
        let mut file = std::fs::File::create(filename).expect("failed to create common.rs");
        file.write_all(
            quote! {
                use crate as pg_sys;
                use pgx_macros::*;

                #[cfg(feature = "pg10")]
                use crate::pg10_specific::*;
                #[cfg(feature = "pg11")]
                use crate::pg11_specific::*;
                #[cfg(feature = "pg12")]
                use crate::pg12_specific::*;
            }
            .to_string()
            .as_bytes(),
        )
        .expect("failed to write to common.rs");

        for item in items.iter() {
            file.write_all(item.as_bytes())
                .expect("failed to write to common.rs");
        }
        rust_fmt(filename)
            .unwrap_or_else(|e| panic!("unable to run rustfmt for {}: {:?}", filename, e));
    }
}
