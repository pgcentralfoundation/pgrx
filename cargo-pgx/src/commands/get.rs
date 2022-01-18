// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use pgx_utils::{exit_with_error, handle_result};
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
}

impl CommandExecute for Get {
    fn execute(self) -> std::result::Result<(), std::io::Error> {
        if let Some(value) = get_property(&self.name) {
            println!("{}", value);
        }
        Ok(())
    }
}

pub fn get_property(name: &str) -> Option<String> {
    let (control_file, extname) = find_control_file();

    if name == "extname" {
        return Some(extname);
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
            return Some(v.trim().to_string());
        }
    }

    None
}

pub(crate) fn find_control_file() -> (PathBuf, String) {
    handle_result!(pgx_utils::find_control_file(), "cannot find control file")
}

fn determine_git_hash() -> Option<String> {
    match Command::new("git").arg("rev-parse").arg("HEAD").output() {
        Ok(output) => {
            if !output.status.success() {
                let stderr = String::from_utf8(output.stderr)
                    .expect("`git rev-parse head` did not return valid utf8");
                exit_with_error!(
                    "problem running `git` to determine the current revision hash: {}",
                    stderr
                );
            }

            Some(
                String::from_utf8(output.stdout)
                    .expect("`git rev-parse head` did not return valid utf8")
                    .trim()
                    .into(),
            )
        }
        Err(e) => exit_with_error!(
            "problem running `git` to determine the current revision hash: {}",
            e
        ),
    }
}
