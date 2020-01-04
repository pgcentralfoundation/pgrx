use std::process::{Command, Stdio};
use std::result::Result;

pub(crate) fn install_extension(target: Option<&str>) -> Result<(), std::io::Error> {
    let is_release = target.unwrap_or("").eq("release");

    let mut command = Command::new("cargo");
    command.arg("build");

    if target.is_some() {
        match target.unwrap() {
            "release" => {
                command.arg("--release");
            }
            "debug" => {
                // noop
            }
            _ => panic!("unsupported installation build target, expect 'debug' or 'release'"),
        }
    }

    let mut process = command
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    let status = process.wait()?;
    if !status.success() {
        return Err(std::io::Error::from_raw_os_error(status.code().unwrap()));
    }

    let pkgdir = get_pkglibdir();
    let extdir = get_extensiondir();
    let (control_file, extname) = find_control_file()?;
    let (libpath, libfile) = find_library_file(&extname, is_release)?;

    println!("copying control file ({}) to: {}", control_file, extdir);
    std::fs::copy(control_file.clone(), format!("{}/{}", extdir, control_file))?;

    println!("copying library ({}) to: {}", libfile, pkgdir);
    std::fs::copy(
        format!("{}/{}", libpath, libfile),
        format!("{}/{}.so", pkgdir, extname),
    )?;

    copy_sql_files(&extdir)?;

    Ok(())
}

fn copy_sql_files(extdir: &str) -> Result<(), std::io::Error> {
    for f in std::fs::read_dir("sql/")? {
        if f.is_ok() {
            let f = f.unwrap();
            let filename = f.file_name().into_string().unwrap();

            if filename.ends_with(".sql") {
                let dest = format!("{}/{}", extdir, filename);

                println!("copying SQL ({}) to: {}", filename, dest);
                std::fs::copy(f.path(), dest)?;
            }
        }
    }

    Ok(())
}

fn find_library_file(extname: &str, is_release: bool) -> Result<(String, String), std::io::Error> {
    let path = if is_release {
        "target/release"
    } else {
        "target/debug"
    };
    for f in std::fs::read_dir(path)? {
        if f.is_ok() {
            let f = f.unwrap();
            let filename = f.file_name().into_string().unwrap();

            if filename.contains(extname)
                && filename.starts_with("lib")
                && (filename.ends_with(".so")
                    || filename.ends_with(".dylib")
                    || filename.ends_with(".dll"))
            {
                return Ok((path.to_string(), filename));
            }
        }
    }

    panic!("couldn't find library file");
}

fn find_control_file() -> Result<(String, String), std::io::Error> {
    for f in std::fs::read_dir(".")? {
        if f.is_ok() {
            let f = f?;
            if f.file_name().to_string_lossy().ends_with(".control") {
                let filename = f.file_name().into_string().unwrap();
                let mut extname: Vec<&str> = filename.split('.').collect();
                extname.pop();
                let extname = extname.pop().unwrap();
                return Ok((filename.clone(), extname.to_string()));
            }
        }
    }

    panic!("couldn't find control file");
}

fn get_pkglibdir() -> String {
    run_pg_config("--pkglibdir")
}

fn get_extensiondir() -> String {
    let mut dir = run_pg_config("--sharedir");

    dir.push_str("/extension");
    dir
}

fn run_pg_config(arg: &str) -> String {
    let output = Command::new("pg_config")
        .arg(arg)
        .output()
        .expect("couldn't run 'pg_config'");

    String::from_utf8(output.stdout).unwrap().trim().to_string()
}
