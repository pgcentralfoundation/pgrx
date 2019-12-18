use bindgen::callbacks::MacroParsingBehavior;
use pg_guard_rewriter::{PgGuardRewriter, RewriteMode};
use quote::quote;
use rayon::prelude::*;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::str::FromStr;
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

fn make_git_repo_path(out_dir: String, branch_name: &str) -> PathBuf {
    let mut pg_git_path = PathBuf::from(out_dir);
    // backup 4 directories
    pg_git_path.pop();
    pg_git_path.pop();
    pg_git_path.pop();
    pg_git_path.pop();

    // and a new dir named "pg_git_repo"
    pg_git_path.push(branch_name);

    // return the path we built
    pg_git_path
}

fn main() -> Result<(), std::io::Error> {
    let pg_git_repo_url = "git://git.postgresql.org/git/postgresql.git";

    &vec![
        ("pg10", "REL_10_STABLE"),
        ("pg11", "REL_11_STABLE"),
        ("pg12", "REL_12_STABLE"),
    ]
    .par_iter()
    .for_each(|v| {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let version = v.0;
        let branch_name = v.1;
        let mut output_rs = PathBuf::new();
        output_rs.push(format!("src/pg_sys/{}.rs", version));
        let pg_git_path = make_git_repo_path(out_dir, branch_name);

        let need_generate = git_clone_postgres(&pg_git_path, pg_git_repo_url, branch_name)
            .expect(&format!("Unable to git clone {}", pg_git_repo_url));

        if !need_generate && output_rs.is_file() {
            eprintln!("{} already exists:  skipping", output_rs.to_str().unwrap());
            return;
        }

        git_clean(&pg_git_path, &branch_name)
            .expect(&format!("Unable to switch to branch {}", branch_name));

        configure_and_make(&pg_git_path, &branch_name).expect(&format!(
            "Unable to make clean and configure postgres branch {}",
            branch_name
        ));

        let bindings = bindgen::Builder::default()
            .header(format!("include/{}.h", version))
            .clang_arg(&format!("-I{}/src/include", pg_git_path.to_str().unwrap()))
            .parse_callbacks(Box::new(IgnoredMacros::default()))
            .rustfmt_bindings(true)
            .derive_debug(false)
            .layout_tests(false)
            .generate()
            .expect(&format!("Unable to generate bindings for {}", version));

        let bindings = apply_pg_guard(bindings.to_string()).unwrap();
        std::fs::write(output_rs.clone(), bindings)
            .expect(&format!("Unable to save bindings for {}", version));

        rust_fmt(output_rs.as_path(), &branch_name)
            .expect(&format!("Unable to run rustfmt for {}", version));
    });

    Ok(())
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
            // do a fetch instead
            let output = run_command(
                Command::new("git")
                    .arg("fetch")
                    .arg("--all")
                    .current_dir(path),
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

fn configure_and_make(path: &Path, branch_name: &str) -> Result<(), std::io::Error> {
    run_command(
        Command::new("sh")
            .arg("-c")
            .arg("./configure")
            .env_clear()
            .current_dir(path),
        branch_name,
    )?;

    let num_jobs = u32::from_str(std::env::var("NUM_JOBS").unwrap().as_str()).unwrap();
    run_command(
        Command::new("make")
            .arg("-j")
            .arg(format!("{}", num_jobs / 3))
            .env_clear()
            .current_dir(path),
        branch_name,
    )?;

    Ok(())
}

fn run_command(command: &mut Command, branch_name: &str) -> Result<Output, std::io::Error> {
    let mut dbg = String::new();

    dbg.push_str(&format!(
        "[{}]: -------- {:?} -------- \n",
        branch_name, command
    ));

    let output = command.output()?;
    let rc = output.clone();

    if !output.stdout.is_empty() {
        for line in String::from_utf8(output.stdout).unwrap().lines() {
            dbg.push_str(&format!("[{}] [stdout]: {}\n", branch_name, line));
        }
    }

    if !output.stderr.is_empty() {
        for line in String::from_utf8(output.stderr).unwrap().lines() {
            dbg.push_str(&format!("[{}] [stderr]: {}\n", branch_name, line));
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

    for item in file.items.into_iter() {
        match item {
            Item::ForeignMod(block) => {
                let block = rewriter.extern_block(block);
                stream.extend(quote! { #block });
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
