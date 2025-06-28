//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use crate::command::get::get_property;
use crate::command::run::Run;
use crate::command::start::collect_postgresql_conf_settings;
use crate::manifest::get_package_manifest;
use crate::CommandExecute;
use owo_colors::OwoColorize;
use pgrx_pg_config::{createdb, dropdb, PgConfig};
use std::collections::HashSet;
use std::env::temp_dir;
use std::fs::{DirEntry, File};
use std::io::{BufRead, BufReader, IsTerminal, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};

/// Run the regression test suite for this crate
#[derive(clap::Args, Debug, Clone)]
#[clap(author)]
pub(crate) struct Regress {
    /// Do you want to run against pg13, pg14, pg15, pg16, pg17?
    #[clap(env = "PG_VERSION")]
    pub(crate) pg_version: Option<String>,
    /// If specified, only run tests containing this string in their names
    pub(crate) test_filter: Option<String>,

    /// If specified, use this database name instead of the auto-generated version of `$extname_regress`
    #[clap(long)]
    pub(crate) dbname: Option<String>,

    /// Recreate the test database, even if it already exists
    #[clap(long)]
    pub(crate) resetdb: bool,
    /// Package to build (see `cargo help pkgid`)
    #[clap(long, short)]
    pub(crate) package: Option<String>,
    /// Path to Cargo.toml
    #[clap(long, value_parser)]
    pub(crate) manifest_path: Option<PathBuf>,
    /// compile for release mode (default is debug)
    #[clap(long, short)]
    pub(crate) release: bool,
    /// Specific profile to use (conflicts with `--release`)
    #[clap(long)]
    pub(crate) profile: Option<String>,
    /// Don't regenerate the schema
    #[clap(long, short)]
    pub(crate) no_schema: bool,
    /// Use `sudo` to initialize and run the Postgres test instance as this system user
    #[clap(long, value_name = "USER")]
    pub(crate) runas: Option<String>,
    /// Initialize the test database cluster here, instead of the default location.  If used with `--runas`, then it must be writable by the user
    #[clap(long, value_name = "DIR")]
    pub(crate) pgdata: Option<PathBuf>,
    #[clap(flatten)]
    pub(crate) features: clap_cargo::Features,
    #[clap(from_global, action = clap::ArgAction::Count)]
    pub(crate) verbose: u8,

    /// Custom `postgresql.conf` settings in the form of `key=value`, ie `log_min_messages=debug1`
    #[clap(long)]
    pub(crate) postgresql_conf: Vec<String>,

    /// Automatically accept output for new tests *and* overwrite output for existing-but-failed tests
    #[clap(long, short)]
    pub(crate) auto: bool,
}

impl Regress {
    #[rustfmt::skip]
    fn is_setup_sql_newer(&self, manifest_path: impl AsRef<Path>) -> bool {
        let sql = manifest_path_to_sql_tests_path(&manifest_path);
        if !sql.exists() { return false; }
        let expected = manifest_path_to_expected_tests_output_path(&manifest_path);
        if !expected.exists() {return false; }

        let setup_sql = sql.join("setup.sql");
        let setup_out = expected.join("setup.out");

        let Ok(setup_sql) = std::fs::metadata(setup_sql) else { return false; };
        let Ok(setup_out) = std::fs::metadata(setup_out) else { return true; }; // there is no output file, so setup.sql is definitely newer 

        let Ok(sql_modified) = setup_sql.modified() else { return false; };
        let Ok(out_modified) = setup_out.modified() else { return false; };

        sql_modified > out_modified
    }

    fn list_sql_tests(
        &self,
        manifest_path: impl AsRef<Path>,
        include_setup: bool,
    ) -> eyre::Result<Vec<DirEntry>> {
        let sql = manifest_path_to_sql_tests_path(manifest_path);
        if !sql.exists() {
            std::fs::create_dir(&sql)?;
        }
        let mut files = std::fs::read_dir(sql)?.collect::<Result<Vec<_>, _>>()?;

        Self::organize_files(&mut files, "sql", include_setup);
        Ok(files)
    }

    fn list_expected_outputs(
        &self,
        manifest_path: impl AsRef<Path>,
        include_setup: bool,
    ) -> eyre::Result<Vec<DirEntry>> {
        let expected = manifest_path_to_expected_tests_output_path(manifest_path);
        if !expected.exists() {
            std::fs::create_dir(&expected)?;
        }
        let mut files = std::fs::read_dir(expected)?.collect::<Result<Vec<_>, _>>()?;

        Self::organize_files(&mut files, "out", include_setup);

        Ok(files)
    }

    fn list_results_outputs(
        &self,
        manifest_path: impl AsRef<Path>,
        include_setup: bool,
    ) -> eyre::Result<Vec<DirEntry>> {
        let results = manifest_path_to_results_output_path(manifest_path);
        if !results.exists() {
            std::fs::create_dir(&results)?;
        }
        let mut files = std::fs::read_dir(results)?.collect::<Result<Vec<_>, _>>()?;

        Self::organize_files(&mut files, "out", include_setup);

        Ok(files)
    }

    fn organize_files(files: &mut Vec<DirEntry>, only: &str, include_setup: bool) {
        // remove any files that don't have `only` as the extension
        files.retain(|entry| {
            entry
                .metadata()
                .map(|metadata| {
                    metadata.is_file()
                        && entry
                            .file_name()
                            .to_str()
                            .map(|filename| filename.ends_with(&format!(".{only}")))
                            .unwrap_or_default()
                })
                .unwrap_or_default()
        });

        // `setup.{only}` is a special file that we handle separately
        let is_setup = |entry: &DirEntry| {
            if let Some(filename) = entry.file_name().to_str() {
                if filename.ends_with(&format!("setup.{only}")) {
                    return true;
                }
            }
            false
        };

        // remove the "setup" file from the list
        let setup_entry = files.iter().position(is_setup).map(|idx| files.remove(idx));

        // not all filesystems list directories sorted and we want some kind of guaranteed evaluation order
        files.sort_unstable_by_key(|entry| entry.file_name());

        // if we detected a setup file and the caller wants to include it, make it the first entry
        if let Some(setup_entry) = setup_entry {
            if include_setup {
                files.insert(0, setup_entry);
            }
        }
    }

    fn accept_new_test(
        &self,
        manifest_path: impl AsRef<Path>,
        test_result_output: impl AsRef<Path>,
        auto: bool,
    ) -> eyre::Result<()> {
        if !std::io::stdin().is_terminal() {
            panic!("not a terminal: cannot perform user interaction to accept tests")
        }
        let test_name = test_result_output
            .as_ref()
            .file_stem()
            .expect("test result output should have a stem")
            .to_str()
            .expect("test result output filename should be valid UTF8")
            .to_string();
        let test_output = std::fs::read_to_string(&test_result_output)?;

        let variant_suffix: Option<String>;

        if auto {
            variant_suffix = None;
            println!(
                "test `{}` is new, automatically accepting its output as expected",
                test_name.bold().green()
            );
        } else {
            println!("-----------");
            println!("{}", test_output.white());
            println!("test `{}` generated the above output:", test_name.bold().green());
            eprint!("Accept [Y, n]? ");

            let mut user_input = String::new();
            std::io::stdin().read_line(&mut user_input)?;
            let user_input = user_input.trim();

            if user_input == "Y" || user_input == "y" || user_input.is_empty() {
                variant_suffix = None
            } else if user_input.as_bytes()[0] >= b'0' && user_input.as_bytes()[0] <= b'9' {
                // currently secret options to create a variant file
                // however, postgres requires the original `test_name.out` to also exist
                variant_suffix = Some(format!("_{user_input}"));
            } else {
                std::process::exit(1);
            }
        }

        let expected_path = manifest_path_to_expected_tests_output_path(manifest_path)
            .join(format!("{test_name}{}.out", variant_suffix.unwrap_or_default()));

        if expected_path.exists() {
            println!(
                "{} test output to {}",
                "   Replacing".bold().green(),
                expected_path.display().bold().cyan()
            );
            std::fs::copy(test_result_output, &expected_path)?;

            // don't "git add" the file if it already exists
            Ok(())
        } else {
            println!(
                "{} test output to {}",
                "     Copying".bold().green(),
                expected_path.display().bold().cyan()
            );
            std::fs::copy(test_result_output, &expected_path)?;

            // make sure to add the file to git
            add_to_git(expected_path)
        }
    }

    fn run_all_tests(
        &self,
        pg_config: &PgConfig,
        manifest_path: impl AsRef<Path>,
        pgregress_path: impl AsRef<Path>,
        dbname: &str,
        test_files: &[&DirEntry],
        output_files: &[&DirEntry],
        include_setup: bool,
        auto: bool,
    ) -> eyre::Result<()> {
        let output_names = output_files.iter().map(|e| make_test_name(e)).collect::<HashSet<_>>();

        // look for new tests (tests without a corresponding output file)
        let new_tests = test_files
            .iter()
            .filter(|entry| {
                let test_name = make_test_name(entry);
                !output_names.contains(&test_name)
            })
            .collect::<Vec<_>>();

        if !new_tests.is_empty() {
            println!(
                "{} {} new tests, running each individually to create output",
                "       Found".bold().green(),
                new_tests.len()
            );
            for new_test in new_tests {
                if let Some(test_result_output) = create_regress_output(
                    pg_config,
                    &manifest_path,
                    &pgregress_path,
                    dbname,
                    new_test,
                )? {
                    self.accept_new_test(&manifest_path, test_result_output, auto)?;
                }
            }
        }

        // now that all tests have outputs, run them all
        let success = run_tests(pg_config, pgregress_path, dbname, test_files)?;

        if !success && auto {
            // tests failed, but the user asked to `auto`matically accept their output as new output
            let results_files = self.list_results_outputs(&manifest_path, include_setup)?;

            println!();
            for entry in results_files {
                let filename =
                    entry.file_name().to_str().expect("filename should be valid UTF8").to_owned();
                let expected_path =
                    manifest_path_to_expected_tests_output_path(&manifest_path).join(filename);

                if !expected_path.exists() {
                    // this is a file from `results/test-name.out` for which we don't have an expected file
                    // we can ignore it
                    continue;
                }

                let src = std::fs::read_to_string(entry.path())?;
                let dst = std::fs::read_to_string(&expected_path)?;
                if src != dst {
                    println!(
                        "{} {}'s output to {}",
                        "   Promoting".bold().yellow(),
                        make_test_name(&entry).bold().bright_red(),
                        expected_path.display().bold().cyan()
                    );
                    std::fs::copy(entry.path(), &expected_path)?;
                }
            }

            std::process::exit(1);
        }

        Ok(())
    }
}

impl CommandExecute for Regress {
    #[tracing::instrument(level = "error", skip(self))]
    fn execute(mut self) -> eyre::Result<()> {
        unsafe {
            std::env::set_var("PGRX_REGRESS_TESTING", "1");
        }
        let (_, manifest_path) = get_package_manifest(
            &self.features,
            self.package.as_ref(),
            self.manifest_path.as_ref(),
        )?;
        let extname = get_property(&manifest_path, "extname")?
            .expect("extension name property `extname` should always be known");
        self.dbname = Some(self.dbname.unwrap_or_else(|| format!("{extname}_regress")));

        // we purposely want as little noise as possible to end up in the expected test output files
        self.postgresql_conf.push("client_min_messages=warning".into());
        let postgresql_conf = collect_postgresql_conf_settings(&self.postgresql_conf)?;

        // install the extension
        let (pg_config, dbname) = Run::from(&self).install(false, &postgresql_conf)?;
        let pgregress_path = pg_config.pg_regress_path()?;

        if self.is_setup_sql_newer(&manifest_path) {
            println!(
                "{} database {} to be (re)created as `setup.sql` is newer than its expected output",
                "     Forcing".bold().yellow(),
                dbname.cyan()
            );
        }

        // NB:  the `is_test` argument for both `dropdb()` and `createdb()` is for `cargo pgrx test`,
        // which creates its own Postgres instance and has its own port and datadir and such, so we
        // say `false` here.
        if self.resetdb || self.is_setup_sql_newer(&manifest_path) {
            dropdb(&pg_config, &dbname, false, self.runas.clone())?;
        }
        // won't re-create it if it already exists
        let created_db = createdb(&pg_config, &dbname, false, true, self.runas.clone())?;
        if !created_db {
            println!("{} existing database {dbname}", "    Re-using".bold().cyan());
        }

        // figure out what test and output files we have
        let mut test_files = self.list_sql_tests(&manifest_path, created_db)?;
        let output_files = self.list_expected_outputs(&manifest_path, created_db)?;

        // filter tests
        if let Some(test_filter) = self.test_filter.as_ref() {
            test_files.retain(|entry| make_test_name(entry).contains(test_filter));
            if test_files.is_empty() {
                println!(
                    "{} no tests matching filter `{test_filter}`",
                    "       ERROR".bold().red()
                );
                std::process::exit(1);
            }
        }

        println!();
        println!("--- beginning regression test run ---");
        self.run_all_tests(
            &pg_config,
            &manifest_path,
            &pgregress_path,
            &dbname,
            &test_files.iter().collect::<Vec<_>>(),
            &output_files.iter().collect::<Vec<_>>(),
            created_db, // include_setup
            self.auto,
        )
    }
}

fn run_tests(
    pg_config: &PgConfig,
    pg_regress_bin: impl AsRef<Path>,
    dbname: &str,
    test_files: &[&DirEntry],
) -> eyre::Result<bool> {
    if test_files.is_empty() {
        return Ok(true);
    }
    let input_dir = test_files[0].path();
    let input_dir = input_dir
        .parent()
        .expect("test file should not be at the root of the filesystem")
        .parent()
        .expect("test file should be in a directory named `sql/`")
        .to_path_buf();
    pg_regress(pg_config, pg_regress_bin, dbname, &input_dir, test_files)
        .map(|status| status.success())
}

fn create_regress_output(
    pg_config: &PgConfig,
    manifest_path: impl AsRef<Path>,
    pg_regress_bin: impl AsRef<Path>,
    dbname: &str,
    test_file: &DirEntry,
) -> eyre::Result<Option<PathBuf>> {
    let test_name = make_test_name(test_file);
    let input_dir = test_file.path();
    let input_dir = input_dir
        .parent()
        .expect("test file should not be at the root of the filesystem")
        .parent()
        .expect("test file should be in a directory named `sql/`")
        .to_path_buf();
    let status = pg_regress(pg_config, pg_regress_bin, dbname, &input_dir, &[test_file])?;

    if !status.success() {
        // pg_regress returned with an error code, but that is most likely because the test's output file
        // doesn't exist, since we are creating the test output.  So if that's the case, if we have
        // a `.out` file for it in the results/ directory, then we're successful
        let out_file =
            manifest_path_to_results_output_path(&manifest_path).join(format!("{test_name}.out"));
        if out_file.exists() {
            return Ok(Some(out_file));
        } else {
            std::process::exit(status.code().unwrap_or(1));
        }
    }

    Ok(None)
}

fn pg_regress(
    pg_config: &PgConfig,
    bin: impl AsRef<Path>,
    dbname: &str,
    input_dir: impl AsRef<Path>,
    tests: &[&DirEntry],
) -> eyre::Result<ExitStatus> {
    if tests.is_empty() {
        eyre::bail!("no tests to run");
    }
    let test_dir = tests[0].path().parent().unwrap().parent().unwrap().to_path_buf();
    let tests = tests.iter().map(|entry| make_test_name(entry));

    let mut command = Command::new(bin.as_ref());
    command
        .current_dir(test_dir)
        .env_remove("PGDATABASE")
        .env_remove("PGHOST")
        .env_remove("PGPORT")
        .env_remove("PGUSER")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .arg("--host")
        .arg(pg_config.host())
        .arg("--port")
        .arg(pg_config.port()?.to_string())
        .arg("--use-existing")
        .arg(format!("--dbname={dbname}"))
        .arg(format!("--inputdir={}", input_dir.as_ref().display()))
        .arg(format!("--outputdir={}", input_dir.as_ref().display()))
        .args(tests);

    #[cfg(not(target_os = "windows"))]
    let launcher_script = {
        fn make_launcher_script() -> eyre::Result<PathBuf> {
            use std::os::unix::fs::PermissionsExt;

            // in order to avoid verbose log output being enshrined in expected test output
            const LAUNCHER_SCRIPT: &[u8] = b"#! /bin/bash\n$* -v VERBOSITY=terse";

            let path = temp_dir().join(format!("pgrx-pg_regress-runner-{}.sh", std::process::id()));
            let mut tmpfile = File::create(&path)?;
            tmpfile.write_all(LAUNCHER_SCRIPT)?;
            let mut perms = path.metadata()?.permissions();
            perms.set_mode(0o700);
            tmpfile.set_permissions(perms)?;
            Ok(path)
        }
        let launcher_script = make_launcher_script()?;
        command.arg(format!("--launcher={}", launcher_script.display()));
        launcher_script
    };

    tracing::trace!("running {command:?}");

    let mut child = command.spawn()?;
    let (Some(stdout), Some(stderr)) = (child.stdout.take(), child.stderr.take()) else {
        panic!("unable to take stdout or stderr from pg_regress process");
    };

    let output_monitor = std::thread::spawn(move || {
        let mut passed_cnt = 0;
        let mut failed_cnt = 0;
        let stdout = BufReader::new(stdout);
        let stderr = BufReader::new(stderr);
        for line in stdout.lines().chain(stderr.lines()) {
            let line = line.unwrap();
            let Some((line, result)) = decorate_output(line) else {
                continue;
            };

            match result {
                Some(TestResult::Passed) => passed_cnt += 1,
                Some(TestResult::Failed) => failed_cnt += 1,
                None => (),
            }

            println!("{line}");
        }
        (passed_cnt, failed_cnt)
    });
    let status = child.wait()?;
    let (passed_cnt, failed_cnt) =
        output_monitor.join().map_err(|_| eyre::eyre!("failed to join output monitor thread"))?;
    println!("passed={passed_cnt} failed={failed_cnt}");

    #[cfg(not(target_os = "windows"))]
    {
        std::fs::remove_file(launcher_script)?;
    }

    Ok(status)
}

enum TestResult {
    Passed,
    Failed,
}

fn decorate_output(mut line: String) -> Option<(String, Option<TestResult>)> {
    let mut decorated = String::with_capacity(line.len());
    let mut test_result: Option<TestResult> = None;
    let mut is_old_line = false;
    let mut is_new_line = false;

    if line.starts_with("ok") {
        // for pg_regress from pg16 forward, rewrite the "ok" into a colored PASS"
        is_new_line = true;
    } else if line.starts_with("not ok") {
        // for pg_regress from pg16 forward, rewrite the "no ok" into a colored FAIL"
        line = line.replace("not ok", "not_ok"); // to make parsing easier down below
        is_new_line = true;
    } else if line.contains("... ok") || line.contains("... FAILED") {
        is_old_line = true;
    }

    let parsed_test_line = if is_new_line {
        fn split_line(line: &str) -> Option<(&str, bool, &str, &str)> {
            let mut parts = line.split_whitespace();

            let passed = parts.next()? == "ok";
            parts.next()?; // throw away the test number
            parts.next()?; // throw away the dash (-)
            let test_name = parts.next()?;
            let execution_time = parts.next()?;
            let execution_units = parts.next()?;
            Some((test_name, passed, execution_time, execution_units))
        }
        split_line(&line)
    } else if is_old_line {
        fn split_line(line: &str) -> Option<(&str, bool, &str, &str)> {
            let mut parts = line.split_whitespace();

            parts.next()?; // throw away "test"
            let test_name = parts.next()?;
            parts.next()?; // throw away "..."
            let passed = parts.next()? == "ok";
            let execution_time = parts.next()?;
            let execution_units = parts.next()?;
            Some((test_name, passed, execution_time, execution_units))
        }
        split_line(&line)
    } else {
        // not a line we care about
        return None;
    };

    if let Some((test_name, passed, execution_time, execution_units)) = parsed_test_line {
        if passed {
            test_result = Some(TestResult::Passed);
        } else {
            test_result = Some(TestResult::Failed);
        }

        decorated.push_str(&format!(
            "{} {test_name} {execution_time}{execution_units}",
            if passed {
                "PASS".bold().bright_green().to_string()
            } else {
                "FAIL".bold().bright_red().to_string()
            }
        ))
    }

    Some((decorated, test_result))
}

fn make_test_name(entry: &DirEntry) -> String {
    let filename = entry.file_name();
    let filename = filename.to_str().unwrap_or_else(|| panic!("bogus file name: {entry:?}"));
    let filename =
        filename.split('.').next().unwrap_or_else(|| panic!("invalid filename: `{filename}`"));
    filename.to_string()
}

fn manifest_path_to_sql_tests_path(manifest_path: impl AsRef<Path>) -> PathBuf {
    let mut path = PathBuf::from(manifest_path.as_ref());
    path.pop(); // pop `Cargo.toml`
    path.push("tests");
    path.push("pg_regress");
    path.push("sql");
    path
}

fn manifest_path_to_expected_tests_output_path(manifest_path: impl AsRef<Path>) -> PathBuf {
    let mut path = PathBuf::from(manifest_path.as_ref());
    path.pop(); // pop `Cargo.toml`
    path.push("tests");
    path.push("pg_regress");
    path.push("expected");
    path
}
fn manifest_path_to_results_output_path(manifest_path: impl AsRef<Path>) -> PathBuf {
    let mut path = PathBuf::from(manifest_path.as_ref());
    path.pop(); // pop `Cargo.toml`
    path.push("tests");
    path.push("pg_regress");
    path.push("results");
    path
}

fn add_to_git(path: impl AsRef<Path>) -> eyre::Result<()> {
    if let Ok(git) = which::which("git") {
        if is_git_repo(&git) && !Command::new(git).arg("add").arg(path.as_ref()).status()?.success()
        {
            panic!("unable to add {} to git", path.as_ref().display());
        }
    }
    Ok(())
}

fn is_git_repo(git: impl AsRef<Path>) -> bool {
    Command::new(git.as_ref())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .status()
        .map(|status| status.success())
        .unwrap_or_default()
}
