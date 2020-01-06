use std::process::{Command, Stdio};

use lazy_static::*;
use std::sync::Mutex;

use colored::*;
use pg_bridge::*;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

mod tests;

struct SetupState {
    pub installed: bool,
}

lazy_static! {
    static ref TEST_MUTEX: Mutex<SetupState> = Mutex::new(SetupState { installed: false });
    static ref SHUTDOWN_HOOKS: Mutex<Vec<Box<dyn Fn() + Send>>> = Mutex::new(Vec::new());
}
const TEST_DB_NAME: &str = "pg_bridge_tests";

fn register_shutdown_hook() {
    extern "C" fn run_shutdown_hooks() {
        for func in SHUTDOWN_HOOKS.lock().unwrap().iter() {
            func();
        }
    }
    shutdown_hooks::add_shutdown_hook(run_shutdown_hooks);
}

fn add_shutdown_hook<F: Fn()>(func: F)
where
    F: Send + 'static,
{
    SHUTDOWN_HOOKS.lock().unwrap().push(Box::new(func));
}

pub fn run_test<F: FnOnce(pg_sys::FunctionCallInfo) -> pg_sys::Datum>(_test_func: F) {
    match TEST_MUTEX.lock() {
        Ok(mut state) => {
            if !state.installed {
                register_shutdown_hook();
                install_extension();
                initdb();
                start_pg();
                dropdb();
                createdb();
                state.installed = true;
            }
        }
        Err(_) => panic!("failed due to poison error"),
    }

    println!("Calling: {}", std::any::type_name::<F>());
}

fn install_extension() {
    let mut command = Command::new("cargo-pgx")
        .arg("pgx")
        .arg("install")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .env("PATH", get_pgbin_envpath())
        .spawn()
        .unwrap();

    let status = command.wait().unwrap();
    if !status.success() {
        panic!("failed to install extension");
    }
}

fn initdb() {
    let pgdata = get_pgdata_path();

    if pgdata.is_dir() {
        // we've already run initdb
        return;
    }

    let status = Command::new("initdb")
        .arg("-D")
        .arg(pgdata.to_str().unwrap())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .env("PATH", get_pgbin_envpath())
        .status()
        .unwrap();

    if !status.success() {
        panic!("initdb failed");
    }
}

fn start_pg() {
    let mut child = Command::new("postmaster")
        .arg("-D")
        .arg(get_pgdata_path().to_str().unwrap())
        .arg("-h")
        .arg(get_pg_host())
        .arg("-p")
        .arg(get_pg_port())
        .arg("-i")
        .stdout(Stdio::inherit())
        .stderr(Stdio::piped())
        .env("PATH", get_pgbin_envpath())
        .spawn()
        .expect("postmaster didn't spawn");

    // monitor the processes stderr in the background
    // and send notify the main thread when it's ready to start
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let pid = child.id();

        // wait for the database to say its ready to start up
        let reader = BufReader::new(
            child
                .stderr
                .take()
                .expect("couldn't take postmaster stdout"),
        );
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    if line.contains("database system is ready to accept connections") {
                        // Postgres says it's ready to go
                        eprintln!("{}", line.bold().blue());
                        sender.send(pid).unwrap();
                        break;
                    } else if !line.contains("LOG: ") {
                        // detected some unexpected output
                        eprintln!("{}", line.bold().red());
                    }
                }
                Err(e) => panic!(e),
            }
        }

        match child.try_wait() {
            Ok(status) => {
                if let Some(_status) = status {
                    // we exited normally
                }
            }
            Err(e) => panic!(e),
        }
    });

    // wait for Postgres to indicate it's ready to accept connection
    let pgpid = receiver.recv().unwrap();

    // add a shutdown hook so we can terminate it when the test framework exits
    add_shutdown_hook(move || unsafe {
        libc::printf(
            std::ffi::CString::new("Stopping Postgres\n\n".bold().blue().to_string())
                .unwrap()
                .as_ptr(),
        );
        libc::kill(pgpid as libc::pid_t, libc::SIGTERM);
    });
}

fn dropdb() {
    let output = Command::new("dropdb")
        .arg("-h")
        .arg(get_pg_host())
        .arg("-p")
        .arg(get_pg_port())
        .arg(TEST_DB_NAME)
        .env("PATH", get_pgbin_envpath())
        .output()
        .unwrap();

    if !output.status.success() {
        // maybe the database didn't exist, and if so that's okay
        let stderr = String::from_utf8_lossy(output.stderr.as_slice());
        if !stderr.contains(&format!(
            "ERROR:  database \"{}\" does not exist",
            TEST_DB_NAME
        )) {
            // got some error we didn't expect
            eprintln!("{}", String::from_utf8_lossy(output.stdout.as_slice()));
            eprintln!("{}", stderr);
            panic!("failed to drop test database");
        }
    }
}

fn createdb() {
    let status = Command::new("createdb")
        .arg("-h")
        .arg(get_pg_host())
        .arg("-p")
        .arg(get_pg_port())
        .arg(TEST_DB_NAME)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .env("PATH", get_pgbin_envpath())
        .status()
        .unwrap();

    if !status.success() {
        panic!("failed to create testing database");
    }
}

fn get_pg_path() -> String {
    let package_name = std::env::var("CARGO_PKG_NAME")
        .unwrap_or_else(|_| panic!("CARGO_PKG_NAME is not an envvar"))
        .replace("-", "_");

    format!(
        "/tmp/cargo-pgx-build-artifacts/{}/REL_{}_STABLE/install/",
        package_name,
        pg_sys::get_pg_major_version_string(),
    )
}

fn get_pgbin_envpath() -> String {
    format!("{}/bin:{}", get_pg_path(), std::env::var("PATH").unwrap())
}

fn get_pgdata_path() -> PathBuf {
    PathBuf::from(format!("{}/data", get_pg_path()))
}

fn get_pg_host() -> String {
    "localhost".to_string()
}

fn get_pg_port() -> String {
    format!("88{}", pg_sys::get_pg_major_version_string())
}
