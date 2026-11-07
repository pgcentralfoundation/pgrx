//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use crate::CommandExecute;
use eyre::{WrapErr, eyre};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Get a property from the extension control file
#[derive(clap::Args, Debug)]
#[clap(author)]
pub(crate) struct Get {
    /// One of the properties from `$EXTENSION.control`
    name: String,
    #[clap(from_global, action = ArgAction::Count)]
    verbose: u8,
    /// Package to determine default `pg_version` with (see `cargo help pkgid`)
    #[clap(long, short)]
    package: Option<String>,
    /// Path to Cargo.toml
    #[clap(long, value_parser)]
    manifest_path: Option<PathBuf>,
}

impl CommandExecute for Get {
    #[tracing::instrument(level = "error", skip(self))]
    fn execute(self) -> eyre::Result<()> {
        let metadata =
            crate::metadata::metadata(&Default::default(), self.manifest_path.as_deref(), &[])
                .wrap_err("couldn't get cargo metadata")?;
        crate::metadata::validate(self.manifest_path.as_deref(), &metadata)?;
        let package_manifest_path =
            crate::manifest::manifest_path(&metadata, self.package.as_deref())
                .wrap_err("Couldn't get manifest path")?;

        if let Some(value) = get_property(&package_manifest_path, &self.name)? {
            println!("{value}");
        }
        Ok(())
    }
}

#[tracing::instrument(level = "error", skip_all, fields(
    %name,
    manifest_path = %manifest_path.display(),
))]
pub fn get_property(manifest_path: &Path, name: &str) -> eyre::Result<Option<String>> {
    let (control_file, extname) = find_control_file(manifest_path)?;

    if name == "extname" {
        return Ok(Some(extname));
    } else if name == "git_hash" {
        return determine_git_hash();
    }

    let control_file = File::open(&control_file)
        .wrap_err_with(|| eyre!("could not find control file `{}`", control_file.display()))?;
    let reader = BufReader::new(control_file);

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split('=').collect();

        if parts.len() != 2 {
            continue;
        }

        let (k, v) = (parts.first().unwrap().trim(), parts.get(1).unwrap().trim());

        if k == name {
            let v = v.trim_start_matches('\'');
            let v = v.trim_end_matches('\'');
            return Ok(Some(v.trim().to_string()));
        }
    }

    Ok(None)
}

/// Find the crate's one `.control` file, whose stem names the extension. This counts files as
/// `pg_module_magic!` does when it records the extension that `rust_regtypein` looks types up in:
/// names that start with `.`, such as editor lock files, and entries that are not files or links
/// to files do not count.
pub(crate) fn find_control_file(manifest_path: &Path) -> eyre::Result<(PathBuf, String)> {
    let parent = manifest_path
        .parent()
        .ok_or_else(|| eyre!("could not get parent of `{}`", manifest_path.display()))?;

    let controls = std::fs::read_dir(parent)
        .wrap_err_with(|| {
            eyre!("cannot open current directory `{}` for reading", parent.display())
        })?
        .flatten()
        .map(|f| f.path())
        .filter(|path| {
            path.extension() == Some("control".as_ref())
                && !path.file_name().is_some_and(|name| name.as_encoded_bytes().starts_with(b"."))
                && std::fs::metadata(path).is_ok_and(|metadata| metadata.is_file())
        })
        .collect::<Vec<_>>();
    let f_path = match controls.as_slice() {
        [] => return Err(eyre!("control file not found in `{}`", manifest_path.display())),
        [control] => control.clone(),
        _ => {
            return Err(eyre!(
                "found {} control files in `{}`; an extension crate needs exactly one",
                controls.len(),
                parent.display()
            ));
        }
    };
    let file_stem = f_path
        .file_stem()
        .ok_or_else(|| eyre!("could not get file stem of `{}`", f_path.display()))?;
    let file_stem = file_stem
        .to_str()
        .ok_or_else(|| eyre!("could not get file stem as String from `{}`", f_path.display()))?
        .to_string();
    Ok((f_path, file_stem))
}

fn determine_git_hash() -> eyre::Result<Option<String>> {
    match Command::new("git").arg("rev-parse").arg("HEAD").output() {
        Ok(output) => {
            if !output.status.success() {
                let stderr = String::from_utf8(output.stderr)
                    .expect("`git rev-parse head` did not return valid utf8");
                return Err(eyre!(
                    "problem running `git` to determine the current revision hash: {stderr}"
                ));
            }

            Ok(Some(
                String::from_utf8(output.stdout)
                    .expect("`git rev-parse head` did not return valid utf8")
                    .trim()
                    .into(),
            ))
        }
        Err(e) => Err(e).wrap_err("problem running `git` to determine the current revision hash"),
    }
}

#[cfg(test)]
mod tests {
    use super::find_control_file;

    #[test]
    #[cfg(unix)]
    fn find_control_file_counts_one_visible_file() {
        use std::os::unix::fs::symlink;

        let dir = tempfile::tempdir().unwrap();
        let manifest = dir.path().join("Cargo.toml");
        assert!(find_control_file(&manifest).is_err());

        std::fs::write(dir.path().join("ext.control"), "").unwrap();
        // An Emacs lock file, a link to a file that does not exist
        symlink("user@host.1234:5678", dir.path().join(".#ext.control")).unwrap();
        symlink("missing.control", dir.path().join("dangling.control")).unwrap();
        let (path, name) = find_control_file(&manifest).unwrap();
        assert_eq!((path, name.as_str()), (dir.path().join("ext.control"), "ext"));

        std::fs::write(dir.path().join("ext_old.control"), "").unwrap();
        let error = find_control_file(&manifest).unwrap_err().to_string();
        assert!(error.starts_with("found 2 control files"), "{error}");
    }
}
