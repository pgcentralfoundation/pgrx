#!/usr/bin/env python3
"""Test the packages that need PostgreSQL against one configured PostgreSQL version.

The pg_test harness installs each extension with `cargo pgrx install`, which builds the
extension with only its own package's features. pgrx-unit-tests' build mostly matches the
workspace's test build and reuses it, but the examples' builds rebuild the bindings and pgrx.
So this script builds the test binaries in ./target and, at the same time, cargo-pgrx and
one example from each group of examples with the same dependencies in ./target-ext. Then it
runs the test binaries in parallel.

Usage:
    test_pg.py --pg VERSION [--run PACKAGES] [--cli-checks]
"""

from __future__ import annotations

import argparse
import os
import subprocess
import sys
import threading
import time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
import parallel_tests as pt  # noqa: E402

ROOT = Path(__file__).resolve().parents[2]
EXE = ".exe" if sys.platform == "win32" else ""
TARGET = ROOT / "target"
EXT_TARGET = ROOT / "target-ext"
CARGO_PGRX = EXT_TARGET / "debug" / f"cargo-pgrx{EXE}"

# Features beyond `pgNN` that a package's tests build and install with.
EXTRA_FEATURES = {
    "pgrx-unit-tests": ["cshim", "proptest"],
}

# The packages whose extension builds share the test build's target directory.
SHARES_TEST_TARGET = {"pgrx-unit-tests"}

# Start these binaries first: they run longest.
FIRST = ["pgrx-unit-tests", "pgrx-pg-sys"]


def pg_config(version: int) -> str:
    home = Path(os.environ["PGRX_HOME"])
    for line in (home / "config.toml").read_text().splitlines():
        key, _, value = line.partition("=")
        if key.strip() == f"pg{version}":
            return value.strip().strip("'\"")
    raise SystemExit(f"PGRX_HOME has no pg{version}")


def install_features(package: str, version: int) -> str:
    return " ".join([f"pg{version}", *EXTRA_FEATURES.get(package, [])])


def target_dir(package: str) -> Path:
    return TARGET if package in SHARES_TEST_TARGET else EXT_TARGET


def install(package: dict, version: int) -> None:
    """Install an extension exactly as the pg_test harness will, so its build is reused."""
    name = package["name"]
    started = time.monotonic()
    command = [
        str(CARGO_PGRX), "pgrx", "install", "--test",
        "--pg-config", pg_config(version),
        "--manifest-path", package["manifest_path"],
        "--features", f"{install_features(name, version)} pg_test",
        "--no-default-features",
    ]
    env = {**os.environ, "CARGO_TARGET_DIR": str(target_dir(name))}
    result = subprocess.run(command, env=env, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE, text=True)
    if result.returncode != 0:
        # The harness repeats the install and reports its error with the failing tests.
        pt.log(f"prebuild: installing {name} failed:\n{result.stderr[-4000:]}")
    pt.log(f"prebuild: installed {name} in {time.monotonic() - started:.0f}s")


def dependency_groups(packages: list[dict]) -> list[dict]:
    """One package from each group whose extension builds resolve the same dependencies."""
    groups: dict[tuple, list[dict]] = {}
    for package in packages:
        enabled = set(package["features"].get("pg_test", []))
        signature = tuple(sorted(
            (dep["name"], dep["kind"] or "normal", tuple(sorted(dep["features"])), dep["uses_default_features"])
            for dep in package["dependencies"]
            if dep["kind"] != "dev"
            and (not dep["optional"] or f"dep:{dep['name']}" in enabled or dep["name"] in enabled)
        ))
        groups.setdefault(signature, []).append(package)
    # The largest group first: most binaries wait on its build.
    return [group[0] for group in sorted(groups.values(), key=len, reverse=True)]


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--pg", type=int, required=True, help="PostgreSQL major version")
    parser.add_argument("--run", default="", help="run only these packages' tests, separated by spaces")
    parser.add_argument("--jobs", type=int, default=os.cpu_count() or 2)
    parser.add_argument("--cli-checks", action="store_true", help="also run the cargo-pgrx checks on examples")
    args = parser.parse_args()
    version = args.pg

    meta = pt.metadata()
    pg_packages = [p for p in meta["packages"] if f"pg{version}" in p["features"]]
    wanted = set(args.run.split()) or {p["name"] for p in pg_packages}
    extensions = [p for p in pg_packages if p["name"] in wanted and "pg_test" in p["features"]]

    cargo_pgrx_ready = threading.Event()
    failures: list[str] = []

    def prebuild_examples() -> None:
        started = time.monotonic()
        build = ["cargo", "build", "--locked", "-p", "cargo-pgrx", "--message-format=short"]
        ok = subprocess.run(build, env={**os.environ, "CARGO_TARGET_DIR": str(EXT_TARGET)}).returncode == 0
        if not ok:
            failures.append("cargo-pgrx")
        cargo_pgrx_ready.set()
        if not ok:
            return
        pt.log(f"prebuild: built cargo-pgrx in {time.monotonic() - started:.0f}s")
        for package in dependency_groups([p for p in extensions if p["name"] not in SHARES_TEST_TARGET]):
            install(package, version)

    examples = threading.Thread(target=prebuild_examples)
    examples.start()

    # The test build. Every PG package builds, whatever runs, so the build matches the cache.
    features = [f"pg{version}"] + [f"{n}/{f}" for n, extra in EXTRA_FEATURES.items() for f in extra]
    cargo_args = ["--locked", "--features", " ".join(features), "--no-default-features"]
    for package in pg_packages:
        cargo_args += ["-p", package["name"]]
    binaries = pt.build(cargo_args)

    # These extensions' builds reuse the test build, so they go right after it.
    cargo_pgrx_ready.wait()
    if "cargo-pgrx" not in failures:
        for package in extensions:
            if package["name"] in SHARES_TEST_TARGET:
                install(package, version)
    examples.join()
    if failures:
        raise SystemExit(f"building {', '.join(failures)} failed")

    binaries = [b for b in binaries if b.package["name"] in wanted]
    first = {name: index for index, name in enumerate(FIRST)}
    binaries.sort(key=lambda b: (first.get(b.package["name"], len(first)), b.name))

    def environment(binary: pt.Binary) -> dict[str, str]:
        name = binary.package["name"]
        env = {**os.environ, **pt.cargo_env(binary)}
        env["CARGO_TARGET_DIR"] = str(target_dir(name))
        env["CARGO_PGRX"] = str(CARGO_PGRX)
        env["PGRX_FEATURES"] = install_features(name, version)
        env["PGRX_NO_DEFAULT_FEATURES"] = "true"
        return env

    status = pt.report(pt.run(binaries, environment, args.jobs))

    # Doctests compile against the libraries that the test build made. Cargo skips the
    # packages without a library that doctests can use.
    doctests = ["cargo", "test", "--doc", *cargo_args]
    pt.log(f"+ {' '.join(doctests)}")
    status |= subprocess.run(doctests).returncode

    if args.cli_checks:
        status |= cli_checks(version, wanted)
    return 1 if status else 0


def cli_checks(version: int, wanted: set[str]) -> int:
    """Exercise `cargo pgrx regress`, `migrate check` and `schema` on the examples that cover them."""
    env = {**os.environ, "CARGO_TARGET_DIR": str(EXT_TARGET)}
    checks = {
        "range": [["regress", f"pg{version}"]],
        "migration_fragments": [["migrate", "check", "--deny-mixed-objects"], ["regress", f"pg{version}"]],
        "versioned_custom_libname_so": [["schema", f"pg{version}"]],
    }
    status = 0
    for example, commands in checks.items():
        if example not in wanted:
            continue
        manifest = ROOT / "pgrx-examples" / example / "Cargo.toml"
        for command in commands:
            full = [str(CARGO_PGRX), "pgrx", *command, "--manifest-path", str(manifest)]
            pt.log(f"+ {' '.join(full)}")
            status |= subprocess.run(full, env=env, cwd=manifest.parent).returncode
    return status


if __name__ == "__main__":
    sys.exit(main())
