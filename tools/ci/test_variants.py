#!/usr/bin/env python3
"""Test the PG build variants that only Linux CI covers, for one PostgreSQL version.

- pgrx-unit-tests without the C shim
- custom_types without schema generation, which compiles its tests out
- pgrx-unit-tests checked for aarch64 against target headers fetched for the cross build

The three builds use separate target directories so they overlap.

Usage:
    test_variants.py --pg VERSION
"""

from __future__ import annotations

import argparse
import os
import subprocess
import sys
import threading
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
import parallel_tests as pt  # noqa: E402
import test_pg  # noqa: E402

ROOT = test_pg.ROOT


def run_all(name: str, commands: list[list[str]], target: Path, failures: list[str]) -> None:
    env = {**os.environ, "CARGO_TARGET_DIR": str(target)}
    for command in commands:
        pt.log(f"+ [{name}] {' '.join(command)}")
        if subprocess.run(command, env=env, cwd=ROOT).returncode != 0:
            failures.append(name)
            return


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--pg", type=int, required=True, help="PostgreSQL major version")
    args = parser.parse_args()
    version = args.pg
    failures: list[str] = []

    cross = threading.Thread(target=run_all, args=(
        "aarch64",
        [["cargo", "check", "--locked", "--tests", "--features", f"pg{version}",
          "--package", "pgrx-unit-tests", "--target", "aarch64-unknown-linux-gnu"]],
        ROOT / "target-cross",
        failures,
    ))
    # cargo-pgrx installs pgrx-unit-tests for its tests. custom_types builds only.
    tools = threading.Thread(target=run_all, args=(
        "cargo-pgrx and no-schema-generation",
        [["cargo", "build", "--locked", "-p", "cargo-pgrx"],
         ["cargo", "test", "--locked", "--no-run",
          "--manifest-path", str(ROOT / "pgrx-examples/custom_types/Cargo.toml"),
          "--features", f"pg{version} no-schema-generation", "--no-default-features"]],
        test_pg.EXT_TARGET,
        failures,
    ))
    cross.start()
    tools.start()

    # pgrx-unit-tests without the C shim: its tests and its extension build in ./target.
    test_pg.EXTRA_FEATURES["pgrx-unit-tests"] = ["proptest"]
    features = test_pg.install_features("pgrx-unit-tests", version)
    binaries = pt.build(["--locked", "--features", features, "--no-default-features", "-p", "pgrx-unit-tests"])
    tools.join()

    status = 0
    if "cargo-pgrx and no-schema-generation" not in failures:
        unit_tests = next(p for p in pt.metadata()["packages"] if p["name"] == "pgrx-unit-tests")
        test_pg.install(unit_tests, version)

        def environment(binary: pt.Binary) -> dict[str, str]:
            env = {**os.environ, **pt.cargo_env(binary)}
            env["CARGO_TARGET_DIR"] = str(test_pg.TARGET)
            env["CARGO_PGRX"] = str(test_pg.CARGO_PGRX)
            env["PGRX_FEATURES"] = features
            env["PGRX_NO_DEFAULT_FEATURES"] = "true"
            return env

        status = pt.report(pt.run(binaries, environment, os.cpu_count() or 2))

    cross.join()
    for failure in failures:
        pt.log(f"FAILED: {failure}")
    return 1 if failures or status else 0


if __name__ == "__main__":
    sys.exit(main())
