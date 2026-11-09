//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Test the packages that need PostgreSQL against one PostgreSQL version.
//!
//! The pg_test harness installs each extension with `cargo pgrx install`, which builds the
//! extension with only its own package's features. pgrx-unit-tests' build nearly matches the
//! workspace's test build and reuses it, but the examples' builds rebuild the bindings and
//! pgrx. So the test binaries build in `target` while cargo-pgrx and the examples' extensions
//! build in `target-ext`, each group of examples with the same dependencies in one Cargo
//! invocation, and then the test binaries run in parallel.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::mpsc;
use std::time::Instant;

use cargo_metadata::camino::{Utf8Path, Utf8PathBuf};
use cargo_metadata::{DependencyKind, Package, TargetKind};
use eyre::{WrapErr, eyre};

use crate::process::{self, log};
use crate::testbin;
use crate::workspace::{self, Workspace};

/// Packages whose extension builds share the test build's target directory.
const SHARES_TEST_TARGET: &[&str] = &["pgrx-unit-tests"];

/// Binaries that run longest, so they start first.
const FIRST: &[&str] = &["pgrx-unit-tests", "pgrx-pg-sys"];

/// Examples whose `cargo pgrx` commands this job exercises, and the commands.
const CLI_CHECKS: &[(&str, &[&[&str]])] = &[
    ("range", &[&["regress", "{pg}"]]),
    ("migration_fragments", &[&["migrate", "check", "--deny-mixed-objects"], &["regress", "{pg}"]]),
    ("versioned_custom_libname_so", &[&["schema", "{pg}"]]),
];

pub(crate) struct Options<'a> {
    pub(crate) version: u16,
    /// Run only these packages' tests, separated by spaces; `None` runs every package's.
    pub(crate) run: Option<&'a str>,
    /// Also exercise `cargo pgrx regress`, `migrate check` and `schema` on examples.
    pub(crate) cli_checks: bool,
}

/// One PostgreSQL version's build and install settings.
pub(crate) struct Postgres {
    pub(crate) version: u16,
    pub(crate) pg_config: PathBuf,
    /// Features beyond `pgNN` that a package's tests build and install with.
    pub(crate) extra_features: BTreeMap<&'static str, &'static [&'static str]>,
}

impl Postgres {
    pub(crate) fn from_env(version: u16) -> eyre::Result<Self> {
        let pg_config = std::env::var_os("PG_CONFIG")
            .ok_or_else(|| eyre!("PG_CONFIG must name the pg_config of PostgreSQL {version}"))?;
        Ok(Self { version, pg_config: pg_config.into(), extra_features: BTreeMap::new() })
    }

    pub(crate) fn feature(&self) -> String {
        format!("pg{}", self.version)
    }

    /// The features a package's tests build and install with.
    pub(crate) fn features(&self, package: &str) -> String {
        let extra = self.extra_features.get(package).copied().unwrap_or_default();
        std::iter::once(self.feature())
            .chain(extra.iter().map(|f| (*f).to_owned()))
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Install an extension exactly as the pg_test harness will, so the harness reuses the
    /// build. A failure is logged rather than returned: the harness repeats the install and
    /// reports its error with the failing tests.
    pub(crate) fn install(&self, cargo_pgrx: &Utf8Path, target_dir: &Utf8Path, package: &Package) {
        let started = Instant::now();
        let mut command = Command::new(cargo_pgrx);
        command
            .args(["pgrx", "install", "--test", "--pg-config"])
            .arg(&self.pg_config)
            .arg("--manifest-path")
            .arg(&package.manifest_path)
            .arg("--features")
            .arg(format!("{} pg_test", self.features(&package.name)))
            .arg("--no-default-features")
            .env("CARGO_TARGET_DIR", target_dir);
        match command.output() {
            Ok(output) if output.status.success() => {}
            Ok(output) => log(format_args!(
                "installing {} failed:\n{}",
                package.name,
                String::from_utf8_lossy(&output.stderr)
            )),
            Err(error) => log(format_args!("installing {} failed: {error}", package.name)),
        }
        log(format_args!("installed {} in {:.0?}", package.name, started.elapsed()));
    }

    /// Build `packages`' extension libraries together, so their own crates compile in
    /// parallel. They resolve the same dependencies, so this build matches each package's own,
    /// and the installs that follow reuse it. cargo-pgrx adds a linker argument on Linux, so
    /// there each install still relinks its final crate. A failure is logged for the installs
    /// to repeat.
    pub(crate) fn build_libraries(&self, target_dir: &Utf8Path, packages: &[&Package]) {
        let Some(first) = packages.first() else { return };
        let mut command = Command::new("cargo");
        command
            .args(["build", "--locked", "--lib", "--message-format=short", "--features"])
            .arg(format!("{} pg_test", self.features(&first.name)))
            .arg("--no-default-features")
            .env("CARGO_TARGET_DIR", target_dir);
        for package in packages {
            command.args(["-p", &package.name]);
        }
        let started = Instant::now();
        match process::run(&mut command) {
            Ok(true) => {}
            Ok(false) => log("building the extension libraries failed"),
            Err(error) => log(format_args!("building the extension libraries failed: {error:?}")),
        }
        log(format_args!(
            "built {} extension libraries in {:.0?}",
            packages.len(),
            started.elapsed()
        ));
    }

    /// Configure a test binary's command to install its extension the way [`install`] did.
    ///
    /// [`install`]: Self::install
    pub(crate) fn configure(
        &self,
        command: &mut Command,
        package: &str,
        cargo_pgrx: &Path,
        target_dir: &Path,
    ) {
        command
            .env("CARGO_TARGET_DIR", target_dir)
            .env("CARGO_PGRX", cargo_pgrx)
            .env("PGRX_FEATURES", self.features(package))
            .env("PGRX_NO_DEFAULT_FEATURES", "true");
    }
}

/// The cargo-pgrx that a target directory builds.
pub(crate) fn cargo_pgrx_in(target_dir: &Utf8Path) -> Utf8PathBuf {
    target_dir.join("debug").join(format!("cargo-pgrx{}", std::env::consts::EXE_SUFFIX))
}

/// Build cargo-pgrx in `target_dir`.
pub(crate) fn build_cargo_pgrx(target_dir: &Utf8Path) -> eyre::Result<bool> {
    let started = Instant::now();
    let built = process::run(
        Command::new("cargo")
            .args(["build", "--locked", "-p", "cargo-pgrx", "--message-format=short"])
            .env("CARGO_TARGET_DIR", target_dir),
    )?;
    log(format_args!("built cargo-pgrx in {:.0?}", started.elapsed()));
    Ok(built)
}

/// Whether the package's tests make the pg_test harness install it as an extension.
fn installs_extension(package: &Package) -> eyre::Result<bool> {
    let cdylib = package.targets.iter().any(|t| t.kind.contains(&TargetKind::CDyLib));
    Ok(cdylib && contains_pg_test(&workspace::package_dir(package).join("src"))?)
}

fn contains_pg_test(dir: &Utf8Path) -> eyre::Result<bool> {
    for entry in dir.read_dir_utf8().wrap_err_with(|| format!("reading {dir}"))? {
        let path = entry.wrap_err_with(|| format!("reading {dir}"))?.into_path();
        if path.is_dir() {
            if contains_pg_test(&path)? {
                return Ok(true);
            }
        } else if path.extension() == Some("rs") {
            let source = std::fs::read(&path).wrap_err_with(|| format!("reading {path}"))?;
            if source.windows(b"#[pg_test".len()).any(|window| window == b"#[pg_test") {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

/// Group the packages whose extension builds resolve the same dependencies, largest group
/// first, since the most test binaries wait on its build.
fn dependency_groups<'a>(packages: &[&'a Package]) -> Vec<Vec<&'a Package>> {
    type Signature = BTreeSet<(String, bool, bool, BTreeSet<String>, Option<String>)>;
    let mut groups: BTreeMap<Signature, Vec<&'a Package>> = BTreeMap::new();
    for &package in packages {
        let enabled: BTreeSet<&str> =
            package.features.get("pg_test").into_iter().flatten().map(String::as_str).collect();
        let signature = package
            .dependencies
            .iter()
            .filter(|dependency| dependency.kind != DependencyKind::Development)
            .filter(|dependency| {
                !dependency.optional
                    || enabled.contains(dependency.name.as_str())
                    || enabled.contains(format!("dep:{}", dependency.name).as_str())
            })
            .map(|dependency| {
                (
                    dependency.name.clone(),
                    dependency.kind == DependencyKind::Build,
                    dependency.uses_default_features,
                    dependency.features.iter().cloned().collect(),
                    dependency.target.as_ref().map(ToString::to_string),
                )
            })
            .collect();
        groups.entry(signature).or_default().push(package);
    }
    let mut groups: Vec<Vec<&Package>> = groups.into_values().collect();
    groups.sort_by_key(|group| std::cmp::Reverse(group.len()));
    groups
}

pub(crate) fn run(options: &Options) -> eyre::Result<bool> {
    let workspace = Workspace::load()?;
    let mut postgres = Postgres::from_env(options.version)?;
    postgres.extra_features.insert("pgrx-unit-tests", &["cshim", "proptest"]);
    let feature = postgres.feature();

    let pg_packages: Vec<&Package> = workspace
        .packages()
        .iter()
        .filter(|package| package.features.contains_key(&feature))
        .collect();
    let wanted: BTreeSet<&str> = match options.run {
        Some(run) if !run.trim().is_empty() => run.split_whitespace().collect(),
        _ => pg_packages.iter().map(|package| package.name.as_str()).collect(),
    };
    let mut extensions = Vec::new();
    for &package in &pg_packages {
        if wanted.contains(package.name.as_str()) && installs_extension(package)? {
            extensions.push(package);
        }
    }

    let test_target = workspace.target_dir("target");
    let ext_target = workspace.target_dir("target-ext");
    let cargo_pgrx = cargo_pgrx_in(&ext_target);

    // Every package that needs PostgreSQL builds, whatever runs, so the build matches the cache.
    let mut features = vec![feature.clone()];
    for (package, extra) in &postgres.extra_features {
        features.extend(extra.iter().map(|feature| format!("{package}/{feature}")));
    }
    let mut cargo_args = vec!["--locked".to_owned(), "--features".to_owned(), features.join(" ")];
    cargo_args.push("--no-default-features".to_owned());
    for package in &pg_packages {
        cargo_args.extend(["-p".to_owned(), package.name.to_string()]);
    }

    let (ready, cargo_pgrx_built) = mpsc::channel();
    let binaries = std::thread::scope(|scope| -> eyre::Result<_> {
        let examples = scope.spawn(|| -> eyre::Result<()> {
            let built = build_cargo_pgrx(&ext_target);
            let succeeded = matches!(built, Ok(true));
            // The receiver outlives this thread; a send can only fail after a panic there.
            let _ = ready.send(succeeded);
            if !built? {
                return Err(eyre!("building cargo-pgrx failed"));
            }
            let examples: Vec<&Package> = extensions
                .iter()
                .copied()
                .filter(|package| !SHARES_TEST_TARGET.contains(&package.name.as_str()))
                .collect();
            for group in dependency_groups(&examples) {
                if group.len() > 1 {
                    postgres.build_libraries(&ext_target, &group);
                }
                for package in group {
                    postgres.install(&cargo_pgrx, &ext_target, package);
                }
            }
            Ok(())
        });
        let binaries = testbin::build(&workspace, &cargo_args);
        // These extensions' builds reuse the test build, so they install right after it.
        if cargo_pgrx_built.recv().unwrap_or(false) {
            for package in
                extensions.iter().filter(|p| SHARES_TEST_TARGET.contains(&p.name.as_str()))
            {
                postgres.install(&cargo_pgrx, &test_target, package);
            }
        }
        examples.join().map_err(|_| eyre!("the extension build thread panicked"))??;
        binaries
    })?;

    let mut binaries: Vec<_> =
        binaries.into_iter().filter(|b| wanted.contains(b.package.as_str())).collect();
    binaries.sort_by_key(|binary| {
        FIRST.iter().position(|first| *first == binary.package).unwrap_or(FIRST.len())
    });
    let outcomes = testbin::run_all(binaries, parallelism(), |binary, command| {
        let target = if SHARES_TEST_TARGET.contains(&binary.package.as_str()) {
            &test_target
        } else {
            &ext_target
        };
        postgres.configure(
            command,
            &binary.package,
            cargo_pgrx.as_std_path(),
            target.as_std_path(),
        );
    });
    let mut passed = testbin::passed(&outcomes);

    // Only Rust libraries hold doctests; Cargo warns about every extension crate it is asked
    // to test this way. Their default features name no PostgreSQL version and keep pgrx's C
    // shim, as the test build did, so the libraries it made are reused.
    let mut doctests = Command::new("cargo");
    doctests.args(["test", "--locked", "--doc", "--features", &feature]);
    let mut any = false;
    for package in pg_packages.iter().filter(|p| wanted.contains(p.name.as_str())) {
        if workspace::only_rust_library(package) {
            doctests.args(["-p", &package.name]);
            any = true;
        }
    }
    if any {
        passed &= process::run(&mut doctests)?;
    }

    if options.cli_checks {
        for (example, commands) in CLI_CHECKS {
            let Some(package) =
                workspace.package(example).filter(|p| wanted.contains(p.name.as_str()))
            else {
                continue;
            };
            for command in *commands {
                let mut check = Command::new(&cargo_pgrx);
                check
                    .arg("pgrx")
                    .args(command.iter().map(|arg| arg.replace("{pg}", &feature)))
                    .arg("--manifest-path")
                    .arg(&package.manifest_path)
                    .current_dir(workspace::package_dir(package))
                    .env("CARGO_TARGET_DIR", &ext_target);
                passed &= process::run(&mut check)?;
            }
        }
    }
    Ok(passed)
}

/// How many test binaries to run at once: one per core.
pub(crate) fn parallelism() -> usize {
    std::thread::available_parallelism().map_or(2, std::num::NonZero::get)
}
