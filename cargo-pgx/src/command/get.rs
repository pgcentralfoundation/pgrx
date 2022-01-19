// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use eyre::{eyre, WrapErr};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::Command;

use crate::CommandExecute;

/// Get a property from the extension control file
#[derive(clap::Args, Debug)]
#[clap(author)]
pub(crate) struct Get {
    /// One of the properties from `$EXTENSION.control`
    name: String,
    #[clap(from_global, parse(from_occurrences))]
    verbose: usize,
}

impl CommandExecute for Get {
    #[tracing::instrument(level = "error", skip(self))]
    fn execute(self) -> eyre::Result<()> {
        if let Some(value) = get_property(&self.name)? {
            println!("{}", value);
        }
        Ok(())
    }
}

#[tracing::instrument(level = "error")]
pub fn get_property(name: &str) -> eyre::Result<Option<String>> {
    let (control_file, extname) = find_control_file()?;

    if name == "extname" {
        return Ok(Some(extname));
    } else if name == "git_hash" {
        return determine_git_hash();
    }

    let control_file = File::open(control_file).unwrap();
    let reader = BufReader::new(control_file);

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split('=').collect();

        if parts.len() != 2 {
            continue;
        }

        let (k, v) = (parts.get(0).unwrap().trim(), parts.get(1).unwrap().trim());

        if k == name {
            let v = v.trim_start_matches('\'');
            let v = v.trim_end_matches('\'');
            return Ok(Some(v.trim().to_string()));
        }
    }

    Ok(None)
}

pub(crate) fn find_control_file() -> eyre::Result<(PathBuf, String)> {
    for f in std::fs::read_dir(".").wrap_err("cannot open current directory for reading")? {
        if f.is_ok() {
            if let Ok(f) = f {
                if f.file_name().to_string_lossy().ends_with(".control") {
                    let filename = f.file_name().into_string().unwrap();
                    let mut extname: Vec<&str> = filename.split('.').collect();
                    extname.pop();
                    let extname = extname.pop().unwrap();
                    return Ok((filename.clone().into(), extname.to_string()));
                }
            }
        }
    }

    Err(eyre!("control file not found in current directory"))
}

fn determine_git_hash() -> eyre::Result<Option<String>> {
    match Command::new("git").arg("rev-parse").arg("HEAD").output() {
        Ok(output) => {
            if !output.status.success() {
                let stderr = String::from_utf8(output.stderr)
                    .expect("`git rev-parse head` did not return valid utf8");
                return Err(eyre!(
                    "problem running `git` to determine the current revision hash: {}",
                    stderr
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
