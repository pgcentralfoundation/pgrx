//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! A small command-line parser: the workflow passes a handful of fixed flags, which doesn't
//! justify compiling an argument-parsing library in every job before the job's real work.

use eyre::{WrapErr, bail, eyre};

pub(crate) struct Args {
    items: Vec<String>,
}

impl Args {
    pub(crate) fn from_env() -> Self {
        Self { items: std::env::args().skip(1).collect() }
    }

    #[cfg(test)]
    fn from(items: &[&str]) -> Self {
        Self { items: items.iter().map(|item| (*item).to_owned()).collect() }
    }

    /// Remove and return the leading command name.
    pub(crate) fn command(&mut self) -> Option<String> {
        (!self.items.is_empty()).then(|| self.items.remove(0))
    }

    /// Remove `name` and report whether it was present.
    pub(crate) fn flag(&mut self, name: &str) -> bool {
        let before = self.items.len();
        self.items.retain(|item| item != name);
        self.items.len() != before
    }

    /// Remove `name VALUE` or `name=VALUE` and return the value.
    pub(crate) fn value(&mut self, name: &str) -> eyre::Result<Option<String>> {
        let prefix = format!("{name}=");
        for index in 0..self.items.len() {
            if self.items[index] == name {
                if index + 1 == self.items.len() {
                    bail!("{name} needs a value");
                }
                let value = self.items.remove(index + 1);
                self.items.remove(index);
                return Ok(Some(value));
            }
            if let Some(value) = self.items[index].strip_prefix(&prefix) {
                let value = value.to_owned();
                self.items.remove(index);
                return Ok(Some(value));
            }
        }
        Ok(None)
    }

    pub(crate) fn required(&mut self, name: &str) -> eyre::Result<String> {
        self.value(name)?.ok_or_else(|| eyre!("missing {name}"))
    }

    pub(crate) fn pg_version(&mut self) -> eyre::Result<u16> {
        let version = self.required("--pg")?;
        version
            .parse()
            .wrap_err_with(|| format!("--pg {version} is not a PostgreSQL major version"))
    }

    /// Remove and return the next argument, which `name` describes in the error for none.
    pub(crate) fn required_positional(&mut self, name: &str) -> eyre::Result<String> {
        self.command().ok_or_else(|| eyre!("missing {name}"))
    }

    /// Take every remaining argument as a positional value.
    pub(crate) fn rest(&mut self) -> Vec<String> {
        std::mem::take(&mut self.items)
    }

    /// Reject arguments that no option consumed.
    pub(crate) fn finish(self) -> eyre::Result<()> {
        match self.items.first() {
            Some(unknown) => bail!("unexpected argument `{unknown}`"),
            None => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Args;

    #[test]
    fn values_flags_and_leftovers() {
        let mut args = Args::from(&["test-pg", "--pg", "18", "--run=a b", "--cli-checks"]);
        assert_eq!(args.command().as_deref(), Some("test-pg"));
        assert_eq!(args.pg_version().unwrap(), 18);
        assert_eq!(args.value("--run").unwrap().as_deref(), Some("a b"));
        assert!(args.flag("--cli-checks"));
        assert!(!args.flag("--cli-checks"));
        args.finish().unwrap();
    }

    #[test]
    fn missing_values_and_unknown_arguments_are_errors() {
        assert!(Args::from(&["--pg"]).value("--pg").is_err());
        assert!(Args::from(&["--pg", "eighteen"]).pg_version().is_err());
        assert!(Args::from(&["--surprise"]).finish().is_err());
    }
}
