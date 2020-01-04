extern crate build_deps;

use bindgen::callbacks::MacroParsingBehavior;
use pg_guard_rewriter::{PgGuardRewriter, RewriteMode};
use quote::quote;
use rayon::prelude::*;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
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

fn make_git_repo_path(out_dir: &str, branch_name: &str) -> PathBuf {
    let mut pg_git_path = PathBuf::from(out_dir);
    // backup 4 directories
    pg_git_path.pop();
    pg_git_path.pop();
    pg_git_path.pop();
    pg_git_path.pop();

    // and a new dir named "pg_git_repo"
    pg_git_path.push(branch_name);

    pg_git_path
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

    // and a new dir named "bridge-c-shim"
    shim_dir.push("bridge-c-shim");

    shim_dir
}

fn main() -> Result<(), std::io::Error> {
    build_deps::rerun_if_changed_paths("include/*").unwrap();
    build_deps::rerun_if_changed_paths("../pg-guard-attr/src/lib.rs").unwrap();
    build_deps::rerun_if_changed_paths("../pg-guard-rewriter/src/lib.rs").unwrap();
    build_deps::rerun_if_changed_paths("../bindings-diff/*").unwrap();
    build_deps::rerun_if_changed_paths("../bindings-diff/src/*").unwrap();
    build_deps::rerun_if_changed_paths("../bridge-c-shim/bridge-c-shim.c").unwrap();

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let cwd = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let pg_git_repo_url = "git://git.postgresql.org/git/postgresql.git";
    let build_rs = PathBuf::from("build.rs");
    let shim_dir = make_shim_path(&manifest_dir);
    let regen_flag = Arc::new(AtomicBool::new(false));

    let shim_mutex = Mutex::new(());
    &vec![
        ("pg10", "REL_10_STABLE", "8810"),
        ("pg11", "REL_11_STABLE", "8811"),
        ("pg12", "REL_12_STABLE", "8812"),
    ]
    .par_iter()
    .for_each(|v| {
        let version = v.0;
        let branch_name = v.1;
        let port_no = u16::from_str(v.2).unwrap();
        let pg_git_path = make_git_repo_path(&out_dir, branch_name);
        let include_path = make_include_path(&pg_git_path);
        let output_rs = PathBuf::from(format!("src/pg_sys/{}_bindings.rs", version));
        let include_h = PathBuf::from(format!("include/{}.h", version));
        let config_status = PathBuf::from(format!("{}/config.status", pg_git_path.display()));
        let common_rs = PathBuf::from(format!("src/pg_sys/common.rs"));
        let version_specific_rs = PathBuf::from(format!("src/pg_sys/{}_specific.rs", version));

        if !common_rs.is_file() || !version_specific_rs.is_file() {
            regen_flag.store(true, Ordering::SeqCst);
        }

        let need_configure_and_make =
            git_clone_postgres(&pg_git_path, pg_git_repo_url, branch_name)
                .expect(&format!("Unable to git clone {}", pg_git_repo_url));

        if need_configure_and_make || !config_status.is_file() {
            eprintln!("[{}] cleaning and building", branch_name);

            git_clean(&pg_git_path, &branch_name)
                .expect(&format!("Unable to switch to branch {}", branch_name));

            configure_and_make(&pg_git_path, &branch_name, port_no).expect(&format!(
                "Unable to make clean and configure postgres branch {}",
                branch_name
            ));
        } else if output_rs.is_file()
            && std::fs::metadata(&build_rs)
                .unwrap()
                .modified()
                .unwrap()
                .gt(&std::fs::metadata(&output_rs).unwrap().modified().unwrap())
        {
            eprintln!(
                "[{}] build.rs is newer, removing and re-generating {}",
                branch_name,
                output_rs.display()
            );
            std::fs::remove_file(&output_rs)
                .expect(&format!("couldn't delete {}", output_rs.display()));
        } else if output_rs.is_file()
            && std::fs::metadata(&include_h)
                .unwrap()
                .modified()
                .unwrap()
                .lt(&std::fs::metadata(&output_rs).unwrap().modified().unwrap())
        {
            eprintln!("{} is up-to-date:  skipping", output_rs.display());
            build_shim(&shim_dir, &shim_mutex, &pg_git_path, version);
            return;
        }

        eprintln!(
            "[{}] Running bindgen on {} with {}",
            branch_name,
            output_rs.display(),
            include_path.display()
        );
        let bindings = bindgen::Builder::default()
            .header(include_h.to_str().unwrap())
            .clang_arg(&format!("-I{}", include_path.display()))
            .parse_callbacks(Box::new(IgnoredMacros::default()))
            .rustfmt_bindings(true)
            .derive_debug(true)
            .layout_tests(false)
            .generate()
            .expect(&format!("Unable to generate bindings for {}", version));

        let bindings = apply_pg_guard(bindings.to_string()).unwrap();
        std::fs::write(output_rs.clone(), bindings)
            .expect(&format!("Unable to save bindings for {}", version));

        rust_fmt(output_rs.as_path(), &branch_name)
            .expect(&format!("Unable to run rustfmt for {}", version));
        regen_flag.store(true, Ordering::SeqCst);

        build_shim(&shim_dir, &shim_mutex, &pg_git_path, version);
    });

    if regen_flag.load(Ordering::SeqCst) {
        generate_common_rs(cwd);
    }

    Ok(())
}

fn build_shim(
    shim_dir: &PathBuf,
    shim_mutex: &Mutex<()>,
    pg_git_path: &PathBuf,
    version: &str,
) -> () {
    // build the shim under a lock b/c this can't be built concurrently
    let _ = shim_mutex.lock().expect("couldn't obtain shim_mutex");

    // then build the shim for the version feature currently being built
    // and tell rustc to link to the library that was built
    if version.eq("pg10") && std::env::var("CARGO_FEATURE_PG10").is_ok() {
        build_shim_for_version(&shim_dir, &pg_git_path, 10).expect("shim build for pg10 failed");
        println!("cargo:rustc-link-search={}", shim_dir.display());
        println!("cargo:rustc-link-lib=static=bridge-c-shim-10");
    } else if version.eq("pg11") && std::env::var("CARGO_FEATURE_PG11").is_ok() {
        build_shim_for_version(&shim_dir, &pg_git_path, 11).expect("shim build for pg11 failed");
        println!("cargo:rustc-link-search={}", shim_dir.display());
        println!("cargo:rustc-link-lib=static=bridge-c-shim-11");
    } else if version.eq("pg12") && std::env::var("CARGO_FEATURE_PG12").is_ok() {
        build_shim_for_version(&shim_dir, &pg_git_path, 12).expect("shim build for pg12 failed");
        println!("cargo:rustc-link-search={}", shim_dir.display());
        println!("cargo:rustc-link-lib=static=bridge-c-shim-12");
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
            .env("PG_TARGET_VERSION", format!("{}", version_no))
            .env("PATH", path_env)
            .current_dir(shim_dir),
        &format!("shim for PG v{}", version_no),
    )?;

    if rc.status.code().unwrap() != 0 {
        panic!("failed to make bridge-c-shim for v{}", version_no);
    }

    Ok(())
}

fn generate_common_rs(mut cwd: PathBuf) -> () {
    eprintln!("[all branches] Regenerating common.rs and XX_specific.rs files...");
    cwd.pop();
    let rc = run_command(
        Command::new("cargo")
            .arg("run")
            .arg("--release")
            .arg("--bin")
            .arg("bindings-diff")
            .current_dir(cwd),
        "all branches",
    )
    .expect("bindings-diff failed");
    if rc.status.code().expect("expected a status code") != 0 {
        panic!("bindings-diff failed")
    }
}

fn git_clone_postgres(
    path: &Path,
    repo_url: &str,
    branch_name: &str,
) -> Result<bool, std::io::Error> {
    if path.exists() {
        let mut gitdir = path.clone().to_path_buf();
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
    let rewriter = PgGuardRewriter::new(RewriteMode::ApplyPgGuardMacro);
    let mut stream = TokenStream2::new();
    let file = syn::parse_file(input.as_str()).unwrap();

    stream.extend(quote! {use crate::DatumCompatible;});
    for item in file.items.into_iter() {
        match item {
            Item::ForeignMod(block) => {
                let block = rewriter.extern_block(block);
                stream.extend(quote! { #block });
            }
            Item::Struct(item_struct) => {
                let item_struct = rewriter.item_struct(item_struct);
                stream.extend(quote! { #item_struct });
            }
            _ => {
                stream.extend(quote! { #item });
            }
        }
    }

    Ok(format!("{}", stream.into_token_stream()))
}

fn rust_fmt(path: &Path, branch_name: &str) -> Result<(), std::io::Error> {
    run_command(
        Command::new("rustfmt").arg(path).current_dir("."),
        branch_name,
    )?;

    Ok(())
}
