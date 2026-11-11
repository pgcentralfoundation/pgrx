//LICENSE Portions Copyright 2026-2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
//! This extension has one released upgrade script and three unreleased SQL fragments. See the
//! README for a walkthrough.
//!
//! Everything here is what a brand new install gets, because `cargo pgrx` generates the base schema
//! from this code. An existing database only gets what's in the upgrade scripts, so every change to
//! the schema appears twice: once here, and once as SQL under `sql/`.
use pgrx::prelude::*;

pgrx::pg_module_magic!(name, version);

/// Part of the first release, 1.0.0.
#[pg_extern]
fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

/// Released in 1.1.0. Existing databases get it from `sql/migration_fragments--1.0.0--1.1.0.sql`.
#[pg_extern]
fn farewell(name: &str) -> String {
    format!("Goodbye, {name}!")
}

/// Not released yet. Existing databases will get it from `sql/unreleased/101.add_shout.sql`.
#[pg_extern]
fn shout(message: &str) -> String {
    message.to_uppercase()
}

// Not released yet. Existing databases will get the table from
// `sql/unreleased/102.add_greeting_log.sql` and the index from
// `sql/unreleased/98.index_shouted_greetings.sql`.
extension_sql!(
    r#"
CREATE TABLE greeting_log (
    greeting text NOT NULL,
    shouted boolean NOT NULL DEFAULT false
);
CREATE INDEX greeting_log_shouted ON greeting_log (shouted);
"#,
    name = "greeting_log",
);

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn shouts() {
        assert_eq!(crate::shout("hello"), "HELLO");
    }

    /// `cargo pgrx test` assembles the fragments like `cargo pgrx install` does, so the extension
    /// under test is one patch past the crate's version, and `extension_version!()` exposes that
    /// version at compile time.
    #[pg_test]
    fn installs_at_the_ephemeral_version() -> Result<(), pgrx::spi::Error> {
        const COMPILED_EXTVERSION: &str = extension_version!();
        assert_eq!(env!("CARGO_PKG_VERSION"), "1.1.0");
        assert_eq!(COMPILED_EXTVERSION, "1.1.1");

        let version = Spi::get_one::<String>(
            "SELECT extversion::text FROM pg_extension WHERE extname = 'migration_fragments'",
        )?;
        assert_eq!(version.as_deref(), Some(COMPILED_EXTVERSION));
        Ok(())
    }

    /// The ephemeral script extends the chain of released ones, so a database on any older version
    /// can reach it.
    #[pg_test]
    fn the_update_path_reaches_the_ephemeral_version() -> Result<(), pgrx::spi::Error> {
        let path = Spi::get_one::<String>(
            "SELECT path FROM pg_extension_update_paths('migration_fragments')
              WHERE source = '1.0.0' AND target = '1.1.1'",
        )?;
        assert_eq!(path.as_deref(), Some("1.0.0--1.1.0--1.1.1"));
        Ok(())
    }
}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {}

    #[must_use]
    pub fn postgresql_conf_options() -> Vec<&'static str> {
        vec![]
    }
}
