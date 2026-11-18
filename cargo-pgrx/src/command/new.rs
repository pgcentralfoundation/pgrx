//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use eyre::eyre;
use std::path::{Path, PathBuf};

use crate::CommandExecute;

/// Create a new extension crate
#[derive(clap::Args, Debug)]
#[clap(author)]
pub(crate) struct New {
    /// The name of the extension
    name: String,
    /// The directory to create it in, created if it does not exist (default is `./<name>`)
    #[clap(value_parser)]
    path: Option<PathBuf>,
    /// Overwrite template files that the directory already contains
    #[clap(long)]
    force: bool,
    /// Create a background worker template
    #[clap(long, short)]
    bgworker: bool,
    #[clap(from_global, action = ArgAction::Count)]
    verbose: u8,
}

impl CommandExecute for New {
    #[tracing::instrument(level = "error", skip(self))]
    fn execute(self) -> eyre::Result<()> {
        validate_extension_name(&self.name)?;
        let path = self.path.unwrap_or_else(|| PathBuf::from(&self.name));
        create_crate_template(&path, &self.name, self.bgworker, self.force)
    }
}

fn validate_extension_name(extname: &str) -> eyre::Result<()> {
    for c in extname.chars() {
        if !c.is_alphanumeric() && c != '_' && !c.is_lowercase() {
            return Err(eyre!("Extension name must be in the set of [a-z0-9_]"));
        }
    }
    Ok(())
}

/// `cargo pgrx install` writes the generated schema here, so the template owns the
/// directory even though it starts out empty.
const TEMPLATE_DIRS: [&str; 1] = ["sql"];

/// Every file the template writes, each path relative to the extension's directory.
///
/// The conflict check and the writing below both walk this list, so a file cannot
/// be written without also being checked for.
fn template_files(name: &str, is_bgworker: bool) -> Vec<(PathBuf, Vec<u8>)> {
    let lib_rs = if is_bgworker {
        format!(include_str!("../templates/bgworker_lib_rs"), name = name)
    } else {
        format!(include_str!("../templates/lib_rs"), name = name)
    };
    vec![
        (
            PathBuf::from(format!("{name}.control")),
            format!(include_str!("../templates/control"), name = name).into_bytes(),
        ),
        (
            PathBuf::from("Cargo.toml"),
            format!(include_str!("../templates/cargo_toml"), name = name).into_bytes(),
        ),
        (
            PathBuf::from_iter([".cargo", "config.toml"]),
            include_bytes!("../templates/cargo_config_toml").to_vec(),
        ),
        (PathBuf::from_iter(["src", "lib.rs"]), lib_rs.into_bytes()),
        (PathBuf::from(".gitignore"), include_bytes!("../templates/gitignore").to_vec()),
        (
            PathBuf::from_iter(["tests", "pg_regress", "sql", "setup.sql"]),
            format!(include_str!("../templates/setup_sql"), name = name).into_bytes(),
        ),
        (
            PathBuf::from_iter(["tests", "pg_regress", "expected", "setup.out"]),
            format!(include_str!("../templates/setup_out"), name = name).into_bytes(),
        ),
    ]
}

/// Write the extension template into `root`, which may already exist.
///
/// Unless `force`, this refuses to replace any template file the directory already
/// has, and writes nothing at all in that case, so a refusal never leaves a
/// half-written crate behind.
#[tracing::instrument(skip_all, fields(root, name))]
fn create_crate_template(
    root: &Path,
    name: &str,
    is_bgworker: bool,
    force: bool,
) -> eyre::Result<()> {
    let files = template_files(name, is_bgworker);

    if !force {
        let existing = files
            .iter()
            .map(|(relative, _)| relative)
            .filter(|relative| root.join(relative).exists())
            .map(|relative| format!("  {}", relative.display()))
            .collect::<Vec<_>>();
        if !existing.is_empty() {
            return Err(eyre!(
                "refusing to overwrite files that `{}` already contains:\n{}\npass --force to overwrite them",
                root.display(),
                existing.join("\n"),
            ));
        }
    }

    for directory in TEMPLATE_DIRS {
        let directory = root.join(directory);
        std::fs::create_dir_all(&directory)
            .map_err(|e| eyre!("could not create `{}`: {e}", directory.display()))?;
    }

    for (relative, contents) in files {
        let destination = root.join(relative);
        if let Some(parent) = destination.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| eyre!("could not create `{}`: {e}", parent.display()))?;
        }
        std::fs::write(&destination, contents)
            .map_err(|e| eyre!("could not write `{}`: {e}", destination.display()))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{TEMPLATE_DIRS, create_crate_template, template_files};
    use std::path::Path;

    fn written_paths(root: &Path) -> Vec<String> {
        fn walk(directory: &Path, root: &Path, found: &mut Vec<String>) {
            for entry in std::fs::read_dir(directory).unwrap() {
                let path = entry.unwrap().path();
                if path.is_dir() {
                    walk(&path, root, found);
                } else {
                    found.push(path.strip_prefix(root).unwrap().display().to_string());
                }
            }
        }
        let mut found = Vec::new();
        walk(root, root, &mut found);
        found.sort();
        found
    }

    #[test]
    fn writes_every_template_file_and_directory() {
        let root = tempfile::tempdir().unwrap();
        create_crate_template(root.path(), "my_ext", false, false).unwrap();

        let mut expected = template_files("my_ext", false)
            .into_iter()
            .map(|(relative, _)| relative.display().to_string())
            .collect::<Vec<_>>();
        expected.sort();
        assert_eq!(written_paths(root.path()), expected);

        for directory in TEMPLATE_DIRS {
            assert!(root.path().join(directory).is_dir(), "{directory} was not created");
        }
    }

    #[test]
    fn defaults_to_the_plain_library_template() {
        let root = tempfile::tempdir().unwrap();
        create_crate_template(root.path(), "my_ext", false, false).unwrap();
        let lib_rs = std::fs::read_to_string(root.path().join("src/lib.rs")).unwrap();
        assert!(!lib_rs.contains("BackgroundWorker"), "{lib_rs}");
    }

    #[test]
    fn bgworker_selects_the_background_worker_template() {
        let root = tempfile::tempdir().unwrap();
        create_crate_template(root.path(), "my_ext", true, false).unwrap();
        let lib_rs = std::fs::read_to_string(root.path().join("src/lib.rs")).unwrap();
        assert!(lib_rs.contains("BackgroundWorker"), "{lib_rs}");
    }

    #[test]
    fn refuses_to_overwrite_and_writes_nothing() {
        let root = tempfile::tempdir().unwrap();
        std::fs::create_dir(root.path().join("src")).unwrap();
        std::fs::write(root.path().join("src/lib.rs"), "fn mine() {}").unwrap();
        std::fs::write(root.path().join("Cargo.toml"), "mine").unwrap();

        let error = create_crate_template(root.path(), "my_ext", false, false)
            .expect_err("an existing lib.rs and Cargo.toml must stop the template")
            .to_string();
        assert!(error.contains("Cargo.toml"), "{error}");
        assert!(error.contains(&Path::new("src").join("lib.rs").display().to_string()), "{error}");
        assert!(error.contains("--force"), "{error}");

        // the refusal must not leave a half-written crate behind
        assert_eq!(
            std::fs::read_to_string(root.path().join("src/lib.rs")).unwrap(),
            "fn mine() {}"
        );
        assert_eq!(std::fs::read_to_string(root.path().join("Cargo.toml")).unwrap(), "mine");
        assert_eq!(written_paths(root.path()), ["Cargo.toml", "src/lib.rs"]);
    }

    #[test]
    fn force_overwrites_existing_template_files() {
        let root = tempfile::tempdir().unwrap();
        std::fs::write(root.path().join("Cargo.toml"), "mine").unwrap();

        create_crate_template(root.path(), "my_ext", false, true).unwrap();

        let cargo_toml = std::fs::read_to_string(root.path().join("Cargo.toml")).unwrap();
        assert!(cargo_toml.contains("my_ext"), "{cargo_toml}");
    }

    #[test]
    fn a_file_the_template_does_not_own_is_not_a_conflict() {
        let root = tempfile::tempdir().unwrap();
        std::fs::write(root.path().join("notes.txt"), "keep me").unwrap();

        create_crate_template(root.path(), "my_ext", false, false).unwrap();

        assert_eq!(std::fs::read_to_string(root.path().join("notes.txt")).unwrap(), "keep me");
    }
}
