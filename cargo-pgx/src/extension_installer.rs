use std::io::{BufRead, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::result::Result;
use std::str::FromStr;

pub(crate) fn install_extension(target: Option<&str>) -> Result<(), std::io::Error> {
    let is_release = target.unwrap_or("") == "release";

    if &std::env::var("PGX_NO_BUILD").unwrap_or_default() != "true" {
        build_extension(is_release)?;
    } else {
        eprintln!(
            "Skipping build due to $PGX_NO_BUILD=true in {}",
            std::env::current_dir().unwrap().display()
        );
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

    crate::generate_schema()?;
    copy_sql_files(&extdir, &extname)?;

    Ok(())
}

fn build_extension(is_release: bool) -> Result<(), std::io::Error> {
    let mut command = Command::new("cargo");
    command.arg("build");
    if is_release {
        command.arg("--release");
    }

    let mut process = command
        .env_remove("CARGO_MANIFEST_DIR")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;
    let status = process.wait()?;
    if !status.success() {
        return Err(std::io::Error::from_raw_os_error(status.code().unwrap()));
    }

    println!();
    Ok(())
}

fn copy_sql_files(extdir: &str, extname: &str) -> Result<(), std::io::Error> {
    let load_order = crate::schema_generator::read_load_order(
        &PathBuf::from_str("./sql/load-order.txt").unwrap(),
    );
    let target_filename =
        PathBuf::from_str(&format!("{}/{}--{}.sql", extdir, extname, get_version())).unwrap();
    let mut sql = std::fs::File::create(&target_filename).unwrap();

    // write each sql file from load-order.txt to the version.sql file
    for file in load_order {
        let file = PathBuf::from_str(&format!("sql/{}", file)).unwrap();
        let contents = std::fs::read_to_string(&file).unwrap();

        println!(
            "writing {} to {}",
            file.display(),
            target_filename.display()
        );
        sql.write_all("--\n".as_bytes())
            .expect("couldn't write version SQL file");
        sql.write_all(format!("-- {}\n", file.display()).as_bytes())
            .expect("couldn't write version SQL file");
        sql.write_all("--\n".as_bytes())
            .expect("couldn't write version SQL file");
        sql.write_all(contents.as_bytes())
            .expect("couldn't write version SQL file");
        sql.write_all("\n\n\n".as_bytes())
            .expect("couldn't write version SQL file");
    }

    // now copy all the version upgrade files too
    for f in std::fs::read_dir("sql/")? {
        if f.is_ok() {
            let f = f.unwrap();
            let filename = f.file_name().into_string().unwrap();

            if filename.starts_with(&format!("{}--", extname)) && filename.ends_with(".sql") {
                let dest = format!("{}/{}", extdir, filename);

                println!("copying SQL: {} to: {}", filename, dest);
                std::fs::copy(f.path(), dest)?;
            }
        }
    }

    Ok(())
}

fn find_library_file(extname: &str, is_release: bool) -> Result<(String, String), std::io::Error> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or(".".to_string());
    let path = if is_release {
        format!("{}/target/release", manifest_dir)
    } else {
        format!("{}/target/debug", manifest_dir)
    };

    let path = PathBuf::from_str(&path).unwrap();

    if !path.is_dir() {
        eprintln!(
            "build directory {}: Not found.  Try setting CARGO_MANIFEST_DIR",
            path.display()
        );
        std::process::exit(1);
    }

    for f in std::fs::read_dir(&path)? {
        if f.is_ok() {
            let f = f.unwrap();
            let filename = f.file_name().into_string().unwrap();

            if filename.contains(extname)
                && filename.starts_with("lib")
                && (filename.ends_with(".so")
                    || filename.ends_with(".dylib")
                    || filename.ends_with(".dll"))
            {
                return Ok((path.display().to_string(), filename));
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

fn get_version() -> String {
    let control_file = std::fs::File::open(find_control_file().unwrap().0).unwrap();
    let reader = std::io::BufReader::new(control_file);

    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with("default_version") {
            let mut parts: Vec<&str> = line.split("=").collect();
            let mut version = parts.pop().unwrap().trim().to_string();

            version = version.trim_matches('\'').trim().to_string();

            return version;
        }
    }

    panic!("couldn't determine version number");
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
    let pg_config = std::env::var("PG_CONFIG").unwrap_or("pg_config".to_string());
    let output = Command::new(pg_config).arg(arg).output();

    match output {
        Ok(output) => String::from_utf8(output.stdout).unwrap().trim().to_string(),

        Err(e) => {
            panic!("Problem running pg_config: {}", e);
        }
    }
}
