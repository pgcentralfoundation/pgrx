// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use eyre::eyre;
use include_dir::{include_dir, Dir, DirEntry};
use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::{io::Write, path::PathBuf, str::FromStr};

use crate::CommandExecute;

use convert_case::{Case, Casing};
use handlebars::handlebars_helper;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use structmap::{value::Value, GenericMap, StringMap, ToMap};
use structmap_derive::ToMap;

/// Create a new extension crate
#[derive(clap::Args, Debug)]
#[clap(author)]
pub(crate) struct New {
    /// The name of the extension
    name: String,
    /// Create a background worker template
    #[clap(long, short, arg_enum)]
    template: Template,
    #[clap(from_global, parse(from_occurrences))]
    verbose: usize,
}

impl CommandExecute for New {
    #[tracing::instrument(level = "error", skip(self))]
    fn execute(self) -> eyre::Result<()> {
        validate_extension_name(&self.name)?;
        let path = PathBuf::from_str(&format!("{}/", self.name)).unwrap();

        let template_vars = TemplateVars { name: self.name };
        create_crate_template(path, template_vars, &self.template)
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
#[derive(clap::ArgEnum, Debug, Clone)]
pub enum Template {
    BGWORKER,
    FUNCTION,
    AGGREGATE,
}

// This could be improved to detect the directories at compile time
// Could even be a single include and then take the first levels at runtime
static FUNCTION_INCLUDE: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/templates/function/");
static BGWORKER_INCLUDE: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/templates/bgworker/");
static AGGREGATE_INCLUDE: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/templates/aggregate/");

impl Template {
    fn as_payload(&self) -> &Dir<'_> {
        match self {
            Template::FUNCTION => &FUNCTION_INCLUDE,
            Template::BGWORKER => &BGWORKER_INCLUDE,
            Template::AGGREGATE => &AGGREGATE_INCLUDE,
        }
    }
}

#[derive(Serialize, Deserialize, ToMap, Default)]
pub struct TemplateVars {
    name: String,
}

#[tracing::instrument(skip_all, fields(path, name))]
pub(crate) fn create_crate_template(
    path: PathBuf,
    template_vars: TemplateVars,
    template: &Template,
) -> eyre::Result<()> {
    let template_map: BTreeMap<String, String> = TemplateVars::to_stringmap(template_vars);
    let glob = "**/*";
    let mut handlebars = Handlebars::new();
    handlebars_helper!(camel_case: |x: String| x.to_case(Case::UpperCamel));
    handlebars.register_helper("camel-case", Box::new(camel_case));
    for entry in template.as_payload().find(glob)? {
        match entry {
            DirEntry::Dir(dir) => {
                println!("Found {}", dir.path().display());
                let mut target = path.clone();
                target.push(dir.path());
                std::fs::create_dir_all(&target)?;
            }
            DirEntry::File(file) => {
                println!("Found {}", file.path().display());
                let mut target = path.clone();
                if file.path().file_name() == Some(OsStr::new("control")) {
                    target.push(format!("{}.control", template_map.get("name").unwrap()));
                } else {
                    target.push(file.path());
                }

                handlebars.register_template_string(
                    file.path().to_str().unwrap(),
                    file.contents_utf8().unwrap(),
                )?;

                let mut write_file = std::fs::File::create(target)?;
                write_file
                    .write_all(
                        handlebars
                            .render(file.path().to_str().unwrap(), &template_map)
                            .expect(&format!(
                                "templating {} failed",
                                file.path().to_str().unwrap()
                            ))
                            .as_bytes(),
                    )
                    .expect("error in writing template code");
            }
        }
    }

    Ok(())
}
