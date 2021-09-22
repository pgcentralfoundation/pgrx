// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use std::io::Write;
use std::fs;
use std::path::PathBuf;
use std::os::unix::fs::PermissionsExt;

pub(crate) fn create_crate_template(
    path: PathBuf,
    name: &str,
    is_bgworker: bool,
) -> Result<(), std::io::Error> {
    create_directory_structure(&path)?;
    create_control_file(&path, name)?;
    create_cargo_toml(&path, name)?;
    create_dotcargo_config(&path, name)?;
    create_lib_rs(&path, name, is_bgworker)?;
    create_git_ignore(&path, name)?;

    Ok(())
}

fn create_directory_structure(path: &PathBuf) -> Result<(), std::io::Error> {
    let mut src_dir = path.clone();

    src_dir.push("src");
    std::fs::create_dir_all(&src_dir)?;

    src_dir.pop();
    src_dir.push(".cargo");
    std::fs::create_dir_all(&src_dir)?;

    src_dir.pop();
    src_dir.push("sql");
    std::fs::create_dir_all(&src_dir)
}

fn create_control_file(path: &PathBuf, name: &str) -> Result<(), std::io::Error> {
    let mut filename = path.clone();

    filename.push(format!("{}.control", name));
    let mut file = std::fs::File::create(filename)?;

    file.write_all(&format!(include_str!("../templates/control"), name = name).as_bytes())?;

    Ok(())
}

fn create_cargo_toml(path: &PathBuf, name: &str) -> Result<(), std::io::Error> {
    let mut filename = path.clone();

    filename.push("Cargo.toml");
    let mut file = std::fs::File::create(filename)?;

    file.write_all(&format!(include_str!("../templates/cargo_toml"), name = name).as_bytes())?;

    Ok(())
}

fn create_dotcargo_config(path: &PathBuf, _name: &str) -> Result<(), std::io::Error> {
    let mut cargo_dir = path.clone();
    cargo_dir.push(".cargo");
    if !cargo_dir.exists() {
        std::fs::create_dir(&cargo_dir).expect(format!("cannot create dir {}", cargo_dir.clone().display()).as_str());
    }

    let mut filepath = cargo_dir.clone();
    filepath.push("config");
    let mut file =  std::fs::File::create(&filepath).expect(format!("cannot create {}", filepath.clone().display()).as_str());
    file.write_all(include_bytes!("../templates/cargo_config")).expect(format!("cannot write {}", filepath.clone().display()).as_str());


    let mut filepath = cargo_dir.clone();
    filepath.push("pgx-linker-script.sh");
    let mut file =  std::fs::File::create(&filepath).expect(format!("cannot create {}", filepath.clone().display()).as_str());
    file.write_all(include_bytes!("../templates/pgx-linker-script.sh")).expect(format!("cannot write {}", filepath.clone().display()).as_str());
    fs::set_permissions(&filepath, PermissionsExt::from_mode(0o771)).expect(format!("cannot set executable {}", filepath.clone().display()).as_str());

    let mut filepath = cargo_dir.clone();
    filepath.push("pgx-dynamic-list.txt");
    let mut file =  std::fs::File::create(&filepath).expect(format!("cannot create {}", filepath.clone().display()).as_str());
    file.write_all(include_bytes!("../templates/pgx-dynamic-list.txt")).expect(format!("cannot write {}", filepath.clone().display()).as_str());

    Ok(())
}

fn create_lib_rs(path: &PathBuf, name: &str, is_bgworker: bool) -> Result<(), std::io::Error> {
    let mut filename = path.clone();

    filename.push("src");
    filename.push("lib.rs");
    let mut file = std::fs::File::create(filename)?;

    if is_bgworker {
        file.write_all(
            &format!(include_str!("../templates/bgworker_lib_rs"), name = name).as_bytes(),
        )?;
    } else {
        file.write_all(&format!(include_str!("../templates/lib_rs"), name = name).as_bytes())?;
    }

    Ok(())
}

fn create_git_ignore(path: &PathBuf, _name: &str) -> Result<(), std::io::Error> {
    let mut filename = path.clone();

    filename.push(".gitignore");
    let mut file = std::fs::File::create(filename)?;

    file.write_all(include_bytes!("../templates/gitignore"))?;

    Ok(())
}
