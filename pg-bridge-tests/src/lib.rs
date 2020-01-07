use std::process::{Command, Stdio};

use lazy_static::*;
use std::sync::Mutex;

use colored::*;
use pg_bridge::*;
use postgres::error::DbError;
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

pub use colored;
#[macro_export]
macro_rules! testmsg {
    ($($arg:tt)*) => (
        std::thread::spawn(move || {
            println!(
                "{}",
                format!($($arg)*).cyan()
            )
        }).join().unwrap();
    )
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
                create_extension();

                state.installed = true;
            }
        }
        Err(_) => panic!("failed due to poison error"),
    }

    let funcname = std::any::type_name::<F>();
    let funcname = &funcname[funcname.rfind(':').unwrap() + 1..];
    let funcname = funcname.trim_end_matches("_wrapper");

    if let Err(e) = client().simple_query(&format!("SELECT {}();", funcname)) {
        let cause = e.into_source();
        match cause {
            Some(e) => {
                if let Some(dberror) = e.downcast_ref::<DbError>() {
                    panic!("{}", dberror.message());
                } else {
                    panic!(e)
                }
            }
            None => {}
        }
    }
}

pub fn client() -> postgres::Client {
    postgres::Config::new()
        .host(&get_pg_host())
        .port(get_pg_port())
        .user(&get_pg_user())
        .dbname(&get_pg_dbname())
        .connect(postgres::NoTls)
        .unwrap()
}

fn install_extension() {
    let mut command = Command::new("cargo-pgx")
        .arg("pgx")
        .arg("install")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .env("PATH", get_pgbin_envpath())
        .env("PGX_TEST_MODE", "1")
        .env(
            "PGX_BUILD_FLAGS",
            format!(
                "--features pg{} --no-default-features",
                pg_sys::get_pg_major_version_string().to_string()
            ),
        )
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
        .arg(get_pg_port().to_string())
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
                    } else {
                        eprintln!("{}", line.bold().purple());
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
        .arg(get_pg_port().to_string())
        .arg(get_pg_dbname())
        .env("PATH", get_pgbin_envpath())
        .output()
        .unwrap();

    if !output.status.success() {
        // maybe the database didn't exist, and if so that's okay
        let stderr = String::from_utf8_lossy(output.stderr.as_slice());
        if !stderr.contains(&format!(
            "ERROR:  database \"{}\" does not exist",
            get_pg_dbname()
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
        .arg(get_pg_port().to_string())
        .arg(get_pg_dbname())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .env("PATH", get_pgbin_envpath())
        .status()
        .unwrap();

    if !status.success() {
        panic!("failed to create testing database");
    }
}

fn create_extension() {
    client()
        .simple_query(&format!("CREATE EXTENSION {};", get_extension_name()))
        .unwrap();
}

fn get_extension_name() -> String {
    std::env::var("CARGO_PKG_NAME")
        .unwrap_or_else(|_| panic!("CARGO_PKG_NAME is not an envvar"))
        .replace("-", "_")
}

fn get_pg_path() -> String {
    let package_name = get_extension_name();

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

fn get_pg_port() -> u16 {
    8800 + pg_sys::get_pg_major_version_num()
}

fn get_pg_dbname() -> String {
    "pg_bridge_tests".to_string()
}

fn get_pg_user() -> String {
    std::env::var("USER").unwrap_or_else(|_| panic!("USER is not an envvar"))
}
