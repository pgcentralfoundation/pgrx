#!/usr/bin/env python3
"""Build test binaries with one `cargo test --no-run`, then run them in parallel.

`cargo test` runs one test binary at a time. Each pg_test binary starts its own Postgres
server on its own port and data directory, so several can run at once.

The runner gives each binary the working directory and package variables that `cargo test`
would. The pg_test harness reads Cargo's arguments from the `cargo test` process that started
it, so binaries started here take the features to install their extension with from
`PGRX_FEATURES` and `PGRX_NO_DEFAULT_FEATURES` instead.

Usage:
    parallel_tests.py [options] -- CARGO_TEST_ARGS...
"""

from __future__ import annotations

import argparse
import concurrent.futures
import json
import os
import shutil
import subprocess
import sys
import threading
import time
from dataclasses import dataclass
from pathlib import Path
from typing import Callable

PRINT_LOCK = threading.Lock()


def log(message: str) -> None:
    with PRINT_LOCK:
        print(message, flush=True)


@dataclass
class Binary:
    package: dict
    target: dict
    executable: str

    @property
    def name(self) -> str:
        return f"{self.package['name']} ({self.target['kind'][0]} {self.target['name']})"

    @property
    def directory(self) -> Path:
        return Path(self.package["manifest_path"]).parent


@dataclass
class Result:
    binary: Binary
    status: int
    seconds: float


def metadata() -> dict:
    output = subprocess.check_output(
        ["cargo", "metadata", "--no-deps", "--format-version", "1", "--offline"], text=True
    )
    return json.loads(output)


def build(cargo_args: list[str], env: dict[str, str] | None = None) -> list[Binary]:
    """Build the test targets of `cargo test CARGO_ARGS` and return their executables."""
    command = ["cargo", "test", "--no-run", "--message-format=json-render-diagnostics", *cargo_args]
    log(f"+ {' '.join(command)}")
    started = time.monotonic()
    process = subprocess.Popen(command, stdout=subprocess.PIPE, text=True, env=env)
    artifacts = []
    assert process.stdout is not None
    for line in process.stdout:
        if line.startswith("{"):
            message = json.loads(line)
            if (
                message.get("reason") == "compiler-artifact"
                and message.get("executable")
                and message["profile"]["test"]
            ):
                artifacts.append(message)
    if process.wait() != 0:
        raise SystemExit(f"`cargo test --no-run` failed with status {process.returncode}")
    packages = {package["id"]: package for package in metadata()["packages"]}
    binaries = [
        Binary(packages[artifact["package_id"]], artifact["target"], artifact["executable"])
        for artifact in artifacts
    ]
    log(f"built {len(binaries)} test binaries in {time.monotonic() - started:.0f}s")
    return binaries


def cargo_env(binary: Binary) -> dict[str, str]:
    """The variables `cargo test` sets for a test binary."""
    package = binary.package
    version = package["version"]
    core, _, pre = version.partition("-")
    major, minor, patch = (core.split(".") + ["0", "0"])[:3]
    env = {
        "CARGO_MANIFEST_DIR": str(binary.directory),
        "CARGO_MANIFEST_PATH": package["manifest_path"],
        "CARGO_PKG_NAME": package["name"],
        "CARGO_PKG_VERSION": version,
        "CARGO_PKG_VERSION_MAJOR": major,
        "CARGO_PKG_VERSION_MINOR": minor,
        "CARGO_PKG_VERSION_PATCH": patch,
        "CARGO_PKG_VERSION_PRE": pre,
        "CARGO_PKG_AUTHORS": ":".join(package.get("authors") or []),
        "CARGO_PKG_DESCRIPTION": package.get("description") or "",
        "CARGO_PKG_HOMEPAGE": package.get("homepage") or "",
        "CARGO_PKG_REPOSITORY": package.get("repository") or "",
        "CARGO_PKG_LICENSE": package.get("license") or "",
        "CARGO_PKG_LICENSE_FILE": package.get("license_file") or "",
        "CARGO_PKG_RUST_VERSION": package.get("rust_version") or "",
        "CARGO_PKG_README": package.get("readme") or "",
        "CARGO_CRATE_NAME": binary.target["name"].replace("-", "_"),
        "CARGO": os.environ.get("CARGO") or shutil.which("cargo") or "cargo",
    }
    if sys.platform == "win32":
        variable = "PATH"
    elif sys.platform == "darwin":
        variable = "DYLD_FALLBACK_LIBRARY_PATH"
    else:
        variable = "LD_LIBRARY_PATH"
    deps = str(Path(binary.executable).parent)
    env[variable] = os.pathsep.join(filter(None, [deps, os.environ.get(variable)]))
    return env


def run(
    binaries: list[Binary],
    environment: Callable[[Binary], dict[str, str]],
    jobs: int,
    test_args: list[str] | None = None,
) -> list[Result]:
    """Run the binaries, `jobs` at a time, printing each one's output as it finishes."""

    def one(binary: Binary) -> tuple[Binary, int, float, str]:
        started = time.monotonic()
        process = subprocess.run(
            [binary.executable, *(test_args or [])],
            cwd=binary.directory,
            env=environment(binary),
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
            errors="replace",
        )
        return binary, process.returncode, time.monotonic() - started, process.stdout

    log(f"running {len(binaries)} test binaries, {jobs} at a time")
    started = time.monotonic()
    results = []
    with concurrent.futures.ThreadPoolExecutor(max_workers=jobs) as pool:
        for future in concurrent.futures.as_completed([pool.submit(one, b) for b in binaries]):
            binary, status, seconds, output = future.result()
            results.append(Result(binary, status, seconds))
            verdict = "ok" if status == 0 else f"FAILED with status {status}"
            with PRINT_LOCK:
                print(f"::group::{binary.name}: {verdict} in {seconds:.1f}s", flush=True)
                print(output.rstrip(), flush=True)
                print("::endgroup::", flush=True)
                if status != 0:
                    # Repeat a failure's output outside the collapsed group.
                    print(f"::error title=Test binary failed::{binary.name}", flush=True)
                    print(output.rstrip(), flush=True)
    log(f"ran {len(results)} test binaries in {time.monotonic() - started:.0f}s; the slowest:")
    for result in sorted(results, key=lambda r: -r.seconds)[:8]:
        log(f"  {result.seconds:6.1f}s  {result.binary.name}")
    return results


def report(results: list[Result]) -> int:
    failed = [result.binary.name for result in results if result.status != 0]
    for name in failed:
        log(f"FAILED: {name}")
    return 1 if failed else 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--jobs", type=int, default=os.cpu_count() or 2, help="binaries to run at once")
    parser.add_argument("--no-run", action="store_true", help="only build the test binaries")
    parser.add_argument("--test-args", default="", help="arguments for every test binary")
    parser.add_argument("cargo_args", nargs=argparse.REMAINDER, help="arguments for `cargo test`")
    args = parser.parse_args()
    cargo_args = args.cargo_args[1:] if args.cargo_args[:1] == ["--"] else args.cargo_args

    binaries = build(cargo_args)
    if args.no_run:
        return 0

    def environment(binary: Binary) -> dict[str, str]:
        return {**os.environ, **cargo_env(binary)}

    return report(run(binaries, environment, args.jobs, args.test_args.split()))


if __name__ == "__main__":
    sys.exit(main())
