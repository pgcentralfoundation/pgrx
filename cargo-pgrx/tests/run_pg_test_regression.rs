//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

#![cfg(unix)]

use pgrx_pg_config::{PgConfigSelector, Pgrx};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::time::{Duration, Instant};

fn cargo_pgrx_bin() -> &'static str {
    env!("CARGO_BIN_EXE_cargo-pgrx")
}

fn workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().expect("cargo-pgrx lives under the workspace")
}

fn unit_tests_manifest_path() -> PathBuf {
    workspace_root().join("pgrx-unit-tests").join("Cargo.toml")
}

fn preferred_pg_config() -> Option<(String, u16, PathBuf)> {
    let pgrx = match Pgrx::from_config() {
        Ok(pgrx) => pgrx,
        Err(err) => {
            eprintln!("skipping run_pg_test_regression: could not load pgrx config: {err}");
            return None;
        }
    };

    let mut configs = pgrx
        .iter(PgConfigSelector::All)
        .filter_map(Result::ok)
        .filter_map(|pg_config| Some((pg_config.major_version().ok()?, pg_config.path()?)))
        .collect::<Vec<_>>();

    if configs.is_empty() {
        eprintln!("skipping run_pg_test_regression: no configured pg_config entries");
        return None;
    }

    // Prefer pg18 (newest in-tree), fall back to highest available.
    configs.sort_by_key(|(major, _)| *major);
    let preferred = configs
        .iter()
        .position(|(major, _)| *major == 18)
        .map(|index| configs.swap_remove(index))
        .unwrap_or_else(|| configs.pop().expect("non-empty after is_empty check"));

    let (major, path) = preferred;
    Some((format!("pg{major}"), major, path))
}

struct StopOnDrop(String);

impl Drop for StopOnDrop {
    fn drop(&mut self) {
        let _ = pgrx_stop(&self.0);
    }
}

/// Wrap `cargo pgrx run <pg_feature> <dbname> --manifest-path <unit-tests> [extra…]`. `run` always launches `psql` at the end (via `exec()` on unix), so we feed it an immediate EOF on stdin so psql exits cleanly the moment the connection is up. The pgrx-managed postmaster keeps running after psql exits, which is exactly what we need to assert against.
fn pgrx_run(pg_feature: &str, dbname: &str, extra_args: &[&str]) -> Output {
    let mut cmd = Command::new(cargo_pgrx_bin());
    cmd.current_dir(workspace_root())
        .arg("pgrx")
        .arg("run")
        .arg(pg_feature)
        .arg(dbname)
        .arg("--manifest-path")
        .arg(unit_tests_manifest_path());
    for a in extra_args {
        cmd.arg(a);
    }
    // Closed stdin → psql sees EOF immediately and exits 0.
    cmd.stdin(Stdio::null());
    cmd.output().expect("cargo-pgrx run should launch")
}

fn pgrx_stop(pg_feature: &str) -> Output {
    Command::new(cargo_pgrx_bin())
        .current_dir(workspace_root())
        .arg("pgrx")
        .arg("stop")
        .arg(pg_feature)
        .arg("--manifest-path")
        .arg(unit_tests_manifest_path())
        .output()
        .expect("cargo-pgrx stop should launch")
}

fn assert_run_ok(output: &Output, context: &str) {
    assert!(
        output.status.success(),
        "{context} failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn assert_banner_announces_port(combined_output: &str, port: u16) {
    assert!(
        combined_output.contains("on port") && combined_output.contains(&port.to_string()),
        "start banner did not announce port {port}\noutput:\n{combined_output}"
    );
}

/// Ask the OS for an unused TCP port; release it immediately so postgres can claim it. Standard ephemeral pattern with a small TOCTOU window, but sufficient for serialized test runs.
fn reserve_free_port() -> u16 {
    TcpListener::bind("127.0.0.1:0")
        .expect("OS should hand out an ephemeral port")
        .local_addr()
        .expect("listener should report its local addr")
        .port()
}

/// Poll TCP connect until success or deadline. `pg_ctl start` already waits for the postmaster to signal ready, but a short poll loop avoids flakes on slow hosts.
fn wait_for_tcp(port: u16, timeout: Duration) -> bool {
    let addr = format!("127.0.0.1:{port}").parse().expect("valid socket addr");
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        if TcpStream::connect_timeout(&addr, Duration::from_millis(200)).is_ok() {
            return true;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    false
}

fn assert_tcp_open(port: u16) {
    assert!(
        wait_for_tcp(port, Duration::from_secs(10)),
        "postgres did not accept connections on port {port} within 10s"
    );
}

fn assert_tcp_closed(port: u16) {
    let addr = format!("127.0.0.1:{port}").parse().expect("valid socket addr");
    assert!(
        TcpStream::connect_timeout(&addr, Duration::from_millis(500)).is_err(),
        "expected port {port} to be closed but a connection succeeded"
    );
}

/// Single orchestrator test that walks through every `--port` scenario in strict sequence. We deliberately collapse all scenarios into one `#[test]` function because each scenario starts/stops the same pgrx-managed postgres instance — splitting them across separate `#[test]` functions would let cargo's default parallel test runner race on the datadir lock.
///
/// Each scenario is heavier than the equivalent `start` scenario because `run` also builds and installs the extension on every invocation; that cost is intrinsic to what `run` does and is the reason this is a single orchestrator test rather than a matrix.
#[test]
fn run_port_e2e() {
    let Some((pg_feature, pg_major, _)) = preferred_pg_config() else {
        return;
    };
    let default_port = 28800 + pg_major;
    let dbname = "pgrx_unit_tests";

    let _ = pgrx_stop(&pg_feature);

    // ───────────────────────────────────────────────────────────────────────
    // Scenario 1 — default port baseline (no --port).
    // This is the regression guard for "did --port accidentally change default behavior?" If this fails after the --port refactor, the override field is leaking through PgConfig somewhere on the run path.
    // ───────────────────────────────────────────────────────────────────────
    {
        let guard = StopOnDrop(pg_feature.clone());
        let out = pgrx_run(&pg_feature, dbname, &[]);
        assert_run_ok(&out, "default-port run");
        let combined = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        assert_banner_announces_port(&combined, default_port);
        assert_tcp_open(default_port);

        let stop = pgrx_stop(&pg_feature);
        assert!(stop.status.success(), "stop after default-port run failed");
        assert_tcp_closed(default_port);
        std::mem::forget(guard); // already stopped cleanly
    }

    // ───────────────────────────────────────────────────────────────────────
    // Scenario 2 — happy path: custom port via --port.
    // The core contract of the new flag on `run`: postgres binds to the requested port (not 28800 + major) before psql is execed.
    // ───────────────────────────────────────────────────────────────────────
    let custom_port = reserve_free_port();
    {
        let guard = StopOnDrop(pg_feature.clone());
        let out = pgrx_run(&pg_feature, dbname, &["--port", &custom_port.to_string()]);
        assert_run_ok(&out, &format!("run --port {custom_port}"));
        let combined = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        assert_banner_announces_port(&combined, custom_port);
        assert_tcp_open(custom_port);
        // The default port must NOT be opened by an override-port run.
        assert_tcp_closed(default_port);

        let stop = pgrx_stop(&pg_feature);
        assert!(stop.status.success(), "stop after custom-port run failed");
        std::mem::forget(guard);
    }

    // ───────────────────────────────────────────────────────────────────────
    // Scenario 3 — idempotency: re-invoking `run` with the same --port observes the already-running instance (after the internal stop/start cycle) and still ends up listening on the same port.
    // ───────────────────────────────────────────────────────────────────────
    let idem_port = reserve_free_port();
    {
        let guard = StopOnDrop(pg_feature.clone());

        let first = pgrx_run(&pg_feature, dbname, &["--port", &idem_port.to_string()]);
        assert_run_ok(&first, &format!("first run --port {idem_port}"));
        assert_tcp_open(idem_port);

        let second = pgrx_run(&pg_feature, dbname, &["--port", &idem_port.to_string()]);
        assert_run_ok(&second, &format!("second run --port {idem_port}"));
        assert_tcp_open(idem_port);

        let stop = pgrx_stop(&pg_feature);
        assert!(stop.status.success(), "stop after idempotent run failed");
        std::mem::forget(guard);
    }

    // ───────────────────────────────────────────────────────────────────────
    // Scenario 4 — no state leak across restarts: run on port A, stop, then run on port B and confirm the new port is bound while the old one is not. Catches a regression where a port override would somehow persist beyond the invocation that set it.
    // ───────────────────────────────────────────────────────────────────────
    let port_a = reserve_free_port();
    let port_b = reserve_free_port();

    if port_a != port_b {
        let guard = StopOnDrop(pg_feature.clone());

        let out_a = pgrx_run(&pg_feature, dbname, &["--port", &port_a.to_string()]);
        assert_run_ok(&out_a, &format!("run on port A={port_a}"));
        assert_tcp_open(port_a);
        let stop_a = pgrx_stop(&pg_feature);
        assert!(stop_a.status.success(), "stop after port A failed");
        assert_tcp_closed(port_a);

        let out_b = pgrx_run(&pg_feature, dbname, &["--port", &port_b.to_string()]);
        assert_run_ok(&out_b, &format!("run on port B={port_b}"));
        assert_tcp_open(port_b);
        // Port A must not somehow be reopened by the second run.
        assert_tcp_closed(port_a);

        let stop_b = pgrx_stop(&pg_feature);
        assert!(stop_b.status.success(), "stop after port B failed");
        std::mem::forget(guard);
    } else {
        eprintln!("scenario 4 skipped: OS handed out the same ephemeral port twice");
    }
}
