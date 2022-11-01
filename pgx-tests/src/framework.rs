/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use std::process::{Command, Stdio};

use eyre::{eyre, WrapErr};
use once_cell::sync::Lazy;
use owo_colors::OwoColorize;
use pgx::prelude::*;
use pgx_pg_config::{createdb, get_target_dir, PgConfig, Pgx, C_LOCALE_FLAGS};
use postgres::error::DbError;
use std::collections::HashMap;
use std::fmt::Write as _;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

mod shutdown;
pub use shutdown::add_shutdown_hook;

type LogLines = Arc<Mutex<HashMap<String, Vec<String>>>>;

struct SetupState {
    installed: bool,
    loglines: LogLines,
    system_session_id: String,
}

static TEST_MUTEX: Lazy<Mutex<SetupState>> = Lazy::new(|| {
    Mutex::new(SetupState {
        installed: false,
        loglines: Arc::new(Mutex::new(HashMap::new())),
        system_session_id: "NONE".to_string(),
    })
});

// The goal of this closure is to allow "wrapping" of anything that might issue
// an SQL simple_quuery or query using either a postgres::Client or
// postgres::Transction and capture the output. The use of this wrapper is
// completely optional, but it might help narrow down some errors later on.
fn query_wrapper<F, T>(
    query: Option<String>,
    query_params: Option<&[&(dyn postgres::types::ToSql + Sync)]>,
    mut f: F,
) -> eyre::Result<T>
where
    T: IntoIterator,
    F: FnMut(
        Option<String>,
        Option<&[&(dyn postgres::types::ToSql + Sync)]>,
    ) -> Result<T, postgres::Error>,
{
    let result = f(query.clone(), query_params.clone());

    match result {
        Ok(result) => Ok(result),
        Err(e) => {
            let dberror = e.as_db_error().unwrap();
            let query = query.unwrap();
            let query_message = dberror.message();

            let code = dberror.code().code();
            let severity = dberror.severity();

            let mut message = format!("{} SQLSTATE[{}]", severity, code).bold().red().to_string();

            message.push_str(format!(": {}", query_message.bold().white()).as_str());
            message.push_str(format!("\nquery: {}", query.bold().white()).as_str());
            message.push_str(
                format!(
                    "\nparams: {}",
                    match query_params {
                        Some(params) => format!("{:?}", params),
                        None => "None".to_string(),
                    }
                )
                .as_str(),
            );

            if let Ok(var) = std::env::var("RUST_BACKTRACE") {
                if var.eq("1") {
                    let detail = dberror.detail().unwrap_or("None");
                    let hint = dberror.hint().unwrap_or("None");
                    let schema = dberror.hint().unwrap_or("None");
                    let table = dberror.table().unwrap_or("None");
                    let more_info = format!(
                        "\ndetail: {detail}\nhint: {hint}\nschema: {schema}\ntable: {table}"
                    );
                    message.push_str(more_info.as_str());
                }
            }

            Err(eyre!(message))
        }
    }
}

pub fn run_test(
    sql_funcname: &str,
    expected_error: Option<&str>,
    postgresql_conf: Vec<&'static str>,
) -> eyre::Result<()> {
    let (loglines, system_session_id) = initialize_test_framework(postgresql_conf)?;

    let (mut client, session_id) = client()?;

    let schema = "tests"; // get_extension_schema();
    let result = match client.transaction() {
        // run the test function in a transaction
        Ok(mut tx) => {
            let result = tx.simple_query(&format!("SELECT \"{schema}\".\"{sql_funcname}\"();"));

            if result.is_ok() {
                // and abort the transaction when complete
                tx.rollback().expect("test rollback didn't work");
            }

            result
        }

        Err(e) => panic!("attempt to run test tx failed:\n{e}"),
    };

    if let Err(e) = result {
        let error_as_string = format!("error in test tx: {e}");

        let cause = e.into_source();
        if let Some(e) = cause {
            if let Some(dberror) = e.downcast_ref::<DbError>() {
                // we got an ERROR
                let received_error_message: &str = dberror.message();

                if let Some(expected_error_message) = expected_error {
                    // and we expected an error, so assert what we got is what we expect
                    assert_eq!(received_error_message, expected_error_message);
                    Ok(())
                } else {
                    // we weren't expecting an error
                    // wait a second for Postgres to get log messages written to stderr
                    std::thread::sleep(std::time::Duration::from_millis(1000));

                    let mut pg_location = String::from("Postgres location: ");
                    pg_location.push_str(match dberror.file() {
                        Some(file) => file,
                        None => "<unknown>",
                    });
                    if let Some(ln) = dberror.line() {
                        let _ = write!(pg_location, ":{ln}");
                    };

                    let mut rust_location = String::from("Rust location: ");
                    rust_location.push_str(match dberror.where_() {
                        Some(place) => place,
                        None => "<unknown>",
                    });
                    // then we can panic with those messages plus those that belong to the system
                    panic!(
                        "\n{sys}...\n{sess}\n{e}\n{pg}\n{rs}\n\n",
                        sys = format_loglines(&system_session_id, &loglines),
                        sess = format_loglines(&session_id, &loglines),
                        e = received_error_message.bold().red(),
                        pg = pg_location.dimmed().white(),
                        rs = rust_location.yellow()
                    );
                }
            } else {
                panic!("Failed downcast to DbError:\n{e}")
            }
        } else {
            panic!("Error without deeper source cause:\n{e}\n", e = error_as_string.bold().red())
        }
    } else if let Some(message) = expected_error {
        // we expected an ERROR, but didn't get one
        return Err(eyre!("Expected error: {message}"));
    } else {
        Ok(())
    }
}

fn format_loglines(session_id: &str, loglines: &LogLines) -> String {
    let mut result = String::new();

    for line in loglines.lock().unwrap().entry(session_id.to_string()).or_default().iter() {
        result.push_str(line);
        result.push('\n');
    }

    result
}

fn initialize_test_framework(
    postgresql_conf: Vec<&'static str>,
) -> eyre::Result<(LogLines, String)> {
    let mut state = TEST_MUTEX.lock().unwrap_or_else(|_| {
        // This used to immediately throw an std::process::exit(1), but it
        // would consume both stdout and stderr, resulting in error messages
        // not being displayed unless you were running tests with --nocapture.
        panic!(
            "Could not obtain test mutex. A previous test may have hard-aborted while holding it."
        );
    });

    if !state.installed {
        shutdown::register_shutdown_hook();
        install_extension()?;
        initdb(postgresql_conf)?;

        let system_session_id = start_pg(state.loglines.clone())?;
        let pg_config = get_pg_config()?;
        dropdb()?;
        createdb(&pg_config, get_pg_dbname(), true, false)?;
        create_extension()?;
        state.installed = true;
        state.system_session_id = system_session_id;
    }

    Ok((state.loglines.clone(), state.system_session_id.clone()))
}

fn get_pg_config() -> eyre::Result<PgConfig> {
    let pgx = Pgx::from_config().wrap_err("Unable to get PGX from config")?;

    let pg_version = pg_sys::get_pg_major_version_num();

    let pg_config = pgx
        .get(&format!("pg{}", pg_version))
        .wrap_err_with(|| {
            format!("Error getting pg_config: {} is not a valid postgres version", pg_version)
        })
        .unwrap()
        .clone();

    Ok(pg_config)
}

pub fn client() -> eyre::Result<(postgres::Client, String)> {
    let pg_config = get_pg_config()?;
    let mut client = postgres::Config::new()
        .host(pg_config.host())
        .port(pg_config.test_port().expect("unable to determine test port"))
        .user(&get_pg_user())
        .dbname(&get_pg_dbname())
        .connect(postgres::NoTls)
        .unwrap();

    let sid_query_result = query_wrapper(
        Some("SELECT to_hex(trunc(EXTRACT(EPOCH FROM backend_start))::integer) || '.' || to_hex(pid) AS sid FROM pg_stat_activity WHERE pid = pg_backend_pid();".to_string()),
        Some(&[]),
        |query, query_params| client.query(&query.unwrap(), query_params.unwrap()),
    )
    .wrap_err("There was an issue attempting to get the session ID from Postgres")?;

    let session_id = match sid_query_result.get(0) {
        Some(row) => row.get::<&str, &str>("sid").to_string(),
        None => Err(eyre!("Failed to obtain a client Session ID from Postgres"))?,
    };

    query_wrapper(Some("SET log_min_messages TO 'INFO';".to_string()), None, |query, _| {
        client.simple_query(query.unwrap().as_str())
    })
    .wrap_err("Postgres Client setup failed to SET log_min_messages TO 'INFO'")?;

    query_wrapper(Some("SET log_min_duration_statement TO 1000;".to_string()), None, |query, _| {
        client.simple_query(query.unwrap().as_str())
    })
    .wrap_err("Postgres Client setup failed to SET log_min_duration_statement TO 1000;")?;

    query_wrapper(Some("SET log_statement TO 'all';".to_string()), None, |query, _| {
        client.simple_query(query.unwrap().as_str())
    })
    .wrap_err("Postgres Client setup failed to SET log_statement TO 'all';")?;

    Ok((client, session_id))
}

fn install_extension() -> eyre::Result<()> {
    eprintln!("installing extension");
    let profile = std::env::var("PGX_BUILD_PROFILE").unwrap_or("debug".into());
    let no_schema = std::env::var("PGX_NO_SCHEMA").unwrap_or("false".into()) == "true";
    let mut features = std::env::var("PGX_FEATURES").unwrap_or("".to_string());
    if !features.contains("pg_test") {
        features += " pg_test";
    }
    let no_default_features =
        std::env::var("PGX_NO_DEFAULT_FEATURES").unwrap_or("false".to_string()) == "true";
    let all_features = std::env::var("PGX_ALL_FEATURES").unwrap_or("false".to_string()) == "true";

    let pg_version = format!("pg{}", pg_sys::get_pg_major_version_string());
    let pgx = Pgx::from_config()?;
    let pg_config = pgx.get(&pg_version)?;

    let mut command = Command::new("cargo");
    command
        .arg("pgx")
        .arg("install")
        .arg("--test")
        .arg("--pg-config")
        .arg(pg_config.path().ok_or(eyre!("No pg_config found"))?)
        .stdout(Stdio::inherit())
        .stderr(Stdio::piped())
        .env("CARGO_TARGET_DIR", get_target_dir()?);

    if let Ok(manifest_path) = std::env::var("PGX_MANIFEST_PATH") {
        command.arg("--manifest-path");
        command.arg(manifest_path);
    }

    if let Ok(rust_log) = std::env::var("RUST_LOG") {
        command.env("RUST_LOG", rust_log);
    }

    if !features.trim().is_empty() {
        command.arg("--features");
        command.arg(features);
    }

    if no_default_features {
        command.arg("--no-default-features");
    }

    if all_features {
        command.arg("--all-features");
    }

    match profile.trim() {
        // For legacy reasons, cargo has two names for the debug profile... (We
        // also ignore the empty string here, just in case).
        "debug" | "dev" | "" => {}
        "release" => {
            command.arg("--release");
        }
        profile => {
            command.args(["--profile", profile]);
        }
    }

    if no_schema {
        command.arg("--no-schema");
    }

    let command_str = format!("{:?}", command);

    let child = command.spawn().wrap_err_with(|| {
        format!(
            "Failed to spawn process for installing extension using command: '{}': ",
            command_str
        )
    })?;

    let output = child.wait_with_output().wrap_err_with(|| {
        format!(
            "Failed waiting for spawned process attempting to install extension using command: '{}': ",
            command_str
        )
    })?;

    if !output.status.success() {
        return Err(eyre!(
            "Failure installing extension using command: {}\n\n{}{}",
            command_str,
            String::from_utf8(output.stdout).unwrap(),
            String::from_utf8(output.stderr).unwrap()
        ));
    }

    Ok(())
}

fn initdb(postgresql_conf: Vec<&'static str>) -> eyre::Result<()> {
    let pgdata = get_pgdata_path()?;

    if !pgdata.is_dir() {
        let pg_config = get_pg_config()?;
        let mut command =
            Command::new(pg_config.initdb_path().wrap_err("unable to determine initdb path")?);

        command
            .args(C_LOCALE_FLAGS)
            .arg("-D")
            .arg(pgdata.to_str().unwrap())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());

        let command_str = format!("{:?}", command);

        let child = command.spawn().wrap_err_with(|| {
            format!(
                "Failed to spawn process for initializing database using command: '{}': ",
                command_str
            )
        })?;

        let output = child.wait_with_output().wrap_err_with(|| {
            format!(
                "Failed waiting for spawned process attempting to initialize database using command: '{}': ",
                command_str
            )
        })?;

        if !output.status.success() {
            return Err(eyre!(
                "Failed to initialize database using command: {}\n\n{}{}",
                command_str,
                String::from_utf8(output.stdout).unwrap(),
                String::from_utf8(output.stderr).unwrap()
            ));
        }
    }

    modify_postgresql_conf(pgdata, postgresql_conf)
}

fn modify_postgresql_conf(pgdata: PathBuf, postgresql_conf: Vec<&'static str>) -> eyre::Result<()> {
    let mut postgresql_conf_file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(format!("{}/postgresql.auto.conf", pgdata.display()))
        .wrap_err("couldn't open postgresql.auto.conf")?;
    postgresql_conf_file
        .write_all("log_line_prefix='[%m] [%p] [%c]: '\n".as_bytes())
        .wrap_err("couldn't append log_line_prefix")?;

    for setting in postgresql_conf {
        postgresql_conf_file
            .write_all(format!("{setting}\n").as_bytes())
            .wrap_err("couldn't append custom setting to postgresql.conf")?;
    }

    postgresql_conf_file
        .write_all(
            format!("unix_socket_directories = '{}'", Pgx::home().unwrap().display()).as_bytes(),
        )
        .wrap_err("couldn't append `unix_socket_directories` setting to postgresql.conf")?;
    Ok(())
}

fn start_pg(loglines: LogLines) -> eyre::Result<String> {
    let pg_config = get_pg_config()?;
    let mut command =
        Command::new(pg_config.postmaster_path().wrap_err("unable to determine postmaster path")?);
    command
        .arg("-D")
        .arg(get_pgdata_path()?.to_str().unwrap())
        .arg("-h")
        .arg(pg_config.host())
        .arg("-p")
        .arg(pg_config.test_port().expect("unable to determine test port").to_string())
        // Redirecting logs to files can hang the test framework, override it
        .args(["-c", "log_destination=stderr", "-c", "logging_collector=off"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::piped());

    let command_str = format!("{command:?}");

    // start Postgres and monitor its stderr in the background
    // also notify the main thread when it's ready to accept connections
    let session_id = monitor_pg(command, command_str, loglines);

    Ok(session_id)
}

fn monitor_pg(mut command: Command, cmd_string: String, loglines: LogLines) -> String {
    let (sender, receiver) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        let mut child = command.spawn().expect("postmaster didn't spawn");

        let pid = child.id();
        // Add a shutdown hook so we can terminate it when the test framework
        // exits. TODO: Consider finding a way to handle cases where we fail to
        // clean up due to a SIGNAL?
        add_shutdown_hook(move || unsafe {
            libc::kill(pid as libc::pid_t, libc::SIGTERM);
            let message_string = std::ffi::CString::new(
                format!("stopping postgres (pid={pid})\n").bold().blue().to_string(),
            )
            .unwrap();
            libc::printf("%s\0".as_ptr().cast(), message_string.as_ptr());
        });

        eprintln!("{cmd}\npid={p}", cmd = cmd_string.bold().blue(), p = pid.to_string().yellow());
        eprintln!("{}", pg_sys::get_pg_version_string().bold().purple());

        // wait for the database to say its ready to start up
        let reader = BufReader::new(child.stderr.take().expect("couldn't take postmaster stderr"));

        let regex = regex::Regex::new(r#"\[.*?\] \[.*?\] \[(?P<session_id>.*?)\]"#).unwrap();
        let mut is_started_yet = false;
        let mut lines = reader.lines();
        while let Some(Ok(line)) = lines.next() {
            let session_id = match get_named_capture(&regex, "session_id", &line) {
                Some(sid) => sid,
                None => "NONE".to_string(),
            };

            if line.contains("database system is ready to accept connections") {
                // Postgres says it's ready to go
                sender.send(session_id.clone()).unwrap();
                is_started_yet = true;
            }

            if !is_started_yet || line.contains("TMSG: ") {
                eprintln!("{}", line.cyan());
            }

            // if line.contains("INFO: ") {
            //     eprintln!("{}", line.cyan());
            // } else if line.contains("WARNING: ") {
            //     eprintln!("{}", line.bold().yellow());
            // } else if line.contains("ERROR: ") {
            //     eprintln!("{}", line.bold().red());
            // } else if line.contains("statement: ") || line.contains("duration: ") {
            //     eprintln!("{}", line.bold().blue());
            // } else if line.contains("LOG: ") {
            //     eprintln!("{}", line.dimmed().white());
            // } else {
            //     eprintln!("{}", line.bold().purple());
            // }

            let mut loglines = loglines.lock().unwrap();
            let session_lines = loglines.entry(session_id).or_insert_with(Vec::new);
            session_lines.push(line);
        }

        // wait for Postgres to really finish
        match child.try_wait() {
            Ok(status) => {
                if let Some(_status) = status {
                    // we exited normally
                }
            }
            Err(e) => panic!("was going to let Postgres finish, but errored this time:\n{e}"),
        }
    });

    // wait for Postgres to indicate it's ready to accept connection
    // and return its pid when it is
    receiver.recv().expect("Postgres failed to start")
}

fn dropdb() -> eyre::Result<()> {
    let pg_config = get_pg_config()?;
    let output = Command::new(pg_config.dropdb_path().expect("unable to determine dropdb path"))
        .env_remove("PGDATABASE")
        .env_remove("PGHOST")
        .env_remove("PGPORT")
        .env_remove("PGUSER")
        .arg("--if-exists")
        .arg("-h")
        .arg(pg_config.host())
        .arg("-p")
        .arg(pg_config.test_port().expect("unable to determine test port").to_string())
        .arg(get_pg_dbname())
        .output()
        .unwrap();

    if !output.status.success() {
        // maybe the database didn't exist, and if so that's okay
        let stderr = String::from_utf8_lossy(output.stderr.as_slice());
        if !stderr.contains(&format!("ERROR:  database \"{}\" does not exist", get_pg_dbname())) {
            // got some error we didn't expect
            let stdout = String::from_utf8_lossy(output.stdout.as_slice());
            eprintln!("unexpected error (stdout):\n{stdout}");
            eprintln!("unexpected error (stderr):\n{stderr}");
            panic!("failed to drop test database");
        }
    }

    Ok(())
}

fn create_extension() -> eyre::Result<()> {
    let (mut client, _) = client()?;
    let extension_name = get_extension_name();

    query_wrapper(
        Some(format!("CREATE EXTENSION {} CASCADE;", &extension_name)),
        None,
        |query, _| client.simple_query(query.unwrap().as_str()),
    )
    .wrap_err(format!(
        "There was an issue creating the extension '{}' in Postgres: ",
        &extension_name
    ))?;

    Ok(())
}

fn get_extension_name() -> String {
    std::env::var("CARGO_PKG_NAME")
        .unwrap_or_else(|_| panic!("CARGO_PKG_NAME environment var is unset or invalid UTF-8"))
        .replace("-", "_")
}

fn get_pgdata_path() -> eyre::Result<PathBuf> {
    let mut target_dir = get_target_dir()?;
    target_dir.push(&format!("pgx-test-data-{}", pg_sys::get_pg_major_version_num()));
    Ok(target_dir)
}

pub(crate) fn get_pg_dbname() -> &'static str {
    "pgx_tests"
}

pub(crate) fn get_pg_user() -> String {
    std::env::var("USER")
        .unwrap_or_else(|_| panic!("USER environment var is unset or invalid UTF-8"))
}

pub fn get_named_capture(
    regex: &regex::Regex,
    name: &'static str,
    against: &str,
) -> Option<String> {
    match regex.captures(against) {
        Some(cap) => Some(cap[name].to_string()),
        None => None,
    }
}
